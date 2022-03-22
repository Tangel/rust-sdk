// THIS FILE IS GENERATED BY api-generator, DO NOT EDIT DIRECTLY!
//
#[doc = "从指定 URL 抓取资源，并将该资源存储到指定空间中。每次只抓取一个文件，抓取时可以指定保存空间名和最终资源名"]
pub mod async_fetch_object;
#[doc = "批量操作意指在单一请求中执行多次（最大限制1000次） 查询元信息、修改元信息、移动、复制、删除、修改状态、修改存储类型、修改生命周期和解冻操作，极大提高对象管理效率。其中，解冻操作仅针对归档存储文件有效"]
pub mod batch_ops;
#[doc = "将源空间的指定对象复制到目标空间"]
pub mod copy_object;
#[doc = "创建一个新的存储空间"]
pub mod create_bucket;
#[doc = "删除指定的存储空间"]
pub mod delete_bucket;
#[doc = "一键删除指定存储空间的所有标签"]
pub mod delete_bucket_taggings;
#[doc = "删除指定对象"]
pub mod delete_object;
#[doc = "从指定 URL 抓取指定名称的对象并存储到该空间中"]
pub mod fetch_object;
#[doc = "查询异步抓取任务"]
pub mod get_async_fetch_task;
#[doc = "查询指定的存储空间已设置的标签信息"]
pub mod get_bucket_taggings;
#[doc = "获取拥有的所有存储空间列表"]
pub mod get_buckets;
#[doc = "获取存储空间的域名列表"]
pub mod get_domains;
#[doc = "列举指定存储空间里的所有对象条目"]
pub mod get_objects;
#[doc = "列举指定存储空间里的所有对象条目"]
pub mod get_objects_v2;
#[doc = "修改已上传对象的生命周期"]
pub mod modify_object_life_cycle;
#[doc = "修改文件元信息"]
pub mod modify_object_metadata;
#[doc = "修改文件的存储状态，即禁用状态和启用状态间的的互相转换"]
pub mod modify_object_status;
#[doc = "将源空间的指定对象移动到目标空间，或在同一空间内对对象重命名"]
pub mod move_object;
#[doc = "对于设置了镜像存储的空间，从镜像源站抓取指定名称的对象并存储到该空间中，如果该空间中已存在该名称的对象，则会将镜像源站的对象覆盖空间中相同名称的对象"]
pub mod prefetch_object;
#[doc = "在一次 HTTP 会话中上传单一的一个文件"]
pub mod put_object;
#[doc = "上传指定块的一片数据，具体数据量可根据现场环境调整，同一块的每片数据必须串行上传"]
pub mod resumable_upload_v1_bput;
#[doc = "为后续分片上传创建一个新的块，同时上传第一片数据"]
pub mod resumable_upload_v1_make_block;
#[doc = "将上传好的所有数据块按指定顺序合并成一个资源文件"]
pub mod resumable_upload_v1_make_file;
#[doc = "根据 UploadId 终止 Multipart Upload"]
pub mod resumable_upload_v2_abort_multipart_upload;
#[doc = "在将所有数据分片都上传完成后，必须调用 completeMultipartUpload API 来完成整个文件的 Multipart Upload。用户需要提供有效数据的分片列表（包括 PartNumber 和调用 uploadPart API 服务端返回的 Etag）。服务端收到用户提交的分片列表后，会逐一验证每个数据分片的有效性。当所有的数据分片验证通过后，会把这些数据分片组合成一个完整的对象"]
pub mod resumable_upload_v2_complete_multipart_upload;
#[doc = "使用 Multipart Upload 方式上传数据前，必须先调用 API 来获取一个全局唯一的 UploadId，后续的块数据通过 uploadPart API 上传，整个文件完成 completeMultipartUpload API，已经上传块的删除 abortMultipartUpload API 都依赖该 UploadId"]
pub mod resumable_upload_v2_initiate_multipart_upload;
#[doc = "列举出指定 UploadId 所属任务所有已经上传成功的分片"]
pub mod resumable_upload_v2_list_parts;
#[doc = "初始化一个 Multipart Upload 任务之后，可以根据指定的对象名称和 UploadId 来分片上传数据"]
pub mod resumable_upload_v2_upload_part;
#[doc = "设置存储空间的访问权限"]
pub mod set_bucket_private;
#[doc = "设置存储空间的标签列表，包括新增和修改"]
pub mod set_bucket_taggings;
#[doc = "设置存储空间的镜像源"]
pub mod set_buckets_mirror;
#[doc = "修改文件的存储类型信息，可以实现标准存储、低频存储和归档存储之间的互相转换"]
pub mod set_object_file_type;
#[doc = "仅获取对象的元信息，不返回对象的内容"]
pub mod stat_object;
#[doc = "解冻归档存储类型的文件，可设置解冻有效期1～7天，完成解冻任务通常需要1～5分钟"]
pub mod unfreeze_object;
#[derive(Debug, Clone)]
pub struct Client<'client>(&'client qiniu_http_client::HttpClient);
impl<'client> Client<'client> {
    pub(super) fn new(http_client: &'client qiniu_http_client::HttpClient) -> Self {
        Self(http_client)
    }
    #[inline]
    #[doc = "从指定 URL 抓取资源，并将该资源存储到指定空间中。每次只抓取一个文件，抓取时可以指定保存空间名和最终资源名"]
    pub fn async_fetch_object(&self) -> async_fetch_object::Client<'client> {
        async_fetch_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "批量操作意指在单一请求中执行多次（最大限制1000次） 查询元信息、修改元信息、移动、复制、删除、修改状态、修改存储类型、修改生命周期和解冻操作，极大提高对象管理效率。其中，解冻操作仅针对归档存储文件有效"]
    pub fn batch_ops(&self) -> batch_ops::Client<'client> {
        batch_ops::Client::new(self.0)
    }
    #[inline]
    #[doc = "将源空间的指定对象复制到目标空间"]
    pub fn copy_object(&self) -> copy_object::Client<'client> {
        copy_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "创建一个新的存储空间"]
    pub fn create_bucket(&self) -> create_bucket::Client<'client> {
        create_bucket::Client::new(self.0)
    }
    #[inline]
    #[doc = "删除指定的存储空间"]
    pub fn delete_bucket(&self) -> delete_bucket::Client<'client> {
        delete_bucket::Client::new(self.0)
    }
    #[inline]
    #[doc = "一键删除指定存储空间的所有标签"]
    pub fn delete_bucket_taggings(&self) -> delete_bucket_taggings::Client<'client> {
        delete_bucket_taggings::Client::new(self.0)
    }
    #[inline]
    #[doc = "删除指定对象"]
    pub fn delete_object(&self) -> delete_object::Client<'client> {
        delete_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "从指定 URL 抓取指定名称的对象并存储到该空间中"]
    pub fn fetch_object(&self) -> fetch_object::Client<'client> {
        fetch_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "查询异步抓取任务"]
    pub fn get_async_fetch_task(&self) -> get_async_fetch_task::Client<'client> {
        get_async_fetch_task::Client::new(self.0)
    }
    #[inline]
    #[doc = "查询指定的存储空间已设置的标签信息"]
    pub fn get_bucket_taggings(&self) -> get_bucket_taggings::Client<'client> {
        get_bucket_taggings::Client::new(self.0)
    }
    #[inline]
    #[doc = "获取拥有的所有存储空间列表"]
    pub fn get_buckets(&self) -> get_buckets::Client<'client> {
        get_buckets::Client::new(self.0)
    }
    #[inline]
    #[doc = "获取存储空间的域名列表"]
    pub fn get_domains(&self) -> get_domains::Client<'client> {
        get_domains::Client::new(self.0)
    }
    #[inline]
    #[doc = "列举指定存储空间里的所有对象条目"]
    pub fn get_objects(&self) -> get_objects::Client<'client> {
        get_objects::Client::new(self.0)
    }
    #[inline]
    #[doc = "列举指定存储空间里的所有对象条目"]
    pub fn get_objects_v2(&self) -> get_objects_v2::Client<'client> {
        get_objects_v2::Client::new(self.0)
    }
    #[inline]
    #[doc = "修改已上传对象的生命周期"]
    pub fn modify_object_life_cycle(&self) -> modify_object_life_cycle::Client<'client> {
        modify_object_life_cycle::Client::new(self.0)
    }
    #[inline]
    #[doc = "修改文件元信息"]
    pub fn modify_object_metadata(&self) -> modify_object_metadata::Client<'client> {
        modify_object_metadata::Client::new(self.0)
    }
    #[inline]
    #[doc = "修改文件的存储状态，即禁用状态和启用状态间的的互相转换"]
    pub fn modify_object_status(&self) -> modify_object_status::Client<'client> {
        modify_object_status::Client::new(self.0)
    }
    #[inline]
    #[doc = "将源空间的指定对象移动到目标空间，或在同一空间内对对象重命名"]
    pub fn move_object(&self) -> move_object::Client<'client> {
        move_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "对于设置了镜像存储的空间，从镜像源站抓取指定名称的对象并存储到该空间中，如果该空间中已存在该名称的对象，则会将镜像源站的对象覆盖空间中相同名称的对象"]
    pub fn prefetch_object(&self) -> prefetch_object::Client<'client> {
        prefetch_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "在一次 HTTP 会话中上传单一的一个文件"]
    pub fn put_object(&self) -> put_object::Client<'client> {
        put_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "上传指定块的一片数据，具体数据量可根据现场环境调整，同一块的每片数据必须串行上传"]
    pub fn resumable_upload_v1_bput(&self) -> resumable_upload_v1_bput::Client<'client> {
        resumable_upload_v1_bput::Client::new(self.0)
    }
    #[inline]
    #[doc = "为后续分片上传创建一个新的块，同时上传第一片数据"]
    pub fn resumable_upload_v1_make_block(&self) -> resumable_upload_v1_make_block::Client<'client> {
        resumable_upload_v1_make_block::Client::new(self.0)
    }
    #[inline]
    #[doc = "将上传好的所有数据块按指定顺序合并成一个资源文件"]
    pub fn resumable_upload_v1_make_file(&self) -> resumable_upload_v1_make_file::Client<'client> {
        resumable_upload_v1_make_file::Client::new(self.0)
    }
    #[inline]
    #[doc = "根据 UploadId 终止 Multipart Upload"]
    pub fn resumable_upload_v2_abort_multipart_upload(
        &self,
    ) -> resumable_upload_v2_abort_multipart_upload::Client<'client> {
        resumable_upload_v2_abort_multipart_upload::Client::new(self.0)
    }
    #[inline]
    #[doc = "在将所有数据分片都上传完成后，必须调用 completeMultipartUpload API 来完成整个文件的 Multipart Upload。用户需要提供有效数据的分片列表（包括 PartNumber 和调用 uploadPart API 服务端返回的 Etag）。服务端收到用户提交的分片列表后，会逐一验证每个数据分片的有效性。当所有的数据分片验证通过后，会把这些数据分片组合成一个完整的对象"]
    pub fn resumable_upload_v2_complete_multipart_upload(
        &self,
    ) -> resumable_upload_v2_complete_multipart_upload::Client<'client> {
        resumable_upload_v2_complete_multipart_upload::Client::new(self.0)
    }
    #[inline]
    #[doc = "使用 Multipart Upload 方式上传数据前，必须先调用 API 来获取一个全局唯一的 UploadId，后续的块数据通过 uploadPart API 上传，整个文件完成 completeMultipartUpload API，已经上传块的删除 abortMultipartUpload API 都依赖该 UploadId"]
    pub fn resumable_upload_v2_initiate_multipart_upload(
        &self,
    ) -> resumable_upload_v2_initiate_multipart_upload::Client<'client> {
        resumable_upload_v2_initiate_multipart_upload::Client::new(self.0)
    }
    #[inline]
    #[doc = "列举出指定 UploadId 所属任务所有已经上传成功的分片"]
    pub fn resumable_upload_v2_list_parts(&self) -> resumable_upload_v2_list_parts::Client<'client> {
        resumable_upload_v2_list_parts::Client::new(self.0)
    }
    #[inline]
    #[doc = "初始化一个 Multipart Upload 任务之后，可以根据指定的对象名称和 UploadId 来分片上传数据"]
    pub fn resumable_upload_v2_upload_part(&self) -> resumable_upload_v2_upload_part::Client<'client> {
        resumable_upload_v2_upload_part::Client::new(self.0)
    }
    #[inline]
    #[doc = "设置存储空间的访问权限"]
    pub fn set_bucket_private(&self) -> set_bucket_private::Client<'client> {
        set_bucket_private::Client::new(self.0)
    }
    #[inline]
    #[doc = "设置存储空间的标签列表，包括新增和修改"]
    pub fn set_bucket_taggings(&self) -> set_bucket_taggings::Client<'client> {
        set_bucket_taggings::Client::new(self.0)
    }
    #[inline]
    #[doc = "设置存储空间的镜像源"]
    pub fn set_buckets_mirror(&self) -> set_buckets_mirror::Client<'client> {
        set_buckets_mirror::Client::new(self.0)
    }
    #[inline]
    #[doc = "修改文件的存储类型信息，可以实现标准存储、低频存储和归档存储之间的互相转换"]
    pub fn set_object_file_type(&self) -> set_object_file_type::Client<'client> {
        set_object_file_type::Client::new(self.0)
    }
    #[inline]
    #[doc = "仅获取对象的元信息，不返回对象的内容"]
    pub fn stat_object(&self) -> stat_object::Client<'client> {
        stat_object::Client::new(self.0)
    }
    #[inline]
    #[doc = "解冻归档存储类型的文件，可设置解冻有效期1～7天，完成解冻任务通常需要1～5分钟"]
    pub fn unfreeze_object(&self) -> unfreeze_object::Client<'client> {
        unfreeze_object::Client::new(self.0)
    }
}
