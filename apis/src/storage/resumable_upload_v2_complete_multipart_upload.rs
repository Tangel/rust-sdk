#[derive(Debug, Clone)]
#[doc = "调用 API 所用的路径参数"]
pub struct PathParams {
    r#bucket_name: Option<std::borrow::Cow<'static, str>>,
    r#object_name: Option<std::borrow::Cow<'static, str>>,
    r#upload_id: Option<std::borrow::Cow<'static, str>>,
    extended_segments: Vec<std::borrow::Cow<'static, str>>,
}
impl PathParams {
    #[inline]
    pub fn push_segment(mut self, segment: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        self.extended_segments.push(segment.into());
        self
    }
    #[inline]
    fn build(self) -> Vec<std::borrow::Cow<'static, str>> {
        let mut all_segments: Vec<_> = Default::default();
        if let Some(segment) = self.r#bucket_name {
            all_segments.push(segment);
        }
        all_segments.push(std::borrow::Cow::Borrowed("objects"));
        all_segments.push(
            self.r#object_name
                .unwrap_or(std::borrow::Cow::Borrowed("~")),
        );
        if let Some(segment) = self.r#upload_id {
            all_segments.push(std::borrow::Cow::Borrowed("uploads"));
            all_segments.push(segment);
        }
        all_segments.extend(self.extended_segments);
        all_segments
    }
}
impl PathParams {
    #[inline]
    #[doc = "存储空间名称"]
    pub fn set_bucket_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#bucket_name = Some(value.into());
        self
    }
    #[inline]
    #[doc = "对象名称"]
    pub fn set_object_name_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#object_name = Some(qiniu_utils::base64::urlsafe(value.into().as_bytes()).into());
        self
    }
    #[inline]
    #[doc = "在服务端申请的 Multipart Upload 任务 id"]
    pub fn set_upload_id_as_str(
        mut self,
        value: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Self {
        self.r#upload_id = Some(value.into());
        self
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "调用 API 所用的请求体参数"]
pub struct RequestBody<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> RequestBody<'a> {
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<RequestBody<'a>> for serde_json::Value {
    #[inline]
    fn from(val: RequestBody<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for RequestBody<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for RequestBody<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "单个分片信息"]
pub struct PartInfo<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> PartInfo<'a> {
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<PartInfo<'a>> for serde_json::Value {
    #[inline]
    fn from(val: PartInfo<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for PartInfo<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for PartInfo<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_int(&self) -> i64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_i64()
            .unwrap()
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_int(&mut self, new: i64) -> Option<i64> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_i64())
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "获取 每一个上传的分片都有一个标识它的号码"]
    pub fn get_part_number_as_uint(&self) -> u64 {
        self.0
            .as_object()
            .unwrap()
            .get("partNumber")
            .unwrap()
            .as_u64()
            .unwrap()
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "设置 每一个上传的分片都有一个标识它的号码"]
    pub fn set_part_number_as_uint(&mut self, new: u64) -> Option<u64> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("partNumber".to_owned(), new.into())
            .and_then(|val| val.as_u64())
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "获取 上传块的 etag"]
    pub fn get_etag_as_str(&self) -> &str {
        self.0
            .as_object()
            .unwrap()
            .get("etag")
            .unwrap()
            .as_str()
            .unwrap()
    }
}
impl<'a> PartInfo<'a> {
    #[inline]
    #[doc = "设置 上传块的 etag"]
    pub fn set_etag_as_str(&mut self, new: String) -> Option<String> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("etag".to_owned(), new.into())
            .and_then(|val| match val {
                serde_json::Value::String(s) => Some(s),
                _ => None,
            })
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "获取 已经上传的分片列表"]
    pub fn get_parts(&self) -> PartInfo {
        PartInfo::new(std::borrow::Cow::Borrowed(
            self.0.as_object().unwrap().get("parts").unwrap(),
        ))
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "设置 已经上传的分片列表"]
    pub fn set_parts(&mut self, new: PartInfo) -> Option<PartInfo> {
        self.0
            .to_mut()
            .as_object_mut()
            .unwrap()
            .insert("parts".to_owned(), new.into())
            .map(std::borrow::Cow::Owned)
            .map(PartInfo::new)
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "获取 上传的原始文件名，若未指定，则魔法变量中无法使用 fname，ext，suffix"]
    pub fn get_file_name_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("fname"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "设置 上传的原始文件名，若未指定，则魔法变量中无法使用 fname，ext，suffix"]
    pub fn set_file_name_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("fname".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "获取 若指定了则设置上传文件的 MIME 类型，若未指定，则根据文件内容自动检测 MIME 类型"]
    pub fn get_mime_type_as_str(&self) -> Option<&str> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("mime_type"))
            .and_then(|val| val.as_str())
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "设置 若指定了则设置上传文件的 MIME 类型，若未指定，则根据文件内容自动检测 MIME 类型"]
    pub fn set_mime_type_as_str(&mut self, new: String) -> Option<String> {
        self.0.to_mut().as_object_mut().and_then(|object| {
            object
                .insert("mime_type".to_owned(), new.into())
                .and_then(|val| match val {
                    serde_json::Value::String(s) => Some(s),
                    _ => None,
                })
        })
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "获取 用户自定义文件 metadata 信息的键值对，可以设置多个，MetaKey 和 MetaValue 都是 string，，其中 可以由字母、数字、下划线、减号组成，且长度小于等于 50，单个文件 MetaKey 和 MetaValue 总和大小不能超过 1024 字节，MetaKey 必须以 `x-qn-meta-` 作为前缀"]
    pub fn get_metadata(&self) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("metadata"))
            .map(|obj| crate::base_types::StringMap::new(std::borrow::Cow::Borrowed(obj)))
    }
    #[inline]
    #[doc = "设置 用户自定义文件 metadata 信息的键值对，可以设置多个，MetaKey 和 MetaValue 都是 string，，其中 可以由字母、数字、下划线、减号组成，且长度小于等于 50，单个文件 MetaKey 和 MetaValue 总和大小不能超过 1024 字节，MetaKey 必须以 `x-qn-meta-` 作为前缀"]
    pub fn set_metadata(
        &mut self,
        new: crate::base_types::StringMap,
    ) -> Option<crate::base_types::StringMap> {
        self.0
            .to_mut()
            .as_object_mut()
            .and_then(|object| object.insert("metadata".to_owned(), new.into()))
            .map(|obj| crate::base_types::StringMap::new(std::borrow::Cow::Owned(obj)))
    }
}
impl<'a> RequestBody<'a> {
    #[inline]
    #[doc = "获取 用户自定义变量"]
    pub fn get_custom_vars(&self) -> Option<crate::base_types::StringMap> {
        self.0
            .as_object()
            .and_then(|obj| obj.get("customVars"))
            .map(|obj| crate::base_types::StringMap::new(std::borrow::Cow::Borrowed(obj)))
    }
    #[inline]
    #[doc = "设置 用户自定义变量"]
    pub fn set_custom_vars(
        &mut self,
        new: crate::base_types::StringMap,
    ) -> Option<crate::base_types::StringMap> {
        self.0
            .to_mut()
            .as_object_mut()
            .and_then(|object| object.insert("customVars".to_owned(), new.into()))
            .map(|obj| crate::base_types::StringMap::new(std::borrow::Cow::Owned(obj)))
    }
}
#[derive(Clone, Debug, serde :: Serialize, serde :: Deserialize)]
#[serde(transparent)]
#[doc = "获取 API 所用的响应体参数"]
pub struct ResponseBody<'a>(std::borrow::Cow<'a, serde_json::Value>);
impl<'a> ResponseBody<'a> {
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(value: std::borrow::Cow<'a, serde_json::Value>) -> Self {
        Self(value)
    }
}
impl<'a> From<ResponseBody<'a>> for serde_json::Value {
    #[inline]
    fn from(val: ResponseBody<'a>) -> Self {
        val.0.into_owned()
    }
}
impl<'a> std::convert::AsRef<serde_json::Value> for ResponseBody<'a> {
    #[inline]
    fn as_ref(&self) -> &serde_json::Value {
        self.0.as_ref()
    }
}
impl<'a> std::convert::AsMut<serde_json::Value> for ResponseBody<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut serde_json::Value {
        self.0.to_mut()
    }
}
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    #[inline]
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
}
impl<'client> Client<'client> {
    #[inline]
    pub fn new_request(
        &self,
        into_endpoints: impl Into<qiniu_http_client::IntoEndpoints<'client>>,
        path_params: PathParams,
        upload_token: std::sync::Arc<dyn qiniu_http_client::upload_token::UploadTokenProvider>,
    ) -> SyncRequestBuilder {
        SyncRequestBuilder(
            self.0
                .post(&[qiniu_http_client::ServiceName::Up], into_endpoints.into())
                .authorization(qiniu_http_client::Authorization::uptoken(upload_token))
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path(crate::base_utils::join_path(
                    "/buckets",
                    "",
                    path_params.build(),
                ))
                .accept_json(),
        )
    }
    #[inline]
    #[cfg(feature = "async")]
    pub fn new_async_request(
        &self,
        into_endpoints: impl Into<qiniu_http_client::IntoEndpoints<'client>>,
        path_params: PathParams,
        upload_token: std::sync::Arc<dyn qiniu_http_client::upload_token::UploadTokenProvider>,
    ) -> AsyncRequestBuilder {
        AsyncRequestBuilder(
            self.0
                .async_post(&[qiniu_http_client::ServiceName::Up], into_endpoints.into())
                .authorization(qiniu_http_client::Authorization::uptoken(upload_token))
                .idempotent(qiniu_http_client::Idempotent::Default)
                .path(crate::base_utils::join_path(
                    "/buckets",
                    "",
                    path_params.build(),
                ))
                .accept_json(),
        )
    }
}
#[derive(Debug)]
pub struct SyncRequestBuilder<'req>(qiniu_http_client::SyncRequestBuilder<'req>);
#[derive(Debug)]
#[cfg(feature = "async")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
pub struct AsyncRequestBuilder<'req>(qiniu_http_client::AsyncRequestBuilder<'req>);
impl<'req> SyncRequestBuilder<'req> {
    #[inline]
    pub fn use_https(mut self, use_https: bool) -> Self {
        self.0 = self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(mut self, version: qiniu_http_client::http::Version) -> Self {
        self.0 = self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> Self {
        self.0 = self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> Self {
        self.0 = self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(mut self, extensions: qiniu_http_client::http::Extensions) -> Self {
        self.0 = self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.0 = self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(mut self, callback: qiniu_http_client::OnProgress) -> Self {
        self.0 = self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(mut self, callback: qiniu_http_client::OnStatusCode) -> Self {
        self.0 = self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(mut self, callback: qiniu_http_client::OnHeader) -> Self {
        self.0 = self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(mut self, callback: qiniu_http_client::OnToResolveDomain) -> Self {
        self.0 = self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(mut self, callback: qiniu_http_client::OnDomainResolved) -> Self {
        self.0 = self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(mut self, callback: qiniu_http_client::OnToChooseIPs) -> Self {
        self.0 = self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(mut self, callback: qiniu_http_client::OnIPsChosen) -> Self {
        self.0 = self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(mut self, callback: qiniu_http_client::OnRequest) -> Self {
        self.0 = self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(mut self, callback: qiniu_http_client::OnRequest) -> Self {
        self.0 = self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_success(mut self, callback: qiniu_http_client::OnSuccess) -> Self {
        self.0 = self.0.on_success(callback);
        self
    }
    #[inline]
    pub fn on_error(mut self, callback: qiniu_http_client::OnError) -> Self {
        self.0 = self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(mut self, callback: qiniu_http_client::OnRetry) -> Self {
        self.0 = self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(mut self, callback: qiniu_http_client::OnRetry) -> Self {
        self.0 = self.0.on_after_backoff(callback);
        self
    }
    pub fn call(
        self,
        body: &RequestBody<'_>,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody<'static>>> {
        let request = self.0.json(body)?;
        let response = request.call()?;
        let parsed = response.parse_json()?;
        Ok(parsed)
    }
}
#[cfg(feature = "async")]
impl<'req> AsyncRequestBuilder<'req> {
    #[inline]
    pub fn use_https(mut self, use_https: bool) -> Self {
        self.0 = self.0.use_https(use_https);
        self
    }
    #[inline]
    pub fn version(mut self, version: qiniu_http_client::http::Version) -> Self {
        self.0 = self.0.version(version);
        self
    }
    #[inline]
    pub fn headers(
        mut self,
        headers: impl Into<std::borrow::Cow<'req, qiniu_http_client::http::HeaderMap>>,
    ) -> Self {
        self.0 = self.0.headers(headers);
        self
    }
    #[inline]
    pub fn query_pairs(
        mut self,
        query_pairs: impl Into<qiniu_http_client::QueryPairs<'req>>,
    ) -> Self {
        self.0 = self.0.query_pairs(query_pairs);
        self
    }
    #[inline]
    pub fn extensions(mut self, extensions: qiniu_http_client::http::Extensions) -> Self {
        self.0 = self.0.extensions(extensions);
        self
    }
    #[inline]
    pub fn add_extension<T: Send + Sync + 'static>(mut self, val: T) -> Self {
        self.0 = self.0.add_extension(val);
        self
    }
    #[inline]
    pub fn on_uploading_progress(mut self, callback: qiniu_http_client::OnProgress) -> Self {
        self.0 = self.0.on_uploading_progress(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_status(mut self, callback: qiniu_http_client::OnStatusCode) -> Self {
        self.0 = self.0.on_receive_response_status(callback);
        self
    }
    #[inline]
    pub fn on_receive_response_header(mut self, callback: qiniu_http_client::OnHeader) -> Self {
        self.0 = self.0.on_receive_response_header(callback);
        self
    }
    #[inline]
    pub fn on_to_resolve_domain(mut self, callback: qiniu_http_client::OnToResolveDomain) -> Self {
        self.0 = self.0.on_to_resolve_domain(callback);
        self
    }
    #[inline]
    pub fn on_domain_resolved(mut self, callback: qiniu_http_client::OnDomainResolved) -> Self {
        self.0 = self.0.on_domain_resolved(callback);
        self
    }
    #[inline]
    pub fn on_to_choose_ips(mut self, callback: qiniu_http_client::OnToChooseIPs) -> Self {
        self.0 = self.0.on_to_choose_ips(callback);
        self
    }
    #[inline]
    pub fn on_ips_chosen(mut self, callback: qiniu_http_client::OnIPsChosen) -> Self {
        self.0 = self.0.on_ips_chosen(callback);
        self
    }
    #[inline]
    pub fn on_before_request_signed(mut self, callback: qiniu_http_client::OnRequest) -> Self {
        self.0 = self.0.on_before_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_after_request_signed(mut self, callback: qiniu_http_client::OnRequest) -> Self {
        self.0 = self.0.on_after_request_signed(callback);
        self
    }
    #[inline]
    pub fn on_success(mut self, callback: qiniu_http_client::OnSuccess) -> Self {
        self.0 = self.0.on_success(callback);
        self
    }
    #[inline]
    pub fn on_error(mut self, callback: qiniu_http_client::OnError) -> Self {
        self.0 = self.0.on_error(callback);
        self
    }
    #[inline]
    pub fn on_before_backoff(mut self, callback: qiniu_http_client::OnRetry) -> Self {
        self.0 = self.0.on_before_backoff(callback);
        self
    }
    #[inline]
    pub fn on_after_backoff(mut self, callback: qiniu_http_client::OnRetry) -> Self {
        self.0 = self.0.on_after_backoff(callback);
        self
    }
    pub async fn call(
        self,
        body: &RequestBody<'_>,
    ) -> qiniu_http_client::ApiResult<qiniu_http_client::Response<ResponseBody<'static>>> {
        let request = self.0.json(body)?;
        let response = request.call().await?;
        let parsed = response.parse_json().await?;
        Ok(parsed)
    }
}