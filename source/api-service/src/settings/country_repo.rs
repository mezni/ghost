use crate::core::errors::AppError;
use crate::settings::country_model::{Country, NewCountry, UpdateCountry};
use chrono::Utc;
use deadpool_postgres::Pool;
use tokio_postgres::types::ToSql;

pub struct CountryRepository;

impl CountryRepository {
    /// Create a new country
    pub async fn create(pool: &Pool, country: NewCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let now = Utc::now().naive_utc();

        let stmt = "
            INSERT INTO dim_countries (
                iso_code,
                country_name,
                created_by,
                created_at
            )
            VALUES ($1, $2, $3, $4)
            RETURNING country_id, iso_code, country_name, created_at, created_by, updated_at, updated_by
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &country.iso_code,
                    &country.country_name,
                    &country.created_by,
                    &now,
                ],
            )
            .await?;

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

    /// Get all countries
    pub async fn get_all(pool: &Pool) -> Result<Vec<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = "SELECT * FROM dim_countries ORDER BY country_id";

        let rows = client.query(stmt, &[]).await?;

        let countries = rows
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
            .collect();

        Ok(countries)
    }

    /// Get a country by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = "SELECT * FROM dim_countries WHERE country_id = $1";

        if let Some(row) = client.query_opt(stmt, &[&id]).await? {
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

    /// Update a country
    pub async fn update(pool: &Pool, id: i32, data: UpdateCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let now = Utc::now().naive_utc();

        let stmt = "
            UPDATE dim_countries SET
                iso_code = COALESCE($1, iso_code),
                country_name = COALESCE($2, country_name),
                updated_by = $3,
                updated_at = $4
            WHERE country_id = $5
            RETURNING country_id, iso_code, country_name, created_at, created_by, updated_at, updated_by
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &data.iso_code,
                    &data.country_name,
                    &data.updated_by,
                    &now,
                    &id,
                ],
            )
            .await?;

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
    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        let client = pool.get().await?;
        let stmt = "DELETE FROM dim_countries WHERE country_id = $1";
        client.execute(stmt, &[&id]).await?;
        Ok(())
    }
}
