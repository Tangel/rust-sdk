use super::{RequestRetrier, RequestRetrierOptions, RetryDecision, RetryResult};
use qiniu_http::Request as HTTPRequest;
use std::any::Any;

const DEFAULT_RETIES: usize = 2;

#[derive(Clone, Debug)]
pub struct LimitedRetrier<R> {
    retrier: R,
    retries: usize,
}

impl<R> LimitedRetrier<R> {
    #[inline]
    pub fn new(retrier: R, retries: usize) -> Self {
        Self { retrier, retries }
    }
}

impl<R: Default> Default for LimitedRetrier<R> {
    #[inline]
    fn default() -> Self {
        Self::new(R::default(), DEFAULT_RETIES)
    }
}

impl<R: RequestRetrier> RequestRetrier for LimitedRetrier<R> {
    #[inline]
    fn retry(&self, request: &mut HTTPRequest, opts: &RequestRetrierOptions) -> RetryResult {
        match self.retrier.retry(request, opts).decision() {
            RetryDecision::RetryRequest | RetryDecision::Throttled
                if opts.retried().retried_on_current_endpoint() >= self.retries =>
            {
                RetryDecision::TryNextServer
            }
            result => result,
        }
        .into()
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
        super::{ErrorRetrier, Idempotent, ResponseError, RetriedStatsInfo},
        *,
    };
    use qiniu_http::{
        Method as HTTPMethod, ResponseErrorKind as HTTPResponseErrorKind, Uri as HTTPUri,
    };
    use std::{convert::TryFrom, error::Error, result::Result};

    #[test]
    fn test_limited_retrier_retries() -> Result<(), Box<dyn Error>> {
        let uri = HTTPUri::try_from("http://localhost/abc")?;

        let retrier = LimitedRetrier::new(ErrorRetrier, 2);
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
        assert_eq!(result.decision(), RetryDecision::TryNextServer);

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