// db.rs
use tokio_postgres::{NoTls, Client, Config};
use tokio_postgres::error::Error;

pub struct PgConn {
    client: Client,
}

impl PgConn {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let (client, connection) = config.connect(NoTls).await?;
        tokio::spawn(connection);
        Ok(PgConn { client })
    }

    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<tokio_postgres::RowStream, Error> {
        self.client.query(query, params).await
    }

    pub async fn execute(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        self.client.execute(query, params).await
    }

    pub async fn query_one(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<tokio_postgres::Row, Error> {
        self.client.query_one(query, params).await
    }
}

pub fn get_config() -> Config {
    let mut config = Config::new();
    config.host("localhost");
    config.port(5432);
    config.user("username");
    config.password("password");
    config.dbname("database");
    config
}

pub async fn establish_connection() -> Result<PgConn, Error> {
    let config = get_config();
    PgConn::new(config).await
}