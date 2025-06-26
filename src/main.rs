mod auth;
mod routes;

use std::{env, process};
use dotenvy::dotenv;
use routes::routes;

#[tokio::main]
async fn main() {
    println!("Starting necora auth service...");
    println!("Loading environment variables...");
    dotenv().ok();
    match env::var("ENCRYPTION_KEY") {
        Ok(_val) => println!("Enviroment variables loaded OK!"),
        Err(e) => {
            println!("Error {}", e);
            process::exit(1)
        }
    }
    
    let app = routes();

    let mut port = env::var("PORT").unwrap_or_default();
    if port == "" {
        port = "3000".to_string();
    }

    let addr: String = ["127.0.0.1",&port].join(":");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Auth server running at http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}