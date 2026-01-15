pub mod blog_grpc {
    tonic::include_proto!("blog");
}

mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use actix_web::{
    App, HttpServer,
    middleware::{DefaultHeaders, Logger},
    web,
};

use crate::{
    application::{AuthService, BlogService},
    data::{posr_repository::PostgresPostRepository, user_repository::PostgresUserRepository},
    infrastructure::{AppConfig, JwtService, create_pool, init_logging, run_migrations},
    presentation::{
        JwtAuthMiddleware, RequestIdMiddleware, TimingMiddleware, grpc_service, handlers,
    },
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_logging();
    tracing::info!("Starting blog server");

    let config = AppConfig::from_env().expect("invalid configuration");

    let pool = create_pool(&config.database_url)
        .await
        .expect("Failed to create pool");
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let auth_service = AuthService::new(
        Arc::clone(&user_repo),
        JwtService::new(config.jwt_secret.clone(), config.jwt_expiration),
    );

    let blog_repo = Arc::new(PostgresPostRepository::new(pool.clone()));
    let blog_service = BlogService::new(Arc::clone(&blog_repo));

    let service_grpc =
        grpc_service::BlogGrpcService::new(auth_service.clone(), blog_service.clone());

    let config_data = config.clone();

    let http_server = HttpServer::new(move || {
        let cors = build_cors(&config_data);
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdMiddleware)
            .wrap(TimingMiddleware)
            .wrap(
                DefaultHeaders::new()
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Referrer-Policy", "no-referrer"))
                    .add(("Permissions-Policy", "geolocation=()"))
                    .add(("Cross-Origin-Opener-Policy", "same-origin")),
            )
            .wrap(cors)
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(web::scope("/api").service(handlers::public::scope()))
            .service(
                web::scope("/protect")
                    .wrap(JwtAuthMiddleware::new())
                    .service(handlers::protect::scope()),
            )
    })
    .bind(config.http_addr)?
    .run();

    let grpc_server = tonic::transport::Server::builder()
        .add_service(blog_grpc::blog_service_server::BlogServiceServer::new(
            service_grpc,
        ))
        .serve(config.grpc_addr.parse()?);

    tokio::select! {
        grpc_result = grpc_server => {
            tracing::info!("gRPC server stopped");
             if let Err(e) = grpc_result {
                return Err(e.into());
            }
        }

        http_result = http_server=> {
            tracing::info!("HTTP server stopped");
            if let Err(e) = http_result {
                return Err(e.into());
            }

        }

    }

    Ok(())
}

fn build_cors(config: &AppConfig) -> actix_cors::Cors {
    let mut cors = actix_cors::Cors::default()
        .allowed_origin("http://127.0.0.1:8080") // Для фронта
        .allowed_origin("http://localhost:8080")
        // .allowed_origin("http://127.0.0.1:8081")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::ACCEPT,
        ])
        // .supports_credentials()
        .max_age(3600);

    if config.cors_origins.iter().any(|origin| origin == "*") {
        cors = cors.allow_any_origin();
    } else {
        cors = cors.supports_credentials();
        for origin in &config.cors_origins {
            cors = cors.allowed_origin(origin);
        }
    }

    cors
}
