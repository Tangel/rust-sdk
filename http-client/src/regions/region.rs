use super::{Endpoint, Endpoints};
use std::sync::Arc;

/// 七牛存储区域
///
/// 提供七牛不同区域的域名
#[derive(Clone, Debug)]
pub struct Region {
    region_id: Box<str>,
    s3_region_id: Box<str>,
    up: Arc<Endpoints>,
    io: Arc<Endpoints>,
    uc: Arc<Endpoints>,
    rs: Arc<Endpoints>,
    rsf: Arc<Endpoints>,
    api: Arc<Endpoints>,
    s3: Arc<Endpoints>,
}

impl Region {
    /// 获取区域 ID
    #[inline]
    pub fn region_id(&self) -> &str {
        &self.region_id
    }

    /// 获取 S3 区域 ID
    #[inline]
    pub fn s3_region_id(&self) -> &str {
        &self.s3_region_id
    }

    /// 获取上传域名列表
    #[inline]
    pub fn up_endpoints(&self) -> &[Endpoint] {
        &self.up.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn up_old_endpoints(&self) -> &[Endpoint] {
        &self.up.old_endpoints()
    }

    /// 获取下载域名列表
    #[inline]
    pub fn io_endpoints(&self) -> &[Endpoint] {
        &self.io.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn io_old_endpoints(&self) -> &[Endpoint] {
        &self.io.old_endpoints()
    }

    /// 获取 UC 域名列表
    #[inline]
    pub fn uc_endpoints(&self) -> &[Endpoint] {
        &self.uc.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn uc_old_endpoints(&self) -> &[Endpoint] {
        &self.uc.old_endpoints()
    }

    /// 获取 RS 域名列表
    #[inline]
    pub fn rs_endpoints(&self) -> &[Endpoint] {
        &self.rs.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn rs_old_endpoints(&self) -> &[Endpoint] {
        &self.rs.old_endpoints()
    }

    /// 获取 RSF 域名列表
    #[inline]
    pub fn rsf_endpoints(&self) -> &[Endpoint] {
        &self.rsf.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn rsf_old_endpoints(&self) -> &[Endpoint] {
        &self.rsf.old_endpoints()
    }

    /// 获取 API 域名列表
    #[inline]
    pub fn api_endpoints(&self) -> &[Endpoint] {
        &self.api.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn api_old_endpoints(&self) -> &[Endpoint] {
        &self.api.old_endpoints()
    }

    /// 获取 S3 域名列表
    #[inline]
    pub fn s3_endpoints(&self) -> &[Endpoint] {
        &self.s3.endpoints()
    }

    #[inline]
    #[doc(hidden)]
    pub fn s3_old_endpoints(&self) -> &[Endpoint] {
        &self.s3.old_endpoints()
    }

    /// 创建新的区域
    #[inline]
    pub fn builder(region_id: impl Into<String>) -> RegionBuilder {
        RegionBuilder::new(region_id.into())
    }

    #[inline]
    pub(super) fn up(&self) -> &Endpoints {
        &self.up
    }

    #[inline]
    pub(super) fn io(&self) -> &Endpoints {
        &self.io
    }

    #[inline]
    pub(super) fn uc(&self) -> &Endpoints {
        &self.uc
    }

    #[inline]
    pub(super) fn rs(&self) -> &Endpoints {
        &self.rs
    }

    #[inline]
    pub(super) fn rsf(&self) -> &Endpoints {
        &self.rsf
    }

    #[inline]
    pub(super) fn api(&self) -> &Endpoints {
        &self.api
    }

    #[inline]
    pub(super) fn s3(&self) -> &Endpoints {
        &self.s3
    }
}

pub struct RegionBuilder {
    region_id: String,
    s3_region_id: String,
    up: Vec<Endpoint>,
    up_old: Vec<Endpoint>,
    io: Vec<Endpoint>,
    io_old: Vec<Endpoint>,
    uc: Vec<Endpoint>,
    uc_old: Vec<Endpoint>,
    rs: Vec<Endpoint>,
    rs_old: Vec<Endpoint>,
    rsf: Vec<Endpoint>,
    rsf_old: Vec<Endpoint>,
    api: Vec<Endpoint>,
    api_old: Vec<Endpoint>,
    s3: Vec<Endpoint>,
    s3_old: Vec<Endpoint>,
}

impl RegionBuilder {
    /// 创建新的区域，传入域名 ID
    pub fn new(region_id: impl Into<String>) -> Self {
        Self {
            region_id: region_id.into(),
            s3_region_id: Default::default(),
            up: Default::default(),
            up_old: Default::default(),
            io: Default::default(),
            io_old: Default::default(),
            uc: Default::default(),
            uc_old: Default::default(),
            rs: Default::default(),
            rs_old: Default::default(),
            rsf: Default::default(),
            rsf_old: Default::default(),
            api: Default::default(),
            api_old: Default::default(),
            s3: Default::default(),
            s3_old: Default::default(),
        }
    }

    /// 设置 S3 区域 ID
    #[inline]
    pub fn s3_region_id(mut self, s3_region_id: impl Into<String>) -> Self {
        self.s3_region_id = s3_region_id.into();
        self
    }

    /// 追加访问地址到上传访问地址列表
    #[inline]
    pub fn push_up_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.up.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_up_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.up_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到下载访问地址列表
    #[inline]
    pub fn push_io_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.io.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_io_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.io_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到 UC 访问地址列表
    #[inline]
    pub fn push_uc_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.uc.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_uc_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.uc_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到 RS 访问地址列表
    #[inline]
    pub fn push_rs_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.rs.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_rs_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.rs_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到 RSF 访问地址列表
    #[inline]
    pub fn push_rsf_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.rsf.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_rsf_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.rsf_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到 API 访问地址列表
    #[inline]
    pub fn push_api_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.api.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_api_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.api_old.push(endpoint.into());
        self
    }

    /// 追加访问地址到 S3 访问地址列表
    #[inline]
    pub fn push_s3_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.s3.push(endpoint.into());
        self
    }

    #[inline]
    #[doc(hidden)]
    pub fn push_s3_old_endpoint(mut self, endpoint: impl Into<Endpoint>) -> Self {
        self.s3_old.push(endpoint.into());
        self
    }
    /// 构建区域
    pub fn build(self) -> Region {
        Region {
            region_id: self.region_id.into_boxed_str(),
            s3_region_id: self.s3_region_id.into_boxed_str(),
            up: Arc::new((self.up, self.up_old).into()),
            io: Arc::new((self.io, self.io_old).into()),
            uc: Arc::new((self.uc, self.uc_old).into()),
            rs: Arc::new((self.rs, self.rs_old).into()),
            rsf: Arc::new((self.rsf, self.rsf_old).into()),
            api: Arc::new((self.api, self.api_old).into()),
            s3: Arc::new((self.s3, self.s3_old).into()),
        }
    }
}
