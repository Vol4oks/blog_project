use crate::{
    BlogCommands,
    blog_grpc::{
        self, AuthResponse, DeletePostResponse, ListPostsResponse, PostResponse,
        blog_service_client::BlogServiceClient,
    },
    error::AppError,
};
use async_trait::async_trait;
use tonic::Request;

pub struct GrpcClient {
    connection: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub async fn new(url: &str) -> Result<Self, AppError> {
        let connection = BlogServiceClient::connect(url.to_string()).await?;
        Ok(Self { connection })
    }
}

#[async_trait]
impl BlogCommands for GrpcClient {
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, AppError> {
        let token = self
            .connection
            .register(blog_grpc::RegisterRequest {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            })
            .await?;

        Ok(token.into_inner())
    }

    async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse, AppError> {
        let token = self
            .connection
            .login(blog_grpc::LoginRequest {
                username: username.to_string(),
                password: password.to_string(),
            })
            .await?;

        Ok(token.into_inner())
    }
    async fn create_post(
        &mut self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<PostResponse, AppError> {
        let mut request = Request::new(blog_grpc::CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        });

        request
            .metadata_mut()
            .insert("authorization", format!("Bearer {}", token).parse()?);

        let post = self.connection.create_post(request).await?;

        Ok(post.into_inner())
    }

    async fn update_post(
        &mut self,
        token: &str,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<PostResponse, AppError> {
        let mut request = Request::new(blog_grpc::UpdatePostRequest {
            id: post_id,
            title,
            content,
        });

        request
            .metadata_mut()
            .insert("authorization", format!("Bearer {}", token).parse()?);

        let post = self.connection.update_post(request).await?;

        Ok(post.into_inner())
    }

    async fn delete_post(
        &mut self,
        token: &str,
        post_id: i64,
    ) -> Result<DeletePostResponse, AppError> {
        let mut request = Request::new(blog_grpc::DeletePostRequest { post_id });

        request
            .metadata_mut()
            .insert("authorization", format!("Bearer {}", token).parse()?);

        let res = self.connection.delete_post(request).await?;

        Ok(res.into_inner())
    }

    async fn get_post(&mut self, post_id: i64) -> Result<PostResponse, AppError> {
        let post = self
            .connection
            .get_post(blog_grpc::GetPostRequest { id: post_id })
            .await?;

        Ok(post.into_inner())
    }

    async fn list_posts(&mut self, limit: i32, offset: i32) -> Result<ListPostsResponse, AppError> {
        let list_posts = self
            .connection
            .list_post(blog_grpc::ListPostsRequest { offset, limit })
            .await?;

        Ok(list_posts.into_inner())
    }
}
