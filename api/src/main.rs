// Using get and post request handlers from the poem crate
use poem::{
    post, get, handler,
    listener::TcpListener,
    web::{Json, Path},
    Route, Server
};

pub mod inputs;
pub mod outputs;

use store::store::Store;
use inputs::CreateWebsiteInput;
use outputs::CreateWebsiteOutput;

#[handler] // get request handler
fn get_website(Path(website_id): Path<String>) -> String {
    format!("BetterUptime: {website_id}")
}

#[handler] // post request handler
async fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let s: Store = Store::default();
    let id = s.create_website(); 
    let response = CreateWebsiteOutput {
        id
    };
    Json(response)
}

#[tokio::main]  // Tokio main is a macro that is used for the main
//function to run on the main thread of tokio runtime
async fn main() -> Result<(), std::io::Error>{
    let app = Route::new()
    .at("/website/:website_id", get(get_website)) // request params
    .at("/website", post(create_website));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .name("BetterUptime")
    .run(app)
    .await
}

