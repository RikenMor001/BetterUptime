use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::error::Error;

pub mod schema;

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let db_url = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&db_url)?;
        Ok(Self { conn })
    }
}
