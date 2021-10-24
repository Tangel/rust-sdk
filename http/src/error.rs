use super::response::{Metrics, ResponseInfo};
use http::uri::{Scheme, Uri};
use std::{error::Error as StdError, fmt, net::IpAddr, num::NonZeroU16};

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
    response_info: ResponseInfo,
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

    #[inline]
    pub fn server_ip(&self) -> Option<IpAddr> {
        self.response_info.server_ip()
    }

    #[inline]
    pub fn server_ip_mut(&mut self) -> &mut Option<IpAddr> {
        self.response_info.server_ip_mut()
    }

    #[inline]
    pub fn server_port(&self) -> Option<NonZeroU16> {
        self.response_info.server_port()
    }

    #[inline]
    pub fn server_port_mut(&mut self) -> &mut Option<NonZeroU16> {
        self.response_info.server_port_mut()
    }

    #[inline]
    pub fn metrics(&self) -> Option<&dyn Metrics> {
        self.response_info.metrics()
    }

    #[inline]
    pub fn metrics_mut(&mut self) -> &mut Option<Box<dyn Metrics>> {
        self.response_info.metrics_mut()
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
                response_info: Default::default(),
            },
        }
    }

    #[inline]
    pub fn build(self) -> Error {
        self.inner
    }

    #[inline]
    pub fn uri(mut self, uri: &Uri) -> Self {
        if let Some(host) = uri.host() {
            if let Ok(ip_addr) = host.parse::<IpAddr>() {
                *self.inner.response_info.server_ip_mut() = Some(ip_addr);
            }
        }
        if let Some(port) = uri.port_u16() {
            *self.inner.response_info.server_port_mut() = NonZeroU16::new(port);
        } else if let Some(scheme) = uri.scheme() {
            if scheme == &Scheme::HTTP {
                *self.inner.response_info.server_port_mut() = NonZeroU16::new(80);
            } else if scheme == &Scheme::HTTPS {
                *self.inner.response_info.server_port_mut() = NonZeroU16::new(443);
            }
        }
        self
    }

    #[inline]
    pub fn server_ip(mut self, server_ip: IpAddr) -> Self {
        *self.inner.response_info.server_ip_mut() = Some(server_ip);
        self
    }

    #[inline]
    pub fn server_port(mut self, server_port: NonZeroU16) -> Self {
        *self.inner.response_info.server_port_mut() = Some(server_port);
        self
    }

    #[inline]
    pub fn metrics(mut self, metrics: Box<dyn Metrics>) -> Self {
        *self.inner.response_info.metrics_mut() = Some(metrics);
        self
    }
}

pub struct MapError<E> {
    error: E,
    info: ResponseInfo,
}

impl<E> MapError<E> {
    #[inline]
    pub(super) fn new(error: E, info: ResponseInfo) -> Self {
        Self { error, info }
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
            response_info: self.info,
        }
    }
}