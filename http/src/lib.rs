#![cfg_attr(feature = "docs", feature(doc_cfg))]

mod authorization;
mod error;
mod request;
mod response;

pub use authorization::{Authorization, AuthorizationError, AuthorizationResult};
pub use error::{Error as ResponseError, ErrorKind as ResponseErrorKind};
pub use qiniu_utils::http::{
    header::{HeaderName, HeaderNameOwned, HeaderValue, HeaderValueOwned, Headers, HeadersOwned},
    method::{InvalidMethod, Method},
};
pub use request::{
    Body as RequestBody, BuildError as RequestBuildError, BuildResult as RequestBuildResult,
    Request, RequestBuilder, URL,
};
pub use response::{
    Body as ResponseBody, Response, ResponseBuilder, Result as ResponseResult, StatusCode,
};

/// 同步 HTTP 响应
pub type SyncResponse = Response<ResponseBody>;
/// 同步 HTTP 响应构建器
pub type SyncResponseBuilder = ResponseBuilder<ResponseBody>;
/// 同步 HTTP 响应结果
pub type SyncResponseResult = ResponseResult<ResponseBody>;

#[cfg(feature = "async")]
mod async_response {
    pub use super::response::{
        AsyncBody as AsyncResponseBody, AsyncFile, Response, ResponseBuilder,
        Result as ResponseResult,
    };

    /// 异步 HTTP 响应
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponse = Response<AsyncResponseBody>;

    /// 异步 HTTP 响应构建器
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponseBuilder = ResponseBuilder<AsyncResponseBody>;

    /// 异步 HTTP 响应结果
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    pub type AsyncResponseResult = ResponseResult<AsyncResponseBody>;
}
#[cfg(feature = "async")]
pub use async_response::*;

use std::any::Any;

#[cfg(feature = "async")]
use futures::future::BoxFuture;

/// HTTP 请求处理函数
///
/// 实现该接口，即可处理所有七牛 SDK 发送的 HTTP 请求
pub trait HTTPCaller: Any + Send + Sync {
    /// 同步发送 HTTP 请求
    fn call(&self, request: &Request) -> SyncResponseResult;

    /// 异步发送 HTTP 请求
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(r#async)))]
    fn async_call<'a>(&'a self, request: &'a Request<'_>) -> BoxFuture<'a, AsyncResponseResult>;

    fn as_http_caller(&self) -> &dyn HTTPCaller;
    fn as_any(&self) -> &dyn Any;
}