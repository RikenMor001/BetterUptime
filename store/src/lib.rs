use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection;
use std::env;
use config::Config;

pub mod schema;
pub mod config;
pub struct Store {
    pub conn: PgConnection
}

impl Store {
    fn default() -> Result<Self, ConnectionError> {
        let config = Config::default()?;

        let conn = PgConnection::establish(&config.db_url).unwrap_or_else(|_| panic!("Error connecting to the database"));
        Ok(Self {
            conn
        })
    } 
}
