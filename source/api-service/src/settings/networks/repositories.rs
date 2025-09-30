use crate::core::errors::AppError;
use crate::settings::networks::models::{Network, NewNetwork, UpdateNetwork};
use chrono::Utc;
use deadpool_postgres::Pool;

pub struct NetworkRepository;

impl NetworkRepository {
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

    /// Create a new network
    pub async fn create(pool: &Pool, new_net: NewNetwork) -> Result<Network, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let (operator_id, country_id) =
            Self::get_operator_and_country_id(pool, &new_net.operator_name, &new_net.country_name)
                .await?;
        let now = Utc::now().naive_utc();

        let stmt = r#"
            INSERT INTO dim_networks
                (plmn_code, plmn, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_by, created_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
            RETURNING network_id, plmn_code, plmn, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_at, created_by, updated_at, updated_by
        "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &new_net.plmn_code,
                    &new_net.plmn,
                    &new_net.mcc,
                    &new_net.mnc,
                    &operator_id,
                    &new_net.tech_2g,
                    &new_net.tech_3g,
                    &new_net.tech_lte,
                    &new_net.created_by,
                    &now,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(Network {
            network_id: row.get("network_id"),
            plmn_code: row.get("plmn_code"),
            plmn: row.get("plmn"),
            mcc: row.get("mcc"),
            mnc: row.get("mnc"),
            operator_id,
            operator_name: new_net.operator_name,
            country_id,
            country_name: new_net.country_name,
            tech_2g: row.get("tech_2g"),
            tech_3g: row.get("tech_3g"),
            tech_lte: row.get("tech_lte"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Get all networks
    pub async fn get_all(pool: &Pool) -> Result<Vec<Network>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
            SELECT n.*, o.operator_name, c.country_id, c.country_name
            FROM dim_networks n
            JOIN dim_operators o ON n.operator_id = o.operator_id
            JOIN dim_countries c ON o.country_id = c.country_id
            ORDER BY n.plmn_code
        "#;

        let rows = client.query(stmt, &[]).await.map_err(AppError::Db)?;

        Ok(rows
            .into_iter()
            .map(|row| Network {
                network_id: row.get("network_id"),
                plmn_code: row.get("plmn_code"),
                plmn: row.get("plmn"),
                mcc: row.get("mcc"),
                mnc: row.get("mnc"),
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                country_id: row.get("country_id"),
                country_name: row.get("country_name"),
                tech_2g: row.get("tech_2g"),
                tech_3g: row.get("tech_3g"),
                tech_lte: row.get("tech_lte"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            })
            .collect())
    }

    /// Get network by ID
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Option<Network>, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = r#"
            SELECT n.*, o.operator_name, c.country_name
            FROM dim_networks n
            JOIN dim_operators o ON n.operator_id = o.operator_id
            JOIN dim_countries c ON n.country_id = c.country_id
            WHERE n.network_id = $1
        "#;

        if let Some(row) = client.query_opt(stmt, &[&id]).await.map_err(AppError::Db)? {
            Ok(Some(Network {
                network_id: row.get("network_id"),
                plmn_code: row.get("plmn_code"),
                plmn: row.get("plmn"),
                mcc: row.get("mcc"),
                mnc: row.get("mnc"),
                operator_id: row.get("operator_id"),
                operator_name: row.get("operator_name"),
                country_id: row.get("country_id"),
                country_name: row.get("country_name"),
                tech_2g: row.get("tech_2g"),
                tech_3g: row.get("tech_3g"),
                tech_lte: row.get("tech_lte"),
                created_at: row.get("created_at"),
                created_by: row.get("created_by"),
                updated_at: row.get("updated_at"),
                updated_by: row.get("updated_by"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Update a network
    pub async fn update(pool: &Pool, id: i32, data: UpdateNetwork) -> Result<Network, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let now = Utc::now().naive_utc();

        let (operator_id, country_id, operator_name, country_name) =
            if let (Some(op_name), Some(ct_name)) =
                (data.operator_name.as_ref(), data.country_name.as_ref())
            {
                let (op_id, ct_id) =
                    Self::get_operator_and_country_id(pool, op_name, ct_name).await?;
                (
                    Some(op_id),
                    Some(ct_id),
                    Some(op_name.clone()),
                    Some(ct_name.clone()),
                )
            } else {
                (None, None, None, None)
            };

        let stmt = r#"
            UPDATE dim_networks SET
                plmn_code = COALESCE($1, plmn_code),
                plmn = COALESCE($2, plmn),
                mcc = COALESCE($3, mcc),
                mnc = COALESCE($4, mnc),
                operator_id = COALESCE($5, operator_id),
                country_id = COALESCE($6, country_id),
                tech_2g = COALESCE($7, tech_2g),
                tech_3g = COALESCE($8, tech_3g),
                tech_lte = COALESCE($9, tech_lte),
                updated_by = $10,
                updated_at = $11
            WHERE network_id = $12
            RETURNING network_id, plmn_code, plmn, mcc, mnc, operator_id, tech_2g, tech_3g, tech_lte, created_at, created_by, updated_at, updated_by
        "#;

        let row = client
            .query_one(
                stmt,
                &[
                    &data.plmn_code,
                    &data.plmn,
                    &data.mcc,
                    &data.mnc,
                    &operator_id,
                    &country_id,
                    &data.tech_2g,
                    &data.tech_3g,
                    &data.tech_lte,
                    &data.updated_by,
                    &now,
                    &id,
                ],
            )
            .await
            .map_err(AppError::Db)?;

        Ok(Network {
            network_id: row.get("network_id"),
            plmn_code: row.get("plmn_code"),
            plmn: row.get("plmn"),
            mcc: row.get("mcc"),
            mnc: row.get("mnc"),
            operator_id: row.get("operator_id"),
            operator_name: operator_name.unwrap_or_default(),
            country_id: row.get("country_id"),
            country_name: country_name.unwrap_or_default(),
            tech_2g: row.get("tech_2g"),
            tech_3g: row.get("tech_3g"),
            tech_lte: row.get("tech_lte"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        })
    }

    /// Delete a network
    pub async fn delete(pool: &Pool, id: i32) -> Result<u64, AppError> {
        let client = pool.get().await.map_err(AppError::Pool)?;
        let stmt = "DELETE FROM dim_networks WHERE network_id = $1";
        let deleted = client.execute(stmt, &[&id]).await.map_err(AppError::Db)?;
        Ok(deleted)
    }
}
