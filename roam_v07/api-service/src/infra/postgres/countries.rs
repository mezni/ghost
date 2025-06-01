use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use crate::domain::countries::{Country, CountryRepository};
use crate::infra::error::AppError;

#[derive(Debug, Clone, FromRow)]
pub struct CountryDb {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_by: Option<String>,
}

impl From<CountryDb> for Country {
    fn from(db: CountryDb) -> Self {
        Country {
            id: db.id,
            name: db.name,
            code: db.code,
            created_at: db.created_at,
            created_by: db.created_by,
            updated_at: db.updated_at,
            updated_by: db.updated_by,
        }
    }
}

impl From<Country> for CountryDb {
    fn from(domain: Country) -> Self {
        CountryDb {
            id: domain.id,
            name: domain.name,
            code: domain.code,
            created_at: domain.created_at,
            created_by: domain.created_by,
            updated_at: domain.updated_at,
            updated_by: domain.updated_by,
        }
    }
}

pub struct PgCountryRepository {
    pool: PgPool,
}

impl PgCountryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CountryRepository for PgCountryRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<Country>, AppError> {
        let country_db = sqlx::query_as!(
            CountryDb,
            r#"SELECT id, name, code, created_at, created_by, updated_at, updated_by FROM countries WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(country_db.map(Into::into))
    }

    async fn find_all(&self) -> Result<Vec<Country>, AppError> {
        let countries_db: Vec<CountryDb> = sqlx::query_as!(
            CountryDb,
            r#"SELECT id, name, code, created_at, created_by, updated_at, updated_by FROM countries ORDER BY name"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(countries_db.into_iter().map(Into::into).collect())
    }

    async fn insert(&self, country: &Country) -> Result<i32, AppError> {
        let created_id = sqlx::query!(
            r#"
            INSERT INTO countries (name, code, created_at, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            country.name,
            country.code,
            country.created_at,
            country.created_by as Option<String>,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(created_id)
    }

    async fn update(&self, country: &Country) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE countries
            SET name = $1, code = $2, updated_at = $3, updated_by = $4
            WHERE id = $5
            "#,
            country.name,
            country.code,
            country.updated_at,
            country.updated_by as Option<String>,
            country.id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Country with ID {} not found for update.", country.id)));
        }

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM countries WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Country with ID {} not found for deletion.", id)));
        }

        Ok(())
    }
}