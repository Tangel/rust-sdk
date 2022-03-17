#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    non_ascii_idents,
    indirect_structural_match,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]

use auto_impl::auto_impl;
use dyn_clonable::clonable;
use hmac::{Hmac, Mac, NewMac};
use http::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    method::Method,
    uri::Uri,
};
use mime::{APPLICATION_OCTET_STREAM, APPLICATION_WWW_FORM_URLENCODED};
use once_cell::sync::Lazy;
use qiniu_utils::base64;
use sha1::Sha1;
use std::{
    collections::VecDeque,
    env,
    fmt::{self, Debug},
    io::{copy, Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult},
    mem::take,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
    time::Duration,
};

mod header_name;
use header_name::make_header_name;

mod key;
pub use key::{AccessKey, SecretKey};

pub mod preclude {
    pub use super::CredentialProvider;
}

/// 认证信息
///
/// 返回认证信息的 AccessKey 和 SecretKey
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Credential {
    access_key: AccessKey,
    secret_key: SecretKey,
}

impl Credential {
    /// 创建认证信息
    #[inline]
    pub fn new(access_key: impl Into<AccessKey>, secret_key: impl Into<SecretKey>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }

    /// 获取认证信息的 AccessKey
    #[inline]
    pub fn access_key(&self) -> &AccessKey {
        &self.access_key
    }

    /// 获取认证信息的 SecretKey
    #[inline]
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    /// 同时返回认证信息的 AccessKey 和 SecretKey
    #[inline]
    pub fn into_pair(self) -> (AccessKey, SecretKey) {
        (self.access_key, self.secret_key)
    }

    /// 使用七牛签名算法对数据进行签名
    ///
    /// 参考[管理凭证的签名算法文档](https://developer.qiniu.com/kodo/manual/1201/access-token)
    pub fn sign(&self, data: &[u8]) -> String {
        self.sign_within::<IoError, _>(|hmac| {
            hmac.update(data);
            Ok(())
        })
        .unwrap()
    }

    pub fn sign_reader(&self, reader: &mut dyn Read) -> IoResult<String> {
        self.sign_within(|hmac| copy(reader, hmac).map(|_| ()))
    }

    fn sign_within<E, F: FnOnce(&mut Hmac<Sha1>) -> Result<(), E>>(&self, f: F) -> Result<String, E> {
        let signature = generate_base64ed_hmac_sha1_digest_within(self.secret_key(), f)?;
        Ok(self.access_key().to_string() + ":" + &signature)
    }

    /// 使用七牛签名算法对数据进行签名，并同时给出签名和原数据
    ///
    /// 参考[上传凭证的签名算法文档](https://developer.qiniu.com/kodo/manual/1208/upload-token)
    pub fn sign_with_data(&self, data: &[u8]) -> String {
        let encoded_data = base64::urlsafe(data);
        self.sign(encoded_data.as_bytes()) + ":" + &encoded_data
    }

    /// 使用七牛签名算法 V1 对 HTTP 请求进行签名，返回 Authorization 的值
    pub fn authorization_v1_for_request(&self, url: &Uri, content_type: Option<&HeaderValue>, body: &[u8]) -> String {
        let authorization_token = sign_request_v1(self, url, content_type, body);
        "QBox ".to_owned() + &authorization_token
    }

    pub fn authorization_v1_for_request_with_body_reader(
        &self,
        url: &Uri,
        content_type: Option<&HeaderValue>,
        body: &mut dyn Read,
    ) -> IoResult<String> {
        let authorization_token = sign_request_v1_with_body_reader(self, url, content_type, body)?;
        Ok("QBox ".to_owned() + &authorization_token)
    }

    /// 使用七牛签名算法 V2 对 HTTP 请求进行签名，返回 Authorization 的值
    pub fn authorization_v2_for_request(&self, method: &Method, url: &Uri, headers: &HeaderMap, body: &[u8]) -> String {
        let authorization_token = sign_request_v2(self, method, url, headers, body);
        "Qiniu ".to_owned() + &authorization_token
    }

    pub fn authorization_v2_for_request_with_body_reader(
        &self,
        method: &Method,
        url: &Uri,
        headers: &HeaderMap,
        body: &mut dyn Read,
    ) -> IoResult<String> {
        let authorization_token = sign_request_v2_with_body_reader(self, method, url, headers, body)?;
        Ok("Qiniu ".to_owned() + &authorization_token)
    }

    /// 对对象的下载 URL 签名，可以生成私有存储空间的下载地址
    pub fn sign_download_url(&self, url: Uri, deadline: Duration) -> Uri {
        let deadline = deadline.as_secs().to_string();
        let to_sign = append_query_pairs_to_url(url, &[("e", &deadline)]);
        let signature = self.sign(to_sign.to_string().as_bytes());
        return append_query_pairs_to_url(to_sign, &[("token", &signature)]);

        fn append_query_pairs_to_url(url: Uri, pairs: &[(&str, &str)]) -> Uri {
            let path_string = url.path().to_owned();
            let query_string = url.query().unwrap_or_default().to_owned();
            let mut serializer = form_urlencoded::Serializer::new(query_string);
            for (key, value) in pairs.iter() {
                serializer.append_pair(key, value);
            }
            let query_string = serializer.finish();
            let mut path_and_query = path_string;
            if !query_string.is_empty() {
                path_and_query.push('?');
                path_and_query.push_str(&query_string);
            }
            let parts = url.into_parts();
            let mut builder = Uri::builder();
            if let Some(scheme) = parts.scheme {
                builder = builder.scheme(scheme);
            }
            if let Some(authority) = parts.authority {
                builder = builder.authority(authority);
            }
            builder.path_and_query(&path_and_query).build().unwrap()
        }
    }
}

#[cfg(feature = "async")]
impl Credential {
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    pub async fn sign_async_reader(&self, reader: &mut (dyn AsyncRead + Send + Unpin)) -> IoResult<String> {
        let mut hmac = new_hmac_sha1(self.secret_key());
        copy_async_reader_to_hmac_sha1(&mut hmac, reader).await?;
        Ok(base64ed_hmac_sha1_with_access_key(self.access_key().to_string(), hmac))
    }

    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    pub async fn authorization_v1_for_request_with_async_body_reader(
        &self,
        url: &Uri,
        content_type: Option<&HeaderValue>,
        body: &mut (dyn AsyncRead + Send + Unpin),
    ) -> IoResult<String> {
        let authorization_token = sign_request_v1_with_async_body_reader(self, url, content_type, body).await?;
        Ok("QBox ".to_owned() + &authorization_token)
    }

    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    pub async fn authorization_v2_for_request_with_async_body_reader(
        &self,
        method: &Method,
        url: &Uri,
        headers: &HeaderMap,
        body: &mut (dyn AsyncRead + Send + Unpin),
    ) -> IoResult<String> {
        let authorization_token = sign_request_v2_with_async_body_reader(self, method, url, headers, body).await?;
        Ok("Qiniu ".to_owned() + &authorization_token)
    }
}

fn sign_request_v1(cred: &Credential, url: &Uri, content_type: Option<&HeaderValue>, body: &[u8]) -> String {
    cred.sign_within::<IoError, _>(|hmac| {
        _sign_request_v1_without_body(hmac, url);
        if let Some(content_type) = content_type {
            if !body.is_empty() && will_push_body_v1(content_type) {
                hmac.update(body);
            }
        }
        Ok(())
    })
    .unwrap()
}

fn sign_request_v1_with_body_reader(
    cred: &Credential,
    url: &Uri,
    content_type: Option<&HeaderValue>,
    body: &mut dyn Read,
) -> IoResult<String> {
    cred.sign_within(|hmac| {
        _sign_request_v1_without_body(hmac, url);
        if let Some(content_type) = content_type {
            if will_push_body_v1(content_type) {
                copy(body, hmac)?;
            }
        }
        Ok(())
    })
}

fn _sign_request_v1_without_body(digest: &mut Hmac<Sha1>, url: &Uri) {
    digest.update(url.path().as_bytes());
    if let Some(query) = url.query() {
        if !query.is_empty() {
            digest.update(b"?");
            digest.update(query.as_bytes());
        }
    }
    digest.update(b"\n");
}

fn sign_request_v2(cred: &Credential, method: &Method, url: &Uri, headers: &HeaderMap, body: &[u8]) -> String {
    cred.sign_within::<IoError, _>(|hmac| {
        _sign_request_v2_without_body(hmac, method, url, headers);
        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            if will_push_body_v2(content_type) {
                hmac.update(body);
            }
        }
        Ok(())
    })
    .unwrap()
}

fn sign_request_v2_with_body_reader(
    cred: &Credential,
    method: &Method,
    url: &Uri,
    headers: &HeaderMap,
    body: &mut dyn Read,
) -> IoResult<String> {
    cred.sign_within(|hmac| {
        _sign_request_v2_without_body(hmac, method, url, headers);
        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            if will_push_body_v2(content_type) {
                copy(body, hmac)?;
            }
        }
        Ok(())
    })
}

fn _sign_request_v2_without_body(digest: &mut Hmac<Sha1>, method: &Method, url: &Uri, headers: &HeaderMap) {
    digest.update(method.as_str().as_bytes());
    digest.update(b" ");
    digest.update(url.path().as_bytes());
    if let Some(query) = url.query() {
        if !query.is_empty() {
            digest.update(b"?");
            digest.update(query.as_bytes());
        }
    }
    if let Some(host) = url.host() {
        digest.update(b"\nHost: ");
        digest.update(host.as_bytes());
    }
    if let Some(port) = url.port() {
        digest.update(b":");
        digest.update(port.to_string().as_bytes());
    }
    digest.update(b"\n");

    if let Some(content_type) = headers.get(CONTENT_TYPE) {
        digest.update(b"Content-Type: ");
        digest.update(content_type.as_bytes());
        digest.update(b"\n");
    }
    _sign_data_for_x_qiniu_headers(digest, headers);
    digest.update(b"\n");
    return;

    fn _sign_data_for_x_qiniu_headers(digest: &mut Hmac<Sha1>, headers: &HeaderMap) {
        let mut x_qiniu_headers = headers
            .iter()
            .map(|(key, value)| (make_header_name(key.as_str().into()), value.as_bytes()))
            .filter(|(key, _)| key.len() > "X-Qiniu-".len())
            .filter(|(key, _)| key.starts_with("X-Qiniu-"))
            .collect::<Vec<_>>();
        if x_qiniu_headers.is_empty() {
            return;
        }
        x_qiniu_headers.sort_unstable();
        for (header_key, header_value) in x_qiniu_headers {
            digest.update(header_key.as_bytes());
            digest.update(b": ");
            digest.update(header_value);
            digest.update(b"\n");
        }
    }
}

fn generate_base64ed_hmac_sha1_digest_within<E, F: FnOnce(&mut Hmac<Sha1>) -> Result<(), E>>(
    secret_key: &str,
    f: F,
) -> Result<String, E> {
    let mut hmac = new_hmac_sha1(secret_key);
    f(&mut hmac)?;
    Ok(base64ed_hmac_sha1(hmac))
}

fn new_hmac_sha1(secret_key: &str) -> Hmac<Sha1> {
    Hmac::<Sha1>::new_from_slice(secret_key.as_bytes()).unwrap()
}

fn base64ed_hmac_sha1(hmac: Hmac<Sha1>) -> String {
    base64::urlsafe(&hmac.finalize().into_bytes())
}

#[cfg(feature = "async")]
fn base64ed_hmac_sha1_with_access_key(access_key: String, hmac: Hmac<Sha1>) -> String {
    access_key + ":" + &base64ed_hmac_sha1(hmac)
}

fn will_push_body_v1(content_type: &HeaderValue) -> bool {
    APPLICATION_WWW_FORM_URLENCODED.as_ref() == content_type
}

fn will_push_body_v2(content_type: &HeaderValue) -> bool {
    APPLICATION_OCTET_STREAM.as_ref() != content_type
}

#[cfg(feature = "async")]
mod async_sign {
    use super::*;
    use futures_lite::io::AsyncRead;
    use std::task::{Context, Poll};

    pub(super) async fn sign_request_v1_with_async_body_reader(
        cred: &Credential,
        url: &Uri,
        content_type: Option<&HeaderValue>,
        body: &mut (dyn AsyncRead + Send + Unpin),
    ) -> IoResult<String> {
        let mut hmac = new_hmac_sha1(cred.secret_key());
        _sign_request_v1_without_body(&mut hmac, url);
        if let Some(content_type) = content_type {
            if will_push_body_v1(content_type) {
                copy_async_reader_to_hmac_sha1(&mut hmac, body).await?;
            }
        }
        Ok(base64ed_hmac_sha1_with_access_key(cred.access_key().to_string(), hmac))
    }

    pub(super) async fn sign_request_v2_with_async_body_reader(
        cred: &Credential,
        method: &Method,
        url: &Uri,
        headers: &HeaderMap,
        body: &mut (dyn AsyncRead + Send + Unpin),
    ) -> IoResult<String> {
        let mut hmac = new_hmac_sha1(cred.secret_key());
        _sign_request_v2_without_body(&mut hmac, method, url, headers);
        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            if will_push_body_v2(content_type) {
                copy_async_reader_to_hmac_sha1(&mut hmac, body).await?;
            }
        }
        Ok(base64ed_hmac_sha1_with_access_key(cred.access_key().to_string(), hmac))
    }

    pub(super) async fn copy_async_reader_to_hmac_sha1(
        hmac: &mut Hmac<Sha1>,
        reader: &mut (dyn AsyncRead + Send + Unpin),
    ) -> IoResult<u64> {
        use futures_lite::io::{copy as async_io_copy, AsyncWrite};

        struct AsyncHmacWriter<'a>(&'a mut Hmac<Sha1>);

        impl AsyncWrite for AsyncHmacWriter<'_> {
            #[inline]
            fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<IoResult<usize>> {
                #[allow(unsafe_code)]
                unsafe { self.get_unchecked_mut() }.0.update(buf);
                Poll::Ready(Ok(buf.len()))
            }

            #[inline]
            fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
                Poll::Ready(Ok(()))
            }

            #[inline]
            fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<IoResult<()>> {
                Poll::Ready(Ok(()))
            }
        }

        async_io_copy(reader, &mut AsyncHmacWriter(hmac)).await
    }
}

#[cfg(feature = "async")]
pub use futures_lite::AsyncRead;

#[cfg(feature = "async")]
use {
    async_sign::*,
    std::{future::Future, pin::Pin},
};

#[cfg(feature = "async")]
type AsyncIoResult<'a, T> = Pin<Box<dyn Future<Output = IoResult<T>> + 'a + Send>>;

/// 认证信息提供者
///
/// 为认证信息提供者的实现提供接口支持
#[clonable]
#[auto_impl(&, &mut, Box, Rc, Arc)]
pub trait CredentialProvider: Clone + Debug + Sync + Send {
    /// 返回七牛认证信息
    fn get(&self, opts: &GetOptions) -> IoResult<GotCredential>;

    /// 异步返回七牛认证信息
    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_get<'a>(&'a self, opts: &'a GetOptions) -> AsyncIoResult<'a, GotCredential> {
        Box::pin(async move { self.get(opts) })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GetOptions {}

#[derive(Debug)]
pub struct GotCredential(Credential);

impl From<GotCredential> for Credential {
    #[inline]
    fn from(result: GotCredential) -> Self {
        result.0
    }
}

impl From<Credential> for GotCredential {
    #[inline]
    fn from(credential: Credential) -> Self {
        Self(credential)
    }
}

impl GotCredential {
    #[inline]
    pub fn credential(&self) -> &Credential {
        &self.0
    }

    #[inline]
    pub fn credential_mut(&mut self) -> &mut Credential {
        &mut self.0
    }

    #[inline]
    pub fn into_credential(self) -> Credential {
        self.0
    }
}

impl Deref for GotCredential {
    type Target = Credential;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GotCredential {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CredentialProvider for Credential {
    #[inline]
    fn get(&self, _opts: &GetOptions) -> IoResult<GotCredential> {
        Ok(self.to_owned().into())
    }
}

/// 全局认证信息提供者，可以将认证信息配置在全局变量中。任何全局认证信息提供者实例都可以设置和访问全局认证信息。
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlobalCredentialProvider;

static GLOBAL_CREDENTIAL: Lazy<RwLock<Option<Credential>>> = Lazy::new(|| RwLock::new(None));

impl GlobalCredentialProvider {
    /// 配置全局认证信息
    #[inline]
    pub fn setup(credential: Credential) {
        let mut global_credential = GLOBAL_CREDENTIAL.write().unwrap();
        *global_credential = Some(credential);
    }

    /// 清空全局认证信息
    #[inline]
    pub fn clear() {
        let mut global_credential = GLOBAL_CREDENTIAL.write().unwrap();
        *global_credential = None;
    }
}

impl CredentialProvider for GlobalCredentialProvider {
    #[inline]
    fn get(&self, _opts: &GetOptions) -> IoResult<GotCredential> {
        if let Some(credential) = GLOBAL_CREDENTIAL.read().unwrap().as_ref() {
            Ok(credential.to_owned().into())
        } else {
            Err(IoError::new(
                IoErrorKind::Other,
                "GlobalCredentialProvider is not setuped, please call GlobalCredentialProvider::setup() to do it",
            ))
        }
    }
}

impl Debug for GlobalCredentialProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("GlobalCredentialProvider");
        d.field("credential", &GLOBAL_CREDENTIAL.read().unwrap());
        d.finish()
    }
}

/// 环境变量认证信息提供者，可以将认证信息配置在环境变量中。
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnvCredentialProvider;

/// 设置七牛 AccessKey 的环境变量
pub const QINIU_ACCESS_KEY_ENV_KEY: &str = "QINIU_ACCESS_KEY";
/// 设置七牛 SecretKey 的环境变量
pub const QINIU_SECRET_KEY_ENV_KEY: &str = "QINIU_SECRET_KEY";

impl EnvCredentialProvider {
    /// 配置环境变量认证信息提供者
    #[inline]
    pub fn setup(credential: &Credential) {
        env::set_var(QINIU_ACCESS_KEY_ENV_KEY, credential.access_key().as_str());
        env::set_var(QINIU_SECRET_KEY_ENV_KEY, credential.secret_key().as_str());
    }
}

impl CredentialProvider for EnvCredentialProvider {
    fn get(&self, _opts: &GetOptions) -> IoResult<GotCredential> {
        match (env::var(QINIU_ACCESS_KEY_ENV_KEY), env::var(QINIU_SECRET_KEY_ENV_KEY)) {
            (Ok(access_key), Ok(secret_key)) if !access_key.is_empty() && !secret_key.is_empty() => {
                Ok(Credential::new(access_key, secret_key).into())
            }
            _ => {
                static ERROR_MESSAGE: Lazy<String> = Lazy::new(|| {
                    format!("EnvCredentialProvider is not setuped, please call EnvCredentialProvider::setup() to do it, or set environment variable `{}` and `{}`", QINIU_ACCESS_KEY_ENV_KEY, QINIU_SECRET_KEY_ENV_KEY)
                });
                Err(IoError::new(IoErrorKind::Other, ERROR_MESSAGE.as_str()))
            }
        }
    }
}

impl Debug for EnvCredentialProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("EnvCredentialProvider");
        if let (Some(access_key), Some(secret_key)) = (
            env::var_os(QINIU_ACCESS_KEY_ENV_KEY),
            env::var_os(QINIU_SECRET_KEY_ENV_KEY),
        ) {
            d.field("access_key", &access_key).field("secret_key", &secret_key);
        }
        d.finish()
    }
}

/// 认证信息串提供者
///
/// 将多个认证信息串联，遍历并找寻第一个可用认证信息
#[derive(Clone, Debug)]
pub struct ChainCredentialsProvider {
    credentials: Arc<[Box<dyn CredentialProvider>]>,
}

impl ChainCredentialsProvider {
    #[inline]
    pub fn builder(credential: impl CredentialProvider + 'static) -> ChainCredentialsProviderBuilder {
        ChainCredentialsProviderBuilder::new(credential)
    }
}

impl CredentialProvider for ChainCredentialsProvider {
    fn get(&self, opts: &GetOptions) -> IoResult<GotCredential> {
        if let Some(credential) = self.credentials.iter().find_map(|c| c.get(opts).ok()) {
            Ok(credential)
        } else {
            Err(IoError::new(IoErrorKind::Other, "All credentials are failed to get"))
        }
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_get<'a>(&'a self, opts: &'a GetOptions) -> AsyncIoResult<'a, GotCredential> {
        Box::pin(async move {
            for provider in self.credentials.iter() {
                if let Ok(credential) = provider.async_get(opts).await {
                    return Ok(credential);
                }
            }
            Err(IoError::new(IoErrorKind::Other, "All credentials are failed to get"))
        })
    }
}

impl Default for ChainCredentialsProvider {
    #[inline]
    fn default() -> Self {
        ChainCredentialsProviderBuilder::new(Box::new(GlobalCredentialProvider))
            .append_credential(Box::new(EnvCredentialProvider))
            .build()
    }
}

impl FromIterator<Box<dyn CredentialProvider>> for ChainCredentialsProvider {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Box<dyn CredentialProvider>>>(iter: T) -> Self {
        ChainCredentialsProviderBuilder::from_iter(iter).build()
    }
}

impl<'a> IntoIterator for &'a ChainCredentialsProvider {
    type Item = &'a Box<dyn CredentialProvider + 'static>;
    type IntoIter = std::slice::Iter<'a, Box<dyn CredentialProvider + 'static>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.credentials.iter()
    }
}

/// 串联认证信息构建器
///
/// 接受多个认证信息提供者并将他们串联成串联认证信息
#[derive(Debug, Clone, Default)]
pub struct ChainCredentialsProviderBuilder {
    credentials: VecDeque<Box<dyn CredentialProvider + 'static>>,
}

impl ChainCredentialsProviderBuilder {
    /// 构建新的串联认证信息构建器
    #[inline]
    pub fn new(credential: impl CredentialProvider + 'static) -> Self {
        let mut builder = Self::default();
        builder.append_credential(credential);
        builder
    }

    /// 将认证信息提供者推送到认证串末端
    #[inline]
    pub fn append_credential(&mut self, credential: impl CredentialProvider + 'static) -> &mut Self {
        self.credentials.push_back(Box::new(credential));
        self
    }

    /// 将认证信息提供者推送到认证串顶端
    #[inline]
    pub fn prepend_credential(&mut self, credential: impl CredentialProvider + 'static) -> &mut Self {
        self.credentials.push_front(Box::new(credential));
        self
    }

    /// 串联认证信息
    #[inline]
    pub fn build(&mut self) -> ChainCredentialsProvider {
        assert!(
            !self.credentials.is_empty(),
            "ChainCredentialsProvider must owns at least one CredentialProvider"
        );
        ChainCredentialsProvider {
            credentials: Vec::from(take(&mut self.credentials)).into_boxed_slice().into(),
        }
    }
}

impl FromIterator<Box<dyn CredentialProvider>> for ChainCredentialsProviderBuilder {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Box<dyn CredentialProvider>>>(iter: T) -> Self {
        ChainCredentialsProviderBuilder {
            credentials: VecDeque::from_iter(iter),
        }
    }
}

impl Extend<Box<dyn CredentialProvider>> for ChainCredentialsProviderBuilder {
    #[inline]
    fn extend<T: IntoIterator<Item = Box<dyn CredentialProvider>>>(&mut self, iter: T) {
        self.credentials.extend(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use http::header::HeaderName;
    use mime::APPLICATION_JSON;
    use std::{io::Cursor, time::Duration};
    use tokio as _;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_sign() -> Result<()> {
        use std::thread::spawn as spawn_thread;

        let credential = get_credential();
        let mut threads = Vec::new();
        {
            let credential = credential.to_owned();
            threads.push(spawn_thread(move || {
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign(b"hello"),
                    "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0="
                );
                assert_eq!(
                    credential
                        .get(&Default::default())
                        .unwrap()
                        .sign_reader(&mut Cursor::new(b"world"))
                        .unwrap(),
                    "abcdefghklmnopq:VjgXt0P_nCxHuaTfiFz-UjDJ1AQ="
                );
            }));
        }
        {
            threads.push(spawn_thread(move || {
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign(b"-test"),
                    "abcdefghklmnopq:vYKRLUoXRlNHfpMEQeewG0zylaw="
                );
                assert_eq!(
                    credential
                        .get(&Default::default())
                        .unwrap()
                        .sign_reader(&mut Cursor::new(b"ba#a-"))
                        .unwrap(),
                    "abcdefghklmnopq:2d_Yr6H1GdTKg3RvMtpHOhi047M="
                );
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
        Ok(())
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_sign_with_data() -> Result<()> {
        use std::thread::spawn as spawn_thread;

        let credential = get_credential();
        let mut threads = Vec::new();
        {
            let credential = credential.to_owned();
            threads.push(spawn_thread(move || {
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign_with_data(b"hello"),
                    "abcdefghklmnopq:BZYt5uVRy1RVt5ZTXbaIt2ROVMA=:aGVsbG8="
                );
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign_with_data(b"world"),
                    "abcdefghklmnopq:Wpe04qzPphiSZb1u6I0nFn6KpZg=:d29ybGQ="
                );
            }));
        }
        {
            threads.push(spawn_thread(move || {
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign_with_data(b"-test"),
                    "abcdefghklmnopq:HlxenSSP_6BbaYNzx1fyeyw8v1Y=:LXRlc3Q="
                );
                assert_eq!(
                    credential.get(&Default::default()).unwrap().sign_with_data(b"ba#a-"),
                    "abcdefghklmnopq:kwzeJrFziPDMO4jv3DKVLDyqud0=:YmEjYS0="
                );
            }));
        }
        threads.into_iter().for_each(|thread| thread.join().unwrap());
        Ok(())
    }

    #[test]
    fn test_authorization_v1_with_body_reader() -> Result<()> {
        let credential = get_credential();
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v1_for_request_with_body_reader(
                    &"http://upload.qiniup.com/".parse()?,
                    None,
                    &mut Cursor::new(b"{\"name\":\"test\"}")
                )?,
            "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\n")
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v1_for_request_with_body_reader(
                    &"http://upload.qiniup.com/".parse()?,
                    Some(&HeaderValue::from_str(APPLICATION_JSON.as_ref())?),
                    &mut Cursor::new(b"{\"name\":\"test\"}")
                )?,
            "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\n")
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v1_for_request_with_body_reader(
                    &"http://upload.qiniup.com/".parse()?,
                    Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\nname=test&language=go")
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v1_for_request_with_body_reader(
                    &"http://upload.qiniup.com/?v=2".parse()?,
                    Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "QBox ".to_owned()
                + &credential
                    .get(&Default::default())?
                    .sign(b"/?v=2\nname=test&language=go")
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v1_for_request_with_body_reader(
                    &"http://upload.qiniup.com/find/sdk?v=2".parse()?,
                    Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "QBox ".to_owned()
                + &credential
                    .get(&Default::default())?
                    .sign(b"/find/sdk?v=2\nname=test&language=go")
        );
        Ok(())
    }

    #[test]
    fn test_authorization_v2_with_body_reader() -> Result<()> {
        let credential = get_global_credential();
        let empty_headers = {
            let mut headers = HeaderMap::new();
            headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
            headers
        };
        let json_headers = {
            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_str(APPLICATION_JSON.as_ref())?);
            headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
            headers.insert(
                HeaderName::from_static("x-qiniu-cxxxx"),
                HeaderValue::from_str("valuec")?,
            );
            headers.insert(
                HeaderName::from_static("x-qiniu-bxxxx"),
                HeaderValue::from_str("valueb")?,
            );
            headers.insert(
                HeaderName::from_static("x-qiniu-axxxx"),
                HeaderValue::from_str("valuea")?,
            );
            headers.insert(HeaderName::from_static("x-qiniu-e"), HeaderValue::from_str("value")?);
            headers.insert(HeaderName::from_static("x-qiniu-"), HeaderValue::from_str("value")?);
            headers.insert(HeaderName::from_static("x-qiniu"), HeaderValue::from_str("value")?);
            headers
        };
        let form_headers = {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?,
            );
            headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
            headers.insert(
                HeaderName::from_static("x-qiniu-cxxxx"),
                HeaderValue::from_str("valuec")?,
            );
            headers.insert(
                HeaderName::from_static("x-qiniu-bxxxx"),
                HeaderValue::from_str("valueb")?,
            );
            headers.insert(
                HeaderName::from_static("x-qiniu-axxxx"),
                HeaderValue::from_str("valuea")?,
            );
            headers.insert(HeaderName::from_static("x-qiniu-e"), HeaderValue::from_str("value")?);
            headers.insert(HeaderName::from_static("x-qiniu-"), HeaderValue::from_str("value")?);
            headers.insert(HeaderName::from_static("x-qiniu"), HeaderValue::from_str("value")?);
            headers
        };
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::GET,
                    &"http://upload.qiniup.com/".parse()?,
                    &json_headers,
                    &mut Cursor::new(b"{\"name\":\"test\"}")
                )?,
            "Qiniu ".to_owned()
                + &credential.get(&Default::default())?.sign(
                    concat!(
                        "GET /\n",
                        "Host: upload.qiniup.com\n",
                        "Content-Type: application/json\n",
                        "X-Qiniu-Axxxx: valuea\n",
                        "X-Qiniu-Bxxxx: valueb\n",
                        "X-Qiniu-Cxxxx: valuec\n",
                        "X-Qiniu-E: value\n\n",
                        "{\"name\":\"test\"}"
                    )
                    .as_bytes()
                )
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::GET,
                    &"http://upload.qiniup.com/".parse()?,
                    &empty_headers,
                    &mut Cursor::new(b"{\"name\":\"test\"}")
                )?,
            "Qiniu ".to_owned()
                + &credential
                    .get(&Default::default())?
                    .sign(concat!("GET /\n", "Host: upload.qiniup.com\n\n").as_bytes())
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::POST,
                    &"http://upload.qiniup.com/".parse()?,
                    &json_headers,
                    &mut Cursor::new(b"{\"name\":\"test\"}")
                )?,
            "Qiniu ".to_owned()
                + &credential.get(&Default::default())?.sign(
                    concat!(
                        "POST /\n",
                        "Host: upload.qiniup.com\n",
                        "Content-Type: application/json\n",
                        "X-Qiniu-Axxxx: valuea\n",
                        "X-Qiniu-Bxxxx: valueb\n",
                        "X-Qiniu-Cxxxx: valuec\n",
                        "X-Qiniu-E: value\n\n",
                        "{\"name\":\"test\"}"
                    )
                    .as_bytes()
                )
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::GET,
                    &"http://upload.qiniup.com/".parse()?,
                    &form_headers,
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "Qiniu ".to_owned()
                + &credential.get(&Default::default())?.sign(
                    concat!(
                        "GET /\n",
                        "Host: upload.qiniup.com\n",
                        "Content-Type: application/x-www-form-urlencoded\n",
                        "X-Qiniu-Axxxx: valuea\n",
                        "X-Qiniu-Bxxxx: valueb\n",
                        "X-Qiniu-Cxxxx: valuec\n",
                        "X-Qiniu-E: value\n\n",
                        "name=test&language=go"
                    )
                    .as_bytes()
                )
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::GET,
                    &"http://upload.qiniup.com/?v=2".parse()?,
                    &form_headers,
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "Qiniu ".to_owned()
                + &credential.get(&Default::default())?.sign(
                    concat!(
                        "GET /?v=2\n",
                        "Host: upload.qiniup.com\n",
                        "Content-Type: application/x-www-form-urlencoded\n",
                        "X-Qiniu-Axxxx: valuea\n",
                        "X-Qiniu-Bxxxx: valueb\n",
                        "X-Qiniu-Cxxxx: valuec\n",
                        "X-Qiniu-E: value\n\n",
                        "name=test&language=go"
                    )
                    .as_bytes()
                )
        );
        assert_eq!(
            credential
                .get(&Default::default())?
                .authorization_v2_for_request_with_body_reader(
                    &Method::GET,
                    &"http://upload.qiniup.com/find/sdk?v=2".parse()?,
                    &form_headers,
                    &mut Cursor::new(b"name=test&language=go")
                )?,
            "Qiniu ".to_owned()
                + &credential.get(&Default::default())?.sign(
                    concat!(
                        "GET /find/sdk?v=2\n",
                        "Host: upload.qiniup.com\n",
                        "Content-Type: application/x-www-form-urlencoded\n",
                        "X-Qiniu-Axxxx: valuea\n",
                        "X-Qiniu-Bxxxx: valueb\n",
                        "X-Qiniu-Cxxxx: valuec\n",
                        "X-Qiniu-E: value\n\n",
                        "name=test&language=go"
                    )
                    .as_bytes()
                )
        );
        Ok(())
    }

    #[test]
    fn test_sign_download_url() -> Result<()> {
        let credential = get_env_credential();
        let url = "http://www.qiniu.com/?go=1".parse()?;
        let url = credential
            .get(&Default::default())?
            .sign_download_url(url, Duration::from_secs(1_234_567_890 + 3600));
        assert_eq!(
            url.to_string(),
            "http://www.qiniu.com/?go=1&e=1234571490&token=abcdefghklmnopq%3AKjQtlGAkEOhSwtFjJfYtYa2-reE%3D",
        );
        Ok(())
    }

    #[test]
    fn test_chain_credentials() -> Result<()> {
        GlobalCredentialProvider::clear();
        let chain_credentials = ChainCredentialsProvider::default();
        env::set_var(QINIU_ACCESS_KEY_ENV_KEY, "TEST2");
        env::set_var(QINIU_SECRET_KEY_ENV_KEY, "test2");
        {
            let cred = chain_credentials.get(&Default::default())?;
            assert_eq!(cred.access_key().as_str(), "TEST2");
        }
        GlobalCredentialProvider::setup(Credential::new("TEST1", "test1"));
        {
            let cred = chain_credentials.get(&Default::default())?;
            assert_eq!(cred.access_key().as_str(), "TEST1");
        }
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_build_empty_chain_credentials() {
        ChainCredentialsProviderBuilder::default().build();
    }

    fn get_credential() -> Credential {
        Credential::new("abcdefghklmnopq", "1234567890")
    }

    fn get_global_credential() -> GlobalCredentialProvider {
        GlobalCredentialProvider::setup(Credential::new("abcdefghklmnopq", "1234567890"));
        GlobalCredentialProvider
    }

    fn get_env_credential() -> EnvCredentialProvider {
        env::set_var(QINIU_ACCESS_KEY_ENV_KEY, "abcdefghklmnopq");
        env::set_var(QINIU_SECRET_KEY_ENV_KEY, "1234567890");
        EnvCredentialProvider
    }

    #[cfg(feature = "async")]
    mod async_test {
        use super::*;
        use futures_lite::io::Cursor;

        #[tokio::test]
        async fn test_sign_async_reader() -> Result<()> {
            let credential = get_credential();
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .sign_async_reader(&mut Cursor::new(b"hello"))
                    .await?,
                "abcdefghklmnopq:b84KVc-LroDiz0ebUANfdzSRxa0="
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .sign_async_reader(&mut Cursor::new(b"world"))
                    .await?,
                "abcdefghklmnopq:VjgXt0P_nCxHuaTfiFz-UjDJ1AQ="
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .sign_async_reader(&mut Cursor::new(b"-test"))
                    .await?,
                "abcdefghklmnopq:vYKRLUoXRlNHfpMEQeewG0zylaw="
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .sign_async_reader(&mut Cursor::new(b"ba#a-"))
                    .await?,
                "abcdefghklmnopq:2d_Yr6H1GdTKg3RvMtpHOhi047M="
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_async_authorization_v1() -> Result<()> {
            let credential = get_credential();
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v1_for_request_with_async_body_reader(
                        &"http://upload.qiniup.com/".parse()?,
                        None,
                        &mut Cursor::new(b"{\"name\":\"test\"}")
                    )
                    .await?,
                "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\n")
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v1_for_request_with_async_body_reader(
                        &"http://upload.qiniup.com/".parse()?,
                        Some(&HeaderValue::from_str(APPLICATION_JSON.as_ref())?),
                        &mut Cursor::new(b"{\"name\":\"test\"}")
                    )
                    .await?,
                "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\n")
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v1_for_request_with_async_body_reader(
                        &"http://upload.qiniup.com/".parse()?,
                        Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "QBox ".to_owned() + &credential.get(&Default::default())?.sign(b"/\nname=test&language=go")
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v1_for_request_with_async_body_reader(
                        &"http://upload.qiniup.com/?v=2".parse()?,
                        Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "QBox ".to_owned()
                    + &credential
                        .get(&Default::default())?
                        .sign(b"/?v=2\nname=test&language=go")
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v1_for_request_with_async_body_reader(
                        &"http://upload.qiniup.com/find/sdk?v=2".parse()?,
                        Some(&HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?),
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "QBox ".to_owned()
                    + &credential
                        .get(&Default::default())?
                        .sign(b"/find/sdk?v=2\nname=test&language=go")
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_async_authorization_v2() -> Result<()> {
            let credential = get_global_credential();
            let empty_headers = {
                let mut headers = HeaderMap::new();
                headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
                headers
            };
            let json_headers = {
                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_str(APPLICATION_JSON.as_ref())?);
                headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
                headers.insert(
                    HeaderName::from_static("x-qiniu-cxxxx"),
                    HeaderValue::from_str("valuec")?,
                );
                headers.insert(
                    HeaderName::from_static("x-qiniu-bxxxx"),
                    HeaderValue::from_str("valueb")?,
                );
                headers.insert(
                    HeaderName::from_static("x-qiniu-axxxx"),
                    HeaderValue::from_str("valuea")?,
                );
                headers.insert(HeaderName::from_static("x-qiniu-e"), HeaderValue::from_str("value")?);
                headers.insert(HeaderName::from_static("x-qiniu-"), HeaderValue::from_str("value")?);
                headers.insert(HeaderName::from_static("x-qiniu"), HeaderValue::from_str("value")?);
                headers
            };
            let form_headers = {
                let mut headers = HeaderMap::new();
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_str(APPLICATION_WWW_FORM_URLENCODED.as_ref())?,
                );
                headers.insert(HeaderName::from_static("x-qbox-meta"), HeaderValue::from_str("value")?);
                headers.insert(
                    HeaderName::from_static("x-qiniu-cxxxx"),
                    HeaderValue::from_str("valuec")?,
                );
                headers.insert(
                    HeaderName::from_static("x-qiniu-bxxxx"),
                    HeaderValue::from_str("valueb")?,
                );
                headers.insert(
                    HeaderName::from_static("x-qiniu-axxxx"),
                    HeaderValue::from_str("valuea")?,
                );
                headers.insert(HeaderName::from_static("x-qiniu-e"), HeaderValue::from_str("value")?);
                headers.insert(HeaderName::from_static("x-qiniu-"), HeaderValue::from_str("value")?);
                headers.insert(HeaderName::from_static("x-qiniu"), HeaderValue::from_str("value")?);
                headers
            };
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::GET,
                        &"http://upload.qiniup.com/".parse()?,
                        &json_headers,
                        &mut Cursor::new(b"{\"name\":\"test\"}")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential.get(&Default::default())?.sign(
                        concat!(
                            "GET /\n",
                            "Host: upload.qiniup.com\n",
                            "Content-Type: application/json\n",
                            "X-Qiniu-Axxxx: valuea\n",
                            "X-Qiniu-Bxxxx: valueb\n",
                            "X-Qiniu-Cxxxx: valuec\n",
                            "X-Qiniu-E: value\n\n",
                            "{\"name\":\"test\"}"
                        )
                        .as_bytes()
                    )
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::GET,
                        &"http://upload.qiniup.com/".parse()?,
                        &empty_headers,
                        &mut Cursor::new(b"{\"name\":\"test\"}")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential
                        .get(&Default::default())?
                        .sign(concat!("GET /\n", "Host: upload.qiniup.com\n\n").as_bytes())
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::POST,
                        &"http://upload.qiniup.com/".parse()?,
                        &json_headers,
                        &mut Cursor::new(b"{\"name\":\"test\"}")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential.get(&Default::default())?.sign(
                        concat!(
                            "POST /\n",
                            "Host: upload.qiniup.com\n",
                            "Content-Type: application/json\n",
                            "X-Qiniu-Axxxx: valuea\n",
                            "X-Qiniu-Bxxxx: valueb\n",
                            "X-Qiniu-Cxxxx: valuec\n",
                            "X-Qiniu-E: value\n\n",
                            "{\"name\":\"test\"}"
                        )
                        .as_bytes()
                    )
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::GET,
                        &"http://upload.qiniup.com/".parse()?,
                        &form_headers,
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential.get(&Default::default())?.sign(
                        concat!(
                            "GET /\n",
                            "Host: upload.qiniup.com\n",
                            "Content-Type: application/x-www-form-urlencoded\n",
                            "X-Qiniu-Axxxx: valuea\n",
                            "X-Qiniu-Bxxxx: valueb\n",
                            "X-Qiniu-Cxxxx: valuec\n",
                            "X-Qiniu-E: value\n\n",
                            "name=test&language=go"
                        )
                        .as_bytes()
                    )
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::GET,
                        &"http://upload.qiniup.com/?v=2".parse()?,
                        &form_headers,
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential.get(&Default::default())?.sign(
                        concat!(
                            "GET /?v=2\n",
                            "Host: upload.qiniup.com\n",
                            "Content-Type: application/x-www-form-urlencoded\n",
                            "X-Qiniu-Axxxx: valuea\n",
                            "X-Qiniu-Bxxxx: valueb\n",
                            "X-Qiniu-Cxxxx: valuec\n",
                            "X-Qiniu-E: value\n\n",
                            "name=test&language=go"
                        )
                        .as_bytes()
                    )
            );
            assert_eq!(
                credential
                    .get(&Default::default())?
                    .authorization_v2_for_request_with_async_body_reader(
                        &Method::GET,
                        &"http://upload.qiniup.com/find/sdk?v=2".parse()?,
                        &form_headers,
                        &mut Cursor::new(b"name=test&language=go")
                    )
                    .await?,
                "Qiniu ".to_owned()
                    + &credential.get(&Default::default())?.sign(
                        concat!(
                            "GET /find/sdk?v=2\n",
                            "Host: upload.qiniup.com\n",
                            "Content-Type: application/x-www-form-urlencoded\n",
                            "X-Qiniu-Axxxx: valuea\n",
                            "X-Qiniu-Bxxxx: valueb\n",
                            "X-Qiniu-Cxxxx: valuec\n",
                            "X-Qiniu-E: value\n\n",
                            "name=test&language=go"
                        )
                        .as_bytes()
                    )
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_async_sign_download_url() -> Result<()> {
            let credential = get_env_credential();
            let url = "http://www.qiniu.com/?go=1".parse()?;
            let url = credential
                .async_get(&Default::default())
                .await?
                .sign_download_url(url, Duration::from_secs(1_234_567_890 + 3600));
            assert_eq!(
                url.to_string(),
                "http://www.qiniu.com/?go=1&e=1234571490&token=abcdefghklmnopq%3AKjQtlGAkEOhSwtFjJfYtYa2-reE%3D",
            );
            Ok(())
        }

        #[tokio::test]
        async fn test_async_chain_credentials() -> Result<()> {
            GlobalCredentialProvider::clear();
            let chain_credentials = ChainCredentialsProvider::default();
            env::set_var(QINIU_ACCESS_KEY_ENV_KEY, "TEST2");
            env::set_var(QINIU_SECRET_KEY_ENV_KEY, "test2");
            {
                let cred = chain_credentials.async_get(&Default::default()).await?;
                assert_eq!(cred.access_key().as_str(), "TEST2");
            }
            GlobalCredentialProvider::setup(Credential::new("TEST1", "test1"));
            {
                let cred = chain_credentials.async_get(&Default::default()).await?;
                assert_eq!(cred.access_key().as_str(), "TEST1");
            }
            Ok(())
        }
    }
}
