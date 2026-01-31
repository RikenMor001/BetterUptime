use std::time::Instant;

use diesel::prelude::*;
use store::store::Store;
use store::schema::user::dsl::*;

pub enum HealthError{
    Connection(diesel::ConnectionError),
    Query(diesel::result::Error)
}

impl From<diesel::ConnectionError> for HealthError{
    fn from(value: diesel::ConnectionError) -> Self {
        Self::Connection((value))
    }
}

impl From<diesel::result::Error> for HealthError{
    fn from(value: diesel::result::Error) -> Self {
        Self::Query((value))
    }
}       

pub struct WebsiteHealthResult{
    pub up:bool,
    pub response_time:u128,
    pub status_code: Option<u16>,
    pub error: Option<String>
}

// Check connection 
pub fn check_user_health(user_id: String) -> Result<(bool, String), HealthError> {
    let mut store = Store::default()?;
    let conn = &mut store.conn;

    // users.fiilter.select.first.optional
    let found = user
    .filter(username.eq(&user_id))
    .select(id)
    .first::<String>(conn)
    .optional()?;

    match found{
        Some(_) => Ok((true, "User exists and server is up".to_string())),
        None => Ok((false, "User does not exist".to_string()))
    }
}

pub async fn check_website_health(url: &str) -> Result<WebsiteHealthResult, String>{
    let client = reqwest::Client::new();
    let start = Instant::now();

    let response = client
    .get(url)
    .send()
    .await
    .map_err(|e| e.to_string())?;

    Ok(WebsiteHealthResult {
        up: response.status().is_success(),
        response_time: start.elapsed().as_millis(),
        status_code: Some(response.status().as_u16()),
        error:None
    })
}
