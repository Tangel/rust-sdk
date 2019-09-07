use super::{method::Method, request::Request, response::Response};
use getset::{CopyGetters, Getters};
use std::{boxed::Box, error, fmt, marker::Send, result};

pub type URL = Box<str>;
pub type RequestID = Box<str>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Getters, CopyGetters)]
pub struct Error {
    #[get_copy = "pub"]
    kind: ErrorKind,

    #[get_copy = "pub"]
    is_retry_safe: bool,

    cause: Box<dyn error::Error + Send>,

    #[get_copy = "pub"]
    method: Option<Method>,

    #[get = "pub"]
    request_id: Option<RequestID>,

    #[get = "pub"]
    url: Option<URL>,
}

impl Error {
    pub fn new<E: error::Error + 'static + Send>(
        kind: ErrorKind,
        cause: E,
        is_retry_safe: bool,
        request: &Request,
        response: Option<&Response>,
    ) -> Error {
        Error {
            kind: kind,
            cause: Box::new(cause),
            is_retry_safe: is_retry_safe,
            method: Some(request.method().to_owned()),
            request_id: Self::extract_req_id_from_response(response),
            url: Some(request.url().into()),
        }
    }

    pub fn new_retryable_error<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        request: &Request,
        response: Option<&Response>,
    ) -> Error {
        Self::new(ErrorKind::RetryableError, cause, is_retry_safe, request, response)
    }

    pub fn new_zone_unretryable_error<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        request: &Request,
        response: Option<&Response>,
    ) -> Error {
        Self::new(ErrorKind::ZoneUnretryableError, cause, is_retry_safe, request, response)
    }

    pub fn new_host_unretryable_error<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        request: &Request,
        response: Option<&Response>,
    ) -> Error {
        Self::new(ErrorKind::HostUnretryableError, cause, is_retry_safe, request, response)
    }

    pub fn new_unretryable_error<E: error::Error + 'static + Send>(
        cause: E,
        request: &Request,
        response: Option<&Response>,
    ) -> Error {
        Self::new(ErrorKind::UnretryableError, cause, false, request, response)
    }

    pub fn new_from_parts<E: error::Error + 'static + Send>(
        kind: ErrorKind,
        cause: E,
        is_retry_safe: bool,
        method: Option<Method>,
        url: Option<URL>,
    ) -> Error {
        Error {
            kind: kind,
            cause: Box::new(cause),
            is_retry_safe: is_retry_safe,
            method: method,
            request_id: None,
            url: url,
        }
    }

    pub fn new_retryable_error_from_parts<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        method: Option<Method>,
        url: Option<URL>,
    ) -> Error {
        Error {
            kind: ErrorKind::RetryableError,
            cause: Box::new(cause),
            is_retry_safe: is_retry_safe,
            method: method,
            request_id: None,
            url: url,
        }
    }

    pub fn new_zone_unretryable_error_from_parts<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        method: Option<Method>,
        url: Option<URL>,
    ) -> Error {
        Error {
            kind: ErrorKind::ZoneUnretryableError,
            cause: Box::new(cause),
            is_retry_safe: is_retry_safe,
            method: method,
            request_id: None,
            url: url,
        }
    }

    pub fn new_host_unretryable_error_from_parts<E: error::Error + 'static + Send>(
        cause: E,
        is_retry_safe: bool,
        method: Option<Method>,
        url: Option<URL>,
    ) -> Error {
        Error {
            kind: ErrorKind::HostUnretryableError,
            cause: Box::new(cause),
            is_retry_safe: is_retry_safe,
            method: method,
            request_id: None,
            url: url,
        }
    }

    pub fn new_unretryable_error_from_parts<E: error::Error + 'static + Send>(
        cause: E,
        method: Option<Method>,
        url: Option<URL>,
    ) -> Error {
        Error {
            kind: ErrorKind::UnretryableError,
            cause: Box::new(cause),
            is_retry_safe: false,
            method: method,
            request_id: None,
            url: url,
        }
    }

    fn extract_req_id_from_response(response: Option<&Response>) -> Option<RequestID> {
        response.and_then(|resp| resp.headers().get("X-Reqid".into()).map(|v| v.as_ref().into()))
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("cause", &self.cause)
            .field("method", &self.method)
            .field("url", &self.url)
            .field("is_retry_safe", &self.is_retry_safe)
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}: {} {}: ",
            self.kind,
            self.method.as_ref().map(|m| m.as_str()).unwrap_or("None"),
            self.url.as_ref().map(|u| &u as &str).unwrap_or("None"),
        )?;
        self.cause.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.cause.description()
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self.cause.as_ref())
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.cause.as_ref())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    RetryableError,
    ZoneUnretryableError,
    HostUnretryableError,
    UnretryableError,
}
