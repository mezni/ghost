use deadpool_postgres::Pool;
use crate::models::{Country, NewCountry};
use crate::errors::AppError;

pub struct CountryRepository;

impl CountryRepository {
    pub async fn list(pool: &Pool) -> Result<Vec<Country>, AppError> {
        let client = pool.get().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare("SELECT id, country_name, iso FROM countries")
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let rows = client.query(&stmt, &[]).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(rows.iter().map(|r| Country {
            id: r.get(0),
            country_name: r.get(1),
            iso: r.get(2),
        }).collect())
    }

    pub async fn insert(pool: &Pool, new_country: &NewCountry) -> Result<Country, AppError> {
        let client = pool.get().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "INSERT INTO countries (country_name, iso) VALUES ($1, $2) RETURNING id, country_name, iso"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = client.query_one(&stmt, &[&new_country.country_name, &new_country.iso])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Country {
            id: row.get(0),
            country_name: row.get(1),
            iso: row.get(2),
        })
    }
}
