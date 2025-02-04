use diesel::{Insertable, Queryable};

// Diesel schema definition
diesel::table! {
    files (id) {
        id -> BigInt,
        created_at -> Timestamptz,
        cid -> Varchar,
        size -> BigInt,
        bundle_tx_id -> Varchar,
        envelope_id -> Varchar,
        name -> Nullable<Varchar>,
        req_id -> Varchar,
    }
}
