use diesel::prelude::*;
use crate::schema::notification;

#[derive(Queryable, Insertable, Selectable)]
#[diesel(table_name = notification)]
pub struct Notification{
    pub id: String,
    pub user_id: String,
    pub email: String,
    pub notify_up: bool,
    pub notify_down: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Store {
    pub fn create_notification(&mut self, website_id: String) -> Result<Vec<String>, diesel::result::Error>{
        use crate::schema::{website, notification};
        website::table
        .inner_join(notification::table.on(notification::website_id.eq(website::user_id)))
        .filter(website::id.eq(website_id))
    }
}
