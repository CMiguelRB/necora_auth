use std::{sync::Arc, time::Duration};

use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
    http::StatusCode
};
use serde::Deserialize;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use crate::sec::security;


#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
}

#[derive(Deserialize)]
struct RefreshToken {
    refresh_token: String,
}

#[derive(Deserialize)]
struct BothTokens {
    access_token: String,
    refresh_token: String,
}

async fn login(Json(payload): Json<LoginInput>) -> impl IntoResponse {
    let username = payload.username;
    let password = payload.password;
    println!("Username: {}", username);
    println!("Password {}", password);
    (StatusCode::OK, "Invalid credentials").into_response()
}

async fn refresh(Json(payload): Json<RefreshToken>) -> impl IntoResponse {
    let refresh_token: String = payload.refresh_token;
    (StatusCode::OK, "Access token").into_response()
}

async fn authorize(Json(payload): Json<AccessToken>) -> impl IntoResponse {
    let access_token: String = payload.access_token;
    (StatusCode::OK, "Valid").into_response()
}

async fn revoke(Json(payload): Json<BothTokens>) -> impl IntoResponse {
    let access_token: String = payload.access_token;
    let refresh_token: String = payload.refresh_token;
    (StatusCode::OK, "Valid").into_response()
}

async fn logout(Json(payload): Json<BothTokens>) -> impl IntoResponse {
    let access_token: String = payload.access_token;
    let refresh_token: String = payload.refresh_token;
    (StatusCode::OK, "Valid").into_response()
}

pub fn routes() -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(5)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        governor_limiter.retain_recent();
    });

    let layer = GovernorLayer {
        config: governor_conf,
    };

    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/authorize", post(authorize))
        .route("/revoke", post(revoke))
        .route("/logout", post(logout))
        .layer(layer)
}
