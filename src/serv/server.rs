use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use crate::conf::config;
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
    let config = config::settings();
    // Allow bursts with up to five requests per IP address
    // and replenishes one element every two seconds
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(config.server.rate_limit)
        .burst_size(config.server.burst_size)
        .finish()
        .unwrap();

    let governor_limiter = governor_conf.limiter().clone();
    let interval = config.server.period;
    // a separate background task to clean up
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        governor_limiter.retain_recent();
    });

    return Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/authorize", post(authorize))
        .route("/revoke", post(revoke))
        .route("/logout", post(logout))
        .layer(GovernorLayer::new(governor_conf));
}
