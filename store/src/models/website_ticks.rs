use crate::store::Store;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = website_ticks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct WebsiteTick {
    pub id: String, 
    pub website_id: String,
    pub response_time_ms: i32,
    pub status: WebsiteStatus,
    
}

impl Store {
    pub fn save_website_check_results(){
        let id = Uuid::new_v4().to_string();
    }
}
