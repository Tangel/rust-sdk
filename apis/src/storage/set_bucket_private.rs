// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[derive(Debug, Default)]
#[doc = "调用 API 所用的请求体参数"]
pub struct RequestBody {
    r#bucket: Option<std::borrow::Cow<'static, str>>,
    r#is_private: Option<std::borrow::Cow<'static, str>>,
    extended_pairs: Vec<(std::borrow::Cow<'static, str>, Option<std::borrow::Cow<'static, str>>)>,
}
impl RequestBody {
    #[inline]
    #[must_use]
    pub fn append_pair(
        mut self,
        key: impl Into<std::borrow::Cow<'static, str>>,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.extended_pairs.push((key.into(), Some(value.into())));
        self
    }
    fn build(self) -> Vec<(std::borrow::Cow<'static, str>, Option<std::borrow::Cow<'static, str>>)> {
        let mut all_pairs: Vec<_> = Default::default();
        if let Some(value) = self.r#bucket {
            all_pairs.push(("bucket".into(), Some(value)));
        }
        if let Some(value) = self.r#is_private {
            all_pairs.push(("private".into(), Some(value)));
        }
        all_pairs.extend(self.extended_pairs);
        all_pairs
    }
}
impl IntoIterator for RequestBody {
    type Item = (std::borrow::Cow<'static, str>, Option<std::borrow::Cow<'static, str>>);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.build().into_iter()
    }
}
impl RequestBody {
    #[inline]
    #[must_use]
    #[doc = "空间名称"]
    pub fn set_bucket_as_str(mut self, value: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.r#bucket = Some(value.into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_i8(mut self, value: i8) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_i16(mut self, value: i16) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_i32(mut self, value: i32) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_i64(mut self, value: i64) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_isize(mut self, value: isize) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_u8(mut self, value: u8) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_u16(mut self, value: u16) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_u32(mut self, value: u32) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_u64(mut self, value: u64) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
    #[inline]
    #[must_use]
    #[doc = "`0`: 公开，`1`: 私有"]
    pub fn set_is_private_as_usize(mut self, value: usize) -> Self {
        self.r#is_private = Some(value.to_string().into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "获取 API 所用的响应体参数"]
pub struct ResponseBody(serde_json::Value);
impl ResponseBody {
    #[allow(dead_code)]
    pub(crate) fn new(value: serde_json::Value) -> Self {
        Self(value)
    }
}
impl Default for ResponseBody {
    #[inline]
    fn default() -> Self {
        Self(serde_json::Value::Object(Default::default()))
    }
}
impl From<ResponseBody> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody) -> Self {
        val.0
    }
}
impl std::convert::AsRef<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl std::convert::AsMut<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
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
            let mut builder = self.0.post(&[qiniu_http_client::ServiceName::Uc], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path("private");
            builder.accept_json();
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
                .async_post(&[qiniu_http_client::ServiceName::Uc], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Always);
            builder.path("private");
            builder.accept_json();
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
        body: RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.post_form(body);
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    pub async fn call(
        &mut self,
        body: RequestBody,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = self.0.post_form(body);
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
