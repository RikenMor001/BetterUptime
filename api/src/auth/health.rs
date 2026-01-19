// Check whether if the user exists

use crate::auth::extractor::AuthUser;

pub struct UserExists{
    user_id: String
}

pub fn check_user_existense(user_id: String) -> Result<bool, diesel::result::Error>{
    let user_check = AuthUser{
        user_id: UserExists?
    };

    if user_check{
       return true 
    }
    return false
}

