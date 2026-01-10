use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;
use crate::{models::website, store::Store};
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Website {
    pub id: String, 
    pub url: String,
    pub user_id: Option<String>,
    pub time_added: chrono::NaiveDateTime
}

impl Store { 
    pub fn create_website(&mut self, user_id: Option<String>, url: String) -> Result<Website, diesel::result::Error>{
        let id = Uuid::new_v4().to_string();
        let website = Website{
            user_id,
            url,
            id: id,
            time_added: Utc::now().naive_utc()
        };
        let website_data = diesel::insert_into(crate::schema::website::table)
        .values(&website).returning(Website::as_returning())
        .get_result(&mut self.conn)?; // Add an question mark just to make sure that the data is unwraped before 
    // its parsed.
        Ok(website_data)
    }

    pub fn get_website(&self) {
         
    }
}
