use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use crate::conf::config;

pub async fn init_db_pool() -> Pool {
    let settings = config::settings();

    let mut cfg = Config::new();
    cfg.host = Some(settings.database.hostname.clone());
    cfg.port = Some(settings.database.port.parse().unwrap());
    cfg.user = Some(settings.database.username.clone());
    cfg.password = Some(settings.database.password.clone());
    cfg.dbname = Some(settings.database.name.clone());

    cfg.create_pool(None, NoTls).unwrap()
}