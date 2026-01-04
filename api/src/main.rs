use poem::{
    post, get, handler,
    listener::TcpListener,
    web::{Json, Path},
    Route, Server
};

pub mod inputs;
pub mod outputs;

use inputs::CreateWebsiteInput;
use outputs::CreateWebsiteOutput;

#[handler] // get request handler
fn get_website(Path(name): Path<String>) -> String{
    format!("BetterUptime: {name}")
}

#[handler] // post request handler
async fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let response = CreateWebsiteOutput {
        url: data.url
    };
    Json(response)
}

#[tokio::main]  // Tokio main is a macro that is used for the main
//function to run on the main thread of tokio runtime
async fn main() -> Result<(), std::io::Error>{
    let app = Route::new()
    .at("/website/:name", get(get_website))
    .at("/website", post(create_website));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .name("BetterUptime")
    .run(app)
    .await
}
