use crate::core::errors::AppError;
use crate::settings::operators::models::{NewOperator, Operator, UpdateOperator};
use chrono::Utc;
use deadpool_postgres::Pool;

pub struct OperatorRepository;

impl OperatorRepository {
    /// Helper: get country_id by country_name
    async fn get_country_id(pool: &Pool, country_name: &str) -> Result<i32, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "SELECT country_id FROM dim_countries WHERE country_name = $1";

        if let Some(row) = client
            .query_opt(stmt, &[&country_name])
            .await
            .map_err(AppError::Db)?
        {
            Ok(row.get("country_id"))
        } else {
            Err(AppError::BadRequest(format!(
                "Country '{}' not found",
                country_name
            )))
        }
    }

    /// Helper: get country_name by country_id
    async fn get_country_name(pool: &Pool, country_id: i32) -> Result<String, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "SELECT country_name FROM dim_countries WHERE country_id = $1";

        if let Some(row) = client
            .query_opt(stmt, &[&country_id])
            .await
            .map_err(AppError::Db)?
        {
            Ok(row.get("country_name"))
        } else {
            Err(AppError::BadRequest(format!(
                "Country with ID '{}' not found",
                country_id
            )))
        }
    }

    /// Create a new operator
    pub async fn create(pool: &Pool, new_op: NewOperator) -> Result<Operator, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let country_id = Self::get_country_id(pool, &new_op.country_name).await?;
        let now = Utc::now().naive_utc();

        let stmt = "
            INSERT INTO dim_operators (operator_name, brand_name, country_id, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING operator_id, operator_name, brand_name, country_id, created_at, created_by, updated_at, updated_by
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &new_op.operator_name,
                    &new_op.brand_name,
                    &country_id,
                    &new_op.created_by,
                    &now,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        let country_name = Self::get_country_name(pool, country_id).await?;

        Ok(Operator {
            operator_id: row.get("operator_id"),
            operator_name: row.get("operator_name"),
            brand_name: row.get("brand_name"),
            country_id,
            country_name,
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Get all operators
    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "
            SELECT o.operator_id, o.operator_name, o.brand_name, o.country_id,
                   c.country_name,
                   o.created_at, o.created_by, o.updated_at, o.updated_by
            FROM dim_operators o
            JOIN dim_countries c ON o.country_id = c.country_id
            ORDER BY o.operator_name
        ";

        let rows = client.query(stmt, &[]).await.map_err(AppError::Db)?;

        Ok(rows
            .into_iter()
            .map(|row| Operator {
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                brand_name: row.get("brand_name"),
                country_id: row.get("country_id"),
                country_name: row.get("country_name"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect())
    }

    /// Get operator by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "
            SELECT o.operator_id, o.operator_name, o.brand_name, o.country_id,
                   c.country_name,
                   o.created_at, o.created_by, o.updated_at, o.updated_by
            FROM dim_operators o
            JOIN dim_countries c ON o.country_id = c.country_id
            WHERE o.operator_id = $1
        ";

        if let Some(row) = client.query_opt(stmt, &[&id]).await.map_err(AppError::Db)? {
            Ok(Some(Operator {
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                brand_name: row.get("brand_name"),
                country_id: row.get("country_id"),
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

    /// Update an operator
    pub async fn update(pool: &Pool, id: i32, data: UpdateOperator) -> Result<Operator, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let now = Utc::now().naive_utc();

        let country_id = if let Some(ref country_name) = data.country_name {
            Some(Self::get_country_id(pool, country_name).await?)
        } else {
            None
        };

        let stmt = "
            UPDATE dim_operators SET
                operator_name = COALESCE($1, operator_name),
                brand_name = COALESCE($2, brand_name),
                country_id = COALESCE($3, country_id),
                updated_by = $4,
                updated_at = $5
            WHERE operator_id = $6
            RETURNING operator_id, operator_name, brand_name, country_id, created_at, created_by, updated_at, updated_by
        ";

        let row = client
            .query_one(
                stmt,
                &[
                    &data.operator_name,
                    &data.brand_name,
                    &country_id,
                    &data.updated_by,
                    &now,
                    &id,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        let country_name = Self::get_country_name(pool, row.get("country_id")).await?;

        Ok(Operator {
            operator_id: row.get("operator_id"),
            operator_name: row.get("operator_name"),
            brand_name: row.get("brand_name"),
            country_id: row.get("country_id"),
            country_name,
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Delete an operator
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "DELETE FROM dim_operators WHERE operator_id = $1";
        let deleted = client.execute(stmt, &[&id]).await.map_err(AppError::Db)?;
        Ok(deleted)
    }
}
