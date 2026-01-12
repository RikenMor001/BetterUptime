use std::env;

pub struct Config {
    pub db_url: String
}

impl Config {
    pub fn default() -> Result<Self, diesel::result::Error> {
        let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Please provide the database url enviornment variable"));
        Ok(Self{
            db_url
        })
    }
}

