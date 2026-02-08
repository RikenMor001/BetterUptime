use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::sql_types::WebsiteStatus;
use crate::schema::website_tick::dsl::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = website_tick)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct WebsiteTick {
    pub id: String,
    pub website_id: String,
    pub region_id: String,
    pub response_time_ms: i32,
    pub status: WebsiteStatus,
    pub updated_at: NaiveDateTime
}

impl Store {
    pub fn save_website_check_results(){
        let id = Uuid::new_v4().to_string();
    }
}
