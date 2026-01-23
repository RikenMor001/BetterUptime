use diesel::prelude::*;
use crate::store::Store;
use crate::schema::users::dsl::*;

pub fn check_user_existence( user_id: String ) -> Result<(bool, String), diesel::result::Error> {

    let mut store = Store::new()?;   // DB/server check
    let conn = &mut store.conn;

    let user = users
        .filter(id.eq(&user_id))
        .select(id)
        .first::<String>(conn)
        .optional()?;   // avoids NotFound error

    match user {
        Some(_) => Ok((true, "User exists and server is up".to_string())),
        None => Ok((false, "User does not exist".to_string())),
    }
}
