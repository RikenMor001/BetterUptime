// TODO
//1. Add authenctication to the current codebase first and then follow up with the other two TODO's 
//2. Adding response time and letting developer know if the website/API/server is up right now
//3. Adding a system that sends notifications via email to let the developer know that the system is down

// Using get and post request handlers from the poem library
use poem::{
    Error, Route, Server, get, handler, http::StatusCode, listener::TcpListener, post, web::{Json, Path}
};

pub mod auth;
pub mod inputs;
pub mod outputs;

use inputs::CreateWebsiteInput;
use outputs::CreateWebsiteOutput;

use store::{ models::user::{AuthError}, store::Store };
use crate::{ auth::validation::sign_jwt, inputs::{CreateUserInput, CreateUserInputSignIn}, outputs::{CreateUserOutput, CreateUserOutputSignin, HealthResponse} };
use crate::auth::health::{check_user_health, HealthError};
use diesel::result::{Error as DieselError, DatabaseErrorKind};

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("BetterUptime: {website_id}" )
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Result<Json<CreateUserOutput>, Error> {
    let mut s = Store::default()
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let id = s
        .sign_up(data.username, data.password)
        .map_err(|e| {
            match e {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Error::from_string("User already exists", StatusCode::CONFLICT) // Returns 409 
                }
                _=> Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
            }
        })?;

        Ok(
            Json(
                CreateUserOutput { 
                    id, 
                    msg: "User signed up".to_string() 
                }
            )
        )
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInputSignIn>) -> Result<Json<CreateUserOutputSignin>, Error> {
    let mut s = Store::default()
    .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let user_id = s.sign_in(data.ref_username, data.ref_password)
    .map_err(|e| match e {
            AuthError::UserNotFound=>Error::from_string("User doesn't exist",StatusCode::NOT_FOUND),
            AuthError::InvalidPassword => Error::from_string("Invalid password", StatusCode::UNAUTHORIZED),
            AuthError::Internal(msg) => Error::from_string(msg, StatusCode::INTERNAL_SERVER_ERROR)
        })?;    

    let jwt_secret = std::env::var("JWT_SECRET")
    .map_err(|_| Error::from_string("JWT_SECRET not found", StatusCode::INTERNAL_SERVER_ERROR))?; // Make sure to unwrap the secret

    let token = sign_jwt(&user_id, &jwt_secret)
    .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(CreateUserOutputSignin{
        id: user_id,
        msg: "User signed in".to_string(),
        token
    }))
}   

#[handler]
async fn create_website(Json(data): Json<CreateWebsiteInput> ) -> Result<Json<CreateWebsiteOutput>, Error> {
    let mut s: Store = Store::default().unwrap();

    // Convert Diesel error into an HTTP 500 instead of crashing
    let website = s
        .create_website(
            data.user_id,
            data.url,
        )
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(CreateWebsiteOutput { id: website.id }))
}

#[handler]
fn health(Path(user_id): Path<String>) -> Result<Json<HealthResponse>, Error>{
    match check_user_health(user_id) {
        Ok((true, msg)) => Ok(Json(HealthResponse { 
            up: true, 
            user_exists: true, 
            message: msg 
        })),

        Ok((false, msg)) => Ok(Json(HealthResponse { 
            up: true, 
            user_exists: false, 
            message: msg 
        })),
        
        Err(HealthError::Connection(e)) => Err(
            Error::from_string(e.to_string(), StatusCode::SERVICE_UNAVAILABLE)
        ),

        Err(HealthError ::Query(e)) => Err(
            Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR)
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let app = Route::new()
        .at("/health/:user_id", get(health))
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("BetterUptime")
        .run(app)
        .await
}
