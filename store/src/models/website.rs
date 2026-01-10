use diesel::prelude::*;
use crate::store::Store;
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website {
    id: String, 
    url: String,
    user_id: Option<String>,
    time_added: chrono::NaiveDateTime
}

impl Store {
    pub fn create_website(&self) {
    }

    pub fn get_website(&self) {
    }
}
