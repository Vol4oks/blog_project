use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSeconds};
use uuid::Uuid;

static TIME_ZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(3 * 3600).unwrap();

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde_as(as = "Option<TimestampSeconds<i64>>")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Post {
    pub fn get_created_at(&self) -> String {
        let now = self.created_at.with_timezone(&TIME_ZONE);
        format!(
            "Создан: {}.{}.{} {}:{}",
            now.day(),
            now.month(),
            now.year(),
            now.hour(),
            now.minute()
        )
    }

    pub fn get_update_at(&self) -> Option<String> {
        if let Some(time_utc) = self.updated_at {
            let now = time_utc.with_timezone(&TIME_ZONE);
            return Some(format!(
                "Обновлен: {}.{}.{} {}:{}",
                now.day(),
                now.month(),
                now.year(),
                now.hour(),
                now.minute()
            ));
        }

        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    pub post: Option<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostPage {
    pub post: Vec<Post>,
    pub total: i32,
    pub limit: i32,
    pub offset: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub uuid: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: DetailsError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailsError {
    pub resource: String,
}
