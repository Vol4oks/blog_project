use actix_web::{HttpResponse, Responder, Scope, get, post, web};

use crate::{
    application::{AuthService, BlogService},
    data::{posr_repository::PostgresPostRepository, user_repository::PostgresUserRepository},
    domain::{
        auth::{Auth, Login},
        error::BlogError,
    },
    presentation::dto::{self, AuthResponse},
};

pub fn scope() -> Scope {
    web::scope("")
        .service(healrh)
        .service(get_post)
        .service(get_post_by_id)
        .service(web::scope("/auth").service(register).service(login))
}

#[get("/health")]
async fn healrh() -> impl Responder {
    HttpResponse::Ok().json(dto::HealthResponse {
        status: "ok",
        timestamp: chrono::Utc::now(),
    })
}

#[post("/register")]
async fn register(
    auth_service: web::Data<AuthService<PostgresUserRepository>>,
    // blog_service: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<Auth>,
) -> Result<impl Responder, BlogError> {
    let payload = payload.into_inner();
    let acc = auth_service.register(payload.clone()).await?;

    tracing::info!(user_id = %acc.id, username = %acc.username, email = %acc.email, "user registered");
    let acc = auth_service
        .login_by_username(&payload.username, &payload.password)
        .await?;
    Ok(HttpResponse::Ok().json(AuthResponse {
        token: acc.token,
        uuid: acc.uuid,
    }))
}

#[post("/login")]
async fn login(
    auth_service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<Login>,
) -> Result<impl Responder, BlogError> {
    let acc = auth_service
        .login_by_username(&payload.username, &payload.password)
        .await?;

    tracing::info!(username = %payload.username, "user logged in");

    Ok(HttpResponse::Ok().json(AuthResponse {
        token: acc.token,
        uuid: acc.uuid,
    }))
}

#[get("/posts/{id}")]
async fn get_post_by_id(
    blog_service: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<i64>,
) -> Result<impl Responder, BlogError> {
    let post = blog_service.get_post_by_id(path.into_inner()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!(
        {
            "post": post,
        }
    )))
}

#[get("/posts")]
async fn get_post(
    blog_service: web::Data<BlogService<PostgresPostRepository>>,
    params: web::Query<dto::PaginationParams>,
) -> Result<impl Responder, BlogError> {
    let post = blog_service
        .get_next_posts(params.offset as i64, params.limit as i64)
        .await?;

    let total = post.len() as i32;

    Ok(
        HttpResponse::Ok().json(serde_json::json!(dto::ListPostsResponse {
            post,
            total,
            limit: params.limit,
            offset: params.offset
        })),
    )
}
