use super::{
    super::{Authorization, Idempotent, QueryPairs, RequestWithoutEndpoints, RetriedStatsInfo},
    context::CallbackContext,
    simplified::SimplifiedCallbackContext,
};
use qiniu_http::{
    uri::Scheme, Extensions, HeaderMap, Method, Request as HTTPRequest, Uri, UserAgent, Version,
};
use std::{borrow::Cow, net::IpAddr};

pub trait ExtendedCallbackContext: CallbackContext {
    fn url(&self) -> &Uri;
    fn version_mut(&mut self) -> &mut Version;
    fn headers_mut(&mut self) -> &mut HeaderMap;
    fn user_agent(&self) -> UserAgent;
    fn set_appended_user_agent(&mut self, appended_user_agent: UserAgent);
    fn resolved_ip_addrs(&self) -> Option<&[IpAddr]>;
    fn set_resolved_ip_addrs(&mut self, resolved_ip_addrs: Vec<IpAddr>);
    fn retried(&self) -> &RetriedStatsInfo;
}

#[derive(Debug)]
pub(in super::super) struct ExtendedCallbackContextImpl<
    'reqref,
    'req,
    'retried,
    'httpreqref,
    'httpreq,
> {
    request: &'reqref RequestWithoutEndpoints<'req>,
    http_request: &'httpreqref mut HTTPRequest<'httpreq>,
    retried: &'retried RetriedStatsInfo,
}

impl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq>
    ExtendedCallbackContextImpl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq>
{
    pub(in super::super) fn new(
        request: &'reqref RequestWithoutEndpoints<'req>,
        http_request: &'httpreqref mut HTTPRequest<'httpreq>,
        retried: &'retried RetriedStatsInfo,
    ) -> Self {
        Self {
            request,
            http_request,
            retried,
        }
    }
}

impl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq> SimplifiedCallbackContext
    for ExtendedCallbackContextImpl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq>
{
    #[inline]
    fn use_https(&self) -> bool {
        self.http_request.url().scheme() == Some(&Scheme::HTTPS)
    }

    #[inline]
    fn method(&self) -> &Method {
        self.http_request.method()
    }

    #[inline]
    fn version(&self) -> Version {
        self.http_request.version()
    }

    #[inline]
    fn path(&self) -> &str {
        self.request.path()
    }

    #[inline]
    fn query(&self) -> &str {
        self.request.query()
    }

    #[inline]
    fn query_pairs(&self) -> &QueryPairs {
        self.request.query_pairs()
    }

    #[inline]
    fn headers(&self) -> &HeaderMap {
        self.http_request.headers()
    }

    #[inline]
    fn body(&self) -> &[u8] {
        self.http_request.body()
    }

    #[inline]
    fn appended_user_agent(&self) -> &UserAgent {
        self.http_request.appended_user_agent()
    }

    #[inline]
    fn authorization(&self) -> Option<&Authorization> {
        self.request.authorization()
    }

    #[inline]
    fn idempotent(&self) -> Idempotent {
        self.request.idempotent()
    }
}

impl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq> CallbackContext
    for ExtendedCallbackContextImpl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq>
{
    #[inline]
    fn extensions(&self) -> &Extensions {
        self.http_request.extensions()
    }

    #[inline]
    fn extensions_mut(&mut self) -> &mut Extensions {
        self.http_request.extensions_mut()
    }
}

impl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq> ExtendedCallbackContext
    for ExtendedCallbackContextImpl<'reqref, 'req, 'retried, 'httpreqref, 'httpreq>
{
    #[inline]
    fn url(&self) -> &Uri {
        self.http_request.url()
    }

    #[inline]
    fn version_mut(&mut self) -> &mut Version {
        self.http_request.version_mut()
    }

    #[inline]
    fn headers_mut(&mut self) -> &mut HeaderMap {
        self.http_request.headers_mut()
    }

    #[inline]
    fn user_agent(&self) -> UserAgent {
        self.http_request.user_agent()
    }

    #[inline]
    fn set_appended_user_agent(&mut self, appended_user_agent: UserAgent) {
        *self.http_request.appended_user_agent_mut() = appended_user_agent;
    }

    #[inline]
    fn resolved_ip_addrs(&self) -> Option<&[IpAddr]> {
        self.http_request.resolved_ip_addrs()
    }

    #[inline]
    fn set_resolved_ip_addrs(&mut self, resolved_ip_addrs: Vec<IpAddr>) {
        *self.http_request.resolved_ip_addrs_mut() = Some(Cow::Owned(resolved_ip_addrs));
    }

    #[inline]
    fn retried(&self) -> &RetriedStatsInfo {
        self.retried
    }
}