use crate::core::errors::AppError;
use crate::settings::models::{
    Country, CreateCountry, UpdateCountry, Operator, CreateOperator, UpdateOperator,
};
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
            .prepare("SELECT * FROM cfg_countries WHERE is_valid = TRUE AND country_id = $1")
            .await?;
        let row = client.query_opt(&stmt, &[&id]).await?;
        Ok(row.map(Self::map_row))
    }

    pub async fn get_by_iso_code(pool: &Pool, iso_code: &str) -> Result<Option<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM cfg_countries WHERE is_valid = TRUE AND UPPER(iso_code) = UPPER($1)")
            .await?;
        let row = client.query_opt(&stmt, &[&iso_code]).await?;
        Ok(row.map(Self::map_row))
    }

    pub async fn create(pool: &Pool, input: CreateCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO cfg_countries (iso_code, country_name, created_by)
                 VALUES (UPPER($1), INITCAP($2), $3)
                 RETURNING *",
            )
            .await?;
        let row = client
            .query_one(&stmt, &[&input.iso_code, &input.country_name, &input.created_by])
            .await?;
        Ok(Self::map_row(row))
    }

    pub async fn update(pool: &Pool, id: i32, input: UpdateCountry) -> Result<Country, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE cfg_countries
                 SET country_name = COALESCE(INITCAP($1), country_name),
                     updated_at = NOW(),
                     updated_by = $2
                 WHERE country_id = $3
                 RETURNING *",
            )
            .await?;
        let row = client.query_opt(&stmt, &[&input.country_name, &input.updated_by, &id]).await?;
        match row {
            Some(r) => Ok(Self::map_row(r)),
            None => Err(AppError::Other("Country not found".into())),
        }
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

pub struct OperatorRepository;

impl OperatorRepository {
    fn map_row(row: Row) -> Operator {
        Operator {
            operator_id: row.get("operator_id"),
            operator_name: row.get("operator_name"),
            brand_name: row.get("brand_name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            is_valid: row.get("is_valid"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        }
    }

    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT op.*, c.country_name
                 FROM cfg_operators op
                 JOIN cfg_countries c ON op.country_id = c.country_id
                 WHERE op.is_valid = TRUE AND c.is_valid = TRUE
                 ORDER BY c.country_name, op.operator_name ASC",
            )
            .await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT op.*, c.country_name
                 FROM cfg_operators op
                 JOIN cfg_countries c ON op.country_id = c.country_id
                 WHERE op.is_valid = TRUE AND op.operator_id = $1",
            )
            .await?;
        let row = client.query_opt(&stmt, &[&id]).await?;
        Ok(row.map(Self::map_row))
    }

    pub async fn create(pool: &Pool, input: CreateOperator) -> Result<Operator, AppError> {
        let client = pool.get().await?;

        // Find country_id
        let country_stmt = client
            .prepare(
                "SELECT country_id FROM cfg_countries 
                 WHERE UPPER(country_name) = UPPER($1) AND is_valid = TRUE",
            )
            .await?;
        let country_row = client.query_opt(&country_stmt, &[&input.country_name]).await?;
        let country_id: i32 = match country_row {
            Some(row) => row.get("country_id"),
            None => return Err(AppError::Other("Country not found".into())),
        };

        // Insert operator
        let insert_stmt = client
            .prepare(
                "INSERT INTO cfg_operators (operator_name, brand_name, country_id, created_by)
                 VALUES (INITCAP($1), INITCAP($2), $3, $4)
                 RETURNING operator_id",
            )
            .await?;
        let row = client
            .query_one(&insert_stmt, &[&input.operator_name, &input.brand_name, &country_id, &input.created_by])
            .await?;
        let operator_id: i32 = row.get("operator_id");

        // Return full operator with country_name
        let select_stmt = client
            .prepare(
                "SELECT op.*, c.country_name
                 FROM cfg_operators op
                 JOIN cfg_countries c ON op.country_id = c.country_id
                 WHERE op.operator_id = $1",
            )
            .await?;
        let created_row = client.query_one(&select_stmt, &[&operator_id]).await?;
        Ok(Self::map_row(created_row))
    }

    pub async fn update(pool: &Pool, input: &UpdateOperator) -> Result<Operator, AppError> {
        let client = pool.get().await?;

        // Update operator_name and brand_name
        let stmt_update = client
            .prepare(
                "UPDATE cfg_operators
                 SET operator_name = COALESCE(INITCAP($1), operator_name),
                     brand_name    = COALESCE(INITCAP($2), brand_name),
                     updated_by    = $3,
                     updated_at    = NOW()
                 WHERE operator_id = $4
                 RETURNING operator_id",
            )
            .await?;
        let row = client
            .query_one(&stmt_update, &[&input.operator_name, &input.brand_name, &input.updated_by, &input.operator_id])
            .await?;
        let operator_id: i32 = row.get("operator_id");

        // Fetch full operator with country_name
        let stmt_select = client
            .prepare(
                "SELECT op.*, c.country_name
                 FROM cfg_operators op
                 JOIN cfg_countries c ON op.country_id = c.country_id
                 WHERE op.operator_id = $1",
            )
            .await?;
        let updated_row = client.query_one(&stmt_select, &[&operator_id]).await?;
        Ok(Self::map_row(updated_row))
    }

    pub async fn delete(pool: &Pool, id: i32) -> Result<(), AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("UPDATE cfg_operators SET is_valid = FALSE WHERE operator_id = $1")
            .await?;
        client.execute(&stmt, &[&id]).await?;
        Ok(())
    }
}
