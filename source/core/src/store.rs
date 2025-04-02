use crate::config::ServerConfig;
use crate::entities::{Prefixes, RoamOutDB};
use crate::errors::AppError;
use deadpool_postgres::{Client, Config, Pool};
use tokio_postgres::NoTls;
use tokio_postgres::types::{FromSql, ToSql};

// Constants for batch execution
const BATCH_STATUS_START: &str = "Started";

// SQL Query Constants
const INSERT_BATCH_EXEC_QUERY: &str =
    "INSERT INTO batch_execs (batch_name, source_type, source_name, start_time, batch_status) 
                                       VALUES ($1, $2, $3, NOW(), $4) RETURNING id";

const UPDATE_BATCH_EXEC_QUERY: &str =
    "UPDATE batch_execs SET end_time = NOW(), batch_status = $1 WHERE id = $2";

const SELECT_ALL_PREFIXES_QUERY: &str = "SELECT p.prefix, p.carrier_name, p.country_alpha2, 
                                         COALESCE(c.country_name, '') AS country_name
                                         FROM prefixes p
                                         LEFT JOIN countries c ON p.country_alpha2 = c.country_alpha2;";

const INSERT_STG_ROAM_OUT_QUERY: &str = "INSERT INTO stg_roam_out 
                                         (batch_id, batch_date, imsi, msisdn, vlr_number, carrier_name, country_name, country_alpha2) 
                                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

const INSERT_DIM_CARRIERS_QUERY: &str = "INSERT INTO dim_carriers (country_name, carrier_name, country_alpha2)
                                         SELECT country_name, carrier_name, country_alpha2
                                         FROM (
                                             SELECT DISTINCT country_name, carrier_name, country_alpha2
                                             FROM stg_roam_out
                                             EXCEPT
                                             SELECT country_name, carrier_name, country_alpha2
                                             FROM dim_carriers
                                         ) 
                                         ORDER BY country_name, carrier_name;";

const INSERT_DIM_IMSI_QUERY: &str = "INSERT INTO dim_imsi(imsi)
                                     SELECT imsi
                                     FROM stg_roam_out
                                     EXCEPT
                                     SELECT imsi
                                     FROM dim_imsi;";

const INSERT_DIM_MSISDN_QUERY: &str = "INSERT INTO dim_msisdn(msisdn)
                                       SELECT msisdn
                                       FROM stg_roam_out
                                       EXCEPT
                                       SELECT msisdn
                                       FROM dim_msisdn;";

const INSERT_FCT_ROAM_OUT_QUERY: &str =
    "INSERT INTO fct_roam_out(batch_id,date_id,imsi_id,msisdn_id,carrier_id)
                                         SELECT stg.batch_id, dt.id , di.id, dm.id, dc.id
                                         FROM stg_roam_out stg
                                         JOIN dim_time dt ON stg.batch_date = dt.date_text
                                         JOIN dim_imsi di ON stg.imsi = di.imsi
                                         JOIN dim_msisdn dm ON stg.msisdn = dm.msisdn
                                         JOIN dim_carriers dc ON stg.carrier_name = dc.carrier_name 
                                         AND stg.country_name = dc.country_name
                                         AND stg.country_alpha2 = dc.country_alpha2;";

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
            .map_err(AppError::CreatePoolError)?;

        Ok(Self { pool })
    }

    pub async fn get_client(&self) -> Result<Client, AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        Ok(client)
    }

    pub async fn insert_batch_exec(
        &self,
        batch_name: &str,
        source_type: &str,
        source_name: &str,
    ) -> Result<i32, AppError> {
        let client = self.get_client().await?;
        let row = client
            .query_one(
                INSERT_BATCH_EXEC_QUERY,
                &[&batch_name, &source_type, &source_name, &BATCH_STATUS_START],
            )
            .await?;
        let id: i32 = row.try_get("id")?;
        Ok(id)
    }

    pub async fn update_batch_exec(
        &self,
        batch_id: i32,
        batch_status: Option<&str>,
    ) -> Result<u64, AppError> {
        let client = self.get_client().await?;

        if let Some(status) = batch_status {
            let rows_affected = client
                .execute(UPDATE_BATCH_EXEC_QUERY, &[&status, &batch_id])
                .await?;
            Ok(rows_affected)
        } else {
            Ok(0)
        }
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
                carrier_name: row.get::<_, Option<String>>(1).unwrap_or_default(),
                country_alpha2: row.get::<_, Option<String>>(2).unwrap_or_default(),
                country_name: row.get::<_, Option<String>>(3).unwrap_or_default(),
            })
            .collect();

        Ok(prefixes)
    }

    pub async fn insert_roam_out_stg(&self, db_records: Vec<RoamOutDB>) -> Result<(), AppError> {
        let client = self.get_client().await?;

        for record in db_records {
            client
                .execute(
                    INSERT_STG_ROAM_OUT_QUERY,
                    &[
                        &record.batch_id,
                        &record.batch_date,
                        &record.imsi,
                        &record.msisdn,
                        &record.vlr_number,
                        &record.carrier_name,
                        &record.country_name,
                        &record.country_alpha2,
                    ],
                )
                .await
                .map_err(AppError::DatabaseError)?;
        }

        Ok(())
    }

    pub async fn insert_dim_carriers(&self) -> Result<(), AppError> {
        let client = self.get_client().await?;
        client.execute(INSERT_DIM_CARRIERS_QUERY, &[]).await?;
        Ok(())
    }

    pub async fn insert_dim_imsi(&self) -> Result<(), AppError> {
        let client = self.get_client().await?;
        client.execute(INSERT_DIM_IMSI_QUERY, &[]).await?;
        Ok(())
    }

    pub async fn insert_dim_msisdn(&self) -> Result<(), AppError> {
        let client = self.get_client().await?;
        client.execute(INSERT_DIM_MSISDN_QUERY, &[]).await?;
        Ok(())
    }

    pub async fn insert_fct_roam_out(&self) -> Result<(), AppError> {
        let client = self.get_client().await?;
        client.execute(INSERT_FCT_ROAM_OUT_QUERY, &[]).await?;
        Ok(())
    }
}
