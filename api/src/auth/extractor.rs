// Get authorization header, convert header to a string,
// extract bearer token, load the jwt secret,
// verify the jwt signature, return authenticated user

use poem::{
    error::Error,
    FromRequest, Request, RequestBody, Result,
    http::StatusCode
};

use crate::validation::verify_jwt;

pub struct AuthUser{
    pub user_id: String
}

#[poem::async_trait]
impl <'a> FromRequest<'a> for AuthUser{
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let auth_header = match req.headers().get("Authorization"){
            Some(h) => h,
            None => {
                return Err(Error::from_string(
                    "Missing Authorization headers",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        let auth_string = match auth_header.to_str(){
            Ok(s) => s,
            None => {
                return Err(Error::from_string(
                    "Missing Authorization header",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        let extract_token = match auth_string.strip_prefix("Bearer "){
            Some(t) => t,
            None => {
                Err(Error::from_string(
                    "Expected a bearer token in string format",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        let jwt_secret = match std::env::var("JWT_SECRET"){
            Some(s) => s,
            None => {
                Err(Error::from_string(
                    "JWT_SECRET missing",
                    StatusCode::INTERNAL_SERVER_ERROR
                ))
            }
        };

        let jwt = match verify_jwt(extract_token, jwt_secret){
            Some(v) => v,
            None => {
                Err(Error::from_string(
                    "Unable to verify token",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        Ok(AuthUser{
            user_id: verify_token.sub
        })
    }
}
