// Get authorization header, convert header to a string,
// extract bearer token, load the jwt secret,
// verify the jwt signature, return authenticated user

use poem::{
    error::Error,
    FromRequest, Request, RequestBody, Result,
    http::StatusCode
};

use crate::auth::validation::verify_jwt;

pub struct AuthUser{
    pub user_id: String
}

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
            Err(_) => {
                return Err(Error::from_string(
                    "Invalid Authorizaiton header",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        let extract_token = match auth_string.strip_prefix("Bearer "){
            Some(t) => t,
            None => {
                return Err(Error::from_string(
                    "Expected a bearer token in string format",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };
        
        let jwt_secret = match std::env::var("JWT_SECRET"){
            Ok(s) => s,
            Err(_) => {
                return Err(Error::from_string(
                    "JWT_SECRET not found",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        let jwt = match verify_jwt(extract_token, &jwt_secret){
            Ok(v) => v,
            Err(_) => {
                return Err(Error::from_string(
                    "Invalid or expired token",
                    StatusCode::UNAUTHORIZED
                ))
            }
        };

        Ok(AuthUser{
            user_id: jwt.user_id
        })
    }
}
