// validation.rs
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Jwt {
    // Standard claims:
    // sub = subject (user id), iat = issued at, exp = expires at
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
}

pub fn sign_jwt(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now().timestamp();

    let claims = Jwt {
        sub: user_id.to_string(),
        iat: now,
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Jwt, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(Algorithm::HS256);

    // Enforce `exp` validation (it is typically validated if present, but be explicit).
    validation.validate_exp = true;

    let data = decode::<Jwt>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    Ok(data.claims)
}
