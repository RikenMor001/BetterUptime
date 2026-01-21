// Check whether if the user exists
use diesel::prelude::*;
use store::schema::user;
use crate::auth::extractor::AuthUser;

pub fn check_user_existense(user_id: String) -> Result<(bool, String), diesel::result::Error>{
    let mut checkConn = establish_connection()?;

    // Will get the user from the database
    // Try to match the user if they match correctly

    match user{
        Some(_) => Ok(true ,"User exists in the database".to_string()),
        None(_) => Ok(false, "User does not exist or the server is down".to_string());
    };
}
