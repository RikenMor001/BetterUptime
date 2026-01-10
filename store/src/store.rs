use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection;
use crate::config::Config;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Self {
        let config = Config::default();

        let conn = PgConnection::establish(&config.db_url).unwrap_or_else(|_| panic!("Error connecting to the database"));
        Self{
            conn
        }
    } 
}
