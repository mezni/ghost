use crate::domain::countries::{Country, CountryRepository};
use crate::errors::AppError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

pub struct PostgresCountryRepository {
    pub pool: PgPool,
}

impl PostgresCountryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CountryRepository for PostgresCountryRepository {
    async fn find_all(&self) -> Result<Vec<Country>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, code, created_at, created_by, updated_at, updated_by
            FROM countries
            ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let countries = rows
            .into_iter()
            .map(|row| Country {
                id: row.get("id"),
                name: row.get("name"),
                code: row.get("code"),
                created_at: row.get("created_at"),
                created_by: row.get::<Option<String>, _>("created_by"),
                updated_at: row.get::<Option<DateTime<Utc>>, _>("updated_at"),
                updated_by: row.get::<Option<String>, _>("updated_by"),
            })
            .collect();

        Ok(countries)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Country>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, code, created_at, created_by, updated_at, updated_by
            FROM countries
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(row.map(|row| Country {
            id: row.get("id"),
            name: row.get("name"),
            code: row.get("code"),
            created_at: row.get("created_at"),
            created_by: row.get::<Option<String>, _>("created_by"),
            updated_at: row.get::<Option<DateTime<Utc>>, _>("updated_at"),
            updated_by: row.get::<Option<String>, _>("updated_by"),
        }))
    }

    async fn insert(&self, country: &Country) -> Result<i32, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO countries (name, code, created_at, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(&country.name)
        .bind(&country.code)
        .bind(country.created_at)
        .bind(&country.created_by)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(row.get("id"))
    }

    async fn update(&self, country: &Country) -> Result<(), AppError> {
        sqlx::query(
            r#"
            UPDATE countries
            SET name = $1, code = $2, updated_at = $3, updated_by = $4
            WHERE id = $5
            "#,
        )
        .bind(&country.name)
        .bind(&country.code)
        .bind(&country.updated_at)
        .bind(&country.updated_by)
        .bind(country.id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM countries
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}
