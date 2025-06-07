use crate::domain::countries::{Country, CountryRepository};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row, postgres::PgRow};
use std::sync::Arc;

pub struct PgCountryRepository {
    pool: Arc<PgPool>,
}

impl PgCountryRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    // Helper: map a sqlx::Row to Country struct
    fn row_to_country(row: PgRow) -> Country {
        Country {
            id: row.try_get("id").ok(),
            name: row.try_get("name").unwrap_or_default(),
            code: row.try_get("code").unwrap_or_default(),
            created_at: row.try_get("created_at").ok(),
            created_by: row.try_get("created_by").ok(),
            updated_at: row.try_get("updated_at").ok(),
            updated_by: row.try_get("updated_by").ok(),
        }
    }
}

#[async_trait]
impl CountryRepository for PgCountryRepository {
    async fn insert(&self, country: Country) -> Result<Country, String> {
        let rec = sqlx::query(
            "INSERT INTO countries (name, code, created_at, created_by, updated_at, updated_by)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, name, code, created_at, created_by, updated_at, updated_by",
        )
        .bind(&country.name)
        .bind(&country.code)
        .bind(&country.created_at)
        .bind(&country.created_by)
        .bind(&country.updated_at)
        .bind(&country.updated_by)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Country {
            id: Some(rec.try_get("id").map_err(|e| e.to_string())?),
            name: rec.try_get("name").unwrap_or_default(),
            code: rec.try_get("code").unwrap_or_default(),
            created_at: rec.try_get("created_at").ok(),
            created_by: rec.try_get("created_by").ok(),
            updated_at: rec.try_get("updated_at").ok(),
            updated_by: rec.try_get("updated_by").ok(),
        })
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Country>, String> {
        let rec_opt = sqlx::query(
            "SELECT id, name, code, created_at, created_by, updated_at, updated_by
             FROM countries WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rec_opt.map(|rec| Self::row_to_country(rec)))
    }

    async fn update(&self, country: Country) -> Result<(), String> {
        let id = country
            .id
            .ok_or_else(|| "Cannot update country without ID".to_string())?;

        sqlx::query(
            "UPDATE countries SET
                name = $1,
                code = $2,
                updated_at = $3,
                updated_by = $4
             WHERE id = $5",
        )
        .bind(&country.name)
        .bind(&country.code)
        .bind(&country.updated_at)
        .bind(&country.updated_by)
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        sqlx::query("DELETE FROM countries WHERE id = $1")
            .bind(id)
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<Country>, String> {
        let rows = sqlx::query(
            "SELECT id, name, code, created_at, created_by, updated_at, updated_by
             FROM countries
             ORDER BY name",
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let countries = rows.into_iter().map(Self::row_to_country).collect();

        Ok(countries)
    }
}
