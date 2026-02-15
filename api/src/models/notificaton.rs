use diesel::prelude::*;

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
