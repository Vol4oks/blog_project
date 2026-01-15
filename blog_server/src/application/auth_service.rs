use std::sync::Arc;

use crate::{
    data::user_repository::UserRepository,
    domain::{
        auth::{Auth, AuthAnswer, User},
        error::BlogError,
    },
    infrastructure::{JwtService, password_hash, password_verify},
};

#[derive(Clone)]
pub struct AuthService<R: UserRepository> {
    repo: Arc<R>,
    keys: JwtService,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(repo: Arc<R>, keys: JwtService) -> Self {
        Self { repo, keys }
    }

    pub fn keys(&self) -> &JwtService {
        &self.keys
    }

    pub async fn get_user(&self, id: uuid::Uuid) -> Result<User, BlogError> {
        self.repo.get_user_by_id(id).await.map_err(BlogError::from)
    }

    pub async fn login_by_username(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthAnswer, BlogError> {
        let user = self.repo.get_user_by_username(username).await?;

        let valid =
            password_verify(password, &user.password_hash).map_err(|_| BlogError::Unautorized)?;

        if !valid {
            return Err(BlogError::Unautorized);
        }

        let token = self
            .keys
            .generate_token(user.id)
            .map_err(|err| BlogError::Internal(err.to_string()))?;

        Ok(AuthAnswer {
            token,
            uuid: user.id,
        })
    }

    pub async fn register(&self, user: Auth) -> Result<User, BlogError> {
        let hash =
            password_hash(&user.password).map_err(|err| BlogError::Internal(err.to_string()))?;
        let user = User::new(user.username, user.email, hash);
        self.repo.create_user(user).await.map_err(BlogError::from)
    }
}
