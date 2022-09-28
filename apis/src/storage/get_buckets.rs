// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
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
        Self(serde_json::Value::Array(Default::default()))
    }
}
impl From<ResponseBody> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody) -> Self {
        val.0
    }
}
impl AsRef<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        &self.0
    }
}
impl AsMut<serde_json::Value> for ResponseBody {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        &mut self.0
    }
}
impl ResponseBody {
    #[doc = "解析 JSON 得到 String 列表"]
    pub fn to_str_vec(&self) -> Vec<&str> {
        self.0
            .as_array()
            .unwrap()
            .iter()
            .map(|ele| ele.as_str().unwrap())
            .collect()
    }
}
impl From<Vec<String>> for ResponseBody {
    #[inline]
    fn from(val: Vec<String>) -> Self {
        Self(serde_json::Value::from(val))
    }
}
impl ResponseBody {
    #[doc = "获取数组的长度"]
    pub fn len(&self) -> usize {
        self.0.as_array().unwrap().len()
    }
    #[doc = "数组是否为空"]
    pub fn is_empty(&self) -> bool {
        self.0.as_array().unwrap().is_empty()
    }
}
impl ResponseBody {
    #[doc = "在列表的指定位置插入 JSON String"]
    pub fn insert_str(&mut self, index: usize, val: String) {
        self.0.as_array_mut().unwrap().insert(index, val.into());
    }
}
impl ResponseBody {
    #[doc = "在列表的指定位置移出 JSON String"]
    pub fn remove_as_str(&mut self, index: usize) -> Option<String> {
        match self.0.as_array_mut().unwrap().remove(index) {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        }
    }
}
impl ResponseBody {
    #[doc = "在列表尾部追加 JSON String"]
    pub fn push_str(&mut self, val: String) {
        self.0.as_array_mut().unwrap().push(val.into());
    }
}
impl ResponseBody {
    #[doc = "在列表尾部取出 JSON String"]
    pub fn pop_as_str(&mut self) -> Option<String> {
        self.0.as_array_mut().unwrap().pop().and_then(|val| match val {
            serde_json::Value::String(s) => Some(s),
            _ => None,
        })
    }
}
#[doc = "API 调用客户端"]
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    #[doc = "创建一个新的阻塞请求，该方法的异步版本为 [`Self::new_async_request`]"]
    pub fn new_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        credential: impl qiniu_http_client::credential::CredentialProvider + Clone + 'client,
    ) -> SyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self.0.get(&[qiniu_http_client::ServiceName::Uc], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("buckets");
            builder.accept_json();
            builder
        })
    }
    #[inline]
    #[cfg(feature = "async")]
    #[doc = "创建一个新的异步请求"]
    pub fn new_async_request<E: qiniu_http_client::EndpointsProvider + 'client>(
        &self,
        endpoints_provider: E,
        credential: impl qiniu_http_client::credential::CredentialProvider + Clone + 'client,
    ) -> AsyncRequestBuilder<'client, E> {
        RequestBuilder({
            let mut builder = self
                .0
                .async_get(&[qiniu_http_client::ServiceName::Uc], endpoints_provider);
            builder.authorization(qiniu_http_client::Authorization::v2(credential));
            builder.idempotent(qiniu_http_client::Idempotent::Default);
            builder.path("buckets");
            builder.accept_json();
            builder
        })
    }
}
#[derive(Debug)]
#[doc = "API 请求构造器"]
pub struct RequestBuilder<'req, B, E>(qiniu_http_client::RequestBuilder<'req, B, E>);
#[doc = "API 阻塞请求构造器"]
pub type SyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::SyncRequestBody<'req>, E>;
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
#[doc = "API 异步请求构造器"]
pub type AsyncRequestBuilder<'req, E> = RequestBuilder<'req, qiniu_http_client::AsyncRequestBody<'req>, E>;
impl<'req, B, E> RequestBuilder<'req, B, E> {
    #[inline]
    #[doc = "设置是否使用 HTTPS"]
    pub fn use_https(&mut self, use_https: bool) -> &mut Self {
        self.0.use_https(use_https);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 协议版本"]
    pub fn version(&mut self, version: qiniu_http_client::http::Version) -> &mut Self {
        self.0.version(version);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求头"]
    pub fn headers(
        &mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> &mut Self {
        self.0.headers(headers);
        self
    }
    #[inline]
    #[doc = "添加 HTTP 请求头"]
    pub fn set_header(
        &mut self,
        header_name: impl qiniu_http_client::http::header::IntoHeaderName,
        header_value: impl Into<qiniu_http_client::http::HeaderValue>,
    ) -> &mut Self {
        self.0.set_header(header_name, header_value);
        self
    }
    #[inline]
    #[doc = "设置查询参数"]
    pub fn query(&mut self, query: impl Into<std::borrow::Cow<'req, str>>) -> &mut Self {
        self.0.query(query);
        self
    }
    #[inline]
    #[doc = "设置查询参数"]
    pub fn query_pairs(&mut self, query_pairs: impl Into<Vec<qiniu_http_client::QueryPair<'req>>>) -> &mut Self {
        self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    #[doc = "追加查询参数"]
    pub fn append_query_pair(
        &mut self,
        query_pair_key: impl Into<qiniu_http_client::QueryPairKey<'req>>,
        query_pair_value: impl Into<qiniu_http_client::QueryPairValue<'req>>,
    ) -> &mut Self {
        self.0.append_query_pair(query_pair_key, query_pair_value);
        self
    }
    #[inline]
    #[doc = "设置扩展信息"]
    pub fn extensions(&mut self, extensions: qiniu_http_client::http::Extensions) -> &mut Self {
        self.0.extensions(extensions);
        self
    }
    #[doc = "添加扩展信息"]
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(&mut self, val: T) -> &mut Self {
        self.0.add_extension(val);
        self
    }
    #[inline]
    #[doc = "上传进度回调函数"]
    pub fn on_uploading_progress(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::TransferProgressInfo,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    #[doc = "设置响应状态码回调函数"]
    pub fn on_receive_response_status(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                qiniu_http_client::http::StatusCode,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    #[doc = "设置响应 HTTP 头回调函数"]
    pub fn on_receive_response_header(
        &mut self,
        callback: impl Fn(
                &dyn qiniu_http_client::SimplifiedCallbackContext,
                &qiniu_http_client::http::HeaderName,
                &qiniu_http_client::http::HeaderValue,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    #[doc = "设置域名解析前回调函数"]
    pub fn on_to_resolve_domain(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &str) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    #[doc = "设置域名解析成功回调函数"]
    pub fn on_domain_resolved(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &str,
                &qiniu_http_client::ResolveAnswers,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    #[doc = "设置 IP 地址选择前回调函数"]
    pub fn on_to_choose_ips(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::CallbackContext, &[qiniu_http_client::IpAddrWithPort]) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    #[doc = "设置 IP 地址选择成功回调函数"]
    pub fn on_ips_chosen(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::CallbackContext,
                &[qiniu_http_client::IpAddrWithPort],
                &[qiniu_http_client::IpAddrWithPort],
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求签名前回调函数"]
    pub fn on_before_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    #[doc = "设置 HTTP 请求前回调函数"]
    pub fn on_after_request_signed(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext) -> anyhow::Result<()> + Send + Sync + 'req,
    ) -> &mut Self {
        self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    #[doc = "设置响应成功回调函数"]
    pub fn on_response(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &qiniu_http_client::http::ResponseParts,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_response(callback);
        self
    }
    #[inline]
    #[doc = "设置响应错误回调函数"]
    pub fn on_error(
        &mut self,
        callback: impl Fn(
                &mut dyn qiniu_http_client::ExtendedCallbackContext,
                &mut qiniu_http_client::ResponseError,
            ) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_error(callback);
        self
    }
    #[inline]
    #[doc = "设置退避前回调函数"]
    pub fn on_before_backoff(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    #[doc = "设置退避后回调函数"]
    pub fn on_after_backoff(
        &mut self,
        callback: impl Fn(&mut dyn qiniu_http_client::ExtendedCallbackContext, std::time::Duration) -> anyhow::Result<()>
            + Send
            + Sync
            + 'req,
    ) -> &mut Self {
        self.0.on_after_backoff(callback);
        self
    }
    #[inline]
    #[doc = "获取 HTTP 请求构建器部分参数"]
    pub fn parts(&self) -> &qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts()
    }
    #[inline]
    #[doc = "获取 HTTP 请求构建器部分参数的可变引用"]
    pub fn parts_mut(&mut self) -> &mut qiniu_http_client::RequestBuilderParts<'req> {
        self.0.parts_mut()
    }
}
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> SyncRequestBuilder<'req, E> {
    #[doc = "阻塞发起 HTTP 请求"]
    pub fn call(&mut self) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req, E: qiniu_http_client::EndpointsProvider + Clone + 'req> AsyncRequestBuilder<'req, E> {
    #[doc = "异步发起 HTTP 请求"]
    pub async fn call(&mut self) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody>> {
        let request = &mut self.0;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}
