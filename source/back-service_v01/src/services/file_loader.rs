use crate::core::errors::AppError;
use csv_async::AsyncReaderBuilder;
use deadpool_postgres::Pool;
use futures::StreamExt;
use tokio::fs::File;
use tokio_postgres::types::ToSql;
use tokio_util::compat::TokioAsyncReadCompatExt;

/// Loader for ROAM_OUT CSV files
pub struct RoamOutLoader {
    pool: Pool,
}

impl RoamOutLoader {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// Load CSV file and insert into stg_roam_out
    pub async fn load_csv(
        &self,
        file_path: &str,
        batch_id: i32,
        batch_date: &str,
    ) -> Result<usize, AppError> {
        let file = File::open(file_path).await?;
        let reader = file.compat(); // convert to futures AsyncRead

        let mut csv_reader = AsyncReaderBuilder::new()
            .has_headers(true)
            .create_deserializer(reader);

        let mut records = csv_reader.deserialize::<(String, String, String)>();
        let client = self.pool.get().await?;
        let mut inserted_count = 0;

        while let Some(record) = records.next().await {
            let (imsi, msisdn, vlr_number) = record?;

            let query = r#"
                INSERT INTO stg_roam_out (batch_id, batch_date, imsi, msisdn, vlr_number)
                VALUES ($1, $2, $3, $4, $5)
            "#;
            let params: [&(dyn ToSql + Sync); 5] =
                [&batch_id, &batch_date, &imsi, &msisdn, &vlr_number];

            client.execute(query, &params).await?;
            inserted_count += 1;
        }

        Ok(inserted_count)
    }
}
