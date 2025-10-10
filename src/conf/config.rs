use once_cell::sync::OnceCell;
use core::time;
use std::{env, fs, path::PathBuf, time::Duration};

#[derive()]
pub struct Config {
    pub app: App,
    pub server: Server,
    pub database: Database,
    pub security: Security,
}

#[derive()]
pub struct App {
    pub name: String,
    pub version: String,
}

#[derive()]
pub struct Server {
    pub hostname: String,
    pub port: String,
    pub rate_limit: u64,
    pub burst_size: u32,
    pub period: time::Duration
}

#[derive()]
pub struct Database {
    pub hostname: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub name: String,
}
pub struct Security {
    pub encryption_key: String,
}

static SETTINGS: OnceCell<Config> = OnceCell::new();

pub fn settings() -> &'static Config {
    SETTINGS.get_or_init(|| {
        // App
        let app = App {
            name: "necora_auth".to_string(),
            version: "0.0.3".to_string(),
        };

        // Server
        let server = Server {
            hostname: env::var("HOSTNAME").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "3810".to_string()),
            rate_limit: 2,
            burst_size: 5,
            period: Duration::from_secs(60)
        };

        // Database (from env)
        let mut database = Database {
            hostname: env::var("DB_HOSTNAME").unwrap_or_default(),
            port: env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
            username: env::var("DB_USERNAME").unwrap_or_default(),
            password: env::var("DB_PASSWORD").unwrap_or_default(),
            name: env::var("DB_NAME").unwrap_or_default(),
        };

        // Security (from env)
        let mut security = Security {
            encryption_key: env::var("ENCRYPTION_KEY").unwrap_or_default(),
        };

        // Override for QA/PROD with secrets
        let env_mode = env::var("ENV").unwrap_or_default();
        if env_mode == "QA" || env_mode == "PROD" {
            database.password =
                load_secret("db_password").expect("DB Password secret not found");
            security.encryption_key =
                load_secret("necora_encryption_key").expect("Encryption key secret not found");
        }

        Config {
            app,
            server,
            database,
            security,
        }
    })
}

fn load_secret(name: &str) -> std::io::Result<String> {
    let mut path = PathBuf::from("/run/secrets");
    path.push(name);

    let data = fs::read_to_string(path)?;
    Ok(data.trim().to_string())
}