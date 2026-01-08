use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Connection;
use std::env;

pub mod schema;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    fn default() -> Result<Self, ConnectionError> {
        let db_url = env::var("DATABASE_URL")?;
        .unwrap_or_else(|_| panic!("Please provide the database url enviornment variable"));

        let conn = PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to the database"));
        Ok(Self {
            conn
        })
    } 
}
