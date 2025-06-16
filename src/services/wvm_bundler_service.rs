use crate::utils::s3::create_new_s3_client;
use aws_sdk_s3::config::http::HttpResponse;
use aws_sdk_s3::operation::put_object::{PutObjectError, PutObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use bundler::utils::core::tags::Tag;
use bundler::utils::errors::Error;
use serde_json::Value;

pub struct WvmBundlerService {
    private_key: String,
}

impl WvmBundlerService {
    pub fn new(private_key: String) -> Self {
        Self { private_key }
    }

    pub async fn send(
        &self,
        content_type: String,
        data: Vec<u8>,
        user_token: String,
        cid: String,
        created_by: i64
    ) -> Result<String, Error> {
        let user_aws_client = create_new_s3_client(user_token);

        let save_file = user_aws_client
            .put_object()
            .content_type(content_type)
            .body(ByteStream::from(data))
            .bucket(format!("ipfs-{}", created_by))
            .metadata("Create-Bucket-If-Not-Exists", "true")
            .key(cid)
            .send()
            .await;

        match save_file {
            Ok(output) => Ok(output.e_tag.unwrap()),
            Err(e) => {
                eprintln!("{:?}", e.into_service_error());
                Err(Error::Other("File could not be sent".to_string()))
            }
        }
    }
}
