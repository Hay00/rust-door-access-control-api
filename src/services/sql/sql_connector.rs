use deadpool_diesel::{mysql::Pool, Manager};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Pool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager).build().unwrap()
}
