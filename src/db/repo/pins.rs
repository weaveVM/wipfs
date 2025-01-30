use crate::db::schema::files;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection, Queryable}; // Ensure this is imported

#[derive(Insertable, Queryable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub cid: &'a str,
    pub size: i64,
}

pub fn create_pin(conn: &mut PgConnection, cid: &str, size: i64) {
    use crate::db::repo::pins::files::dsl::files;
    let row = NewFile { cid, size };

    let _ = diesel::insert_into(files).values(&row).execute(conn);
}
