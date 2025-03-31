use crate::config::ServerConfig;
use crate::entities::{Prefixes, RoamOutDB};
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;

const BATCH_STATUS_START: &str = "Started";

pub struct StoreManager {
    pub pool: Pool,
}

impl StoreManager {
    pub fn new(config: ServerConfig) -> Result<Self, AppError> {
        let mut pg_config = Config::new();
        pg_config.dbname = Some(config.dbname.unwrap_or_default());
        pg_config.user = Some(config.user.unwrap_or_default());
        pg_config.password = Some(config.password.unwrap_or_default());
        pg_config.host = Some(config.host.unwrap_or_default());

        let pool = pg_config
            .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
            .map_err(|e| AppError::CreatePoolError(e))?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        let client = self.pool.get().await.map_err(|e| AppError::PoolError(e))?;
        Ok(client)
    }
}

pub async fn insert_batch_exec(store_mgr: &StoreManager, path_name: &str) -> Result<i32, AppError> {
    let client = store_mgr.get_client().await?;
    let query = "INSERT INTO batch_execs (batch_name, start_time, batch_status) VALUES ($1, NOW(), $2) RETURNING id";
    let row = client
        .query_one(query, &[&path_name, &BATCH_STATUS_START])
        .await?;
    let id: i32 = row.try_get("id")?;
    Ok(id)
}

pub async fn update_batch_execs(
    store_mgr: &StoreManager,
    batch_id: i32,
    status: &str,
) -> Result<u64, AppError> {
    let client = store_mgr.get_client().await?;
    let query = "UPDATE batch_execs SET batch_status = $1, end_time = NOW() WHERE id = $2";
    let rows_affected = client.execute(query, &[&status, &batch_id]).await?;
    Ok(rows_affected as u64)
}

pub async fn insert_roam_out_stg(
    store_mgr: &StoreManager,
    db_records: Vec<RoamOutDB>,
) -> Result<(), AppError> {
    let client = store_mgr.get_client().await?;
    let query = "
        INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number, carrier_name, country_name)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ";

    for record in db_records {
        client
            .execute(
                query,
                &[
                    &record.batch_id,
                    &record.batch_date,
                    &record.imsi,
                    &record.msisdn,
                    &record.vlr_number,
                    &record.carrier_name,
                    &record.country_name,
                ],
            )
            .await
            .map_err(AppError::DatabaseError)?;
    }

    Ok(())
}

pub async fn select_all_prefixes(store_mgr: &StoreManager) -> Result<Vec<Prefixes>, AppError> {
    let client = store_mgr.get_client().await?;
    let query = "SELECT p.prefix, p.carrier_name, COALESCE(c.country_name, '') AS country_name
                    FROM prefixes p
                    LEFT JOIN countries c ON p.country_alpha2 = c.country_alpha2;";

    let rows = client.query(query, &[]).await.map_err(AppError::from)?;

    let prefixes = rows
        .into_iter()
        .map(|row| Prefixes {
            prefix: row.get(0),
            carrier_name: row.get::<_, Option<String>>(1).unwrap_or_default(), // Replace null with ""
            country_name: row.get::<_, Option<String>>(2).unwrap_or_default(), // Replace null with ""
        })
        .collect();

    Ok(prefixes)
}
