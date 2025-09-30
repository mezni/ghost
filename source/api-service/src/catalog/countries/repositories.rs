use crate::catalog::countries::models::{Country, NewCountry, UpdateCountry};
use crate::core::errors::AppError;
use chrono::Utc;
use deadpool_postgres::Pool;

pub struct CountryRepository;

impl CountryRepository {
    /// Get all countries
    pub async fn get_all(pool: &Pool) -> Result<Vec<Country>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"SELECT * FROM dim_countries ORDER BY country_name"#;

        let rows = client.query(stmt, &[]).await.map_err(AppError::Db)?;
        Ok(rows
            .into_iter()
            .map(|row| Country {
                country_id: row.get("country_id"),
                iso_code: row.get("iso_code"),
                country_name: row.get("country_name"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect())
    }

    /// Get country by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Country>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "SELECT * FROM dim_countries WHERE country_id = $1";

        if let Some(row) = client.query_opt(stmt, &[&id]).await.map_err(AppError::Db)? {
            Ok(Some(Country {
                country_id: row.get("country_id"),
                iso_code: row.get("iso_code"),
                country_name: row.get("country_name"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get country by name
    pub async fn get_by_name(pool: &Pool, name: &str) -> Result<Option<Country>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "SELECT * FROM dim_countries WHERE country_name = $1";

        if let Some(row) = client
            .query_opt(stmt, &[&name])
            .await
            .map_err(AppError::Db)?
        {
            Ok(Some(Country {
                country_id: row.get("country_id"),
                iso_code: row.get("iso_code"),
                country_name: row.get("country_name"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Create a new country
    pub async fn create(pool: &Pool, new_country: NewCountry) -> Result<Country, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let now = Utc::now().naive_utc();

        let stmt = r#"
            INSERT INTO dim_countries (iso_code, country_name, created_at, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &new_country.iso_code,
                    &new_country.country_name,
                    &now,
                    &new_country.created_by,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(Country {
            country_id: row.get("country_id"),
            iso_code: row.get("iso_code"),
            country_name: row.get("country_name"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Update country
    pub async fn update(pool: &Pool, id: i32, data: UpdateCountry) -> Result<Country, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let now = Utc::now().naive_utc();

        // Update the row
        let stmt = r#"
            UPDATE dim_countries
            SET iso_code = $1, country_name = $2, updated_at = $3, updated_by = $4
            WHERE country_id = $5
            RETURNING *
        "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &data.iso_code,
                    &data.country_name,
                    &now,
                    &data.updated_by,
                    &id,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(Country {
            country_id: row.get("country_id"),
            iso_code: row.get("iso_code"),
            country_name: row.get("country_name"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Delete a country
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "DELETE FROM dim_countries WHERE country_id = $1";
        let deleted = client.execute(stmt, &[&id]).await.map_err(AppError::Db)?;
        Ok(deleted)
    }
}
