use serde::{ Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteOutput {
    pub id: String,
}

pub struct CreateUserOutput{
    pub id: String  
}
