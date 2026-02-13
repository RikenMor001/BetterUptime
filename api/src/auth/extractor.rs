use poem::{
    error::Error,
    http::{header, StatusCode},
    FromRequest, Request, RequestBody, Result,
};

use crate::auth::validation::verify_jwt;

pub struct AuthUser {
    pub user_id: String,
}
impl<'a> FromRequest<'a> for AuthUser {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        // Prefer typed header name
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(h) => h,
            None => {
                return Err(Error::from_string(
                    "Missing Authorization header",
                    StatusCode::UNAUTHORIZED,
                ))
            }
        };

        let auth_string = match auth_header.to_str() {
            Ok(s) => s.trim(),
            Err(_) => {
                return Err(Error::from_string(
                    "Invalid Authorization header",
                    StatusCode::UNAUTHORIZED,
                ))
            }
        };

        // Be tolerant: accept Bearer/bearer and extra spaces
        let token = {
            // Split by whitespace: "Bearer <token>"
            let mut parts = auth_string.split_whitespace();
            let scheme = parts.next().unwrap_or("");
            let t = parts.next().unwrap_or("");

            if !scheme.eq_ignore_ascii_case("bearer") || t.is_empty() {
                return Err(Error::from_string(
                    "Expected: Authorization: Bearer <token>",
                    StatusCode::UNAUTHORIZED,
                ));
            }

            // If there are extra segments, tr
            eat as invalid (helps catch malformed headers)
            if parts.next().is_some() {
                return Err(Error::from_string(
                    "Malformed Authorization header",
                    StatusCode::UNAUTHORIZED,
                ));
            }

            t
        };

        let jwt_secret = match std::env::var("JWT_SECRET") {
            Ok(s) if !s.trim().is_empty() => s,
            _ => {
                return Err(Error::from_string(
                    "JWT_SECRET not found",
                    StatusCode::UNAUTHORIZED,
                ))
            }
        };

        let jwt = match verify_jwt(token, &jwt_secret) {
            Ok(v) => v,
            Err(e) => {
                // Keep client error generic, but log the real reason (helpful in dev)
                eprintln!("JWT verify error: {e}");
                return Err(Error::from_string(
                    "Invalid or expired token",
                    StatusCode::UNAUTHORIZED,
                ));
            }
        };

        Ok(AuthUser {
            user_id: jwt.sub,
        })
    }
}
