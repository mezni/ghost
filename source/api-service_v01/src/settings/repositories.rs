use crate::core::errors::AppError;
use crate::settings::models::{Country, CreateCountry, UpdateCountry};
use deadpool_postgres::Pool;
use tokio_postgres::Row;

pub struct CountryRepository;

impl CountryRepository {
    fn map_row(row: Row) -> Country {
        Country {
            country_id: row.get("country_id"),
            iso_code: row.get("iso_code"),
            country_name: row.get("country_name"),
            is_valid: row.get("is_valid"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        }
    }

    pub async fn get_all(pool: &Pool) -> Result<Vec<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM cfg_countries WHERE is_valid = TRUE ORDER BY country_name ASC")
            .await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM cfg_countries WHERE  is_valid = TRUE AND country_id = $1")
            .await?;
        let rows = client.query(&stmt, &[&id]).await?;

        Ok(rows.into_iter().next().map(Self::map_row))
    }

    pub async fn get_by_iso_code(pool: &Pool, iso_code: &str) -> Result<Option<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM cfg_countries WHERE  is_valid = TRUE AND UPPER(iso_code) = UPPER($1)")
            .await?;
        let rows = client.query(&stmt, &[&iso_code]).await?;

        Ok(rows.into_iter().next().map(Self::map_row))
    }

    pub async fn create(pool: &Pool, input: CreateCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO cfg_countries (iso_code, country_name, created_by)
                 VALUES (UPPER($1), $2, $3)
                 RETURNING *",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[&input.iso_code, &input.country_name, &input.created_by],
            )
            .await?;

        Ok(Self::map_row(row))
    }

    pub async fn update(pool: &Pool, id: i32, input: UpdateCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE cfg_countries
                 SET iso_code = COALESCE($1, iso_code),
                     country_name = COALESCE($2, country_name),
                     updated_at = NOW(),
                     updated_by = $3
                 WHERE country_id = $4
                 RETURNING *",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[&input.iso_code, &input.country_name, &input.updated_by, &id],
            )
            .await?;

        Ok(Self::map_row(row))
    }

    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("UPDATE cfg_countries SET is_valid = FALSE WHERE country_id = $1")
            .await?;
        client.execute(&stmt, &[&id]).await?;
        Ok(())
    }
}
// -------------------------
// Operator Repository
// -------------------------
pub struct OperatorRepository;

impl OperatorRepository {
    /// Create a new operator using country_name
    pub async fn create(pool: &Pool, new_operator: &CreateOperator) -> Result<Operator, AppError> {
        let client = pool.get().await?;

        // Insert operator and get operator_id
        let stmt = client
            .prepare(
                r#"
                INSERT INTO cfg_operators (operator_name, brand_name, country_id, created_at, created_by)
                VALUES (
                    $1,
                    $2,
                    (SELECT country_id FROM cfg_countries WHERE country_name = $3),
                    $4,
                    $5
                )
                RETURNING operator_id
                "#,
            )
            .await?;

        let operator_id: i32 = client
            .query_one(
                &stmt,
                &[
                    &new_operator.operator_name,
                    &new_operator.brand_name,
                    &new_operator.country_name,
                    &Utc::now().naive_utc(),
                    &new_operator.created_by,
                ],
            )
            .await?
            .get(0);

        // Return full operator info
        Self::get_by_id(pool, operator_id)
            .await?
            .ok_or_else(|| AppError::Other("Failed to create operator".into()))
    }

    /// Update operator using country_name
    pub async fn update(
        pool: &Pool,
        operator_id: i32,
        update: &UpdateOperator,
    ) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;

        let stmt = client
            .prepare(
                r#"
                UPDATE cfg_operators
                SET
                    operator_name = COALESCE($1, operator_name),
                    brand_name = COALESCE($2, brand_name),
                    country_id = COALESCE(
                        (SELECT country_id FROM cfg_countries WHERE country_name = $3),
                        country_id
                    ),
                    updated_at = $4,
                    updated_by = $5
                WHERE operator_id = $6
                "#,
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &update.operator_name,
                    &update.brand_name,
                    &update.country_name,
                    &Utc::now().naive_utc(),
                    &update.updated_by,
                    &operator_id,
                ],
            )
            .await?;

        // Return updated operator
        Self::get_by_id(pool, operator_id).await
    }

    /// Get all operators
    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
                SELECT o.operator_id, o.operator_name, o.brand_name,
                       c.country_id, c.country_name,
                       o.created_at, o.created_by, o.updated_at, o.updated_by
                FROM cfg_operators o
                JOIN cfg_countries c ON o.country_id = c.country_id
                ORDER BY o.operator_name ASC
                "#,
            )
            .await?;

        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    /// Get operator by ID
    pub async fn get_by_id(pool: &Pool, operator_id: i32) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
                SELECT o.operator_id, o.operator_name, o.brand_name,
                       c.country_id, c.country_name,
                       o.created_at, o.created_by, o.updated_at, o.updated_by
                FROM cfg_operators o
                JOIN cfg_countries c ON o.country_id = c.country_id
                WHERE o.operator_id = $1
                "#,
            )
            .await?;

        let row = client.query_opt(&stmt, &[&operator_id]).await?;
        Ok(row.map(Self::map_row))
    }

    /// Get operators by country_id
    pub async fn get_by_country_id(pool: &Pool, country_id: i32) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                r#"
                SELECT o.operator_id, o.operator_name, o.brand_name,
                       c.country_id, c.country_name,
                       o.created_at, o.created_by, o.updated_at, o.updated_by
                FROM cfg_operators o
                JOIN cfg_countries c ON o.country_id = c.country_id
                WHERE c.country_id = $1
                ORDER BY o.operator_name ASC
                "#,
            )
            .await?;

        let rows = client.query(&stmt, &[&country_id]).await?;
        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    /// Delete operator (hard delete)
    pub async fn delete(pool: &Pool, operator_id: i32) -> Result<(), AppError> {
        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM cfg_operators WHERE operator_id = $1").await?;
        client.execute(&stmt, &[&operator_id]).await?;
        Ok(())
    }

    /// Map query row to Operator struct
    fn map_row(row: tokio_postgres::Row) -> Operator {
        Operator {
            operator_id: row.get("operator_id"),
            operator_name: row.get("operator_name"),
            brand_name: row.get("brand_name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        }
    }
}
