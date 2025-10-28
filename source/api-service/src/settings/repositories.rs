use crate::settings::models::{Country, CreateCountry, UpdateCountry};
use crate::core::errors::AppError;
use sqlx::PgPool;

pub struct CountryRepository;

impl CountryRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Country>, AppError> {
        let countries = sqlx::query_as::<_, Country>(
            r#"
            SELECT country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            FROM cfg_countries
            ORDER BY country_name
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(countries)
    }

    pub async fn get_by_id(pool: &PgPool, country_id: i32) -> Result<Option<Country>, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            SELECT country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            FROM cfg_countries
            WHERE country_id = $1
            "#
        )
        .bind(country_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn create(pool: &PgPool, data: CreateCountry) -> Result<Country, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            INSERT INTO cfg_countries (iso_code, country_name, created_by, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            "#
        )
        .bind(&data.iso_code)
        .bind(&data.country_name)
        .bind(&data.created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn update(pool: &PgPool, country_id: i32, data: UpdateCountry) -> Result<Option<Country>, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            UPDATE cfg_countries
            SET 
                iso_code = COALESCE($1, iso_code),
                country_name = COALESCE($2, country_name),
                is_valid = COALESCE($3, is_valid),
                updated_by = $4,
                updated_at = NOW()
            WHERE country_id = $5
            RETURNING country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            "#
        )
        .bind(&data.iso_code)
        .bind(&data.country_name)
        .bind(data.is_valid)
        .bind(&data.updated_by)
        .bind(country_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn delete(pool: &PgPool, country_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE cfg_countries
            SET is_valid = FALSE, updated_at = NOW()
            WHERE country_id = $1
            "#
        )
        .bind(country_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(result.rows_affected())
    }
}
