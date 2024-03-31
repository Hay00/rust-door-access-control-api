use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::models::schema::users;
use crate::utils::{error_mapper, MappedErrors};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name =  crate::models::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn all(conn: &mut MysqlConnection) -> Vec<User> {
        use crate::models::schema::users::dsl::*;

        users.select(User::as_select()).load(conn).expect("Err")
    }

    pub async fn find(
        pool: &deadpool_diesel::mysql::Pool,
        email: String,
        password: String,
    ) -> Result<i32, MappedErrors> {
        let conn = pool.get().await.map_err(error_mapper)?;

        let result = conn
            .interact(move |conn| {
                users::table
                    .filter(users::email.eq(email))
                    .filter(users::password.eq(password))
                    .first::<User>(conn)
            })
            .await
            .map_err(error_mapper)?
            .map_err(error_mapper)?;

        Ok(result.id)
    }
}
