use diesel::prelude::*;
use store::store::Store;
use store::schema::user::dsl::*;

// Check connection 
pub fn check_user_health(user_id: String) -> Result<(bool, String), diesel::ConnectionError> {
    let mut store = Store::default()?;
    let conn = &mut store.conn;

    // users.fiilter.select.first.optional
    let found = user
    .filter(id.eq(&user_id))
    .select(id)
    .first::<String>(conn)
    .optional();

    match found{
        Some(_) => Ok((true, "User exists and server is up".to_string())),
        None(_) => Ok((false, "Either user doesn't exist or the server is down".to_string())),
    }
}
