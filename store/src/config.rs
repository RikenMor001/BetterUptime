use std::env;
use std::error::Error;
use serde::Deserialize;

pub struct Config {
    pub db_url: String
}

impl Config {
    fn default() -> Result<Self, Error> {
        let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Please provide the database url enviornment variable"));
        Ok(Self {
            db_url
        })
    }
}