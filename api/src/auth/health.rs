// Check whether if the user exists

use crate::auth::extractor::AuthUser;

pub struct UserExists{
    user_id: String
}

pub fn check_user_existense(user_id: String) -> Result<String, diesel::result::Error>{
    let user_check = AuthUser{
        user_id: user_id
    };
    
}
