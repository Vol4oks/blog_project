use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::post::Post;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default = "default_offset")]
    pub offset: i32,
}

fn default_limit() -> i32 {
    10
}
fn default_offset() -> i32 {
    0
}

#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    pub post: Vec<Post>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub uuid: Uuid,
}
