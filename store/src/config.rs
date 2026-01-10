use std::env;

pub struct Config {
    pub db_url: String
}

impl Config {
    pub fn default() -> Self {
        let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("Please provide the database url enviornment variable"));
        Self { 
            db_url
        }
    }
}

