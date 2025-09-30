use crate::core::errors::AppError;
use crate::catalog::operators::models::{NewOperator, Operator, UpdateOperator};
use chrono::Utc;
use deadpool_postgres::Pool;

pub struct OperatorRepository;

impl OperatorRepository {
    /// Create a new operator
    pub async fn create(pool: &Pool, new_operator: &NewOperator) -> Result<Operator, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO dim_operators (operator_name, brand_name, country_id, created_at, created_by)
                 VALUES ($1, 
                         (SELECT country_id FROM dim_countries WHERE country_name = $2), 
                         $3, $4)
                 RETURNING o.operator_id, o.operator_name, o.brand_name,
                           c.country_id, c.country_name,
                           o.created_at, o.created_by, o.updated_at, o.updated_by
                 FROM dim_operators o
                 JOIN dim_countries c ON o.country_id = c.country_id
                 WHERE o.operator_id = currval('dim_operators_operator_id_seq')",
            )
            .await?;

        let row = client
            .query_one(
                &stmt,
                &[
                    &new_operator.operator_name,
                    &new_operator.brand_name,
                    &Utc::now().naive_utc(),
                    &new_operator.created_by,
                ],
            )
            .await?;

        Ok(Operator {
            operator_id: row.get(0),
            operator_name: row.get(1),
            brand_name: row.get(2),
            country_id: row.get(3),
            country_name: row.get(4),
            created_at: row.get(5),
            created_by: row.get(6),
            updated_at: row.get(7),
            updated_by: row.get(8),
        })
    }

    /// Get all operators
    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT o.operator_id, o.operator_name, o.brand_name,
                        c.country_id, c.country_name,
                        o.created_at, o.created_by, o.updated_at, o.updated_by
                 FROM dim_operators o
                 JOIN dim_countries c ON o.country_id = c.country_id",
            )
            .await?;

        let rows = client.query(&stmt, &[]).await?;
        let operators = rows
            .into_iter()
            .map(|row| Operator {
                operator_id: row.get(0),
                operator_name: row.get(1),
                brand_name: row.get(2),
                country_id: row.get(3),
                country_name: row.get(4),
                created_at: row.get(5),
                created_by: row.get(6),
                updated_at: row.get(7),
                updated_by: row.get(8),
            })
            .collect();

        Ok(operators)
    }

    /// Get operator by ID
    pub async fn get_by_id(pool: &Pool, operator_id: i32) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT o.operator_id, o.operator_name, o.brand_name,
                        c.country_id, c.country_name,
                        o.created_at, o.created_by, o.updated_at, o.updated_by
                 FROM dim_operators o
                 JOIN dim_countries c ON o.country_id = c.country_id
                 WHERE o.operator_id = $1",
            )
            .await?;

        let row = client.query_opt(&stmt, &[&operator_id]).await?;
        Ok(row.map(|row| Operator {
            operator_id: row.get(0),
            operator_name: row.get(1),
            brand_name: row.get(2),
            country_id: row.get(3),
            country_name: row.get(4),
            created_at: row.get(5),
            created_by: row.get(6),
            updated_at: row.get(7),
            updated_by: row.get(8),
        }))
    }

    /// Get operators by country_id
    pub async fn get_by_country_id(
        pool: &Pool,
        country_id: i32,
    ) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT o.operator_id, o.operator_name, o.brand_name,
                        c.country_id, c.country_name,
                        o.created_at, o.created_by, o.updated_at, o.updated_by
                 FROM dim_operators o
                 JOIN dim_countries c ON o.country_id = c.country_id
                 WHERE o.country_id = $1",
            )
            .await?;

        let rows = client.query(&stmt, &[&country_id]).await?;
        let operators = rows
            .into_iter()
            .map(|row| Operator {
                operator_id: row.get(0),
                operator_name: row.get(1),
                brand_name: row.get(2),
                country_id: row.get(3),
                country_name: row.get(4),
                created_at: row.get(5),
                created_by: row.get(6),
                updated_at: row.get(7),
                updated_by: row.get(8),
            })
            .collect();

        Ok(operators)
    }

    /// Update operator
    pub async fn update(
        pool: &Pool,
        operator_id: i32,
        update: &UpdateOperator,
    ) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE dim_operators 
                 SET operator_name = COALESCE($1, operator_name),
                     brand_name = COALESCE($2, brand_name),
                     country_id = COALESCE((SELECT country_id FROM dim_countries WHERE country_name = $3), country_id),
                     updated_at = $4,
                     updated_by = $5
                 WHERE operator_id = $6
                 RETURNING o.operator_id, o.operator_name, o.brand_name,
                           c.country_id, c.country_name,
                           o.created_at, o.created_by, o.updated_at, o.updated_by
                 FROM dim_operators o
                 JOIN dim_countries c ON o.country_id = c.country_id
                 WHERE o.operator_id = $6",
            )
            .await?;

        let row = client
            .query_opt(
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

        Ok(row.map(|row| Operator {
            operator_id: row.get(0),
            operator_name: row.get(1),
            brand_name: row.get(2),
            country_id: row.get(3),
            country_name: row.get(4),
            created_at: row.get(5),
            created_by: row.get(6),
            updated_at: row.get(7),
            updated_by: row.get(8),
        }))
    }

    /// Delete operator (hard delete)
    pub async fn delete(pool: &Pool, operator_id: i32) -> Result<(), AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("DELETE FROM dim_operators WHERE operator_id = $1")
            .await?;

        client.execute(&stmt, &[&operator_id]).await?;
        Ok(())
    }
}
