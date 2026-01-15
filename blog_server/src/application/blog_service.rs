use std::sync::Arc;

use uuid::Uuid;

use crate::{
    data::posr_repository::{PostRepository, PostRepositoryError},
    domain::{
        error::BlogError,
        post::{CreatePost, Post, UpdatePost},
    },
};

#[derive(Clone)]
pub struct BlogService<R: PostRepository> {
    data: Arc<R>,
}

impl<R: PostRepository> BlogService<R> {
    pub fn new(data: Arc<R>) -> Self {
        Self { data }
    }

    pub async fn get_next_posts(&self, offset: i64, count: i64) -> Result<Vec<Post>, BlogError> {
        match self.data.get_last_posts(count, Some(offset)).await {
            Ok(p) => Ok(p),
            Err(e) => Err(BlogError::Internal(e.to_string())),
        }
    }
    #[allow(dead_code)]
    pub async fn get_posts_by_user(&self, user_id: Uuid) -> Result<Vec<Post>, BlogError> {
        match self.data.get_posts_by_author(user_id).await {
            Ok(p) => Ok(p),
            Err(PostRepositoryError::NotFound) => {
                Err(BlogError::NotFound("Posts not found".to_string()))
            }
            Err(PostRepositoryError::AutorNotFound) => {
                Err(BlogError::NotFound(format!("Not found Uuid: {}", user_id)))
            }
            Err(e) => Err(BlogError::Internal(e.to_string())),
        }
    }

    pub async fn get_post_by_id(&self, id: i64) -> Result<Post, BlogError> {
        match self.data.get_post(id).await {
            Ok(p) => Ok(p),
            Err(PostRepositoryError::NotFound) => {
                Err(BlogError::NotFound("Posts not found".to_string()))
            }
            Err(e) => Err(BlogError::Internal(e.to_string())),
        }
    }

    pub async fn create_post(&self, post: CreatePost, author_id: Uuid) -> Result<Post, BlogError> {
        match self.data.create_post(post, author_id).await {
            Ok(p) => Ok(p),
            Err(PostRepositoryError::AutorNotFound) => Err(BlogError::NotFound(format!(
                "Not found Uuid: {}",
                author_id
            ))),
            Err(e) => Err(BlogError::Internal(e.to_string())),
        }
    }

    pub async fn update_post(&self, post: &Post, update: UpdatePost) -> Result<Post, BlogError> {
        match self.data.update_post(post.id, update).await {
            Ok(p) => Ok(p),
            Err(PostRepositoryError::NotFound) => {
                Err(BlogError::NotFound("Posts not found".to_string()))
            }
            Err(e) => Err(BlogError::Internal(e.to_string())),
        }
    }

    pub async fn delete_post(&self, id: i64) -> Result<(), BlogError> {
        self.data
            .delete_post(id)
            .await
            .map_err(|e| BlogError::Internal(e.to_string()))
    }
}
