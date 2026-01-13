use serde::{ Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserOutput{
    pub id: String  
}
