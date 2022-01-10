use anyhow::Result;
use qiniu_apis::{http_client::CallbackResult, upload_token::ObjectUploadTokenProvider};
use qiniu_upload_manager::{
    apis::credential::Credential, ObjectParams, SinglePartUploader, UploadManager,
};
use std::{path::PathBuf, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "form-upload-object")]
struct Opt {
    /// Qiniu Access Key
    #[structopt(long)]
    access_key: String,
    /// Qiniu Secret Key
    #[structopt(long)]
    secret_key: String,
    /// Qiniu Bucket Name
    #[structopt(long)]
    bucket_name: String,
    /// Qiniu Object Name
    #[structopt(long)]
    object_name: String,
    /// Upload File Path
    #[structopt(long)]
    file: PathBuf,
}

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opt: Opt = Opt::from_args();
    let credential = Credential::new(opt.access_key, opt.secret_key);
    let upload_manager = UploadManager::new(ObjectUploadTokenProvider::new(
        &opt.bucket_name,
        &opt.object_name,
        Duration::from_secs(3600),
        credential,
    ));
    let value = upload_manager
        .form_uploader()
        .on_upload_progress(|transfer| {
            println!(
                "{} / {} => {}%",
                transfer.transferred_bytes(),
                transfer.total_bytes(),
                transfer.transferred_bytes() as f64 * 100f64 / transfer.total_bytes() as f64
            );
            CallbackResult::Continue
        })
        .async_upload_path(
            &opt.file,
            ObjectParams::builder()
                .object_name(&opt.object_name)
                .build(),
        )
        .await?;
    println!("{:?}", value);

    Ok(())
}