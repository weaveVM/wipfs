use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::pooled_connection::{bb8::Pool, AsyncDieselConnectionManager};
use diesel_async::RunQueryDsl;
use std::sync::Arc;

use diesel_async::pg::AsyncPgConnection;

pub type PgConnection<'a> = PooledConnection<'a, AsyncPgConnection>;

pub struct DbService {
    pub db_pool: Arc<Pool<AsyncPgConnection>>,
}

impl DbService {
    pub async fn new(db_url: String) -> Self {
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
        let pool = Pool::builder().build(config).await.unwrap();

        Self {
            db_pool: Arc::new(pool),
        }
    }
}
