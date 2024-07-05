use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub mod schema;
pub mod models;

pub type ConnectionPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection_pool() -> ConnectionPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    ConnectionPool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}