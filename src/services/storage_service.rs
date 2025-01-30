use cloud_storage::{Bucket, Client, Object};

pub struct StorageService {
    bucket: Bucket,
    client: Client,
}

impl StorageService {
    pub async fn new(bucket_name: String) -> Self {
        let storage_client = Client::new();
        let bucket = storage_client.bucket().read(&bucket_name).await.unwrap();

        Self {
            client: storage_client,
            bucket,
        }
    }

    pub async fn upload(
        &self,
        content: Vec<u8>,
        file_name: &str,
        content_type: &str,
    ) -> cloud_storage::Result<Object> {
        self.client
            .object()
            .create(&self.bucket.name, content, file_name, content_type)
            .await
    }
}
