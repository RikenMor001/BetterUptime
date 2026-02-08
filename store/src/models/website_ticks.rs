use crate::store::Store;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = website_ticks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct WebsiteTick {
    pub id: String, 
    pub website_id: String,
    pub response_time_ms: i32,
}

impl Store {

}