use crate::db::schema::files;
use crate::services::db_service::PgConnection;
use crate::services::pin_service::GetPinsParams;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use diesel_async::AsyncPgConnection;
// Ensure this is imported
use diesel_async::RunQueryDsl;

#[derive(Insertable, Queryable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub cid: &'a str,
    pub size: i64,
    pub bundle_tx_id: &'a str,
    pub envelope_id: &'a str,
    pub name: Option<String>,
    pub req_id: &'a str,
}

pub async fn create_pin<'a>(
    conn: &mut PgConnection<'a>,
    cid: &str,
    size: usize,
    bundle_tx_id: &str,
    envelope_id: &str,
    name: Option<String>,
    req_id: &str,
) -> Result<usize, diesel::result::Error> {
    use crate::db::repo::pins::files::dsl::files;
    let row = NewFile {
        cid,
        size: size as i64,
        bundle_tx_id,
        envelope_id,
        name,
        req_id,
    };

    diesel::insert_into(files).values(&row).execute(conn).await
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = files)]
pub struct IpfsFile {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub cid: String,
    pub size: i64,
    pub bundle_tx_id: String,
    pub envelope_id: String,
    pub name: Option<String>,
    pub req_id: String,
}

pub async fn find_pins<'a>(
    conn: &mut PgConnection<'a>,
    params: &GetPinsParams,
) -> Result<Vec<IpfsFile>, diesel::result::Error> {
    use crate::db::repo::pins::files::dsl::*;

    let mut query = files.into_boxed::<diesel::pg::Pg>();

    if let Some(cids) = &params.cid {
        query = query.filter(cid.eq_any(cids));
    }

    if let Some(p_name) = &params.name {
        query = query.filter(name.eq(p_name))
    }

    if let Some(before) = &params.before {
        query = query.filter(created_at.lt(before))
    }

    if let Some(after) = &params.after {
        query = query.filter(created_at.gt(after))
    }

    if let Some(limit) = &params.limit {
        query = query.limit(limit.clone() as i64)
    }

    let results = query.load::<IpfsFile>(conn).await?; // Execute query

    Ok(results)
}

pub async fn find_pin<'a>(
    conn: &mut PgConnection<'a>,
    q_cid: String,
) -> Result<Option<IpfsFile>, diesel::result::Error> {
    use crate::db::repo::pins::files::dsl::*;

    match files
        .filter(cid.eq(q_cid.clone()))
        .or_filter(req_id.eq(q_cid))
        .first::<IpfsFile>(conn)
        .await
    {
        Ok(record) => Ok(Some(record)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}
