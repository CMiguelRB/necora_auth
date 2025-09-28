mod serv;
mod sec;
mod conf;
mod db;

use std::{env, net::SocketAddr, process};
use dotenvy::dotenv;
use conf::config;
use serv::server;
use db::database;

#[tokio::main]
async fn main() {
    println!("Starting necora auth service...");
    println!("Loading environment variables...");
    dotenv().ok();
    match env::var("DB_USERNAME") {
        Ok(_val) => println!("Enviroment variables loaded OK!"),
        Err(e) => {
            println!("Error {}", e);
            process::exit(1)
        }
    }
    println!("Loading configuration...");
    let config = config::settings();
    println!("Configuration loaded OK!");

    println!("Connecting to the database...");
    database::init_db_pool().await;
    println!("Database connection OK!");
    
    let routes = server::routes();

    let port = &config.server.port;

    let hostname = &config.server.hostname;

    let addr: String = [hostname.to_string(),port.to_string()].join(":");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Auth server running at http://{}", &addr);
    println!("{} v{} is ready to rust!", &config.app.name, &config.app.version);
    axum::serve(listener, routes.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();

}