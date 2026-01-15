use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{auth::User, error::DomainError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, DomainError>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn get_user_by_username(&self, username: &str) -> Result<User, DomainError>;
    #[allow(dead_code)]
    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: sqlx::PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create_user(&self, new_user: User) -> Result<User, DomainError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        // Проверяем, существует ли пользователь с таким же именем или email
        let existing_user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1 OR email = $2",
            new_user.username,
            new_user.email,
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        // Если пользователь существует, возвращаем ошибку
        if let Some(ex_user) = existing_user {
            if ex_user.username == new_user.username {
                return Err(DomainError::Validation("Username exists".to_string()));
            }
            if ex_user.email == new_user.email {
                return Err(DomainError::Validation("email exists".to_string()));
            }
        }

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, email, password_hash) 
            VALUES ($1, $2, $3, $4) 
            RETURNING id, username, email, password_hash, created_at
            "#,
            new_user.id,
            new_user.username,
            new_user.email,
            new_user.password_hash
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<User, DomainError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 ", id)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        if let Some(user) = user {
            return Ok(user);
        }
        Err(DomainError::UserNotFound)
    }
    async fn get_user_by_username(&self, username: &str) -> Result<User, DomainError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1 ", username)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        if let Some(user) = user {
            return Ok(user);
        }
        Err(DomainError::UserNotFound)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 ", email)
            .fetch_optional(&mut *conn)
            .await
            .map_err(|e| DomainError::Internal(format!("database error: {}", e)))?;

        if let Some(user) = user {
            return Ok(user);
        }
        Err(DomainError::UserNotFound)
    }
}
