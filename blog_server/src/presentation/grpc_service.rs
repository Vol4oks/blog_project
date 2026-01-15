use crate::blog_grpc::blog_service_server::BlogService;
use crate::blog_grpc::{
    self, AuthResponse, CreatePostRequest, DeletePostRequest, DeletePostResponse, GetPostRequest,
    ListPostsRequest, ListPostsResponse, LoginRequest, PostResponse, RegisterRequest,
    UpdatePostRequest,
};
use crate::data::posr_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::auth::Auth;
use crate::domain::post::{Post, UpdatePost};
use crate::presentation::auth::extract_user_from_token;

use tonic::metadata::MetadataMap;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct BlogGrpcService {
    auth_service: crate::application::AuthService<PostgresUserRepository>,
    blog_service: crate::application::BlogService<PostgresPostRepository>,
}

impl BlogGrpcService {
    pub fn new(
        auth_service: crate::application::AuthService<PostgresUserRepository>,
        blog_service: crate::application::BlogService<PostgresPostRepository>,
    ) -> Self {
        Self {
            auth_service,
            blog_service,
        }
    }
}

#[tonic::async_trait]
impl BlogService for BlogGrpcService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let user = request.into_inner();
        let acc = self
            .auth_service
            .register(Auth {
                username: user.username.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
            })
            .await?;

        tracing::info!(user_id = %acc.id, username = %acc.username, email = %acc.email, "user registered");

        let acc = self
            .auth_service
            .login_by_username(&user.username, &user.password)
            .await?;

        Ok(Response::new(AuthResponse { token: acc.token }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let user = request.into_inner();
        let acc = self
            .auth_service
            .login_by_username(&user.username, &user.password)
            .await?;

        Ok(Response::new(AuthResponse { token: acc.token }))
    }

    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let token = extract_token_from_metadata(request.metadata())?;
        let user_id = extract_user_from_token(token, &self.auth_service).await?;
        let post = request.into_inner();
        let post = self
            .blog_service
            .create_post(post.into(), user_id.id)
            .await?;
        return Ok(Response::new(PostResponse {
            post: Some(post.into()),
        }));
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let id = request.into_inner().id;
        let post = self.blog_service.get_post_by_id(id).await?;
        let post = <Post as std::convert::Into<blog_grpc::Post>>::into(post);
        Ok(Response::new(PostResponse { post: Some(post) }))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<PostResponse>, Status> {
        let token = extract_token_from_metadata(request.metadata())?;
        let user_id = extract_user_from_token(token, &self.auth_service).await?;
        let update_post = request.into_inner();
        let post = self.blog_service.get_post_by_id(update_post.id).await?;
        if post.author_id != user_id.id {
            return Err(Status::permission_denied(
                "You are not the author of this post",
            ));
        }

        let check_update = UpdatePost {
            title: update_post.title.unwrap_or(post.title.clone()),
            content: update_post.content.unwrap_or(post.content.clone()),
        };

        let post = self.blog_service.update_post(&post, check_update).await?;
        return Ok(Response::new(PostResponse {
            post: Some(post.into()),
        }));
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let token = extract_token_from_metadata(request.metadata())?;
        let user_id = extract_user_from_token(token, &self.auth_service).await?;
        let delete_post = request.into_inner();
        let post = self
            .blog_service
            .get_post_by_id(delete_post.post_id)
            .await?;
        if post.author_id != user_id.id {
            return Err(Status::permission_denied(
                "You are not the author of this post",
            ));
        }

        self.blog_service.delete_post(post.id).await?;
        return Ok(Response::new(DeletePostResponse { success: true }));
    }

    async fn list_post(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let request = request.into_inner();

        let posts = self
            .blog_service
            .get_next_posts(request.offset.into(), request.limit.into())
            .await?;
        let total = posts.len() as i32;
        Ok(Response::new(ListPostsResponse {
            post: posts.into_iter().map(|p| p.into()).collect(),
            total,
            limit: request.limit,
            offset: request.offset,
        }))
    }
}

fn extract_token_from_metadata(metadata: &MetadataMap) -> Result<&str, Status> {
    if let Some(token) = metadata.get("authorization") {
        let token_str = token
            .to_str()
            .map_err(|_| Status::invalid_argument("Invalid token"))?;
        let clear_token_str = token_str
            .strip_prefix("Bearer ")
            .ok_or_else(|| Status::invalid_argument("invalid authorization header"))?;
        Ok(clear_token_str)
    } else {
        Err(Status::unauthenticated("Token not found"))
    }
}
