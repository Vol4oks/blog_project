use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{TimestampSeconds, serde_as};
use uuid::Uuid;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    pub title: String,
    pub content: String,
}

impl Post {
    pub fn new(id: i64, title: String, content: String, author_id: Uuid) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }
}

impl TryFrom<crate::blog_grpc::Post> for Post {
    type Error = crate::domain::error::BlogError;
    fn try_from(value: crate::blog_grpc::Post) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.title,
            content: value.content,
            author_id: Uuid::from_str(&value.author_id)
                .map_err(|e| crate::domain::error::BlogError::Validation(e.to_string()))?,
            created_at: chrono::DateTime::from_timestamp(value.created_at, 0).ok_or_else(|| {
                crate::domain::error::BlogError::Validation(value.created_at.to_string())
            })?,
            updated_at: chrono::DateTime::from_timestamp(value.created_at, 0),
        })
    }
}

impl From<Post> for crate::blog_grpc::Post {
    fn from(value: Post) -> Self {
        Self {
            id: value.id,
            title: value.title,
            content: value.content,
            author_id: value.author_id.to_string(),
            created_at: value.created_at.timestamp(),
            updated_at: value.updated_at.map(|e| e.timestamp()),
        }
    }
}

impl From<crate::blog_grpc::CreatePostRequest> for CreatePost {
    fn from(value: crate::blog_grpc::CreatePostRequest) -> Self {
        Self {
            title: value.title,
            content: value.content,
        }
    }
}
