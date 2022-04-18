use super::{DownloadUrlsGenerator, DownloadingObject};
use qiniu_apis::http_client::{ApiResult, HttpClient};
use std::sync::Arc;

/// 下载管理器
#[derive(Debug)]
pub struct DownloadManager<G> {
    urls_generator: Arc<G>,
    http_client: HttpClient,
}

/// 下载管理构建器
#[derive(Debug)]
pub struct DownloadManagerBuilder<G> {
    http_client: Option<HttpClient>,
    urls_generator: G,
}

impl<G> Clone for DownloadManager<G> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            urls_generator: self.urls_generator.to_owned(),
            http_client: self.http_client.to_owned(),
        }
    }
}

impl<G> DownloadManager<G> {
    /// 创建下载管理构建器
    #[inline]
    pub fn builder(urls_generator: G) -> DownloadManagerBuilder<G> {
        DownloadManagerBuilder::new(urls_generator)
    }

    /// 创建下载管理器
    #[inline]
    pub fn new(urls_generator: G) -> Self {
        Self::builder(urls_generator).build()
    }
}

impl<G: DownloadUrlsGenerator> DownloadManager<G> {
    /// 获取下载输出流
    pub fn download(&self, object_name: impl AsRef<str>) -> ApiResult<DownloadingObject> {
        let urls = self.urls_generator.generate(object_name.as_ref(), Default::default())?;
        Ok(DownloadingObject::new(self.http_client.to_owned(), urls))
    }

    /// 获取异步下载输出流
    #[cfg(feature = "async")]
    #[cfg_attr(feature = "docs", doc(cfg(feature = "async")))]
    pub async fn async_download(&self, object_name: impl AsRef<str>) -> ApiResult<DownloadingObject> {
        let urls = self
            .urls_generator
            .async_generate(object_name.as_ref(), Default::default())
            .await?;
        Ok(DownloadingObject::new(self.http_client.to_owned(), urls))
    }
}

impl<G> DownloadManagerBuilder<G> {
    /// 创建下载管理构建器
    #[inline]
    pub fn new(urls_generator: G) -> Self {
        Self {
            http_client: None,
            urls_generator,
        }
    }

    /// 设置 HTTP 客户端
    pub fn http_client(mut self, http_client: HttpClient) -> Self {
        self.http_client = Some(http_client);
        self
    }

    /// 是否启用 HTTPS 协议
    ///
    /// 默认为 HTTPS 协议
    pub fn use_https(self, use_https: bool) -> Self {
        self.http_client(HttpClient::build_default().use_https(use_https).build())
    }

    /// 构建下载管理器
    pub fn build(self) -> DownloadManager<G> {
        DownloadManager {
            urls_generator: Arc::new(self.urls_generator),
            http_client: self.http_client.unwrap_or_default(),
        }
    }
}
