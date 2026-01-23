use diesel::prelude::*;
use store::store::Store;
use store::schema::user::dsl::*;

// Check connection 
pub fn check_user_health(user_id: String) -> Result<(bool, String), diesel::result::Error> {
    let mut store = Store::default()?;
    let conn = &mut store.conn;
}
