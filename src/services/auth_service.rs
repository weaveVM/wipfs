use crate::db::planetscale_driver::PlanetScaleDriver;
use crate::db::schema::{AccessKey, Account, NewAccessKeys, NewAccount};
use crate::internal_vars::AUTH_HOST;
use crate::utils::http::get_internal_call;
use std::sync::Arc;

pub struct AuthService {
    pub(crate) db_service: Arc<PlanetScaleDriver>,
}

impl AuthService {
    pub fn new(db_service: Arc<PlanetScaleDriver>) -> Self {
        Self { db_service }
    }

    pub async fn create_account(&self, account: NewAccount) -> anyhow::Result<()> {
        ureq::post(format!("{}/internal/account", &*AUTH_HOST))
            .send_json(account)
            .map(|e| ())
            .map_err(|e| anyhow::Error::msg(e.to_string()))
    }

    pub async fn create_access_key(&self, access_key: NewAccessKeys) -> anyhow::Result<()> {
        ureq::post(format!("{}/internal/access-key", &*AUTH_HOST))
            .send_json(access_key)
            .map(|e| ())
            .map_err(|e| anyhow::Error::msg(e.to_string()))
    }

    pub async fn find_account(&self, account_name: String) -> anyhow::Result<Account> {
        let url = format!("{}/internal/account/{}", &*AUTH_HOST, account_name);

        get_internal_call(url)
    }

    pub async fn list_keys_for_owner(&self, owner_id: i64) -> anyhow::Result<Vec<AccessKey>> {
        let url = format!("{}/internal/keys/{}", &*AUTH_HOST, owner_id);

        get_internal_call(url)
    }

    pub async fn verify_access(&self, authorization_header_key: String) -> Option<AccessKey> {
        let url = format!(
            "{}/internal/verify/{}",
            &*AUTH_HOST, authorization_header_key
        );

        get_internal_call(url).ok()
    }
}
