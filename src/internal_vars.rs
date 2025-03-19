use std::cell::LazyCell;
use std::sync::LazyLock;

pub static IPFS_HOST: LazyLock<String> = LazyLock::new(|| {
    std::env::var("IPFS_HOST").unwrap_or_else(|_| "http://72.46.84.15:8080".to_string())
});

pub static AUTH_HEADER: &str = "X-Load-Auth-Token";
