use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Grpc error: {0}")]
    GrpcError(#[from] tonic::transport::Error),
    #[error("Grpc server error: {0}")]
    GrpcServerError(#[from] tonic::Status),

    #[error("Http error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Invalid Metadata Value: {0}")]
    InvalidMetadataValue(#[from] tonic::metadata::errors::InvalidMetadataValue),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,
}
