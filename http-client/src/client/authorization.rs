use qiniu_credential::{Credential, CredentialProvider};
use qiniu_http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Request,
};
use qiniu_upload_token::UploadTokenProvider;
use std::{fmt, io::Error as IOError, result::Result, sync::Arc};
use thiserror::Error;
use url::ParseError as UrlParseError;

/// API 鉴权方式
#[derive(Clone)]
pub struct Authorization {
    inner: AuthorizationInner,
}

#[derive(Clone, Debug)]
enum AuthorizationInner {
    UpToken(Arc<dyn UploadTokenProvider>),
    V1(Arc<dyn CredentialProvider>),
    V2(Arc<dyn CredentialProvider>),
}

impl Authorization {
    #[inline]
    pub fn uptoken(provider: Arc<dyn UploadTokenProvider>) -> Self {
        Self {
            inner: AuthorizationInner::UpToken(provider),
        }
    }

    #[inline]
    pub fn v1(provider: Arc<dyn CredentialProvider>) -> Self {
        Self {
            inner: AuthorizationInner::V1(provider),
        }
    }

    #[inline]
    pub fn v2(provider: Arc<dyn CredentialProvider>) -> Self {
        Self {
            inner: AuthorizationInner::V2(provider),
        }
    }

    /// 使用指定的鉴权方式对 HTTP 请求进行签名
    pub fn sign(&self, request: &mut Request) -> AuthorizationResult<()> {
        let authorization = match &self.inner {
            AuthorizationInner::UpToken(provider) => {
                uptoken_authorization(&provider.to_token_string(&Default::default())?)
            }
            AuthorizationInner::V1(provider) => authorization_v1_for_request(
                provider.get(&Default::default())?.credential(),
                request,
            )?,
            AuthorizationInner::V2(provider) => authorization_v2_for_request(
                provider.get(&Default::default())?.credential(),
                request,
            )?,
        };
        set_authorization(request, HeaderValue::from_str(&authorization).unwrap());
        Ok(())
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    /// 使用指定的鉴权方式对 HTTP 请求进行异步签名
    pub async fn async_sign(&self, request: &mut Request<'_>) -> AuthorizationResult<()> {
        let authorization = match &self.inner {
            AuthorizationInner::UpToken(provider) => {
                uptoken_authorization(&provider.async_to_token_string(&Default::default()).await?)
            }
            AuthorizationInner::V1(provider) => authorization_v1_for_request(
                provider.async_get(&Default::default()).await?.credential(),
                request,
            )?,
            AuthorizationInner::V2(provider) => authorization_v2_for_request(
                provider.async_get(&Default::default()).await?.credential(),
                request,
            )?,
        };
        set_authorization(request, HeaderValue::from_str(&authorization).unwrap());
        Ok(())
    }
}

#[inline]
fn set_authorization(request: &mut Request, authorization: HeaderValue) {
    request.headers_mut().insert(AUTHORIZATION, authorization);
}

#[inline]
fn uptoken_authorization(upload_token: &str) -> String {
    "UpToken ".to_owned() + upload_token
}

#[inline]
fn authorization_v1_for_request(
    credential: &Credential,
    request: &Request,
) -> AuthorizationResult<String> {
    Ok(credential.authorization_v1_for_request(
        request.url(),
        request.headers().get(CONTENT_TYPE),
        request.body(),
    ))
}

#[inline]
fn authorization_v2_for_request(
    credential: &Credential,
    request: &Request,
) -> AuthorizationResult<String> {
    Ok(credential.authorization_v2_for_request(
        request.method(),
        request.url(),
        request.headers(),
        request.body(),
    ))
}

impl From<Arc<dyn UploadTokenProvider>> for Authorization {
    #[inline]
    fn from(provider: Arc<dyn UploadTokenProvider>) -> Self {
        Self::uptoken(provider)
    }
}

impl From<Arc<dyn CredentialProvider>> for Authorization {
    #[inline]
    fn from(provider: Arc<dyn CredentialProvider>) -> Self {
        Self::v2(provider)
    }
}

/// API 鉴权错误
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AuthorizationError {
    /// 获取认证信息或上传凭证错误
    #[error("Get Upload Token or Credential error: {0}")]
    IOError(#[from] IOError),
    /// URL 解析错误
    #[error("Parse URL error: {0}")]
    UrlParseError(#[from] UrlParseError),
}
/// API 鉴权结果
pub type AuthorizationResult<T> = Result<T, AuthorizationError>;

impl fmt::Debug for Authorization {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}