use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{post},
    Router
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

// Login: retorna JWT si usuario y pass son válidos (dummy login)
async fn login(Json(payload): Json<LoginInput>) -> impl IntoResponse {
    let username = payload.username;
    let password = payload.password;
    println!("Username: {}", username);
    println!("Password {}", password);
    (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()    
}

// Define rutas
pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
}