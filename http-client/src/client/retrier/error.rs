use super::{
    super::{Idempotent, ResponseErrorKind},
    RequestRetrier, RequestRetrierOptions, RetryDecision, RetryResult,
};
use qiniu_http::{Request as HTTPRequest, ResponseErrorKind as HTTPResponseErrorKind};
use std::any::Any;

#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorRetrier;

impl RequestRetrier for ErrorRetrier {
    #[inline]
    fn retry(&self, request: &mut HTTPRequest, opts: &RequestRetrierOptions) -> RetryResult {
        return match opts.response_error().kind() {
            ResponseErrorKind::HTTPError(http_err_kind) => match http_err_kind {
                HTTPResponseErrorKind::ProtocolError => RetryDecision::RetryRequest,
                HTTPResponseErrorKind::InvalidURL => RetryDecision::TryNextServer,
                HTTPResponseErrorKind::ConnectError => RetryDecision::TryNextServer,
                HTTPResponseErrorKind::ProxyError => RetryDecision::RetryRequest,
                HTTPResponseErrorKind::DNSServerError => RetryDecision::RetryRequest,
                HTTPResponseErrorKind::UnknownHostError => RetryDecision::TryNextServer,
                HTTPResponseErrorKind::SendError => RetryDecision::RetryRequest,
                HTTPResponseErrorKind::ReceiveError | HTTPResponseErrorKind::UnknownError => {
                    if is_idempotent(request, opts.idempotent()) {
                        RetryDecision::RetryRequest
                    } else {
                        RetryDecision::DontRetry
                    }
                }
                HTTPResponseErrorKind::LocalIOError => RetryDecision::DontRetry,
                HTTPResponseErrorKind::TimeoutError => RetryDecision::RetryRequest,
                HTTPResponseErrorKind::SSLError => RetryDecision::TryAlternativeEndpoints,
                HTTPResponseErrorKind::TooManyRedirect => RetryDecision::DontRetry,
                HTTPResponseErrorKind::UserCanceled => RetryDecision::DontRetry,
                _ => RetryDecision::RetryRequest,
            },
            ResponseErrorKind::UnexpectedStatusCode(_) => RetryDecision::DontRetry,
            ResponseErrorKind::StatusCodeError(status_code) => match status_code.as_u16() {
                0..=399 => panic!("Should not arrive here"),
                400..=501
                | 579
                | 599
                | 608
                | 612
                | 614
                | 616
                | 618
                | 630
                | 631
                | 632
                | 640
                | 701 => RetryDecision::DontRetry,
                509 | 573 => RetryDecision::Throttled,
                _ => RetryDecision::TryNextServer,
            },
            ResponseErrorKind::ParseResponseError | ResponseErrorKind::UnexpectedEof => {
                if is_idempotent(request, opts.idempotent()) {
                    RetryDecision::RetryRequest
                } else {
                    RetryDecision::DontRetry
                }
            }
            ResponseErrorKind::MaliciousResponse => RetryDecision::RetryRequest,
            ResponseErrorKind::NoTry => RetryDecision::DontRetry,
        }
        .into();

        #[inline]
        fn is_idempotent(request: &HTTPRequest, idempotent: Idempotent) -> bool {
            match idempotent {
                Idempotent::Always => true,
                Idempotent::Default => request.method().is_idempotent(),
                Idempotent::Never => false,
            }
        }
    }

    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline]
    fn as_request_retrier(&self) -> &dyn RequestRetrier {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::super::{ResponseError, RetriedStatsInfo},
        *,
    };
    use qiniu_http::{Method as HTTPMethod, Uri as HTTPUri};
    use std::{convert::TryFrom, error::Error, result::Result};

    #[test]
    fn test_error_retrier_idempotent() -> Result<(), Box<dyn Error>> {
        let uri = HTTPUri::try_from("http://localhost/abc")?;

        let retrier = ErrorRetrier;
        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri.to_owned())
                .method(HTTPMethod::GET)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Default,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &RetriedStatsInfo::default(),
            ),
        );
        assert_eq!(result.decision(), RetryDecision::RetryRequest);

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri.to_owned())
                .method(HTTPMethod::GET)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Never,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &RetriedStatsInfo::default(),
            ),
        );
        assert_eq!(result.decision(), RetryDecision::DontRetry);

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri.to_owned())
                .method(HTTPMethod::POST)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Default,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &RetriedStatsInfo::default(),
            ),
        );
        assert_eq!(result.decision(), RetryDecision::DontRetry);

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri.to_owned())
                .method(HTTPMethod::POST)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Always,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &RetriedStatsInfo::default(),
            ),
        );
        assert_eq!(result.decision(), RetryDecision::RetryRequest);

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri)
                .method(HTTPMethod::POST)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Always,
                &ResponseError::new(HTTPResponseErrorKind::InvalidURL.into(), "Test Error"),
                &RetriedStatsInfo::default(),
            ),
        );
        assert_eq!(result.decision(), RetryDecision::TryNextServer);

        Ok(())
    }

    #[test]
    fn test_error_retrier_retries() -> Result<(), Box<dyn Error>> {
        let uri = HTTPUri::try_from("http://localhost/abc")?;

        let retrier = ErrorRetrier;
        let mut retried = RetriedStatsInfo::default();
        retried.increase();
        retried.increase();

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri.to_owned())
                .method(HTTPMethod::GET)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Default,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &retried,
            ),
        );
        assert_eq!(result.decision(), RetryDecision::RetryRequest);

        retried.switch_endpoint();

        let result = retrier.retry(
            &mut HTTPRequest::builder()
                .url(uri)
                .method(HTTPMethod::GET)
                .build(),
            &RequestRetrierOptions::new(
                Idempotent::Default,
                &ResponseError::new(HTTPResponseErrorKind::ReceiveError.into(), "Test Error"),
                &retried,
            ),
        );
        assert_eq!(result.decision(), RetryDecision::RetryRequest);

        Ok(())
    }
}