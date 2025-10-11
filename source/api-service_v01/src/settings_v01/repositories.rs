use crate::core::errors::AppError;
use crate::settings::models::{
    Country, CreateCountry, UpdateCountry, Operator, CreateOperator, UpdateOperator,
};
use deadpool_postgres::Pool;
use tokio_postgres::Row;

// -------------------------
// Country Repository
// -------------------------
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
        let rows = client.query(&stmt, &[&id]).await?;
        Ok(rows.into_iter().next().map(Self::map_row))
    }

    pub async fn get_by_iso_code(pool: &Pool, iso_code: &str) -> Result<Option<Country>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT * FROM cfg_countries WHERE is_valid = TRUE AND UPPER(iso_code) = UPPER($1)",
            )
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
            .query_one(&stmt, &[&input.iso_code, &input.country_name, &input.created_by])
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
            .query_one(&stmt, &[&input.iso_code, &input.country_name, &input.updated_by, &id])
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

    // Resolve country_id from country_name
    async fn get_country_id(pool: &Pool, country_name: &str) -> Result<i32, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT country_id FROM cfg_countries WHERE country_name = $1 AND is_valid = TRUE")
            .await?;
        let row = client.query_one(&stmt, &[&country_name]).await?;
        Ok(row.get("country_id"))
    }

    pub async fn get_all(pool: &Pool) -> Result<Vec<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT co.*, cc.country_name
                 FROM cfg_operators co
                 JOIN cfg_countries cc ON co.country_id = cc.country_id
                 WHERE co.is_valid = TRUE AND cc.is_valid = TRUE
                 ORDER BY co.country_id, co.operator_name ASC",
            )
            .await?;
        let rows = client.query(&stmt, &[]).await?;
        Ok(rows.into_iter().map(Self::map_row).collect())
    }

    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Operator>, AppError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT co.*, cc.country_name
                 FROM cfg_operators co
                 JOIN cfg_countries cc ON co.country_id = cc.country_id
                 WHERE co.is_valid = TRUE AND co.operator_id = $1",
            )
            .await?;
        let rows = client.query(&stmt, &[&id]).await?;
        Ok(rows.into_iter().next().map(Self::map_row))
    }

    pub async fn create(pool: &Pool, input: CreateOperator) -> Result<Operator, AppError> {
        let country_id = Self::get_country_id(pool, &input.country_name).await?;
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "INSERT INTO cfg_operators (operator_name, brand_name, country_id, created_by)
                 VALUES ($1, $2, $3, $4)
                 RETURNING *",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[&input.operator_name, &input.brand_name, &country_id, &input.created_by],
            )
            .await?;
        Ok(Self::map_row(row))
    }

    pub async fn update(pool: &Pool, id: i32, input: UpdateOperator) -> Result<Operator, AppError> {
        let client = pool.get().await?;

        // Resolve country_id if country_name is provided
        let country_id: Option<i32> = if !input.country_name.is_empty() {
            Some(Self::get_country_id(pool, &input.country_name).await?)
        } else {
            None
        };

        let stmt = client
            .prepare(
                "UPDATE cfg_operators
                 SET brand_name = COALESCE($1, brand_name),
                     country_id = COALESCE($2, country_id),
                     is_valid = COALESCE($3, is_valid),
                     updated_at = NOW(),
                     updated_by = $4
                 WHERE operator_id = $5
                 RETURNING *",
            )
            .await?;
        let row = client
            .query_one(
                &stmt,
                &[&input.brand_name, &country_id, &input.is_valid, &input.updated_by, &id],
            )
            .await?;
        Ok(Self::map_row(row))
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
