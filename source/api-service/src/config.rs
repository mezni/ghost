use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub fn get_db_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let mut cfg = Config::new();
    cfg.dbname = Some(env::var("PG_DB")?);
    cfg.user = Some(env::var("PG_USER")?);
    cfg.password = Some(env::var("PG_PASSWORD")?);
    cfg.host = Some(env::var("PG_HOST")?);

    Ok(cfg.create_pool(None, NoTls)?)
}
