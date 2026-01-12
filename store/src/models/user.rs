use diesel::{prelude::*};
use uuid::Uuid;
use crate::store::Store;
use argonautica::Hasher;
use dotenv::dotenv;
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]


pub struct User {
    id: String, 
    username: String,
    password: String
}

impl Store {
    pub fn sign_up(&mut self, username: String, password: String) -> Result<String, diesel::result::Error> {
        dotenv().ok();
        let secret_message = std::env::var("SECRET_KEY").expect("SECRET_KEY expected here, not found");
        let hashed_password = Hasher::default().with_password(password).with_secret_key(secret_message).hash().unwrap();
        let id = Uuid::new_v4().to_string(); //Uuid trait not from diesel
        let u = User{
            username,
            password: hashed_password,
            id: id.clone()
        };
        diesel::insert_into(crate::schema::user::table).values(&u)
        .returning(User::as_returning())
        .get_result(&mut self.conn)?; // had to import Insertable
        Ok(id.to_string())
    }

    pub fn sign_in(&mut self, ref_username: String, ref_password: String) -> Result<bool, diesel::result::Error> {
        use crate::schema::user::dsl::*;

        let user_data = user // fetches the user data and returns it
        .filter(username.eq(ref_username))
        .select(User::as_select())
        .first(&mut self.conn)?;

        if user_data.password != ref_password {
            return Ok(false);
        }
        Ok(true)
    }
}
