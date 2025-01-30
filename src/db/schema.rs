use diesel::{Insertable, Queryable};

// Diesel schema definition
diesel::table! {
    files (id) {
        id -> BigInt,
        created_at -> Timestamptz,
        cid -> Varchar,
        size -> BigInt,
    }
}
