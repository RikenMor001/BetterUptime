use diesel::pg::PgConnection;
use diesel::{Connection, RunQueryDsl};
use diesel::query_dsl::methods::SelectDsl;
use crate::config::Config;
use diesel::SelectableHelper;

use crate::models::website::Website;

pub struct Store {
    pub conn: PgConnection
}

impl Store {
    pub fn default() -> Result<Self, diesel::ConnectionError>{
        let config = Config::default();

        let conn: PgConnection = PgConnection::establish(&config.db_url)?;
        Ok(Self { 
            conn 
        }) // Had to return Result<Self, diesel::result::Error> for using .unwrap()
    } 

    pub fn list_websites(&mut self) -> Result<Vec<Website>, diesel::result::Error>{
        use crate::schema::website::dsl::*;

        let rows = website.select(Website::as_select()).load(&mut self.conn)?;
        Ok(rows)
    }
}
