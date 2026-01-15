use crate::{
    dto::{
        AuthResponse, CreatePostRequest, ErrorResponse, LoginRequest, Post, PostPage, PostResponse,
        RegisterRequest, UpdatePostRequest,
    },
    API_PATH,
};

use reqwest::Client;

pub async fn get_list_posts(limit: i32, offset: i32) -> Result<PostPage, String> {
    let request_path = format!("{}/api/posts?limit={}&offset={}", API_PATH, limit, offset);

    let response = Client::new()
        .get(&request_path)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        return response.json::<PostPage>().await.map_err(|e| e.to_string());
    }

    Err(response.status().to_string())
}

pub async fn get_post(post_id: i64) -> Result<Post, String> {
    let request_path = format!("{}/api/posts/{}", API_PATH, post_id);
    let response = Client::new()
        .get(request_path)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();

    if status.is_success() {
        let res = response
            .json::<PostResponse>()
            .await
            .map_err(|e| format!("Error parse: {}", e))?;
        if let Some(post) = res.post {
            return Ok(post);
        }
    }

    Err(status.to_string())
}

pub async fn register_user(
    username: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, String> {
    let request_path = format!("{}/api/auth/register", API_PATH);
    let request_body = RegisterRequest {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = Client::new()
        .post(request_path)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Error request: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let err = response
            .json::<ErrorResponse>()
            .await
            .map_err(|e| format!("Error parse ErrorResponse: {}", e))?;
        return Err(err.details.resource);
    }

    response
        .json::<AuthResponse>()
        .await
        .map_err(|e| format!("Error parse: {}", e))
}

pub async fn login_user(username: &str, password: &str) -> Result<AuthResponse, String> {
    let request_path = format!("{}/api/auth/login", API_PATH);
    let request_body = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };

    let response = Client::new()
        .post(request_path)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Error request: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let err = response
            .json::<ErrorResponse>()
            .await
            .map_err(|e| format!("Error parse ErrorResponse: {}", e))?;
        return Err(err.details.resource);
    }

    response
        .json::<AuthResponse>()
        .await
        .map_err(|e| format!("Error parse: {}", e))
}

pub async fn create_post(title: &str, content: &str, token: &str) -> Result<Post, String> {
    let request_path = format!("{}/protect/post", API_PATH);
    let request_body = CreatePostRequest {
        title: title.to_string(),
        content: content.to_string(),
    };

    let response = Client::new()
        .post(request_path)
        .header(reqwest::header::AUTHORIZATION, token)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Error request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        return response
            .json::<Post>()
            .await
            .map_err(|e| format!("Error parse: {}", e));
    }

    Err(status.to_string())
}

pub async fn update_post(id: i64, title: &str, content: &str, token: &str) -> Result<Post, String> {
    let request_path = format!("{}/protect/post/{}", API_PATH, id);
    let request_body = UpdatePostRequest {
        id,
        title: title.to_string(),
        content: content.to_string(),
    };

    let response = Client::new()
        .put(request_path)
        .header(reqwest::header::AUTHORIZATION, token)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Error request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        return response
            .json::<Post>()
            .await
            .map_err(|e| format!("Error parse: {}", e));
    }

    Err(status.to_string())
}

pub async fn delete_post(id: i64, token: &str) -> Result<(), String> {
    let request_path = format!("{}/protect/post/{}", API_PATH, id);

    let response = Client::new()
        .delete(request_path)
        .header(reqwest::header::AUTHORIZATION, token)
        .send()
        .await
        .map_err(|e| format!("Error request: {}", e))?;

    let status = response.status();

    if status.is_success() {
        return Ok(());
    }

    Err(status.to_string())
}
