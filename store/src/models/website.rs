use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
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
    pub fn create_website(&mut self, user_id: Option<String>, url: String) {
        let id = Uuid::new_v4().to_string();
        let website = Website{
            user_id,
            url,
            id: id,
            time_added: Utc::now().naive_utc()
        };
        diesel::insert_into(crate::schema::website::table)
        .values(&website).returning(Website::as_returning())
        .get_result(&mut self.conn);
    }

    pub fn get_website(&self) {
    }
}
