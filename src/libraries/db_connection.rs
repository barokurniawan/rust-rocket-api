use diesel::r2d2::{ConnectionManager, Pool};
use diesel::mysql::MysqlConnection;
use dotenvy::dotenv;
use std::env;

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection manager for MySQL
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    // Create a connection pool with a maximum of 10 connections
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create pool")
}
