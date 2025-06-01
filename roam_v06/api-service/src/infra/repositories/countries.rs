// infra/repositories/postgres_country_repository.rs
use crate::domain::entities::country::Country;
use crate::domain::repositories::country_repository::CountryRepository;
use crate::errors::AppError;
use async_trait::async_trait;
use deadpool_postgres::{Pool, Client};
use tokio_postgres::Row;

pub struct PostgresCountryRepository {
    pool: Pool,
}

impl PostgresCountryRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn row_to_country(row: Row) -> Country {
        Country {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
        }
    }
}

#[async_trait]
impl CountryRepository for PostgresCountryRepository {
    async fn get_all(&self) -> Result<Vec<Country>, AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        let stmt = client.prepare("SELECT id, name, code FROM countries").await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Self::row_to_country).collect())
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Country>, AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        let stmt = client.prepare("SELECT id, name, code FROM countries WHERE id = $1").await?;
        let row_opt = client.query_opt(&stmt, &[&id]).await?;
        Ok(row_opt.map(Self::row_to_country))
    }

    async fn create(&self, country: Country) -> Result<Country, AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        let stmt = client
            .prepare("INSERT INTO countries (name, code) VALUES ($1, $2) RETURNING id")
            .await?;
        let row = client.query_one(&stmt, &[&country.name, &country.code]).await?;
        let id: i32 = row.get("id");
        Ok(Country { id, ..country })
    }

    async fn update(&self, country: Country) -> Result<(), AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        let stmt = client
            .prepare("UPDATE countries SET name = $1, code = $2 WHERE id = $3")
            .await?;
        client.execute(&stmt, &[&country.name, &country.code, &country.id]).await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let client = self.pool.get().await.map_err(AppError::PoolError)?;
        let stmt = client.prepare("DELETE FROM countries WHERE id = $1").await?;
        client.execute(&stmt, &[&id]).await?;
        Ok(())
    }
}
