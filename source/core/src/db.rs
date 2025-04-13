use crate::config::ServerConfig;
use crate::entities::Prefixes;
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize)]
pub struct LogRecord {
    pub batch_id: Option<i32>,
    pub batch_name: Option<String>,
    pub source_type: Option<String>,
    pub source_name: Option<String>,
    pub corr_id: Option<i32>,
    pub batch_status: Option<String>,
}

pub struct DBManager {
    pub pool: Pool,
}

const SELECT_ALL_PREFIXES_QUERY: &str =
    "SELECT prefix, country_id, operator_id FROM dim_prefixes WHERE prefix IS NOT NULL;";

impl DBManager {
    pub fn new(config: ServerConfig) -> Result<Self, AppError> {
        let mut pg_config = Config::new();
        pg_config.dbname = Some(config.dbname.ok_or(AppError::MissingConfig("DB_NAME"))?);
        pg_config.user = Some(config.user.ok_or(AppError::MissingConfig("DB_USER"))?);
        pg_config.password = Some(
            config
                .password
                .ok_or(AppError::MissingConfig("DB_PASSWORD"))?,
        );
        pg_config.host = Some(config.host.ok_or(AppError::MissingConfig("DB_HOST"))?);

        let pool = pg_config
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::CreatePoolError(e))?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        self.pool.get().await.map_err(AppError::PoolError)
    }

    pub async fn insert_batch(&self, record: &LogRecord) -> Result<i32, AppError> {
        let client = self.get_client().await?;

        let batch_name = record
            .batch_name
            .as_ref()
            .ok_or(AppError::MissingConfig("batch_name"))?;

        let batch_status = "Started";

        let mut query = String::from("INSERT INTO batch_execs (");
        let mut values = Vec::new();
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
        let mut fields = Vec::new();

        let mut param_index = 1;

        // batch_name
        fields.push("batch_name");
        values.push(format!("${}", param_index));
        params.push(batch_name);
        param_index += 1;

        // source_type
        if let Some(source_type) = &record.source_type {
            fields.push("source_type");
            values.push(format!("${}", param_index));
            params.push(source_type);
            param_index += 1;
        }

        // source_name
        if let Some(source_name) = &record.source_name {
            fields.push("source_name");
            values.push(format!("${}", param_index));
            params.push(source_name);
            param_index += 1;
        }

        // start_time
        fields.push("start_time");
        values.push("NOW()".to_string());

        // batch_status
        fields.push("batch_status");
        values.push(format!("${}", param_index));
        params.push(&batch_status);
        param_index += 1;

        // build query
        query.push_str(&fields.join(", "));
        query.push_str(") VALUES (");
        query.push_str(&values.join(", "));
        query.push_str(") RETURNING id;");

        client
            .query_one(&query, &params)
            .await
            .map(|row| row.get(0))
            .map_err(AppError::DatabaseError)
    }

    pub async fn update_batch(&self, record: &LogRecord) -> Result<(), AppError> {
        let client = self.get_client().await?;

        let batch_id = record
            .batch_id
            .ok_or(AppError::MissingConfig("batch_id is required for update"))?;

        let mut query = String::from("UPDATE batch_execs SET ");
        let mut updates: Vec<String> = Vec::new();
        let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();

        if let Some(batch_status) = &record.batch_status {
            updates.push(format!("batch_status = ${}", params.len() + 1));
            params.push(batch_status);
        }

        if let Some(corr_id) = &record.corr_id {
            updates.push(format!("corr_id = ${}", params.len() + 1));
            params.push(corr_id);
        }

        updates.push(format!("end_time = NOW()"));

        if updates.is_empty() {
            return Err(AppError::MissingConfig("No update fields provided"));
        }

        query.push_str(&updates.join(", "));
        query.push_str(&format!(" WHERE id = ${}", params.len() + 1));
        params.push(&batch_id);

        client
            .execute(&query, &params)
            .await
            .map(|_| ())
            .map_err(|e| AppError::DatabaseError(e))
    }

    pub async fn select_all_prefixes(&self) -> Result<Vec<Prefixes>, AppError> {
        let client = self.get_client().await?;
        let rows = client
            .query(SELECT_ALL_PREFIXES_QUERY, &[])
            .await
            .map_err(AppError::from)?;

        let prefixes = rows
            .into_iter()
            .map(|row| Prefixes {
                prefix: row.get(0),
                country_id: row.get(1),
                operator_id: row.get(2),
            })
            .collect();

        Ok(prefixes)
    }
}
