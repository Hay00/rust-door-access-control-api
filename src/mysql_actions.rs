use std::{
    env,
    sync::{Arc, Mutex},
};

use log::info;
use mysql::{params, prelude::Queryable, Pool, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

pub async fn init_db_connection() -> Result<mysql::PooledConn> {
    let user = env::var("GCA_DOOR_SQL_USER").expect("GCA_DOOR_SQL_USER not set");
    let password = env::var("GCA_DOOR_SQL_PASS").expect("GCA_DOOR_SQL_PASSWORD not set");
    let host = env::var("GCA_DOOR_SQL_HOST").expect("GCA_DOOR_SQL_HOST not set");
    let port = env::var("GCA_DOOR_SQL_PORT").expect("GCA_DOOR_SQL_PORT not set");
    let db_name = env::var("GCA_DOOR_SQL_DB").expect("GCA_DOOR_SQL_DB_NAME not set");

    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, db_name
    );

    let pool = Pool::new(db_url.as_str()).unwrap();
    return pool.get_conn();
}

pub async fn is_user_valid(
    sql_con: Arc<Mutex<mysql::PooledConn>>,
    user: String,
    pass: String,
) -> bool {
    let mut sql_con = sql_con.lock().unwrap();

    let stmt = sql_con
        .prep("SELECT user_name, password FROM users WHERE user_name = :user AND password = :pass")
        .unwrap();

    let result = sql_con
        .exec_first(
            &stmt,
            params! {
                "user" => user,
                "pass" => pass,
            },
        )
        .map(|row| {
            row.map(|(user_name, password)| User {
                username: user_name,
                password: password,
            })
        });

    match result.unwrap() {
        Some(user) => {
            info!("User {} is valid", user.username);
            true
        }
        None => false,
    }
}
