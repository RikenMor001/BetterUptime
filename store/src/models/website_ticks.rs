use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::sql_types::WebsiteStatus;
use crate::schema::website_tick::dsl::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = website_tick)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

// Structuring the struct according to the website_tick table in schema
pub struct WebsiteTick {
    pub id: String,
    pub website_id: String,
    pub region_id: String,
    pub response_time_ms: i32,
    pub status: WebsiteStatus,
    pub updated_at: NaiveDateTime
}

impl Store {
    pub fn save_website_check_results() -> Result<WebsiteTick, diesel::result::Error>{
        let id = Uuid::new_v4().to_string();

        let tick = WebsiteTick{
            id: id, 
            website_id,
            region_id,
            response_time_ms,
            status,
            updated_at: Utc::now().naive_utc()
        };
    }
}
