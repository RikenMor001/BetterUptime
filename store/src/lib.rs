pub mod schema;

use diesel::prelude::*;
use diesel::result::ConnectionError;
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn new() -> Result<Self, ConnectionError> {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self::from_url(&db_url)
    }

    pub fn from_url(database_url: &str) -> Result<Self, ConnectionError> {
        let conn = PgConnection::establish(database_url)?;
        Ok(Self { conn })
    }
}
