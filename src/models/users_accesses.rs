use chrono::{Datelike, NaiveTime, Utc};
use chrono_tz::Brazil;
use diesel::prelude::*;

use crate::models::schema::users_accesses;
use crate::utils::{error_mapper, MappedErrors};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::models::schema::users_accesses)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct UserAccess {
    pub user_id: i32,
    pub day_of_week: i32,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl UserAccess {
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
}
