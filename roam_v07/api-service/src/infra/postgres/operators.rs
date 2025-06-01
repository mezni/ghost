use async_trait::async_trait;
use sqlx::{PgPool, FromRow};
use crate::domain::operators::{Operator, OperatorRepository};
use crate::infra::error::AppError;

#[derive(Debug, Clone, FromRow)]
pub struct OperatorDb {
    pub id: i32,
    pub name: String,
    pub country_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_by: Option<String>,
}

impl From<OperatorDb> for Operator {
    fn from(db: OperatorDb) -> Self {
        Operator {
            id: db.id,
            name: db.name,
            country_id: db.country_id,
            created_at: db.created_at,
            created_by: db.created_by,
            updated_at: db.updated_at,
            updated_by: db.updated_by,
        }
    }
}

impl From<Operator> for OperatorDb {
    fn from(domain: Operator) -> Self {
        OperatorDb {
            id: domain.id,
            name: domain.name,
            country_id: domain.country_id,
            created_at: domain.created_at,
            created_by: domain.created_by,
            updated_at: domain.updated_at,
            updated_by: domain.updated_by,
        }
    }
}

pub struct PgOperatorRepository {
    pool: PgPool,
}

impl PgOperatorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OperatorRepository for PgOperatorRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<Operator>, AppError> {
        let operator_db = sqlx::query_as!(
            OperatorDb,
            r#"SELECT id, name, country_id, created_at, created_by, updated_at, updated_by FROM operators WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(operator_db.map(Into::into))
    }

    async fn find_by_country_id(&self, country_id: i32) -> Result<Vec<Operator>, AppError> {
        let operators_db: Vec<OperatorDb> = sqlx::query_as!(
            OperatorDb,
            r#"SELECT id, name, country_id, created_at, created_by, updated_at, updated_by FROM operators WHERE country_id = $1 ORDER BY name"#,
            country_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(operators_db.into_iter().map(Into::into).collect())
    }

    async fn find_all(&self) -> Result<Vec<Operator>, AppError> {
        let operators_db: Vec<OperatorDb> = sqlx::query_as!(
            OperatorDb,
            r#"SELECT id, name, country_id, created_at, created_by, updated_at, updated_by FROM operators ORDER BY name"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(operators_db.into_iter().map(Into::into).collect())
    }

    async fn insert(&self, operator: &Operator) -> Result<i32, AppError> {
        let created_id = sqlx::query!(
            r#"
            INSERT INTO operators (name, country_id, created_at, created_by)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            operator.name,
            operator.country_id,
            operator.created_at,
            operator.created_by as Option<String>,
        )
        .fetch_one(&self.pool)
        .await?
        .id;

        Ok(created_id)
    }

    async fn update(&self, operator: &Operator) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE operators
            SET name = $1, country_id = $2, updated_at = $3, updated_by = $4
            WHERE id = $5
            "#,
            operator.name,
            operator.country_id,
            operator.updated_at,
            operator.updated_by as Option<String>,
            operator.id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Operator with ID {} not found for update.", operator.id)));
        }

        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM operators WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::NotFound(format!("Operator with ID {} not found for deletion.", id)));
        }

        Ok(())
    }
}