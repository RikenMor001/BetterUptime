// TODO
//1. Add authenctication to the current codebase first and then follow up with the other two TODO's 
//2. Adding response time and letting developer know if the website/API/server is up right now
//3. Adding a system that sends notifications via email to let the developer know that the system is down

// Using get and post request handlers from the poem library
use poem::{
    get, handler, post,
    http::StatusCode,
    listener::TcpListener,
    web::{Json, Path},
    Error, Route, Server,
};

pub mod auth;
pub mod inputs;
pub mod outputs;

use inputs::CreateWebsiteInput;
use outputs::CreateWebsiteOutput;

use store::{ models::user::AuthError, store::Store };
use crate::{ inputs::{CreateUserInput, CreateUserInputSignIn}, outputs::{CreateUserOutput, CreateUserOutputSignin, HealthResponse} };
use crate::auth::health::{check_user_health, HealthError};

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
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(CreateUserOutput { id }))
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

    Ok(Json(CreateUserOutputSignin{
        id: user_id,
        msg: "User signed in".to_string()
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
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(sign_up))
        .at("/signin", post(sign_in));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("BetterUptime")
        .run(app)
        .await
}
