use aws_config::{AppName, SdkConfig};
use aws_sdk_s3::config::{Credentials, Region};
use aws_sdk_s3::{Client, Config};
use crate::internal_vars::S3_ENDPOINT;

pub fn create_new_s3_client(access_key_id: String) -> Client {
    let config = Config::builder()
        .app_name(AppName::new("LoadNetwork").unwrap())
        .endpoint_url(&*S3_ENDPOINT)
        .credentials_provider(
            Credentials::builder()
                .provider_name("Load")
                .access_key_id(access_key_id)
                .secret_access_key("")
                .build(),
        )
        .region(Some(Region::new("us-east-1")))
        .force_path_style(true)
        .build();

    Client::from_conf(config)
}
