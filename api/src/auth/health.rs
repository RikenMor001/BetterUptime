use diesel::prelude::*;
use store::store::Store;
use store::schema::user::dsl::*;

pub enum HealthError{
    Connection(diesel::ConnectionError),
    Query(diesel::result::Error)
}

impl From<diesel::ConnectionError> for HealthError{
    fn from(value: diesel::ConnectionError) -> Self {
        Self::Connection((value))
    }
}

impl From<diesel::result::Error> for HealthError{
    fn from(value: diesel::result::Error) -> Self {
        Self::Query((value))
    }
}       

// Check connection 
pub fn check_user_health(user_id: String) -> Result<(bool, String), HealthError> {
    let mut store = Store::default()?;
    let conn = &mut store.conn;

    // users.fiilter.select.first.optional
    let found = user
    .filter(id.eq(&user_id))
    .select(id)
    .first::<String>(conn)
    .optional()?;

    match found{
        Some(_) => Ok((true, "User exists and server is up".to_string())),
        None => Ok((false, "User does not exist".to_string()))
    }
}
