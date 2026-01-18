use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Duration, Utc};

#[derive(Deserialize, Serialize)]
pub struct Jwt{
    // user_id, created_at, expired_at
    pub user_id: String,
    pub jwt_created_at: usize,
    pub jwt_expired_at: usize
}

// To create a jwt_token it requires user_id and secret

pub fn sign_jwt(user_id: &str, secret: &str) -> Result <String, jsonwebtoken::errors::Error>{
    let current_timestamp = Utc::now();
    let jwt_impl = Jwt{
        user_id: user_id.to_string(),
        jwt_created_at: current_timestamp.timestamp() as usize,
        jwt_expired_at: (current_timestamp + Duration::hours(24)).timestamp() as usize
    };

    // Encode creates a JWt string, it asks for these 3 arguments
    encode(
        &Header::new(Algorithm::HS256),
        &jwt_impl,
        &EncodingKey::from_secret(secret.as_bytes())
    )
}

// To verify jwt token and secret are the 2 things required
pub fn verify_jwt(token: &str, secret: &str) -> Result<Jwt, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Jwt>(
        token,
        DecodingKey::from_secret(secret.as_bytes()),
        &validation
    )?;

    Ok(data.jwt_impl)
}
