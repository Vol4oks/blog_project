use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::post::{CreatePost, Post, UpdatePost};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(
        &self,
        post: CreatePost,
        author_id: Uuid,
    ) -> Result<Post, PostRepositoryError>;
    async fn get_post(&self, post_id: i64) -> Result<Post, PostRepositoryError>;

    /// len: i32 количество постов
    /// offset: Option<i32> отступ от самого последнего поста
    async fn get_last_posts(
        &self,
        len: i64,
        offset: Option<i64>,
    ) -> Result<Vec<Post>, PostRepositoryError>;
    #[allow(dead_code)]
    async fn get_posts_by_author(&self, user_id: Uuid) -> Result<Vec<Post>, PostRepositoryError>;
    async fn update_post(
        &self,
        post_id: i64,
        post: UpdatePost,
    ) -> Result<Post, PostRepositoryError>;
    async fn delete_post(&self, post_id: i64) -> Result<(), PostRepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum PostRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Post not found")]
    NotFound,

    #[allow(dead_code)]
    #[error("Autor not found")]
    AutorNotFound,
}

#[derive(Clone)]
pub struct PostgresPostRepository {
    pool: sqlx::PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create_post(
        &self,
        post: CreatePost,
        author_id: Uuid,
    ) -> Result<Post, PostRepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let new_post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id) 
            VALUES ($1, $2, $3) 
            RETURNING 
            id, 
            title, 
            content as "content!: String", 
            author_id as "author_id!: Uuid", 
            created_at as "created_at!: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            "#,
            post.title,
            post.content,
            author_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;
        Ok(new_post)
    }

    async fn get_post(&self, post_id: i64) -> Result<Post, PostRepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let new_post = sqlx::query_as!(
            Post,
            r#"
            SELECT 
            id,
            title,
            content as "content!: String",
            author_id as "author_id!: Uuid",
            created_at as "created_at!: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            FROM posts WHERE id = $1
            "#,
            post_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(post) = new_post {
            return Ok(post);
        }
        Err(PostRepositoryError::NotFound)
    }

    async fn get_last_posts(
        &self,
        len: i64,
        offset: Option<i64>,
    ) -> Result<Vec<Post>, PostRepositoryError> {
        let offset = offset.unwrap_or(0);
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT 
            id,
            title,
            content as "content!: String",
            author_id as "author_id!: Uuid",
            created_at as "created_at!: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            FROM posts 
            ORDER BY created_at DESC 
            LIMIT $1 
            OFFSET $2
            "#,
            len,
            offset
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        Ok(posts)
    }

    async fn get_posts_by_author(&self, user_id: Uuid) -> Result<Vec<Post>, PostRepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT 
            id,
            title,
            content as "content!: String",
            author_id as "author_id!: Uuid",
            created_at as "created_at!: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            FROM posts 
            WHERE author_id = $1
            "#,
            user_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        Ok(posts)
    }

    async fn update_post(
        &self,
        post_id: i64,
        post: UpdatePost,
    ) -> Result<Post, PostRepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let new_post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts 
            SET 
                title = COALESCE($1, title),
                content = COALESCE($2, content),
                updated_at = NOW()
            WHERE id = $3 
            RETURNING 
            id, 
            title, 
            content as "content!: String", 
            author_id as "author_id!: Uuid", 
            created_at as "created_at!: chrono::DateTime<chrono::Utc>", 
            updated_at as "updated_at?: chrono::DateTime<chrono::Utc>"
            "#,
            post.title,
            post.content,
            post_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;
        Ok(new_post)
    }

    async fn delete_post(&self, post_id: i64) -> Result<(), PostRepositoryError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        let res = sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&mut *conn)
            .await
            .map_err(|e| PostRepositoryError::DatabaseError(e.to_string()))?;

        if res.rows_affected() == 0 {
            return Err(PostRepositoryError::NotFound);
        }
        Ok(())
    }
}
