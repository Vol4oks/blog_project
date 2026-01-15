use async_trait::async_trait;

use crate::{
    BlogCommands,
    blog_grpc::{self, AuthResponse, DeletePostResponse, ListPostsResponse, Post, PostResponse},
    error::AppError,
};

pub struct HttpClient {
    addr: String,
    connection: reqwest::Client,
}

impl HttpClient {
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            connection: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl BlogCommands for HttpClient {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, AppError> {
        let request_path = format!("{}/api/auth/register", self.addr);
        let request_body = blog_grpc::RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        };
        let responce = self
            .connection
            .post(request_path)
            .json(&request_body)
            .send()
            .await?;
        if responce.status().is_success() {
            if let Ok(token) = responce.json::<AuthResponse>().await {
                return Ok(token);
            }
            Err(AppError::Unauthorized)
        } else {
            Err(AppError::Internal(format!(
                "Server responce: {}",
                responce.status()
            )))
        }
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, AppError> {
        let request_path = format!("{}/api/auth/login", self.addr);
        let request_body = blog_grpc::LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };
        let responce = self
            .connection
            .post(request_path)
            .json(&request_body)
            .send()
            .await?;
        if responce.status().is_success() {
            if let Ok(token) = responce.json::<AuthResponse>().await {
                return Ok(token);
            }
            Err(AppError::Unauthorized)
        } else {
            Err(AppError::Internal(format!(
                "Server responce: {}",
                responce.status()
            )))
        }
    }

    async fn create_post(
        &mut self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<PostResponse, AppError> {
        let request_path = format!("{}/protect/post", self.addr);
        let request_body = blog_grpc::CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        };

        let responce = self
            .connection
            .post(request_path)
            .header(reqwest::header::AUTHORIZATION, get_auth_header(token))
            .json(&request_body)
            .send()
            .await?;

        dbg!(&responce);
        let status = responce.status();
        if status.is_success()
            && let Ok(res) = responce.json::<Post>().await
        {
            return Ok(PostResponse { post: Some(res) });
        }

        Err(AppError::Internal(format!("Server responce: {}", status)))
    }

    async fn update_post(
        &mut self,
        token: &str,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<PostResponse, AppError> {
        let request_path = format!("{}/protect/post/{}", self.addr, post_id);
        let request_body = blog_grpc::UpdatePostRequest {
            id: post_id,
            title,
            content,
        };

        let responce = self
            .connection
            .put(request_path)
            .header(reqwest::header::AUTHORIZATION, get_auth_header(token))
            .json(&request_body)
            .send()
            .await?;

        let status = responce.status();
        if status.is_success()
            && let Ok(res) = responce.json::<Post>().await
        {
            return Ok(PostResponse { post: Some(res) });
        }

        Err(AppError::Internal(format!("Server responce: {}", status)))
    }

    async fn delete_post(
        &mut self,
        token: &str,
        post_id: i64,
    ) -> Result<DeletePostResponse, AppError> {
        let request_path = format!("{}/protect/post/{}", self.addr, post_id);

        let responce = self
            .connection
            .delete(request_path)
            .header(reqwest::header::AUTHORIZATION, get_auth_header(token))
            .send()
            .await?;
        let status = responce.status();
        if status.is_success()
            && let Ok(res) = responce.json::<DeletePostResponse>().await
        {
            return Ok(res);
        }
        Err(AppError::Internal(format!("Server responce: {}", status)))
    }

    async fn get_post(&mut self, post_id: i64) -> Result<PostResponse, AppError> {
        let request_path = format!("{}/api/posts/{}", self.addr, post_id);
        let responce = self.connection.get(request_path).send().await?;
        if responce.status().is_success() {
            return responce
                .json::<PostResponse>()
                .await
                .map_err(|e| AppError::Internal(e.to_string()));
        }

        Err(AppError::NotFound)
    }
    async fn list_posts(
        &mut self,
        limmit: i32,
        offset: i32,
    ) -> Result<ListPostsResponse, AppError> {
        let request_path = format!("{}/api/posts?limit={}&offset={}", self.addr, limmit, offset);
        let responce = self.connection.get(request_path).send().await?;
        if responce.status().is_success() {
            return responce
                .json::<ListPostsResponse>()
                .await
                .map_err(|e| AppError::Internal(e.to_string()));
        }

        Err(AppError::NotFound)
    }
}

fn get_auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}
