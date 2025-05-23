use crate::errors::AppError;
use crate::models::{Country, NewCountry, NewOperator, Operator};
use deadpool_postgres::Pool;

pub struct CountryRepository;
pub struct OperatorRepository;

impl CountryRepository {
    pub async fn list(pool: &Pool) -> Result<Vec<Country>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "SELECT country_id, country_name, iso, created_at, updated_at, created_by, updated_by FROM countries"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let rows = client
            .query(&stmt, &[])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(rows
            .iter()
            .map(|r| Country {
                country_id: r.get(0),
                country_name: r.get(1),
                iso: r.get(2),
                created_at: r.get(3),
                updated_at: r.get(4),
                created_by: r.get(5),
                updated_by: r.get(6),
            })
            .collect())
    }

    pub async fn insert(pool: &Pool, new_country: &NewCountry) -> Result<Country, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "INSERT INTO countries (country_name, iso, created_by, created_at) VALUES ($1, $2, 'TEST', NOW()) \
             RETURNING country_id, country_name, iso, created_at, updated_at, created_by, updated_by"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = client
            .query_one(&stmt, &[&new_country.country_name, &new_country.iso])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Country {
            country_id: row.get(0),
            country_name: row.get(1),
            iso: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            created_by: row.get(5),
            updated_by: row.get(6),
        })
    }

    pub async fn update(
        pool: &Pool,
        country_id: i32,
        country_name: &str,
        iso: &str,
        updated_by: &str,
    ) -> Result<Country, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "UPDATE countries SET country_name = $1, iso = $2, updated_by = $3, updated_at = NOW()
             WHERE country_id = $4
             RETURNING country_id, country_name, iso, created_at, updated_at, created_by, updated_by"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = client
            .query_one(&stmt, &[&country_name, &iso, &updated_by, &country_id])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Country {
            country_id: row.get(0),
            country_name: row.get(1),
            iso: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            created_by: row.get(5),
            updated_by: row.get(6),
        })
    }
}

impl OperatorRepository {
    pub async fn list(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "SELECT operator_id, operator_name, country_id, created_at, updated_at, created_by, updated_by FROM operators"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let rows = client
            .query(&stmt, &[])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(rows
            .iter()
            .map(|r| Operator {
                operator_id: r.get(0),
                operator_name: r.get(1),
                country_id: r.get(2),
                created_at: r.get(3),
                updated_at: r.get(4),
                created_by: r.get(5),
                updated_by: r.get(6),
            })
            .collect())
    }

    pub async fn insert(pool: &Pool, new_operator: &NewOperator) -> Result<Operator, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "INSERT INTO operators (operator_name, country_id, created_by, created_at) \
             VALUES ($1, $2, $3, NOW()) \
             RETURNING operator_id, operator_name, country_id, created_at, updated_at, created_by, updated_by"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = client
            .query_one(
                &stmt,
                &[
                    &new_operator.operator_name,
                    &new_operator.country_id,
                    &new_operator.created_by,
                ],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Operator {
            operator_id: row.get(0),
            operator_name: row.get(1),
            country_id: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            created_by: row.get(5),
            updated_by: row.get(6),
        })
    }

    pub async fn update(
        pool: &Pool,
        operator_id: i32,
        operator_name: &str,
        country_id: i32,
        updated_by: &str,
    ) -> Result<Operator, AppError> {
        let client = pool
            .get()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let stmt = client.prepare(
            "UPDATE operators SET operator_name = $1, country_id = $2, updated_by = $3, updated_at = NOW()
             WHERE operator_id = $4
             RETURNING operator_id, operator_name, country_id, created_at, updated_at, created_by, updated_by"
        ).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let row = client
            .query_one(
                &stmt,
                &[&operator_name, &country_id, &updated_by, &operator_id],
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(Operator {
            operator_id: row.get(0),
            operator_name: row.get(1),
            country_id: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
            created_by: row.get(5),
            updated_by: row.get(6),
        })
    }
}
