use super::{super::authorization::Authorization, Idempotent, Queries};
use qiniu_http::{Headers, Method, RequestBody};
use std::{fmt, time::Duration};

pub(super) struct RequestData<'r> {
    pub(super) use_https: Option<bool>,
    pub(super) method: Method,
    pub(super) queries: Queries<'r>,
    pub(super) headers: Headers<'r>,
    pub(super) body: RequestBody<'r>,
    pub(super) authorization: Option<Authorization>,
    pub(super) idempotent: Idempotent,
    pub(super) read_body: bool,
    pub(super) follow_redirection: bool,
    pub(super) connect_timeout: Option<Duration>,
    pub(super) request_timeout: Option<Duration>,
    pub(super) tcp_keepalive_idle_timeout: Option<Duration>,
    pub(super) tcp_keepalive_probe_interval: Option<Duration>,
    pub(super) low_transfer_speed: Option<u32>,
    pub(super) low_transfer_speed_timeout: Option<Duration>,
}

impl fmt::Debug for RequestData<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! field {
            ($ctx:ident,$method:ident) => {
                $ctx.field("$method", &self.$method)
            };
        }
        let s = &mut f.debug_struct("RequestData");
        field!(s, use_https);
        field!(s, method);
        field!(s, queries);
        field!(s, headers);
        field!(s, body);
        field!(s, authorization);
        field!(s, idempotent);
        field!(s, read_body);
        field!(s, follow_redirection);
        field!(s, connect_timeout);
        field!(s, request_timeout);
        field!(s, tcp_keepalive_idle_timeout);
        field!(s, tcp_keepalive_probe_interval);
        field!(s, low_transfer_speed);
        field!(s, low_transfer_speed_timeout);
        s.finish()
    }
}
