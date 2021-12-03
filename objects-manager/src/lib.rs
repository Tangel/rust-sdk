#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    non_ascii_idents,
    indirect_structural_match,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

mod bucket;
mod list;
mod objects_manager;

pub use bucket::{Bucket, ListBuilder};
pub use list::{ListIter, ListVersion};
pub use objects_manager::{ObjectsManager, ObjectsManagerBuilder};
pub use qiniu_apis as apis;

#[cfg(feature = "async")]
pub use list::ListStream;
