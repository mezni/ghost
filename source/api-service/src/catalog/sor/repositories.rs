use crate::catalog::sor::models::{NewSorPlan, SorPlan, UpdateSorPlan};
use crate::core::errors::AppError;
use chrono::Utc;
use deadpool_postgres::Pool;

pub struct SorPlanRepository;

impl SorPlanRepository {
    /// Helper: get operator_id and country_id by operator_name and country_name
    async fn get_operator_and_country_id(
        pool: &Pool,
        operator_name: &str,
        country_name: &str,
    ) -> Result<(i32, i32), AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
            SELECT o.operator_id, c.country_id
            FROM dim_operators o
            JOIN dim_countries c ON o.country_id = c.country_id
            WHERE o.operator_name = $1 AND c.country_name = $2
        "#;

        if let Some(row) = client
            .query_opt(stmt, &[&operator_name, &country_name])
            .await
            .map_err(AppError::Db)?
        {
            Ok((row.get("operator_id"), row.get("country_id")))
        } else {
            Err(AppError::BadRequest(format!(
                "Operator '{}' in country '{}' not found",
                operator_name, country_name
            )))
        }
    }

    /// Helper: get routage_type_id by name
    async fn get_routage_type_id(pool: &Pool, routage_type_name: &str) -> Result<i32, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
        SELECT routage_type_id
        FROM dim_routage_types
        WHERE routage_type_name = $1
    "#;

        if let Some(row) = client
            .query_opt(stmt, &[&routage_type_name])
            .await
            .map_err(AppError::Db)?
        {
            Ok(row.get("routage_type_id"))
        } else {
            Err(AppError::BadRequest(format!(
                "Routage type '{}' not found",
                routage_type_name
            )))
        }
    }

    /// Create a new SOR plan
    pub async fn create(pool: &Pool, new_plan: NewSorPlan) -> Result<SorPlan, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;

        // Resolve operator_id and country_id
        let (operator_id, country_id) = Self::get_operator_and_country_id(
            pool,
            &new_plan.operator_name,
            &new_plan.country_name,
        )
        .await?;

        // Resolve routage_type_id from name
        let routage_type_id = Self::get_routage_type_id(pool, &new_plan.routage_type_name).await?;

        let now = Utc::now().naive_utc();

        let stmt = r#"
        INSERT INTO sor_plan
            (operator_id, routage_type_id, barring, rate, created_by, created_at, is_current, version)
        VALUES ($1, $2, $3, $4, $5, $6, TRUE, 1)
        RETURNING *
    "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &operator_id,
                    &routage_type_id,
                    &new_plan.barring,
                    &new_plan.rate,
                    &new_plan.created_by,
                    &now,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(SorPlan {
            sor_plan_id: row.get("sor_plan_id"),
            country_id,
            country_name: new_plan.country_name,
            operator_id,
            operator_name: new_plan.operator_name,
            routage_type_id,
            routage_type_name: new_plan.routage_type_name,
            barring: row.get("barring"),
            rate: row.get("rate"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
            is_current: row.get("is_current"),
            version: row.get("version"),
        })
    }

    /// Get all SOR plans
    pub async fn get_all(pool: &Pool) -> Result<Vec<SorPlan>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
            SELECT sp.*, o.operator_name, c.country_id, c.country_name, rt.routage_type_name
            FROM sor_plan sp
            JOIN dim_operators o ON sp.operator_id = o.operator_id
            JOIN dim_countries c ON o.country_id = c.country_id
            JOIN dim_routage_types rt ON sp.routage_type_id = rt.routage_type_id
            WHERE sp.is_current IS TRUE
            ORDER BY sp.sor_plan_id DESC
        "#;

        let rows = client.query(stmt, &[]).await.map_err(AppError::Db)?;
        Ok(rows
            .into_iter()
            .map(|row| SorPlan {
                sor_plan_id: row.get("sor_plan_id"),
                country_id: row.get("country_id"),
                country_name: row.get("country_name"),
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                routage_type_id: row.get("routage_type_id"),
                routage_type_name: row.get("routage_type_name"),
                barring: row.get("barring"),
                rate: row.get("rate"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
                is_current: row.get("is_current"),
                version: row.get("version"),
            })
            .collect())
    }

    /// Get SOR plan by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<SorPlan>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
            SELECT sp.*, o.operator_name, c.country_name, c.country_id, rt.routage_type_name
            FROM sor_plan sp
            JOIN dim_operators o ON sp.operator_id = o.operator_id
            JOIN dim_countries c ON o.country_id = c.country_id
            JOIN dim_routage_types rt ON sp.routage_type_id = rt.routage_type_id
            WHERE sp.sor_plan_id = $1 AND sp.is_current IS TRUE
        "#;

        if let Some(row) = client.query_opt(stmt, &[&id]).await.map_err(AppError::Db)? {
            Ok(Some(SorPlan {
                sor_plan_id: row.get("sor_plan_id"),
                country_id: row.get("country_id"),
                country_name: row.get("country_name"),
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                routage_type_id: row.get("routage_type_id"),
                routage_type_name: row.get("routage_type_name"),
                barring: row.get("barring"),
                rate: row.get("rate"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
                is_current: row.get("is_current"),
                version: row.get("version"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update SOR plan with versioning
    pub async fn update(
        pool: &Pool,
        id: i32,
        data: UpdateSorPlan,
        routage_type_id: Option<i32>,
    ) -> Result<SorPlan, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let now = Utc::now().naive_utc();

        // 1️⃣ Fetch current row
        let current = Self::get_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("SOR plan {} not found", id)))?;

        // 2️⃣ Soft delete current row
        client
            .execute(
                "UPDATE sor_plan SET is_current = FALSE WHERE sor_plan_id = $1",
                &[&id],
            )
            .await
            .map_err(AppError::Db)?;

        // 3️⃣ Resolve operator_id / country_id if names changed
        let (operator_id, country_id) = if !data.operator_name.is_empty()
            && !data.country_name.is_empty()
        {
            Self::get_operator_and_country_id(pool, &data.operator_name, &data.country_name).await?
        } else {
            (current.operator_id, current.country_id)
        };

        // 4️⃣ Insert new row with incremented version
        let new_version = current.version + 1;
        let stmt = r#"
        INSERT INTO sor_plan
            (operator_id, routage_type_id, barring, rate, created_by, created_at, updated_by, updated_at, is_current, version)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, TRUE, $9)
        RETURNING *
    "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &operator_id,
                    &routage_type_id.unwrap_or(current.routage_type_id),
                    &data.barring.clone().or(current.barring.clone()),
                    &data.rate.clone().or(current.rate.clone()),
                    &current.created_by, // preserve original creator
                    &current.created_at, // preserve original creation date
                    &data.updated_by,    // new updater
                    &now,                // updated_at
                    &new_version,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(SorPlan {
            sor_plan_id: row.get("sor_plan_id"),
            country_id,
            country_name: if data.country_name.is_empty() {
                current.country_name
            } else {
                data.country_name
            },
            operator_id,
            operator_name: if data.operator_name.is_empty() {
                current.operator_name
            } else {
                data.operator_name
            },
            routage_type_id: row.get("routage_type_id"),
            routage_type_name: "".to_string(),
            barring: row.get("barring"),
            rate: row.get("rate"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
            is_current: row.get("is_current"),
            version: row.get("version"),
        })
    }

    /// Delete SOR plan (soft delete)
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "UPDATE sor_plan SET is_current = FALSE WHERE sor_plan_id = $1";
        let deleted = client.execute(stmt, &[&id]).await.map_err(AppError::Db)?;
        Ok(deleted)
    }
}
