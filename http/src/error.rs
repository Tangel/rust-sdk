use super::response::{Metrics, ResponseParts};
use http::{
    uri::{Scheme, Uri},
    Extensions, HeaderMap, HeaderValue, StatusCode, Version,
};
use std::{
    error::Error as StdError,
    fmt,
    net::IpAddr,
    num::NonZeroU16,
    ops::{Deref, DerefMut},
};

/// HTTP 响应错误类型
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// 协议错误，该协议不能支持
    ProtocolError,

    /// 非法的请求 / 响应错误
    InvalidRequestResponse,

    /// 非法的 URL
    InvalidURL,

    /// 非法的 HTTP 头
    InvalidHeader,

    /// 网络连接失败
    ConnectError,

    /// 代理连接失败
    ProxyError,

    /// DNS 服务器连接失败
    DNSServerError,

    /// 域名解析失败
    UnknownHostError,

    /// 发送失败
    SendError,

    /// 接受失败
    ReceiveError,

    /// 本地 IO 失败
    LocalIOError,

    /// 超时失败
    TimeoutError,

    /// SSL 客户端证书错误
    ClientCertError,

    /// SSL 服务器端证书错误
    ServerCertError,

    /// SSL 错误
    SSLError,

    /// 重定向次数过多
    TooManyRedirect,

    /// 未知错误
    UnknownError,

    /// 用户取消
    UserCanceled,
}

/// HTTP 响应错误
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<dyn StdError + Send + Sync>,
    parts: ResponseParts,
}

impl Error {
    /// 创建 HTTP 响应错误
    #[inline]
    pub fn builder(
        kind: ErrorKind,
        err: impl Into<Box<dyn StdError + Send + Sync>>,
    ) -> ErrorBuilder {
        ErrorBuilder::new(kind, err)
    }

    /// 获取 HTTP 响应错误类型
    #[inline]
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    #[inline]
    pub fn into_inner(self) -> Box<dyn StdError + Send + Sync> {
        self.error
    }
}

impl Deref for Error {
    type Target = ResponseParts;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}

impl DerefMut for Error {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl StdError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.error.as_ref())
    }
}

#[derive(Debug)]
pub struct ErrorBuilder {
    inner: Error,
}

impl ErrorBuilder {
    #[inline]
    fn new(kind: ErrorKind, err: impl Into<Box<dyn StdError + Send + Sync>>) -> Self {
        Self {
            inner: Error {
                kind,
                error: err.into(),
                parts: Default::default(),
            },
        }
    }

    #[inline]
    pub fn build(self) -> Error {
        self.inner
    }

    #[inline]
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        *self.inner.status_code_mut() = status_code;
        self
    }

    #[inline]
    pub fn version(mut self, version: Version) -> Self {
        *self.inner.version_mut() = version;
        self
    }

    #[inline]
    pub fn headers(mut self, headers: HeaderMap<HeaderValue>) -> Self {
        *self.inner.headers_mut() = headers;
        self
    }

    #[inline]
    pub fn extensions(mut self, extensions: Extensions) -> Self {
        *self.inner.extensions_mut() = extensions;
        self
    }

    #[inline]
    pub fn uri(mut self, uri: &Uri) -> Self {
        if let Some(host) = uri.host() {
            if let Ok(ip_addr) = host.parse::<IpAddr>() {
                *self.inner.server_ip_mut() = Some(ip_addr);
            }
        }
        if let Some(port) = uri.port_u16() {
            *self.inner.server_port_mut() = NonZeroU16::new(port);
        } else if let Some(scheme) = uri.scheme() {
            if scheme == &Scheme::HTTP {
                *self.inner.server_port_mut() = NonZeroU16::new(80);
            } else if scheme == &Scheme::HTTPS {
                *self.inner.server_port_mut() = NonZeroU16::new(443);
            }
        }
        self
    }

    #[inline]
    pub fn server_ip(mut self, server_ip: IpAddr) -> Self {
        *self.inner.server_ip_mut() = Some(server_ip);
        self
    }

    #[inline]
    pub fn server_port(mut self, server_port: NonZeroU16) -> Self {
        *self.inner.server_port_mut() = Some(server_port);
        self
    }

    #[inline]
    pub fn metrics(mut self, metrics: Box<dyn Metrics>) -> Self {
        *self.inner.metrics_mut() = Some(metrics);
        self
    }
}

pub struct MapError<E> {
    error: E,
    parts: ResponseParts,
}

impl<E> MapError<E> {
    #[inline]
    pub(super) fn new(error: E, parts: ResponseParts) -> Self {
        Self { error, parts }
    }

    #[inline]
    pub fn into_inner(self) -> E {
        self.error
    }
}

impl<E: StdError + Sync + Send + 'static> MapError<E> {
    pub fn into_response_error(self, kind: ErrorKind) -> Error {
        Error {
            kind,
            error: Box::new(self.error),
            parts: self.parts,
        }
    }
}
