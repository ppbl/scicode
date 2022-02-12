use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct ConnectionPool {
    pub connection_pool: PgPool,
}

// pub fn establish_connection() -> PgConnection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }

lazy_static! {
    pub static ref CONN_POOL: ConnectionPool = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        ConnectionPool {
            connection_pool: r2d2::Pool::builder()
                .max_size(15)
                .build(ConnectionManager::new(&database_url))
                .expect(&format!("Error connecting to {}", database_url)),
        }
    };
}

pub fn can_connect() -> bool {
    self::CONN_POOL.connection_pool.get().is_ok()
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    self::CONN_POOL.connection_pool.get().unwrap()
}
