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
    println!("                                                                                                                                
                                                                                                                               88           
                                                                                                                        ,d     88           
                                                                                                                        88     88           
            8b,dPPYba,    ,adPPYba,   ,adPPYba,   ,adPPYba,   8b,dPPYba,  ,adPPYYba,         ,adPPYYba,  88       88  MM88MMM  88,dPPYba,   
            88P'   '\"8a  a8P_____88  a8\"     \"\"  a8\"     \"8a  88P'   \"Y8  \"\"     'Y8         \"\"     'Y8  88       88    88     88P'    \"8a  
            88       88  8PP\"\"\"\"\"\"\"  8b          8b       d8  88          ,adPPPPP88         ,adPPPPP88  88       88    88     88       88  
            88       88  \"8b,   ,aa  \"8a,   ,aa  \"8a,   ,a8\"  88          88,    ,88         88,    ,88  \"8a,   ,a88    88,    88       88  
            88       88   '\"Ybbd8\"'   '\"Ybbd8\"'   '\"YbbdP\"'   88          '\"8bbdP\"Y8         '\"8bbdP\"Y8   '\"YbbdP'Y8    \"Y888  88       88\n");

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