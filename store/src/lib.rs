use diesel::PgConnection;
use std::env;
pub mod schema;
use diesel::prelude::*;
use diesel::result::Error;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    fn default() -> Result<Self, Error> {
        let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Please provide a database_url from the enviornment"));

        let conn:PgConnection = PgConnection::establish(&db_url)?;
        Ok(Self {
            conn
        })
    }
}
