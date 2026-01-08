use diesel::prelude::*;
use diesel::PgConnection;
use diesel::result::ConnectionError;
use dotenvy::dotenv;
use std::env;

pub mod schema;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn new() -> Result<Self, ConnectionError> {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL")
            .expect("Please provide a DATABASE_URL in the environment");

        let conn = PgConnection::establish(&db_url)?;

        Ok(Self { conn })
    }
}
