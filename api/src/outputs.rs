use serde::{ Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserOutput {
    pub id: String, 
    pub msg: String
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserOutputSignin{
    pub id: String,
    pub msg: String,
    pub token: String
}

#[derive(Deserialize, Serialize)]
pub struct HealthResponse{
    pub up: bool,
    pub user_exists: bool,
    pub message: String
}
