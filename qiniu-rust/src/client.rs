use super::{config::Config, storage::bucket_manager::BucketManager, utils::auth::Auth};
use getset::Getters;
use std::borrow::Cow;

#[derive(Getters)]
pub struct Client {
    #[get = "pub"]
    bucket_manager: BucketManager,
}

impl Client {
    pub fn new<AccessKey: Into<Cow<'static, str>>, SecretKey: Into<Cow<'static, str>>>(
        access_key: AccessKey,
        secret_key: SecretKey,
        config: Config,
    ) -> Client {
        let auth = Auth::new(access_key, secret_key);
        Client {
            bucket_manager: BucketManager::new(auth, config),
        }
    }
}
