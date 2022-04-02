#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    single_use_lifetimes,
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

//! # qiniu-upload-manager
//!
//! ## 七牛上传管理
//!
//! 基于 `qiniu-apis` 提供针对七牛对象的上传功能
//! （同时提供阻塞客户端和异步客户端，异步客户端则需要启用 `async` 功能）。
//!
//! ### 用自动上传器上传文件
//!
//! ```
//! use qiniu_upload_manager::{
//!     apis::credential::Credential, AutoUploader, AutoUploaderObjectParams, UploadManager,
//!     UploadTokenSigner,
//! };
//! use std::time::Duration;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let bucket_name = "test-bucket";
//! let object_name = "test-object";
//! # let file_path = std::path::Path::new("test.txt");
//! let upload_manager = UploadManager::builder(UploadTokenSigner::new_credential_provider(
//!     Credential::new("abcdefghklmnopq", "1234567890"),
//!     bucket_name,
//!     Duration::from_secs(3600),
//! ))
//! .build();
//! let params = AutoUploaderObjectParams::builder().object_name(object_name).file_name(object_name).build();
//! let mut uploader: AutoUploader = upload_manager.auto_uploader();
//! uploader.async_upload_path(file_path, params).await?;
//! # Ok(())
//! # }
//! ```

mod auto_uploader;
mod callbacks;
mod concurrency_provider;
mod data_partition_provider;
mod data_source;
mod multi_parts_uploader;
mod object_params;
mod resumable_policy;
mod resumable_recorder;
mod scheduler;
mod single_part_uploader;
mod upload_manager;
mod upload_token;

pub use qiniu_apis as apis;

pub use auto_uploader::{
    AutoUploader, AutoUploaderBuilder, AutoUploaderObjectParams, AutoUploaderObjectParamsBuilder,
    MultiPartsUploaderPrefer, MultiPartsUploaderSchedulerPrefer, SinglePartUploaderPrefer,
};
pub use callbacks::{MultiPartsUploaderWithCallbacks, UploaderWithCallbacks, UploadingProgressInfo};
pub use concurrency_provider::{
    Concurrency, ConcurrencyProvider, ConcurrencyProviderFeedback, FixedConcurrencyProvider,
};
pub use data_partition_provider::{
    DataPartitionProvider, DataPartitionProviderFeedback, FixedDataPartitionProvider, LimitedDataPartitionProvider,
    MultiplyDataPartitionProvider, PartSize,
};
pub use data_source::{DataSource, DataSourceReader, FileDataSource, SeekableSource, SourceKey};
pub use multi_parts_uploader::{
    InitializedParts, MultiPartsUploader, MultiPartsV1Uploader, MultiPartsV1UploaderInitializedObject,
    MultiPartsV1UploaderUploadedPart, MultiPartsV2Uploader, MultiPartsV2UploaderInitializedObject,
    MultiPartsV2UploaderUploadedPart, UploadedPart,
};
pub use object_params::{ObjectParams, ObjectParamsBuilder};
pub use resumable_policy::{
    AlwaysMultiParts, AlwaysSinglePart, DynRead, FixedThresholdResumablePolicy, GetPolicyOptions,
    MultiplePartitionsResumablePolicyProvider, ResumablePolicy, ResumablePolicyProvider,
};
pub use resumable_recorder::{
    AppendOnlyResumableRecorderMedium, DummyResumableRecorder, DummyResumableRecorderMedium,
    FileSystemResumableRecorder, ReadOnlyResumableRecorderMedium, ResumableRecorder,
};
pub use scheduler::{
    ConcurrentMultiPartsUploaderScheduler, MultiPartsUploaderScheduler, MultiPartsUploaderSchedulerExt,
    SerialMultiPartsUploaderScheduler,
};
pub use single_part_uploader::{FormUploader, SinglePartUploader};
pub use upload_manager::{UploadManager, UploadManagerBuilder};
pub use upload_token::UploadTokenSigner;

#[cfg(feature = "async")]
pub use {
    data_source::{AsyncDataSourceReader, AsyncSeekableSource},
    resumable_policy::DynAsyncRead,
    resumable_recorder::{AppendOnlyAsyncResumableRecorderMedium, ReadOnlyAsyncResumableRecorderMedium},
};

/// 将所有 Trait 全部重新导出，方便统一导入
pub mod prelude {
    pub use super::apis::http_client::prelude::*;
    pub use super::{
        AppendOnlyResumableRecorderMedium, ConcurrencyProvider, DataPartitionProvider, DataSource, InitializedParts,
        MultiPartsUploader, MultiPartsUploaderScheduler, MultiPartsUploaderSchedulerExt,
        MultiPartsUploaderWithCallbacks, ReadOnlyResumableRecorderMedium, ResumablePolicyProvider, ResumableRecorder,
        SinglePartUploader, UploadedPart, UploaderWithCallbacks,
    };

    #[cfg(feature = "async")]
    pub use super::{AppendOnlyAsyncResumableRecorderMedium, DynAsyncRead, ReadOnlyAsyncResumableRecorderMedium};
}
