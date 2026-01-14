use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteInput {
    pub url: String, 
    pub user_id: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserInput{
    pub username: String,
    pub password: String
}

pub struct CreateUserInputSignIn{
    pub ref_username: String,
    pub ref_password: String
}
