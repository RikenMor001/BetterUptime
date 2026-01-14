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

pub mod inputs;
pub mod outputs;

use inputs::CreateWebsiteInput;
use outputs::CreateWebsiteOutput;

use store::store::Store;
use crate::{ inputs::{CreateUserInput, CreateUserInputSignIn}, outputs::{CreateUserOutput, CreateUserOutputSignin} };

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

    let id = s.sign_in(data.ref_username, data.ref_password)
    .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(CreateUserOutputSignin{
        id: id.to_string()
    }))
}   

#[handler]
async fn create_website(
    Json(data): Json<CreateWebsiteInput>
) -> Result<Json<CreateWebsiteOutput>, Error> {
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
