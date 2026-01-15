use async_trait::async_trait;

use crate::{
    blog_grpc::{AuthResponse, DeletePostResponse, ListPostsResponse, PostResponse},
    error::AppError,
    grpc_client::GrpcClient,
    http_client::HttpClient,
};

pub mod blog_grpc {
    tonic::include_proto!("blog");
}

mod error;
mod grpc_client;
mod http_client;

#[derive(Clone)]
pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClient {
    transport: Transport,
    http_client: Option<HttpClient>,
    grpc_client: Option<GrpcClient>,
    token: Option<String>,
}

#[async_trait]
trait BlogCommands {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, AppError>;
    async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, AppError>;
    async fn get_post(&mut self, post_id: i64) -> Result<PostResponse, AppError>;
    async fn update_post(
        &mut self,
        token: &str,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<PostResponse, AppError>;
    async fn delete_post(
        &mut self,
        token: &str,
        post_id: i64,
    ) -> Result<DeletePostResponse, AppError>;
    async fn create_post(
        &mut self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<PostResponse, AppError>;
    async fn list_posts(&mut self, limit: i32, offset: i32) -> Result<ListPostsResponse, AppError>;
}

impl BlogClient {
    pub async fn new(transport: Transport) -> Result<Self, AppError> {
        let m_transport = transport.clone();

        match m_transport {
            Transport::Http(url) => {
                let client = HttpClient::new(&url);
                Ok(Self {
                    transport,
                    http_client: Some(client),
                    grpc_client: None,
                    token: None,
                })
            }

            Transport::Grpc(url) => {
                let client = GrpcClient::new(&url).await?;

                Ok(Self {
                    transport,
                    http_client: None,
                    grpc_client: Some(client),
                    token: None,
                })
            }
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, AppError> {
        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.register(username, email, password).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.register(username, email, password).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, AppError> {
        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.login(username, password).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.login(username, password).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> Result<PostResponse, AppError> {
        if self.token.is_none() {
            return Err(AppError::Unauthorized);
        }
        let token = self.token.clone().unwrap();

        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.create_post(&token, title, content).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.create_post(&token, title, content).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn update_post(
        &mut self,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<PostResponse, AppError> {
        if self.token.is_none() {
            return Err(AppError::Unauthorized);
        }
        let token = self.token.clone().unwrap();

        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.update_post(&token, post_id, title, content).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.update_post(&token, post_id, title, content).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn delete_post(&mut self, post_id: i64) -> Result<DeletePostResponse, AppError> {
        if self.token.is_none() {
            return Err(AppError::Unauthorized);
        }
        let token = self.token.clone().unwrap();

        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.delete_post(&token, post_id).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.delete_post(&token, post_id).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn get_post(&mut self, post_id: i64) -> Result<PostResponse, AppError> {
        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.get_post(post_id).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.get_post(post_id).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }

    pub async fn list_posts(
        &mut self,
        limit: i32,
        offset: i32,
    ) -> Result<ListPostsResponse, AppError> {
        match self.transport {
            Transport::Http(_) => {
                if let Some(client) = &mut self.http_client {
                    return client.list_posts(limit, offset).await;
                }

                Err(AppError::Internal("Http client not set".to_string()))
            }
            Transport::Grpc(_) => {
                if let Some(client) = &mut self.grpc_client {
                    return client.list_posts(limit, offset).await;
                }

                Err(AppError::Internal("Grpc client not set".to_string()))
            }
        }
    }
}
