use diesel::pg::PgConnection;
use diesel::Connection;
use crate::config::Config;
use diesel::ConnectionError;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Result<Self, diesel::ConnectionError>{
        let config = Config::default();

        let conn = PgConnection::establish(&config.db_url)?;
        Ok(Self { 
            conn 
        }) // Write diesel::ConnectionError instead of unwrapping, makes it easy
    } 
}
