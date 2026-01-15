use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,

    #[error("Validation error: {0}")]
    Validation(String),

    // #[error("Invalid credentials")]
    // InvalidCredentials,
    #[error("Post not found")]
    PostNotFound,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Forbidden")]
    Forbidden,

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unautorized,

    #[error("Internal: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for BlogError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BlogError::Forbidden => StatusCode::FORBIDDEN,
            BlogError::NotFound(_) => StatusCode::NOT_FOUND,
            BlogError::Unautorized => StatusCode::UNAUTHORIZED,
            BlogError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BlogError::Validation(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let message = self.to_string();
        tracing::info!(message = message);

        let details = match self {
            BlogError::Validation(msg) => Some(json!({"resource": msg })),
            BlogError::Forbidden => None,
            BlogError::NotFound(res) => Some(json!({"resource": res})),
            BlogError::Unautorized => None,
            BlogError::Internal(_) => None,
        };

        let body = ErrorBody {
            error: &message,
            details,
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<DomainError> for BlogError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::UserNotFound => BlogError::NotFound("user not found".to_string()),
            DomainError::Validation(e) => BlogError::Validation(e),
            DomainError::PostNotFound => BlogError::NotFound("Post not found".to_string()),
            DomainError::Forbidden => BlogError::Forbidden,
            DomainError::Internal(e) => BlogError::Internal(e),
        }
    }
}

impl From<BlogError> for Status {
    fn from(value: BlogError) -> Self {
        match value {
            BlogError::Validation(e) => Status::invalid_argument(e),
            BlogError::Forbidden => Status::permission_denied("Forbiden"),
            BlogError::NotFound(e) => Status::not_found(e),
            BlogError::Unautorized => Status::unauthenticated("Unautorized"),
            BlogError::Internal(e) => Status::internal(e),
        }
    }
}
