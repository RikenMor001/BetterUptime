use diesel::{prelude::*};
use uuid::Uuid;
use crate::store::Store;
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]

struct User {
    id: String, 
    username: String,
    password: String
}

impl Store {
    pub fn sign_up(&self, username: String, password: String) {
        let id = Uuid::new_v4().to_string(); //Uuid trait not from diesel
        let u = User{
            username,
            password,
            id: id
        };
        diesel::insert_into(crate::schema::user::table).values(u); // had to import Insertable
    }

    pub fn sign_in(&self, username: String, password: String) {
    }
}
