use crate::db::planetscale_driver::PlanetScaleDriver;
use crate::db::repo::auth::{create_access_key, create_account, NewAccessKeys, NewAccount};
use shuttle_runtime::SecretStore;
use std::sync::Arc;

pub struct AuthService {
    pub(crate) db_service: Arc<PlanetScaleDriver>,
}

impl AuthService {
    pub fn new(db_service: Arc<PlanetScaleDriver>) -> Self {
        Self { db_service }
    }

    pub async fn create_account(&self, account: NewAccount) -> anyhow::Result<()> {
        create_account(self.db_service.get_conn(), account).await
    }

    pub async fn create_access_key(&self, access_key: NewAccessKeys) -> anyhow::Result<()> {
        create_access_key(self.db_service.get_conn(), access_key).await
    }
}
