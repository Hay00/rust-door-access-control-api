use chrono::{Datelike, NaiveTime, Utc};
use chrono_tz::Brazil;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::schema::users_accesses;
use crate::utils::{error_mapper, MappedErrors};

#[derive(Insertable, Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::models::schema::users_accesses)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserAccess {
    pub user_id: i32,
    pub day_of_week: i32,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserAccessCreate {
    #[validate(range(min = 1, max = 7))]
    pub day_of_week: i32,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserAccessUpdate {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

pub async fn create(
    pool: &deadpool_diesel::mysql::Pool,
    access: UserAccess,
) -> Result<(), MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    conn.interact(move |conn| {
        diesel::insert_into(users_accesses::table)
            .values(&access)
            .execute(conn)
    })
    .await
    .map_err(error_mapper)?
    .map_err(error_mapper)?;

    Ok(())
}

pub async fn find(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: u32,
) -> Result<Vec<UserAccess>, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    let results = conn
        .interact(move |conn| {
            users_accesses::table
                .filter(users_accesses::user_id.eq(user_id as i32))
                .load::<UserAccess>(conn)
        })
        .await
        .map_err(error_mapper)?
        .map_err(error_mapper)?;

    Ok(results)
}

pub async fn list_all(
    pool: &deadpool_diesel::mysql::Pool,
) -> Result<Vec<UserAccess>, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    let results = conn
        .interact(move |conn| users_accesses::table.load::<UserAccess>(conn))
        .await
        .map_err(error_mapper)?
        .map_err(error_mapper)?;

    Ok(results)
}

pub async fn update(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: u32,
    day_id: u32,
    user_accesses: UserAccessUpdate,
) -> Result<(), MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    conn.interact(move |conn| {
        diesel::update(users_accesses::table.find((user_id as i32, day_id as i32)))
            .set((
                users_accesses::start.eq(user_accesses.start),
                users_accesses::end.eq(user_accesses.end),
            ))
            .execute(conn)
    })
    .await
    .map_err(error_mapper)?
    .map_err(error_mapper)?;

    Ok(())
}

pub async fn delete(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: u32,
    day_id: u32,
) -> Result<(), MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    conn.interact(move |conn| {
        diesel::delete(users_accesses::table.find((user_id as i32, day_id as i32))).execute(conn)
    })
    .await
    .map_err(error_mapper)?
    .map_err(error_mapper)?;

    Ok(())
}

pub async fn has_access_now(
    pool: &deadpool_diesel::mysql::Pool,
    user_id: i32,
) -> Result<bool, MappedErrors> {
    let conn = pool.get().await.map_err(error_mapper)?;

    // Get current day and hour
    let current_day = Utc::now().with_timezone(&Brazil::East);
    let current_hour = current_day.time();

    // Get current day of week, it starts from 1 (Sunday), same as on the database migration
    let day_of_week = current_day.weekday().number_from_sunday() as i32;

    conn.interact(move |conn| {
        users_accesses::table
            .filter(users_accesses::user_id.eq(user_id))
            .filter(users_accesses::day_of_week.eq(day_of_week))
            .filter(users_accesses::start.le(current_hour))
            .filter(users_accesses::end.ge(current_hour))
            .first::<UserAccess>(conn)
    })
    .await
    .map_err(error_mapper)?
    .map_err(error_mapper)?;

    Ok(true)
}
