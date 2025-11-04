use crate::core::errors::AppError;
use crate::settings::models::{
    Country, CreateCountry, CreateNetwork, CreateOperator, CreatePrefix, CreateSorPlan, Network,
    Operator, Prefix, SorPlan, UpdateCountry, UpdateNetwork, UpdateOperator, UpdatePrefix,
    UpdateSorPlan,
};
use sqlx::PgPool;
use sqlx::Row;

pub struct CountryRepository;
pub struct OperatorRepository;
pub struct NetworkRepository;
pub struct SorPlanRepository;
pub struct PrefixRepository;

// =========================
// Country Repository
// =========================

impl CountryRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Country>, AppError> {
        let countries = sqlx::query_as::<_, Country>(
            r#"
            SELECT country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            FROM cfg_countries
            ORDER BY country_name
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(countries)
    }

    pub async fn get_by_id(pool: &PgPool, country_id: i32) -> Result<Option<Country>, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            SELECT country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            FROM cfg_countries
            WHERE country_id = $1
            "#
        )
        .bind(country_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn create(pool: &PgPool, data: CreateCountry) -> Result<Country, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            INSERT INTO cfg_countries (iso_code, country_name, created_by, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            "#
        )
        .bind(&data.iso_code)
        .bind(&data.country_name)
        .bind(&data.created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn update(
        pool: &PgPool,
        country_id: i32,
        data: UpdateCountry,
    ) -> Result<Option<Country>, AppError> {
        let country = sqlx::query_as::<_, Country>(
            r#"
            UPDATE cfg_countries
            SET 
                iso_code = COALESCE($1, iso_code),
                country_name = COALESCE($2, country_name),
                is_valid = COALESCE($3, is_valid),
                updated_by = $4,
                updated_at = NOW()
            WHERE country_id = $5
            RETURNING country_id, iso_code, country_name, is_valid, created_at, created_by, updated_at, updated_by
            "#
        )
        .bind(&data.iso_code)
        .bind(&data.country_name)
        .bind(data.is_valid)
        .bind(&data.updated_by)
        .bind(country_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(country)
    }

    pub async fn delete(pool: &PgPool, country_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE cfg_countries
            SET is_valid = FALSE, updated_at = NOW()
            WHERE country_id = $1
            "#,
        )
        .bind(country_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(result.rows_affected())
    }
}

// =========================
// Operator Repository
// =========================

impl OperatorRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Operator>, AppError> {
        let operators = sqlx::query_as::<_, Operator>(
            r#"
            SELECT 
                o.operator_id,
                o.operator_name,
                o.brand_name,
                o.country_id,
                c.country_name,
                o.is_valid,
                o.created_at,
                o.created_by,
                o.updated_at,
                o.updated_by
            FROM cfg_operators o
            JOIN cfg_countries c ON o.country_id = c.country_id
            ORDER BY o.operator_name
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(operators)
    }

    pub async fn get_by_id(pool: &PgPool, operator_id: i32) -> Result<Option<Operator>, AppError> {
        let operator = sqlx::query_as::<_, Operator>(
            r#"
            SELECT 
                o.operator_id,
                o.operator_name,
                o.brand_name,
                o.country_id,
                c.country_name,
                o.is_valid,
                o.created_at,
                o.created_by,
                o.updated_at,
                o.updated_by
            FROM cfg_operators o
            JOIN cfg_countries c ON o.country_id = c.country_id
            WHERE o.operator_id = $1
            "#,
        )
        .bind(operator_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(operator)
    }

    // NEW: Get operators by country ID
    pub async fn get_by_country_id(
        pool: &PgPool,
        country_id: i32,
    ) -> Result<Vec<Operator>, AppError> {
        let operators = sqlx::query_as::<_, Operator>(
            r#"
            SELECT 
                o.operator_id,
                o.operator_name,
                o.brand_name,
                o.country_id,
                c.country_name,
                o.is_valid,
                o.created_at,
                o.created_by,
                o.updated_at,
                o.updated_by
            FROM cfg_operators o
            JOIN cfg_countries c ON o.country_id = c.country_id
            WHERE o.country_id = $1
            ORDER BY o.operator_name
            "#,
        )
        .bind(country_id)
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(operators)
    }

    pub async fn create(pool: &PgPool, data: CreateOperator) -> Result<Operator, AppError> {
        let operator = sqlx::query_as::<_, Operator>(
            r#"
            INSERT INTO cfg_operators (operator_name, brand_name, country_id, created_by, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            RETURNING 
                operator_id,
                operator_name,
                brand_name,
                country_id,
                (SELECT country_name FROM cfg_countries WHERE country_id = cfg_operators.country_id) AS country_name,
                is_valid,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(&data.operator_name)
        .bind(&data.brand_name)
        .bind(data.country_id)
        .bind(&data.created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(operator)
    }

    pub async fn update(
        pool: &PgPool,
        operator_id: i32,
        data: UpdateOperator,
    ) -> Result<Option<Operator>, AppError> {
        let operator = sqlx::query_as::<_, Operator>(
            r#"
            UPDATE cfg_operators
            SET 
                operator_name = COALESCE($1, operator_name),
                brand_name = COALESCE($2, brand_name),
                country_id = COALESCE($3, country_id),
                is_valid = COALESCE($4, is_valid),
                updated_by = $5,
                updated_at = NOW()
            WHERE operator_id = $6
            RETURNING 
                operator_id,
                operator_name,
                brand_name,
                country_id,
                (SELECT country_name FROM cfg_countries WHERE country_id = cfg_operators.country_id) AS country_name,
                is_valid,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(&data.operator_name)
        .bind(&data.brand_name)
        .bind(data.country_id)
        .bind(data.is_valid)
        .bind(&data.updated_by)
        .bind(operator_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(operator)
    }

    pub async fn delete(pool: &PgPool, operator_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE cfg_operators
            SET is_valid = FALSE, updated_at = NOW()
            WHERE operator_id = $1
            "#,
        )
        .bind(operator_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(result.rows_affected())
    }
}

// =========================
// Network Repository
// =========================

impl NetworkRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Network>, AppError> {
        let networks = sqlx::query_as::<_, Network>(
            r#"
            SELECT 
                n.network_id,
                n.plmn_code,
                n.plmn,
                n.mcc,
                n.mnc,
                n.operator_id,
                o.operator_name,
                c.country_name,
                n.tech_2g,
                n.tech_3g,
                n.tech_lte,
                n.is_valid,
                n.created_at,
                n.created_by,
                n.updated_at,
                n.updated_by
            FROM cfg_networks n
            JOIN cfg_operators o ON n.operator_id = o.operator_id
            JOIN cfg_countries c ON o.country_id = c.country_id
            ORDER BY n.plmn_code
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(networks)
    }

    pub async fn get_by_id(pool: &PgPool, network_id: i32) -> Result<Option<Network>, AppError> {
        let network = sqlx::query_as::<_, Network>(
            r#"
            SELECT 
                n.network_id,
                n.plmn_code,
                n.plmn,
                n.mcc,
                n.mnc,
                n.operator_id,
                o.operator_name,
                c.country_name,
                n.tech_2g,
                n.tech_3g,
                n.tech_lte,
                n.is_valid,
                n.created_at,
                n.created_by,
                n.updated_at,
                n.updated_by
            FROM cfg_networks n
            JOIN cfg_operators o ON n.operator_id = o.operator_id
            JOIN cfg_countries c ON o.country_id = c.country_id
            WHERE n.network_id = $1
            "#,
        )
        .bind(network_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(network)
    }

    pub async fn create(pool: &PgPool, data: CreateNetwork) -> Result<Network, AppError> {
        let network = sqlx::query_as::<_, Network>(
            r#"
            INSERT INTO cfg_networks (
                plmn_code, plmn, mcc, mnc, operator_id, 
                tech_2g, tech_3g, tech_lte, created_by, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())
            RETURNING 
                network_id,
                plmn_code,
                plmn,
                mcc,
                mnc,
                operator_id,
                (SELECT operator_name FROM cfg_operators WHERE operator_id = cfg_networks.operator_id) AS operator_name,
                (SELECT c.country_name FROM cfg_countries c 
                    JOIN cfg_operators o ON c.country_id = o.country_id
                    WHERE o.operator_id = cfg_networks.operator_id) AS country_name,
                tech_2g,
                tech_3g,
                tech_lte,
                is_valid,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(&data.plmn_code)
        .bind(&data.plmn)
        .bind(&data.mcc)
        .bind(&data.mnc)
        .bind(data.operator_id)
        .bind(data.tech_2g)
        .bind(data.tech_3g)
        .bind(data.tech_lte)
        .bind(&data.created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(network)
    }

    pub async fn update(
        pool: &PgPool,
        network_id: i32,
        data: UpdateNetwork,
    ) -> Result<Option<Network>, AppError> {
        let network = sqlx::query_as::<_, Network>(
            r#"
            UPDATE cfg_networks
            SET 
                plmn_code = COALESCE($1, plmn_code),
                plmn = COALESCE($2, plmn),
                mcc = COALESCE($3, mcc),
                mnc = COALESCE($4, mnc),
                operator_id = COALESCE($5, operator_id),
                tech_2g = COALESCE($6, tech_2g),
                tech_3g = COALESCE($7, tech_3g),
                tech_lte = COALESCE($8, tech_lte),
                is_valid = COALESCE($9, is_valid),
                updated_by = $10,
                updated_at = NOW()
            WHERE network_id = $11
            RETURNING 
                network_id,
                plmn_code,
                plmn,
                mcc,
                mnc,
                operator_id,
                (SELECT operator_name FROM cfg_operators WHERE operator_id = cfg_networks.operator_id) AS operator_name,
                (SELECT c.country_name FROM cfg_countries c 
                    JOIN cfg_operators o ON c.country_id = o.country_id
                    WHERE o.operator_id = cfg_networks.operator_id) AS country_name,
                tech_2g,
                tech_3g,
                tech_lte,
                is_valid,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(&data.plmn_code)
        .bind(&data.plmn)
        .bind(&data.mcc)
        .bind(&data.mnc)
        .bind(data.operator_id)
        .bind(data.tech_2g)
        .bind(data.tech_3g)
        .bind(data.tech_lte)
        .bind(data.is_valid)
        .bind(&data.updated_by)
        .bind(network_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(network)
    }

    pub async fn delete(pool: &PgPool, network_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE cfg_networks
            SET is_valid = FALSE, updated_at = NOW()
            WHERE network_id = $1
            "#,
        )
        .bind(network_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(result.rows_affected())
    }
}

impl SorPlanRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<SorPlan>, AppError> {
        let plans = sqlx::query_as::<_, SorPlan>(
            r#"
            SELECT 
                s.sor_plan_id,
                s.operator_id,
                o.operator_name,
                c.country_name,
                s.routage_type_id,
                r.routage_type_name,
                s.barring,
                s.rate,
                s.is_current,
                s.created_at,
                s.created_by,
                s.updated_at,
                s.updated_by
            FROM cfg_sor_plan s
            JOIN cfg_operators o ON s.operator_id = o.operator_id
            JOIN cfg_countries c ON o.country_id = c.country_id
            LEFT JOIN ref_routage_types r ON s.routage_type_id = r.routage_type_id
            ORDER BY s.sor_plan_id
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(plans)
    }

    pub async fn get_by_id(pool: &PgPool, sor_plan_id: i32) -> Result<Option<SorPlan>, AppError> {
        let plan = sqlx::query_as::<_, SorPlan>(
            r#"
            SELECT 
                s.sor_plan_id,
                s.operator_id,
                o.operator_name,
                c.country_name,
                s.routage_type_id,
                r.routage_type_name,
                s.barring,
                s.rate,
                s.is_current,
                s.created_at,
                s.created_by,
                s.updated_at,
                s.updated_by
            FROM cfg_sor_plan s
            JOIN cfg_operators o ON s.operator_id = o.operator_id
            JOIN cfg_countries c ON o.country_id = c.country_id
            LEFT JOIN ref_routage_types r ON s.routage_type_id = r.routage_type_id
            WHERE s.sor_plan_id = $1
            "#,
        )
        .bind(sor_plan_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(plan)
    }

    pub async fn create(pool: &PgPool, data: CreateSorPlan) -> Result<SorPlan, AppError> {
        let plan = sqlx::query_as::<_, SorPlan>(
            r#"
            INSERT INTO cfg_sor_plan (operator_id, routage_type_id, barring, rate, created_by, is_current, created_at)
            VALUES ($1, $2, COALESCE($3, FALSE), $4, $5, COALESCE($6, TRUE), NOW())
            RETURNING 
                sor_plan_id,
                operator_id,
                (SELECT operator_name FROM cfg_operators WHERE operator_id = cfg_sor_plan.operator_id) AS operator_name,
                (SELECT country_name FROM cfg_countries WHERE country_id = (SELECT country_id FROM cfg_operators WHERE operator_id = cfg_sor_plan.operator_id)) AS country_name,
                routage_type_id,
                (SELECT routage_type_name FROM ref_routage_types WHERE routage_type_id = cfg_sor_plan.routage_type_id) AS routage_type_name,
                barring,
                rate,
                is_current,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(data.operator_id)
        .bind(data.routage_type_id)
        .bind(data.barring)
        .bind(data.rate)
        .bind(data.created_by)
        .bind(data.is_current)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(plan)
    }

    pub async fn update(
        pool: &PgPool,
        sor_plan_id: i32,
        data: UpdateSorPlan,
    ) -> Result<Option<SorPlan>, AppError> {
        let plan = sqlx::query_as::<_, SorPlan>(
            r#"
            UPDATE cfg_sor_plan
            SET 
                operator_id = COALESCE($1, operator_id),
                routage_type_id = $2,
                barring = COALESCE($3, barring),
                rate = COALESCE($4, rate),
                is_current = COALESCE($5, is_current),
                updated_by = $6,
                updated_at = NOW()
            WHERE sor_plan_id = $7
            RETURNING 
                sor_plan_id,
                operator_id,
                (SELECT operator_name FROM cfg_operators WHERE operator_id = cfg_sor_plan.operator_id) AS operator_name,
                (SELECT country_name FROM cfg_countries WHERE country_id = (SELECT country_id FROM cfg_operators WHERE operator_id = cfg_sor_plan.operator_id)) AS country_name,
                routage_type_id,
                (SELECT routage_type_name FROM ref_routage_types WHERE routage_type_id = cfg_sor_plan.routage_type_id) AS routage_type_name,
                barring,
                rate,
                is_current,
                created_at,
                created_by,
                updated_at,
                updated_by
            "#
        )
        .bind(data.operator_id)
        .bind(data.routage_type_id)
        .bind(data.barring)
        .bind(data.rate)
        .bind(data.is_current)
        .bind(&data.updated_by)
        .bind(sor_plan_id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(plan)
    }

    pub async fn delete(pool: &PgPool, sor_plan_id: i32) -> Result<u64, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM cfg_sor_plan
            WHERE sor_plan_id = $1
            "#,
        )
        .bind(sor_plan_id)
        .execute(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(result.rows_affected())
    }
}

impl PrefixRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Prefix>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                p.prefix_id, p.country_id, p.operator_id, p.prefix, p.is_valid, 
                p.created_at, p.created_by, p.updated_at, p.updated_by,
                c.country_name, o.operator_name
            FROM cfg_prefixes p
            LEFT JOIN cfg_countries c ON p.country_id = c.country_id
            LEFT JOIN cfg_operators o ON p.operator_id = o.operator_id
            ORDER BY p.prefix_id
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::Sqlx)?;

        let prefixes = rows
            .into_iter()
            .map(|r| Prefix {
                prefix_id: r.get("prefix_id"),
                country_id: r.get("country_id"),
                operator_id: r.get("operator_id"),
                prefix: r.get("prefix"),
                is_valid: r.get("is_valid"),
                created_at: r.get("created_at"),
                created_by: r.get("created_by"),
                updated_at: r.get("updated_at"),
                updated_by: r.get("updated_by"),
                country_name: r.get("country_name"),
                operator_name: r.get("operator_name"),
            })
            .collect();

        Ok(prefixes)
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Prefix, AppError> {
        let row = sqlx::query(
            r#"
            SELECT 
                p.prefix_id, p.country_id, p.operator_id, p.prefix, p.is_valid, 
                p.created_at, p.created_by, p.updated_at, p.updated_by,
                c.country_name, o.operator_name
            FROM cfg_prefixes p
            LEFT JOIN cfg_countries c ON p.country_id = c.country_id
            LEFT JOIN cfg_operators o ON p.operator_id = o.operator_id
            WHERE p.prefix_id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(Prefix {
            prefix_id: row.get("prefix_id"),
            country_id: row.get("country_id"),
            operator_id: row.get("operator_id"),
            prefix: row.get("prefix"),
            is_valid: row.get("is_valid"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
            country_name: row.get("country_name"),
            operator_name: row.get("operator_name"),
        })
    }

    pub async fn create(pool: &PgPool, data: CreatePrefix) -> Result<Prefix, AppError> {
        let row = sqlx::query(
            r#"
            INSERT INTO cfg_prefixes (country_id, operator_id, prefix, is_valid, created_by)
            VALUES ($1, $2, $3, COALESCE($4, true), $5)
            RETURNING prefix_id, country_id, operator_id, prefix, is_valid, created_at, created_by, updated_at, updated_by,
                      NULL as country_name, NULL as operator_name
            "#
        )
        .bind(data.country_id)
        .bind(data.operator_id)
        .bind(data.prefix)
        .bind(data.is_valid)
        .bind(data.created_by)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(Prefix {
            prefix_id: row.get("prefix_id"),
            country_id: row.get("country_id"),
            operator_id: row.get("operator_id"),
            prefix: row.get("prefix"),
            is_valid: row.get("is_valid"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
            country_name: None,
            operator_name: None,
        })
    }

    pub async fn update(pool: &PgPool, id: i32, data: UpdatePrefix) -> Result<Prefix, AppError> {
        let row = sqlx::query(
            r#"
            UPDATE cfg_prefixes
            SET
                country_id = COALESCE($1, country_id),
                operator_id = COALESCE($2, operator_id),
                prefix = COALESCE($3, prefix),
                is_valid = COALESCE($4, is_valid),
                updated_by = $5,
                updated_at = NOW()
            WHERE prefix_id = $6
            RETURNING prefix_id, country_id, operator_id, prefix, is_valid, created_at, created_by, updated_at, updated_by,
                      NULL as country_name, NULL as operator_name
            "#
        )
        .bind(data.country_id)
        .bind(data.operator_id)
        .bind(data.prefix)
        .bind(data.is_valid)
        .bind(data.updated_by)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(AppError::Sqlx)?;

        Ok(Prefix {
            prefix_id: row.get("prefix_id"),
            country_id: row.get("country_id"),
            operator_id: row.get("operator_id"),
            prefix: row.get("prefix"),
            is_valid: row.get("is_valid"),
            created_at: row.get("created_at"),
            created_by: row.get("created_by"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
            country_name: None,
            operator_name: None,
        })
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, AppError> {
        let res = sqlx::query("DELETE FROM cfg_prefixes WHERE prefix_id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(AppError::Sqlx)?;

        Ok(res.rows_affected())
    }
}
