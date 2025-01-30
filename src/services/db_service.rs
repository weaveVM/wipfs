use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;

pub struct DbService {
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbService {
    pub fn new(db_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(db_url);

        let db_pool = Pool::builder().build(manager).unwrap();

        Self { db_pool }
    }
}
