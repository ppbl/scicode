use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use once_cell::sync::Lazy;
use std::{env, sync::Mutex};

type PgPool = Pool<ConnectionManager<PgConnection>>;

static PG_POOL: Lazy<Mutex<PgPool>> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_pool = r2d2::Pool::builder()
        .max_size(15)
        .build(ConnectionManager::new(&database_url))
        .expect(&format!("Failed to create pool to {}", database_url));
    Mutex::new(pg_pool)
});

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    PG_POOL
        .lock()
        .unwrap()
        .get()
        .expect("Failed to connect to db")
}
