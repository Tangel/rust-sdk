// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    missing_debug_implementations,
    large_assignments,
    exported_private_dependencies,
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_docs,
    non_ascii_idents,
    indirect_structural_match,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![doc = r" # qiniu-apis"]
#![doc = r""]
#![doc = r" ## 七牛 HTTP API 库"]
#![doc = r""]
#![doc = r" 这是一个基于 `qiniu-apis-specs` 自动生成的 Rust 库，基于 `qiniu-http-client`，用于调用七牛 HTTP API。"]
#![doc = r" 该库同时提供阻塞客户端和异步客户端，异步客户端则需要启用 `async` 功能。"]
#![doc = r" 该库致力于根据 [`qiniu-apis-specs`](https://github.com/bachue/rust-sdk/tree/master/api-specs) 提供的 YAML 描述文件，在不理解业务逻辑的前提下，提供简单的封装方法方便用户正确调用 API。"]
#![doc = r""]
#![doc = r" 该库可以通过启用不同的功能来选择不同的 HTTP 客户端实现，"]
#![doc = r" 例如可以通过启用 `ureq` 功能导入 `qiniu-ureq` 库作为 HTTP 客户端，"]
#![doc = r" 通过启用 `reqwest` 功能导入 `qiniu-reqwest` 库作为 HTTP 客户端，"]
#![doc = r" 通过启用 `isahc` 功能导入 `qiniu-isahc` 库作为 HTTP 客户端。"]
#![doc = r" 您也可以显式传入任何基于 `qiniu-http` 接口的 HTTP 客户端实现来提供给 `qiniu-apis` 使用。"]
#![doc = r""]
#![doc = r" 由于是自动生成库，无法为每个接口提供代码示例，下面选择几个典型的场景来讲解如何使用该库："]
#![doc = r""]
#![doc = r" ### 功能描述"]
#![doc = r""]
#![doc = r" #### `async`"]
#![doc = r""]
#![doc = r" 启用异步接口。"]
#![doc = r""]
#![doc = r" #### `ureq`"]
#![doc = r""]
#![doc = r" 导入 `qiniu-ureq` 作为 HTTP 客户端。"]
#![doc = r""]
#![doc = r" #### `isahc`"]
#![doc = r""]
#![doc = r" 导入 `qiniu-isahc` 作为 HTTP 客户端。"]
#![doc = r""]
#![doc = r" #### `reqwest`"]
#![doc = r""]
#![doc = r" 导入 `qiniu-reqwest` 作为 HTTP 客户端。"]
#![doc = r""]
#![doc = r" #### `c_ares`"]
#![doc = r""]
#![doc = r" 启用 `c-ares` 库作为 DNS 解析器。"]
#![doc = r""]
#![doc = r" #### `trust_dns`"]
#![doc = r""]
#![doc = r" 启用 `trust-dns` 库作为 DNS 解析器。"]
#![doc = r""]
#![doc = r" #### `dns-over-https`"]
#![doc = r""]
#![doc = r" 启用 `trust-dns` 库作为 DNS 解析器，并使用 DOH 协议。"]
#![doc = r""]
#![doc = r" #### `dns-over-tls`"]
#![doc = r""]
#![doc = r" 启用 `trust-dns` 库作为 DNS 解析器，并使用 DOT 协议。"]
#![doc = r""]
#![doc = r" ### 代码示例"]
#![doc = r""]
#![doc = r" #### 创建存储空间"]
#![doc = r""]
#![doc = r" API 参考文档：<https://developer.qiniu.com/kodo/1382/mkbucketv3>"]
#![doc = r""]
#![doc = r" 通过该参考文档可知，创建存储空间需要通过 URL 路径提供参数，因此 `qiniu-apis` 代码如下："]
#![doc = r""]
#![doc = r" ##### 阻塞代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{AllRegionsProvider, RegionsProvider, RegionsProviderEndpoints},"]
#![doc = r"     storage::create_bucket::PathParams,"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r" let region = AllRegionsProvider::new(credential.to_owned())"]
#![doc = r"     .get(Default::default())?;"]
#![doc = r" Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .create_bucket()"]
#![doc = r"     .new_request("]
#![doc = r"         RegionsProviderEndpoints::new(&region),"]
#![doc = r"         PathParams::default()"]
#![doc = r#"             .set_bucket_as_str("new-bucket-name")"#]
#![doc = r#"             .set_region_as_str("z1"),"#]
#![doc = r"         credential,"]
#![doc = r"     )"]
#![doc = r"     .call()?;"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
#![doc = r""]
#![doc = r" ##### 异步代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{AllRegionsProvider, RegionsProvider, RegionsProviderEndpoints},"]
#![doc = r"     storage::create_bucket::PathParams,"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # async fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r" let region = AllRegionsProvider::new(credential.to_owned())"]
#![doc = r"     .async_get(Default::default())"]
#![doc = r"     .await?;"]
#![doc = r" Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .create_bucket()"]
#![doc = r"     .new_async_request("]
#![doc = r"         RegionsProviderEndpoints::new(&region),"]
#![doc = r"         PathParams::default()"]
#![doc = r#"             .set_bucket_as_str("new-bucket-name")"#]
#![doc = r#"             .set_region_as_str("z1"),"#]
#![doc = r"         credential,"]
#![doc = r"     )"]
#![doc = r"     .call()"]
#![doc = r"     .await?;"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
#![doc = r""]
#![doc = r" 这里的 [`storage::create_bucket::PathParams`] 提供了设置路径参数的方法。"]
#![doc = r""]
#![doc = r" #### 设置存储空间标签"]
#![doc = r""]
#![doc = r" API 参考文档：<https://developer.qiniu.com/kodo/6314/put-bucket-tagging>"]
#![doc = r""]
#![doc = r" 通过该参考文档可知，设置存储空间标签需要提供 URL 查询参数作为设置目标，并且通过 JSON 参数传输标签列表，因此 `qiniu-apis` 代码如下："]
#![doc = r""]
#![doc = r" ##### 阻塞代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{BucketRegionsQueryer, RegionsProviderEndpoints},"]
#![doc = r"     storage::set_bucket_taggings::{QueryParams, RequestBody, TagInfo, Tags},"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r#" let bucket_name = "test-bucket";"#]
#![doc = r" let region = BucketRegionsQueryer::new().query(credential.access_key().to_owned(), bucket_name);"]
#![doc = r" let mut tag1 = TagInfo::default();"]
#![doc = r#" tag1.set_key_as_str("tag_key1".to_owned());"#]
#![doc = r#" tag1.set_value_as_str("tag_val1".to_owned());"#]
#![doc = r" let mut tag2 = TagInfo::default();"]
#![doc = r#" tag2.set_key_as_str("tag_key2".to_owned());"#]
#![doc = r#" tag2.set_value_as_str("tag_val2".to_owned());"#]
#![doc = r" let mut tags = Tags::default();"]
#![doc = r" tags.push_tag_info(tag1);"]
#![doc = r" tags.push_tag_info(tag2);"]
#![doc = r" let mut req_body = RequestBody::default();"]
#![doc = r" req_body.set_tags(tags);"]
#![doc = r" Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .set_bucket_taggings()"]
#![doc = r"     .new_request(RegionsProviderEndpoints::new(&region), credential)"]
#![doc = r"     .query_pairs(QueryParams::default().set_bucket_as_str(bucket_name))"]
#![doc = r"     .call(&req_body)?;"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
#![doc = r""]
#![doc = r" ##### 异步代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{BucketRegionsQueryer, RegionsProviderEndpoints},"]
#![doc = r"     storage::set_bucket_taggings::{QueryParams, RequestBody, TagInfo, Tags},"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # async fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r#" let bucket_name = "test-bucket";"#]
#![doc = r" let region = BucketRegionsQueryer::new().query(credential.access_key().to_owned(), bucket_name);"]
#![doc = r" let mut tag1 = TagInfo::default();"]
#![doc = r#" tag1.set_key_as_str("tag_key1".to_owned());"#]
#![doc = r#" tag1.set_value_as_str("tag_val1".to_owned());"#]
#![doc = r" let mut tag2 = TagInfo::default();"]
#![doc = r#" tag2.set_key_as_str("tag_key2".to_owned());"#]
#![doc = r#" tag2.set_value_as_str("tag_val2".to_owned());"#]
#![doc = r" let mut tags = Tags::default();"]
#![doc = r" tags.push_tag_info(tag1);"]
#![doc = r" tags.push_tag_info(tag2);"]
#![doc = r" let mut req_body = RequestBody::default();"]
#![doc = r" req_body.set_tags(tags);"]
#![doc = r" Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .set_bucket_taggings()"]
#![doc = r"     .new_async_request(RegionsProviderEndpoints::new(&region), credential)"]
#![doc = r"     .query_pairs(QueryParams::default().set_bucket_as_str(bucket_name))"]
#![doc = r"     .call(&req_body)"]
#![doc = r"     .await?;"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
#![doc = r""]
#![doc = r" 这里的 [`storage::set_bucket_taggings::QueryParams`] 提供了设置查询参数的方法，"]
#![doc = r" 而 [`storage::set_bucket_taggings::RequestBody`] 提供了设置请求体参数的方法 。"]
#![doc = r""]
#![doc = r" #### 列出存储空间标签"]
#![doc = r""]
#![doc = r" API 参考文档：<https://developer.qiniu.com/kodo/6315/get-bucket-tagging>"]
#![doc = r""]
#![doc = r" 通过该参考文档可知，该 API 通过 JSON 响应体返回标签列表，因此 `qiniu-apis` 代码如下："]
#![doc = r""]
#![doc = r" ##### 阻塞代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{BucketRegionsQueryer, RegionsProviderEndpoints},"]
#![doc = r"     storage::get_bucket_taggings::QueryParams,"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r#" let bucket_name = "test-bucket";"#]
#![doc = r" let region = BucketRegionsQueryer::new().query(credential.access_key().to_owned(), bucket_name);"]
#![doc = r" let tags = Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .get_bucket_taggings()"]
#![doc = r"     .new_request(RegionsProviderEndpoints::new(&region), credential)"]
#![doc = r"     .query_pairs(QueryParams::default().set_bucket_name_as_str(bucket_name))"]
#![doc = r"     .call()?"]
#![doc = r"     .into_body()"]
#![doc = r"     .get_tags()"]
#![doc = r"     .to_tag_info_vec();"]
#![doc = r" for tag in tags {"]
#![doc = r#"     println!("{}: {}", tag.get_key_as_str(), tag.get_value_as_str());"#]
#![doc = r" }"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
#![doc = r""]
#![doc = r" ##### 异步代码示例"]
#![doc = r""]
#![doc = r" ```"]
#![doc = r" use qiniu_apis::{"]
#![doc = r"     credential::Credential,"]
#![doc = r"     http_client::{BucketRegionsQueryer, RegionsProviderEndpoints},"]
#![doc = r"     storage::get_bucket_taggings::QueryParams,"]
#![doc = r"     Client,"]
#![doc = r" };"]
#![doc = r" # async fn example() -> anyhow::Result<()> {"]
#![doc = r#" let credential = Credential::new("abcdefghklmnopq", "1234567890");"#]
#![doc = r#" let bucket_name = "test-bucket";"#]
#![doc = r" let region = BucketRegionsQueryer::new().query(credential.access_key().to_owned(), bucket_name);"]
#![doc = r" let tags = Client::default()"]
#![doc = r"     .storage()"]
#![doc = r"     .get_bucket_taggings()"]
#![doc = r"     .new_async_request(RegionsProviderEndpoints::new(&region), credential)"]
#![doc = r"     .query_pairs(QueryParams::default().set_bucket_name_as_str(bucket_name))"]
#![doc = r"     .call()"]
#![doc = r"     .await?"]
#![doc = r"     .into_body()"]
#![doc = r"     .get_tags()"]
#![doc = r"     .to_tag_info_vec();"]
#![doc = r" for tag in tags {"]
#![doc = r#"     println!("{}: {}", tag.get_key_as_str(), tag.get_value_as_str());"#]
#![doc = r" }"]
#![doc = r" # Ok(())"]
#![doc = r" # }"]
#![doc = r" ```"]
pub use qiniu_http_client as http_client;
pub use qiniu_http_client::credential;
pub use qiniu_http_client::http;
#[cfg(feature = "isahc")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "isahc")))]
pub use qiniu_http_client::isahc;
#[cfg(feature = "reqwest")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "reqwest")))]
pub use qiniu_http_client::reqwest;
pub use qiniu_http_client::upload_token;
#[cfg(feature = "ureq")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "ureq")))]
pub use qiniu_http_client::ureq;
#[doc = "七牛 API 所用的基础类型库"]
pub mod base_types;
pub(crate) mod base_utils;
#[allow(missing_docs)]
pub mod storage;
#[doc = "七牛 API 调用客户端"]
#[derive(Debug, Clone, Default)]
pub struct Client(qiniu_http_client::HttpClient);
impl Client {
    #[inline]
    #[must_use]
    #[doc = "创建七牛 API 调用客户端"]
    pub fn new(client: qiniu_http_client::HttpClient) -> Self {
        Self(client)
    }
    #[inline]
    #[allow(missing_docs)]
    pub fn storage(&self) -> storage::Client {
        storage::Client::new(&self.0)
    }
}
impl From<qiniu_http_client::HttpClient> for Client {
    #[inline]
    fn from(client: qiniu_http_client::HttpClient) -> Self {
        Self(client)
    }
}
