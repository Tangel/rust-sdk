use super::{UploadPolicy, UploadPolicyBuilder};
use auto_impl::auto_impl;
use dyn_clonable::clonable;
use once_cell::sync::OnceCell;
use qiniu_credential::{AccessKey, CredentialProvider};
use qiniu_utils::{base64, BucketName, ObjectName};
use std::{
    borrow::Cow,
    fmt::{self, Debug},
    io::{Error as IoError, Result as IoResult},
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use tap::Tap;
use thiserror::Error;

#[cfg(feature = "async")]
use {
    futures::lock::Mutex as AsyncMutex,
    std::{future::Future, pin::Pin},
};

#[cfg(feature = "async")]
type AsyncParseResult<'a, T> = Pin<Box<dyn Future<Output = ParseResult<T>> + 'a + Send>>;

#[cfg(feature = "async")]
type AsyncIoResult<'a, T> = Pin<Box<dyn Future<Output = IoResult<T>> + 'a + Send>>;

/// 上传凭证提供者
///
/// 可以点击[这里](https://developer.qiniu.com/kodo/manual/1208/upload-token)了解七牛安全机制。
#[clonable]
#[auto_impl(&, &mut, Box, Rc, Arc)]
pub trait UploadTokenProvider: Clone + Debug + Sync + Send {
    /// 从上传凭证内获取 AccessKey
    fn access_key(&self, opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey>;

    /// 异步从上传凭证内获取 AccessKey
    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_access_key<'a>(&'a self, opts: &'a GetAccessKeyOptions) -> AsyncParseResult<'a, GotAccessKey> {
        Box::pin(async move { self.access_key(opts) })
    }

    /// 从上传凭证内获取上传策略
    fn policy<'a>(&'a self, opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>>;

    /// 异步从上传凭证内获取上传策略
    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_policy<'a>(&'a self, opts: &'a GetPolicyOptions) -> AsyncParseResult<'a, GotUploadPolicy<'a>> {
        Box::pin(async move { self.policy(opts) })
    }

    /// 生成字符串
    fn to_token_string<'a>(&'a self, opts: &ToStringOptions) -> IoResult<GotString<'a>>;

    /// 异步生成字符串
    #[inline]
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_to_token_string<'a>(&'a self, opts: &'a ToStringOptions) -> AsyncIoResult<'a, GotString<'a>> {
        Box::pin(async move { self.to_token_string(opts) })
    }
}

#[derive(Clone, Debug, Default)]
pub struct GetAccessKeyOptions {}

#[derive(Clone, Debug, Default)]
pub struct GetPolicyOptions {}

#[derive(Clone, Debug, Default)]
pub struct ToStringOptions {}

#[derive(Debug)]
pub struct GotAccessKey(AccessKey);

impl From<GotAccessKey> for AccessKey {
    #[inline]
    fn from(result: GotAccessKey) -> Self {
        result.0
    }
}

impl From<AccessKey> for GotAccessKey {
    #[inline]
    fn from(result: AccessKey) -> Self {
        Self(result)
    }
}

impl GotAccessKey {
    #[inline]
    pub fn access_key(&self) -> &AccessKey {
        &self.0
    }

    #[inline]
    pub fn access_key_mut(&mut self) -> &mut AccessKey {
        &mut self.0
    }

    #[inline]
    pub fn into_access_key(self) -> AccessKey {
        self.0
    }
}

impl Deref for GotAccessKey {
    type Target = AccessKey;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GotAccessKey {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct GotUploadPolicy<'a>(Cow<'a, UploadPolicy>);

impl<'a> From<GotUploadPolicy<'a>> for Cow<'a, UploadPolicy> {
    #[inline]
    fn from(result: GotUploadPolicy<'a>) -> Self {
        result.0
    }
}

impl From<GotUploadPolicy<'_>> for UploadPolicy {
    #[inline]
    fn from(result: GotUploadPolicy<'_>) -> Self {
        result.into_upload_policy()
    }
}

impl<'a> From<Cow<'a, UploadPolicy>> for GotUploadPolicy<'a> {
    #[inline]
    fn from(policy: Cow<'a, UploadPolicy>) -> Self {
        Self(policy)
    }
}

impl<'a> From<&'a UploadPolicy> for GotUploadPolicy<'a> {
    #[inline]
    fn from(policy: &'a UploadPolicy) -> Self {
        Self::from(Cow::Borrowed(policy))
    }
}

impl From<UploadPolicy> for GotUploadPolicy<'_> {
    #[inline]
    fn from(policy: UploadPolicy) -> Self {
        Self::from(Cow::Owned(policy))
    }
}

impl GotUploadPolicy<'_> {
    #[inline]
    pub fn upload_policy(&self) -> &UploadPolicy {
        &self.0
    }

    #[inline]
    pub fn upload_policy_mut(&mut self) -> &mut UploadPolicy {
        self.0.to_mut()
    }

    #[inline]
    pub fn into_upload_policy(self) -> UploadPolicy {
        self.0.into_owned()
    }
}

impl Deref for GotUploadPolicy<'_> {
    type Target = UploadPolicy;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GotUploadPolicy<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.to_mut()
    }
}

#[derive(Debug, Clone)]
pub struct GotString<'a>(Cow<'a, str>);

impl<'a> From<GotString<'a>> for Cow<'a, str> {
    #[inline]
    fn from(result: GotString<'a>) -> Self {
        result.0
    }
}

impl From<GotString<'_>> for String {
    #[inline]
    fn from(result: GotString<'_>) -> Self {
        result.0.into_owned()
    }
}

impl<'a> From<Cow<'a, str>> for GotString<'a> {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        Self(s)
    }
}

impl<'a> From<&'a str> for GotString<'a> {
    #[inline]
    fn from(s: &'a str) -> Self {
        Self::from(Cow::Borrowed(s))
    }
}

impl From<String> for GotString<'_> {
    #[inline]
    fn from(s: String) -> Self {
        Self::from(Cow::Owned(s))
    }
}

impl From<Box<str>> for GotString<'_> {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self::from(Cow::Owned(s.into_string()))
    }
}

impl AsRef<str> for GotString<'_> {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsMut<str> for GotString<'_> {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.0.to_mut()
    }
}

impl<'a> fmt::Display for GotString<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<'a> Deref for GotString<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for GotString<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.to_mut()
    }
}

pub trait UploadTokenProviderExt: UploadTokenProvider {
    fn bucket_name(&self, opts: &GetPolicyOptions) -> ParseResult<BucketName> {
        self.policy(opts).and_then(|policy| {
            policy
                .bucket()
                .map_or(Err(ParseError::InvalidUploadTokenFormat), |bucket_name| {
                    Ok(BucketName::from(bucket_name))
                })
        })
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_bucket_name<'a>(&'a self, opts: &'a GetPolicyOptions) -> AsyncParseResult<'a, BucketName> {
        Box::pin(async move {
            self.async_policy(opts).await.and_then(|policy| {
                policy
                    .bucket()
                    .map_or(Err(ParseError::InvalidUploadTokenFormat), |bucket_name| {
                        Ok(BucketName::from(bucket_name))
                    })
            })
        })
    }
}

impl<T: UploadTokenProvider> UploadTokenProviderExt for T {}

/// 静态上传凭证提供者
///
/// 根据已经被生成好的上传凭证字符串生成上传凭证提供者实例，可以将上传凭证解析为 Access Token 和上传策略
#[derive(Clone)]
pub struct StaticUploadTokenProvider {
    upload_token: Box<str>,
    policy: OnceCell<UploadPolicy>,
    access_key: OnceCell<AccessKey>,
}

impl StaticUploadTokenProvider {
    /// 构建一个静态上传凭证，只需要传入静态的上传凭证字符串即可
    pub fn new(upload_token: impl Into<String>) -> Self {
        Self {
            upload_token: upload_token.into().into_boxed_str(),
            policy: OnceCell::new(),
            access_key: OnceCell::new(),
        }
    }
}

impl Debug for StaticUploadTokenProvider {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StaticUploadTokenProvider")
            .field("upload_token", &self.upload_token)
            .finish()
    }
}

impl UploadTokenProvider for StaticUploadTokenProvider {
    fn access_key(&self, _opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey> {
        self.access_key
            .get_or_try_init(|| {
                self.upload_token
                    .find(':')
                    .map(|i| self.upload_token.split_at(i).0.to_owned().into())
                    .ok_or(ParseError::InvalidUploadTokenFormat)
            })
            .map(|access_key| access_key.to_owned())
            .map(GotAccessKey::from)
    }

    fn policy<'a>(&'a self, _opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>> {
        self.policy
            .get_or_try_init(|| {
                let encoded_policy = self
                    .upload_token
                    .splitn(3, ':')
                    .last()
                    .ok_or(ParseError::InvalidUploadTokenFormat)?;
                let decoded_policy =
                    base64::decode(encoded_policy.as_bytes()).map_err(ParseError::Base64DecodeError)?;
                UploadPolicy::from_json(&decoded_policy).map_err(ParseError::JsonDecodeError)
            })
            .map(Cow::Borrowed)
            .map(GotUploadPolicy::from)
    }

    #[inline]
    fn to_token_string<'a>(&'a self, _opts: &ToStringOptions) -> IoResult<GotString<'a>> {
        Ok(Cow::Borrowed(self.upload_token.as_ref()).into())
    }
}

impl<T: Into<String>> From<T> for StaticUploadTokenProvider {
    #[inline]
    fn from(s: T) -> Self {
        Self::new(s)
    }
}

#[derive(Debug, Clone)]
pub struct FromUploadPolicy<C: Clone> {
    upload_policy: UploadPolicy,
    credential: C,
}

impl<C: Clone> FromUploadPolicy<C> {
    /// 基于上传策略和认证信息生成上传凭证实例
    pub fn new(upload_policy: UploadPolicy, credential: C) -> Self {
        Self {
            upload_policy,
            credential,
        }
    }
}

impl<C: CredentialProvider + Clone> UploadTokenProvider for FromUploadPolicy<C> {
    fn access_key(&self, _opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey> {
        Ok(self
            .credential
            .get(&Default::default())?
            .into_credential()
            .into_pair()
            .0
            .into())
    }

    #[inline]
    fn policy<'a>(&'a self, _opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>> {
        Ok(Cow::Borrowed(&self.upload_policy).into())
    }

    fn to_token_string<'a>(&'a self, _opts: &ToStringOptions) -> IoResult<GotString<'a>> {
        Ok(self
            .credential
            .get(&Default::default())?
            .sign_with_data(self.upload_policy.as_json().as_bytes())
            .into())
    }
}

type OnPolicyGeneratedCallback = Arc<dyn Fn(&mut UploadPolicyBuilder) + Sync + Send + 'static>;

/// 基于存储空间的动态生成
///
/// 根据存储空间的快速生成上传凭证实例
#[derive(Clone)]
pub struct BucketUploadTokenProvider<C: Clone> {
    bucket: BucketName,
    upload_token_lifetime: Duration,
    credential: C,
    on_policy_generated: Option<OnPolicyGeneratedCallback>,
}

impl<C: Clone> BucketUploadTokenProvider<C> {
    /// 基于存储空间和认证信息动态生成上传凭证实例
    #[inline]
    pub fn new(bucket: impl Into<BucketName>, upload_token_lifetime: Duration, credential: C) -> Self {
        Self::builder(bucket, upload_token_lifetime, credential).build()
    }

    #[inline]
    pub fn builder(
        bucket: impl Into<BucketName>,
        upload_token_lifetime: Duration,
        credential: C,
    ) -> BucketUploadTokenProviderBuilder<C> {
        BucketUploadTokenProviderBuilder {
            inner: Self {
                bucket: bucket.into(),
                upload_token_lifetime,
                credential,
                on_policy_generated: None,
            },
        }
    }

    fn make_policy(&self) -> UploadPolicy {
        UploadPolicyBuilder::new_policy_for_bucket(self.bucket.to_string(), self.upload_token_lifetime)
            .tap_mut(|policy| {
                if let Some(on_policy_generated) = self.on_policy_generated.as_ref() {
                    on_policy_generated(policy);
                }
            })
            .build()
    }
}

impl<C: Clone> Debug for BucketUploadTokenProvider<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BucketUploadTokenProvider")
            .field("bucket", &self.bucket)
            .field("upload_token_lifetime", &self.upload_token_lifetime)
            .finish()
    }
}

impl<C: CredentialProvider + Clone> UploadTokenProvider for BucketUploadTokenProvider<C> {
    #[inline]
    fn access_key(&self, _opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey> {
        Ok(self
            .credential
            .get(&Default::default())?
            .into_credential()
            .into_pair()
            .0
            .into())
    }

    fn policy<'a>(&'a self, _opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>> {
        Ok(self.make_policy().into())
    }

    fn to_token_string<'a>(&'a self, _opts: &ToStringOptions) -> IoResult<GotString<'a>> {
        let upload_token = self
            .credential
            .get(&Default::default())?
            .sign_with_data(self.make_policy().as_json().as_bytes());
        Ok(upload_token.into())
    }
}

#[derive(Clone)]
pub struct BucketUploadTokenProviderBuilder<C: Clone> {
    inner: BucketUploadTokenProvider<C>,
}

impl<C: Clone> BucketUploadTokenProviderBuilder<C> {
    #[inline]
    #[must_use]
    pub fn on_policy_generated(mut self, callback: impl Fn(&mut UploadPolicyBuilder) + Sync + Send + 'static) -> Self {
        self.inner.on_policy_generated = Some(Arc::new(callback));
        self
    }

    #[inline]
    pub fn build(self) -> BucketUploadTokenProvider<C> {
        self.inner
    }
}

impl<C: Clone> Debug for BucketUploadTokenProviderBuilder<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BucketUploadTokenProviderBuilder")
            .field("bucket", &self.inner.bucket)
            .field("upload_token_lifetime", &self.inner.upload_token_lifetime)
            .finish()
    }
}

/// 基于对象的动态生成
///
/// 根据对象的快速生成上传凭证实例
#[derive(Clone)]
pub struct ObjectUploadTokenProvider<C: Clone> {
    bucket: BucketName,
    object: ObjectName,
    upload_token_lifetime: Duration,
    credential: C,
    on_policy_generated: Option<OnPolicyGeneratedCallback>,
}

impl<C: Clone> ObjectUploadTokenProvider<C> {
    /// 基于存储空间和对象名称和认证信息动态生成上传凭证实例
    #[inline]
    pub fn new(
        bucket: impl Into<BucketName>,
        object: impl Into<ObjectName>,
        upload_token_lifetime: Duration,
        credential: C,
    ) -> Self {
        Self::builder(bucket, object, upload_token_lifetime, credential).build()
    }

    #[inline]
    pub fn builder(
        bucket: impl Into<BucketName>,
        object: impl Into<ObjectName>,
        upload_token_lifetime: Duration,
        credential: C,
    ) -> ObjectUploadTokenProviderBuilder<C> {
        ObjectUploadTokenProviderBuilder {
            inner: Self {
                bucket: bucket.into(),
                object: object.into(),
                upload_token_lifetime,
                credential,
                on_policy_generated: None,
            },
        }
    }

    fn make_policy(&self) -> UploadPolicy {
        UploadPolicyBuilder::new_policy_for_object(
            self.bucket.to_string(),
            self.object.to_string(),
            self.upload_token_lifetime,
        )
        .tap_mut(|policy| {
            if let Some(on_policy_generated) = self.on_policy_generated.as_ref() {
                on_policy_generated(policy);
            }
        })
        .build()
    }
}

impl<C: Clone> Debug for ObjectUploadTokenProvider<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ObjectUploadTokenProvider")
            .field("bucket", &self.bucket)
            .field("object", &self.object)
            .field("upload_token_lifetime", &self.upload_token_lifetime)
            .finish()
    }
}

impl<C: CredentialProvider + Clone> UploadTokenProvider for ObjectUploadTokenProvider<C> {
    fn access_key(&self, _opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey> {
        Ok(self
            .credential
            .get(&Default::default())?
            .into_credential()
            .into_pair()
            .0
            .into())
    }

    fn policy<'a>(&'a self, _opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>> {
        Ok(self.make_policy().into())
    }

    fn to_token_string<'a>(&'a self, _opts: &ToStringOptions) -> IoResult<GotString<'a>> {
        let upload_token = self
            .credential
            .get(&Default::default())?
            .sign_with_data(self.make_policy().as_json().as_bytes());
        Ok(upload_token.into())
    }
}

#[derive(Clone)]
pub struct ObjectUploadTokenProviderBuilder<C: Clone> {
    inner: ObjectUploadTokenProvider<C>,
}

impl<C: Clone> ObjectUploadTokenProviderBuilder<C> {
    #[inline]
    #[must_use]
    pub fn on_policy_generated(mut self, callback: impl Fn(&mut UploadPolicyBuilder) + Sync + Send + 'static) -> Self {
        self.inner.on_policy_generated = Some(Arc::new(callback));
        self
    }

    #[inline]
    pub fn build(self) -> ObjectUploadTokenProvider<C> {
        self.inner
    }
}

impl<C: Clone> Debug for ObjectUploadTokenProviderBuilder<C> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ObjectUploadTokenProviderBuilder")
            .field("bucket", &self.inner.bucket)
            .field("object", &self.inner.object)
            .field("upload_token_lifetime", &self.inner.upload_token_lifetime)
            .finish()
    }
}

#[derive(Debug)]
struct Cache<T> {
    cached_at: Instant,
    value: T,
}

#[derive(Debug, Default, Clone)]
struct SyncCache(Arc<SyncCacheInner>);

#[derive(Debug, Default)]
struct SyncCacheInner {
    access_key: RwLock<Option<Cache<AccessKey>>>,
    upload_policy: RwLock<Option<Cache<UploadPolicy>>>,
    upload_token: RwLock<Option<Cache<String>>>,
}

#[cfg(feature = "async")]
#[derive(Debug, Default, Clone)]
struct AsyncCache(Arc<AsyncCacheInner>);

#[cfg(feature = "async")]
#[derive(Debug, Default)]
struct AsyncCacheInner {
    access_key: AsyncMutex<Option<Cache<AccessKey>>>,
    upload_policy: AsyncMutex<Option<Cache<UploadPolicy>>>,
    upload_token: AsyncMutex<Option<Cache<String>>>,
}

#[derive(Debug, Clone)]
pub struct CachedUploadTokenProvider<P: Clone> {
    inner_provider: P,
    cache_lifetime: Duration,
    sync_cache: SyncCache,

    #[cfg(feature = "async")]
    async_cache: AsyncCache,
}

impl<P: Clone> CachedUploadTokenProvider<P> {
    #[inline]
    pub fn new(inner_provider: P, cache_lifetime: Duration) -> Self {
        Self {
            inner_provider,
            cache_lifetime,
            sync_cache: Default::default(),

            #[cfg(feature = "async")]
            async_cache: Default::default(),
        }
    }
}

macro_rules! sync_method {
    ($provider:expr, $cache_field:ident, $opts_field:ident, $opts_type:ty, $method_name:ident, $return_type:ty) => {{
        let guard = $provider.sync_cache.0.$cache_field.read().unwrap();
        return if let Some(cache) = &*guard {
            if cache.cached_at.elapsed() < $provider.cache_lifetime {
                Ok(cache.value.to_owned().into())
            } else {
                drop(guard);
                update_cache(&$provider, $opts_field)
            }
        } else {
            drop(guard);
            update_cache(&$provider, $opts_field)
        };

        #[allow(unused_lifetimes)]
        fn update_cache<'a>(
            provider: &'a CachedUploadTokenProvider<impl UploadTokenProvider + Clone>,
            opts: &$opts_type,
        ) -> $return_type {
            let mut guard = provider.sync_cache.0.$cache_field.write().unwrap();
            if let Some(cache) = &*guard {
                if cache.cached_at.elapsed() < provider.cache_lifetime {
                    return Ok(cache.value.to_owned().into());
                }
            }
            match provider.inner_provider.$method_name(opts) {
                Ok(value) => {
                    *guard = Some(Cache {
                        cached_at: Instant::now(),
                        value: value.to_owned().into(),
                    });
                    Ok(value)
                }
                Err(err) => Err(err),
            }
        }
    }};
}

#[cfg(feature = "async")]
macro_rules! async_method {
    ($provider:expr, $cache_field:ident, $opts_field:ident, $method_name:ident) => {{
        Box::pin(async move {
            let mut cache = $provider.async_cache.0.$cache_field.lock().await;
            if let Some(cache) = &*cache {
                if cache.cached_at.elapsed() < $provider.cache_lifetime {
                    return Ok(cache.value.to_owned().into());
                }
            }
            match $provider.inner_provider.$method_name($opts_field).await {
                Ok(value) => {
                    *cache = Some(Cache {
                        cached_at: Instant::now(),
                        value: value.to_owned().into(),
                    });
                    Ok(value)
                }
                Err(err) => Err(err),
            }
        })
    }};
}

impl<P: UploadTokenProvider + Clone> UploadTokenProvider for CachedUploadTokenProvider<P> {
    fn access_key(&self, opts: &GetAccessKeyOptions) -> ParseResult<GotAccessKey> {
        sync_method!(
            self,
            access_key,
            opts,
            GetAccessKeyOptions,
            access_key,
            ParseResult<GotAccessKey>
        )
    }

    fn policy<'a>(&'a self, opts: &GetPolicyOptions) -> ParseResult<GotUploadPolicy<'a>> {
        sync_method!(
            self,
            upload_policy,
            opts,
            GetPolicyOptions,
            policy,
            ParseResult<GotUploadPolicy<'a>>
        )
    }

    fn to_token_string<'a>(&'a self, opts: &ToStringOptions) -> IoResult<GotString<'a>> {
        sync_method!(
            self,
            upload_token,
            opts,
            ToStringOptions,
            to_token_string,
            IoResult<GotString<'a>>
        )
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_access_key<'a>(&'a self, opts: &'a GetAccessKeyOptions) -> AsyncParseResult<'a, GotAccessKey> {
        async_method!(self, access_key, opts, async_access_key)
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_policy<'a>(&'a self, opts: &'a GetPolicyOptions) -> AsyncParseResult<'a, GotUploadPolicy<'a>> {
        async_method!(self, upload_policy, opts, async_policy)
    }

    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    fn async_to_token_string<'a>(&'a self, opts: &'a ToStringOptions) -> AsyncIoResult<'a, GotString<'a>> {
        async_method!(self, upload_token, opts, async_to_token_string)
    }
}

/// 上传凭证解析错误
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// 上传凭证格式错误
    #[error("Invalid upload token format")]
    InvalidUploadTokenFormat,
    /// 上传凭证 Base64 解码错误
    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),
    /// 上传凭证 JSON 解析错误
    #[error("JSON decode error: {0}")]
    JsonDecodeError(#[from] serde_json::Error),
    /// 上传凭证获取认证信息错误
    #[error("Credential get error: {0}")]
    CredentialGetError(#[from] IoError),
}

/// 上传凭证解析结果
pub type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::{super::UploadPolicyBuilder, *};
    use qiniu_credential::Credential;
    use std::{boxed::Box, error::Error, result::Result};
    use structopt as _;
    use tokio as _;

    #[test]
    fn test_build_upload_token_from_upload_policy() -> Result<(), Box<dyn Error>> {
        let policy =
            UploadPolicyBuilder::new_policy_for_object("test_bucket", "test:file", Duration::from_secs(3600)).build();
        let token = FromUploadPolicy::new(policy, get_credential())
            .to_token_string(&Default::default())?
            .to_string();
        assert!(token.starts_with(get_credential().get(&Default::default())?.access_key().as_str()));
        let token = StaticUploadTokenProvider::from(token);
        let policy = token.policy(&Default::default())?;
        assert_eq!(policy.bucket(), Some("test_bucket"));
        assert_eq!(policy.key(), Some("test:file"));
        Ok(())
    }

    #[test]
    fn test_build_upload_token_for_bucket() -> Result<(), Box<dyn Error>> {
        let provider = BucketUploadTokenProvider::builder("test_bucket", Duration::from_secs(3600), get_credential())
            .on_policy_generated(|policy| {
                policy.return_body("{\"key\":$(key)}");
            })
            .build();

        let token = provider.to_token_string(&Default::default())?.to_string();
        assert!(token.starts_with(get_credential().get(&Default::default())?.access_key().as_str()));

        let policy = provider.policy(&Default::default())?;
        assert_eq!(policy.bucket(), Some("test_bucket"));
        assert_eq!(policy.key(), None);
        assert_eq!(policy.return_body(), Some("{\"key\":$(key)}"));

        Ok(())
    }

    #[cfg(feature = "async")]
    mod async_test {
        use super::*;

        #[tokio::test]
        async fn test_async_build_upload_token_from_upload_policy() -> Result<(), Box<dyn Error>> {
            let policy =
                UploadPolicyBuilder::new_policy_for_object("test_bucket", "test:file", Duration::from_secs(3600))
                    .build();
            let token = FromUploadPolicy::new(policy, get_credential())
                .async_to_token_string(&Default::default())
                .await?
                .to_string();
            assert!(token.starts_with(
                get_credential()
                    .async_get(&Default::default())
                    .await?
                    .access_key()
                    .as_str()
            ));
            let token = StaticUploadTokenProvider::from(token);
            let get_policy_from_size_options = Default::default();
            let policy = token.async_policy(&get_policy_from_size_options).await?;
            assert_eq!(policy.bucket(), Some("test_bucket"));
            assert_eq!(policy.key(), Some("test:file"));
            Ok(())
        }
    }

    fn get_credential() -> Credential {
        Credential::new("abcdefghklmnopq", "1234567890")
    }
}
