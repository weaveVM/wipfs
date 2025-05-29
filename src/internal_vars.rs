use std::sync::LazyLock;

pub static IPFS_HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("IPFS_HOST").unwrap_or_else(|_| "http://72.46.84.15:8080".to_string())
});

pub static AUTH_HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("AUTH_API").unwrap_or_else(|_| "https://load-auth-vskt.shuttle.app".to_string())
});

pub static S3_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
    std::env::var("S3_API").unwrap_or_else(|_| "https://s3.load.rs".to_string())
});

pub static AUTH_HEADER: &str = "X-Load-Auth-Token";
