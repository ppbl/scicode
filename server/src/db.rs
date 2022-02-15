use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref PG_POOL: PgPool = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        r2d2::Pool::builder()
            .max_size(15)
            .build(ConnectionManager::new(&database_url))
            .expect(&format!("Failed to create pool to {}", database_url))
    };
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    PG_POOL.get().expect("Failed to connect to db")
}
