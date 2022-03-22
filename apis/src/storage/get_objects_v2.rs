// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Clone, Default)]
#[doc = "调用 API 所用的 URL 查询参数"]
pub struct QueryParams<'a> {
    map: indexmap::IndexMap<qiniu_http_client::QueryPairKey<'a>, qiniu_http_client::QueryPairValue<'a>>,
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    pub fn insert(
        mut self,
        query_pair_key: qiniu_http_client::QueryPairKey<'a>,
        query_pair_value: qiniu_http_client::QueryPairValue<'a>,
    ) -> Self {
        self.map.insert(query_pair_key, query_pair_value);
        self
    }
    fn build(self) -> Vec<qiniu_http_client::QueryPair<'a>> {
        Vec::from_iter(self.map)
    }
}
impl<'a> From<QueryParams<'a>> for Vec<qiniu_http_client::QueryPair<'a>> {
    #[inline]
    fn from(map: QueryParams<'a>) -> Self {
        map.build()
    }
}
impl<'a> QueryParams<'a> {
    #[inline]
    #[must_use]
    #[doc = "指定存储空间"]
    pub fn set_bucket_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("bucket".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "上一次列举返回的位置标记，作为本次列举的起点信息"]
    pub fn set_marker_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("marker".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i8(self, value: i8) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i16(self, value: i16) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i32(self, value: i32) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_i64(self, value: i64) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_isize(self, value: isize) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u8(self, value: u8) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u16(self, value: u16) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u32(self, value: u32) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_u64(self, value: u64) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "本次列举的条目数，范围为 1-1000"]
    pub fn set_limit_as_usize(self, value: usize) -> Self {
        self.insert("limit".into(), value.to_string().into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定前缀，只有资源名匹配该前缀的资源会被列出"]
    pub fn set_prefix_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("prefix".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "指定目录分隔符，列出所有公共前缀（模拟列出目录效果）"]
    pub fn set_delimiter_as_str(self, value: impl Into<qiniu_http_client::QueryPairValue<'a>>) -> Self {
        self.insert("delimiter".into(), value.into())
    }
    #[inline]
    #[must_use]
    #[doc = "如果文件是通过分片上传的，是否返回对应的分片信息"]
    pub fn set_need_parts_as_bool(self, value: bool) -> Self {
        self.insert("needparts".into(), value.to_string().into())
    }
}
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    pub fn new_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        credential: impl qiniu_http_client::credential::CredentialProvider + std::clone::Clone + 'client,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self.0.get(&[qiniu_http_client::ServiceName::Rsf], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("v2/list");
            builder.accept_application_octet_stream();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        credential: impl qiniu_http_client::credential::CredentialProvider + std::clone::Clone + 'client,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_get(&[qiniu_http_client::ServiceName::Rsf], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("v2/list");
            builder.accept_application_octet_stream();
            builder
        })
    }
}
#[derive(Debug)]
pub struct RequestBuilder<'req, B: 'req, E: 'req>(qiniu_http_client::RequestBuilder<'req, B, E>);
pub type SyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::SyncRequestBody<'req>, E>;
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
pub type AsyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::AsyncRequestBody<'req>, E>;
impl<'req, B: 'req, E: 'req> RequestBuilder<'req, B, E> {
    #[inline]
    pub fn use_https(&mut self, use_https: bool) -> &mut Self {
        self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(&mut self, version: qiniu_http_client::http::Version) -> &mut Self {
        self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        &mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> &mut Self {
        self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(&mut self, query_pairs: impl Into<Vec<qiniu_http_client::QueryPair<'req>>>) -> &mut Self {
        self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(&mut self, extensions: qiniu_http_client::http::Extensions) -> &mut Self {
        self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(&mut self, val: T) -> &mut Self {
        self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::TransferProgressInfo,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_response(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::http::ResponseParts,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_response(callback);
        self
    }
    #[inline]
    pub fn on_error(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::ResponseError,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                std::time::Duration,
            ) -> qiniu_http_client::CallbackResult
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_backoff(callback);
        self
    }
    #[inline]
    pub fn parts_mut(&mut self) -> &mut qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts_mut()
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> SyncRequestBuilder<'req, E> {
    pub fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<qiniu_http_client::SyncResponseBody>> {
        let request = &mut self.0;
        let response = request.call()?;
        let parsed = response;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        &mut self,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<qiniu_http_client::AsyncResponseBody>> {
        let request = &mut self.0;
        let response = request.call().await?;
        let parsed = response;
        Ok(parsed)
    }
}
