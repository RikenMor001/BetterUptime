// Using get and post request handlers from the poem crate
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
use crate::{ inputs::CreateUserInput, outputs::CreateUserOutput };

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("BetterUptime: {website_id}" )
}

#[handler]
fn user_sign_in(Json(data): Json<CreateUserInput>) -> Result<Json<CreateUserOutput>, Error> {
    let mut s = Store::default()
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    let id = s
        .sign_up(data.username, data.password)
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(Json(CreateUserOutput { id }))
}

#[handler]
async fn create_website(
    Json(data): Json<CreateWebsiteInput>
) -> Result<Json<CreateWebsiteOutput>, Error> {
    let mut s: Store = Store::default().unwrap();

    // Convert Diesel error into an HTTP 500 instead of crashing
    let website = s
        .create_website(
            Some(String::from("a78d3a54-e03e-40bf-a929-4c439e46af91")),
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
        .at("/website", post(create_website));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("BetterUptime")
        .run(app)
        .await
}
