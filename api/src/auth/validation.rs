use serde::{Deserialize, Serialize};
use Uuid::uuid;
use jsonwebtoken::{Encode, Decode};
use chrono::{Duration, Utc};

pub struct Jwt{
    // user_id, created_at, expired_at
    pub user_id: String,
    pub jwt_created_at: usize,
    pub jwt_expired_at: usize
}

pub fn sign_jwt(user_id: &str, secret: &str) -> Result <String, jsonwebtoken::errors::Error>{
    let current_timestamp = Utc::now();
    let jwt_impl = Jwt{
        user_id: user_id.to_string(),
        jwt_created_at: current_timestamp.timestamp() as usize,
        jwt_expired_at: (current_timestamp + Duration::hours(24)).timestamp() as usize
    };
}
