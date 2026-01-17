use serde::{Deserialize, Serialize};
use Uuid::uuid;

pub struct Jwt{
    // user_id, created_at, expired_at
    pub user_id: String,
    pub jwt_created_at: usize,
    pub jwt_expired_at: usize
}