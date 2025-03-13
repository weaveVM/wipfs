use planetscale_driver::PSConnection;
use shuttle_runtime::SecretStore;

#[derive(Debug, Default, Clone)]
pub struct PlanetScaleDriver {
    pub host: String,
    pub username: String,
    pub password: String,
}

impl PlanetScaleDriver {
    pub fn new(host: String, username: String, password: String) -> Self {
        Self {
            host,
            username,
            password,
        }
    }

    pub fn get_conn(&self) -> PSConnection {
        PSConnection::new(&self.host, &self.username, &self.password)
    }
}

impl<'a> From<&'a SecretStore> for PlanetScaleDriver {
    fn from(value: &'a SecretStore) -> Self {
        Self {
            host: value.get("DATABASE_HOST").unwrap(),
            username: value.get("DATABASE_USERNAME").unwrap(),
            password: value.get("DATABASE_PASSWORD").unwrap(),
        }
    }
}
