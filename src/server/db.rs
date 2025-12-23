#![cfg(feature = "server")]

use sqlx::PgPool;
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn init_db(pool: PgPool) {
    let _ = DB_POOL.set(pool);
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB Pool not initialized")
}
