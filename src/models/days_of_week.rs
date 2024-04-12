use diesel::prelude::*;
use serde::Serialize;

use crate::models::schema::days_of_week;
use crate::utils::{error_mapper, MappedErrors};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::models::schema::days_of_week)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DayOfWeek {
    pub id: i32,
    pub name: String,
}

impl DayOfWeek {
    pub async fn get_day_id_by_name(
        pool: &deadpool_diesel::mysql::Pool,
        name: String,
    ) -> Result<i32, MappedErrors> {
        let conn = pool.get().await.map_err(error_mapper)?;

        let result = conn
            .interact(move |conn| {
                days_of_week::table
                    .filter(days_of_week::name.eq(name))
                    .select(days_of_week::id)
                    .first::<i32>(conn)
            })
            .await
            .map_err(error_mapper)?
            .map_err(error_mapper)?;

        Ok(result)
    }
}
