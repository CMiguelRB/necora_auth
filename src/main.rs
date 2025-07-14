mod auth;
mod routes;

use std::{env, net::SocketAddr, process};
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
    
    let routes = routes();

    let mut port = env::var("PORT").unwrap_or_default();
    if port == "" {
        port = "3810".to_string();
    }

    let mut hostname = env::var("HOSTNAME").unwrap_or_default();
    if hostname == "" {
        hostname = "127.0.0.1".to_string();
    }

    let addr: String = [hostname,port].join(":");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Auth server running at http://{}", &addr);
    axum::serve(listener, routes.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}