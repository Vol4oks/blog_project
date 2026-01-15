use actix_web::{Error, FromRequest, HttpMessage, error::ErrorUnauthorized};
use futures_util::future::{Ready, ready};
use uuid::Uuid;

use crate::{
    application::AuthService, data::user_repository::PostgresUserRepository,
    domain::error::BlogError,
};

#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<AuthenticatedUser>() {
            Some(user) => ready(Ok(user.clone())),
            None => ready(Err(ErrorUnauthorized("missing authenticated user"))),
        }
    }
}

pub async fn extract_user_from_token(
    token: &str,
    auth_service: &AuthService<PostgresUserRepository>,
) -> Result<AuthenticatedUser, BlogError> {
    let claims = auth_service
        .keys()
        .verify_token(token)
        .map_err(|_| BlogError::Unautorized)?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| BlogError::Unautorized)?;

    let user = auth_service
        .get_user(user_id)
        .await
        .map_err(|_| BlogError::Unautorized)?;

    Ok(AuthenticatedUser { id: user.id })
}
