use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::schema::users;
use crate::utils::{error_mapper, MappedErrors};

#[derive(Queryable, Deserialize, Debug, Validate)]
#[diesel(table_name =  crate::models::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: i32,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(email)]
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, Clone, Validate)]
#[diesel(table_name =  crate::models::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

// Update user
#[derive(Deserialize, Debug, Validate)]
pub struct UpdateUser {
    pub username: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name =  crate::models::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct ListUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

pub async fn create(
    pool: &deadpool_diesel::mysql::Pool,
    user: CreateUser,
) -> Result<usize, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    let user_id = conn
        .interact(move |conn| {
            diesel::insert_into(users::table)
                .values((
                    users::username.eq(user.username),
                    users::email.eq(user.email),
                    users::password.eq(user.password),
                ))
                .execute(conn)
        })
        .await
        .map_err(error_mapper)?
        .map_err(error_mapper)
        .unwrap();

    Ok(user_id)
}

pub async fn find(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: u32,
) -> Result<ListUser, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    let user = conn
        .interact(move |conn| users::table.find(user_id as i32).first::<User>(conn))
        .await
        .map_err(error_mapper)?
        .map_err(error_mapper)?;

    Ok(ListUser {
        id: user.id,
        username: user.username,
        email: user.email,
        created_at: user.created_at,
    })
}

pub async fn update(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: u32,
    user: UpdateUser,
) -> Result<(), MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    conn.interact(move |conn| {
        diesel::update(users::table.find(user_id as i32))
            .set((
                users::username.eq(user.username),
                users::email.eq(user.email),
            ))
            .execute(conn)
    })
    .await
    .map_err(error_mapper)?
    .map_err(error_mapper)?;

    Ok(())
}

pub async fn list(pool: &deadpool_diesel::mysql::Pool) -> Result<Vec<ListUser>, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    let results = conn
        .interact(move |conn| users::table.load::<User>(conn))
        .await
        .map_err(error_mapper)?
        .map_err(error_mapper)?;

    let user_list = results
        .into_iter()
        .map(|user| ListUser {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
        .collect();

    Ok(user_list)
}

pub async fn delete(pool: &deadpool_diesel::mysql::Pool, user_id: u32) -> Result<(), MappedErrors> {
    // First create a field to disable user
    todo!()
}

pub async fn find_by_login(
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
