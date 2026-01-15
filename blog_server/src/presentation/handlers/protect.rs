use actix_web::{HttpMessage, HttpRequest, HttpResponse, Scope, delete, post, put, web};

use uuid::Uuid;

use crate::{
    application::BlogService,
    data::posr_repository::PostgresPostRepository,
    domain::{
        error::BlogError,
        post::{CreatePost, UpdatePost},
    },
    presentation::auth::AuthenticatedUser,
};

pub fn scope() -> Scope {
    web::scope("")
        .service(create_post)
        .service(update_post)
        .service(delipe_post)
}

fn ensure_owner(autor: Uuid, user: &AuthenticatedUser) -> Result<(), BlogError> {
    if autor != user.id {
        Err(BlogError::Forbidden)
    } else {
        Ok(())
    }
}

#[post("/post")]
async fn create_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog_service: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<CreatePost>,
) -> Result<HttpResponse, BlogError> {
    let post = blog_service
        .create_post(payload.into_inner(), user.id)
        .await?;

    tracing::info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post.id,
        "Post created",
    );

    Ok(HttpResponse::Created().json(post))
}

#[put("/post/{id}")]
async fn update_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog_service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<i64>,
    payload: web::Json<UpdatePost>,
) -> Result<HttpResponse, BlogError> {
    let post_id = path.into_inner();
    let post = blog_service.get_post_by_id(post_id).await?;
    ensure_owner(post.author_id, &user)?;

    let update_post = blog_service
        .update_post(&post, payload.into_inner())
        .await?;

    tracing::info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post.id,
        "Post update",
    );

    Ok(HttpResponse::Ok().json(update_post))
}

#[delete("/post/{id}")]
async fn delipe_post(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog_service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<i64>,
) -> Result<HttpResponse, BlogError> {
    let post_id = path.into_inner();
    let post = blog_service.get_post_by_id(post_id).await?;
    ensure_owner(post.author_id, &user)?;

    blog_service.delete_post(post.id).await?;

    tracing::info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        post_id = %post.id,
        "Post delete",
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({"post": post_id, "delete": true})))
}

fn request_id(req: &HttpRequest) -> String {
    req.extensions()
        .get::<crate::presentation::RequestId>()
        .map(|rid| rid.0.clone())
        .unwrap_or_else(|| "unknown".into())
}
