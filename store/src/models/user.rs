use diesel::{prelude::*};
use uuid::Uuid;
use crate::store::Store;
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
        let id = Uuid::new_v4().to_string(); //Uuid trait not from diesel
        let u = User{
            username,
            password,
            id: id.clone()
        };
        diesel::insert_into(crate::schema::user::table).values(&u)
        .returning(User::as_returning())
        .get_result(&mut self.conn); // had to import Insertable
        Ok(id.to_string())
    }

    pub fn sign_in(&self, username: String, password: String) {

    }
}
