use diesel::{prelude::*};
use uuid::Uuid;
use crate::store::Store;
use argonautica::{ Hasher, Verifier };
use dotenv::dotenv;
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User {
    id: String, 
    username: String,
    password: String
}

#[derive(Debug)]
pub enum AuthError{
    UserNotFound,
    InvalidPassword,
    Internal(String)
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error> {
        dotenv().ok();
        let secret_message = std::env::var("SECRET_KEY")
        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

        let hashed_password = Hasher::default().with_password(password).with_secret_key(secret_message).hash()
        .map_err(|_| diesel::result::Error::RollbackTransaction)?;

        let id = Uuid::new_v4().to_string(); //Uuid trait not from diesel

        let u = User{
            id: id.clone(),
            username,
            password: hashed_password
        };
        
        diesel::insert_into(crate::schema::user::table).values(&u)
        .execute(&mut self.conn)?;// had to import Insertable

        Ok(id.to_string())
    }

    pub fn sign_in(&mut self, ref_username: String, ref_password: String) -> Result<String, AuthError> {
        use crate::schema::user::dsl::*;

        let user_data = user // fetches the user data and returns it
        .filter(username.eq(ref_username))
        .select(User::as_select())
        .first(&mut self.conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => AuthError::UserNotFound,
            _ => AuthError::Internal(e.to_string())
        })?;
        
        let secret_message = std::env::var("SECRET_KEY")
        .map_err(|e| AuthError::Internal(e.to_string()))?;

        let is_valid = Verifier::default()
        .with_hash(user_data.password)
        .with_password(ref_password)
        .with_secret_key(secret_message)
        .verify()
        .unwrap_or(false);

        if !is_valid {
            return Err(AuthError::InvalidPassword);
        }

        Ok(user_data.id)
    }
}
