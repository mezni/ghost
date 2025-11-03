use crate::analytics::models::{Filter, ValidatedMetricsRequest};
use crate::core::errors::AppError;
use serde_json::json;
use sqlx::{PgPool, Row};

pub struct MetricsRepository;

impl MetricsRepository {
    const GET_GLOBAL_METRICS_QUERY: &str = r#"
SELECT 
    date_str AS date, 
    value
FROM (
    SELECT 
        rd.date_str, 
        fct.value,
        ROW_NUMBER() OVER (PARTITION BY fct.date_id ORDER BY fct.batch_id DESC) as rn
    FROM 
        trx_metrics_global fct
    JOIN 
        ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
    JOIN 
        ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN 
        ref_dates rd ON rd.date_id = fct.date_id
    WHERE 
        UPPER(rrd.direction) = UPPER($1)
        -- DATE_FILTER_PLACEHOLDER --
) t
WHERE rn = 1
ORDER BY 
    date_str
    "#;

    const GET_COUNTRY_METRICS_QUERY: &str = r#"
SELECT 
    date_str AS date, 
    country_name AS country,
    value
FROM (
    SELECT 
        rd.date_str, 
        cc.country_name,
        fct.value,
        ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.country_id ORDER BY fct.batch_id DESC) as rn
    FROM 
        trx_metrics_country fct
    JOIN 
        ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
    JOIN 
        ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN 
        ref_dates rd ON rd.date_id = fct.date_id
    JOIN 
        cfg_countries cc ON cc.country_id = fct.country_id
    WHERE 
        UPPER(rrd.direction) = UPPER($1)
        -- COUNTRY_FILTER_PLACEHOLDER --
        -- DATE_FILTER_PLACEHOLDER --
) t
WHERE rn = 1
ORDER BY 
    date_str
    "#;

    const GET_OPERATOR_METRICS_QUERY: &str = r#"
SELECT 
    date_str AS date, 
    operator_name AS operator,
    value
FROM (
    SELECT 
        rd.date_str, 
        co.operator_name,
        fct.value,
        ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.operator_id ORDER BY fct.batch_id DESC) as rn
    FROM 
        trx_metrics_operator fct
    JOIN 
        ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
    JOIN 
        ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN 
        ref_dates rd ON rd.date_id = fct.date_id
    JOIN 
        cfg_operators co ON co.operator_id = fct.operator_id
    WHERE 
        UPPER(rrd.direction) = UPPER($1)
        -- OPERATOR_FILTER_PLACEHOLDER --
        -- COUNTRY_FILTER_PLACEHOLDER --
        -- DATE_FILTER_PLACEHOLDER --
) t
WHERE rn = 1
ORDER BY 
    date_str
    "#;

    const GET_SUBSCRIBER_METRICS_QUERY: &str = r#"
SELECT 
    date_str AS date, 
    imsi AS imsi,
    msisdn AS msisdn,
    value
FROM (
    SELECT 
        rd.date_str, 
        cs.imsi,
        cs.msisdn,
        fct.value,
        ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.subscriber_id ORDER BY fct.batch_id DESC) as rn
    FROM 
        trx_metrics_subscriber fct
    JOIN 
        ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
    JOIN 
        ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN 
        ref_dates rd ON rd.date_id = fct.date_id
    JOIN 
        cfg_subscribers cs ON cs.subscriber_id = fct.subscriber_id
    JOIN 
        cfg_operators co ON co.operator_id = fct.operator_id
    JOIN 
        cfg_countries cc ON cc.country_id = fct.country_id
    WHERE 
        UPPER(rrd.direction) = UPPER($1)
        -- SUBSCRIBER_FILTER_PLACEHOLDER -- -- COUNTRY_FILTER_PLACEHOLDER -- -- OPERATOR_FILTER_PLACEHOLDER -- -- DATE_FILTER_PLACEHOLDER --
) t
WHERE rn = 1
ORDER BY 
    date_str
    "#;

    const GET_COUNTRY_METRICS_TOP_QUERY: &str = r#"
SELECT date, country, SUM(value)::bigint AS value
FROM (
    SELECT 
        date_str AS date,
        country_name AS country,
        value
    FROM (
        SELECT 
            rd.date_str, 
            cc.country_name,
            fct.value,
            ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.country_id ORDER BY fct.batch_id DESC) as rn
        FROM 
            trx_metrics_country fct
        JOIN 
            ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN 
            ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN 
            ref_dates rd ON rd.date_id = fct.date_id
        JOIN 
            cfg_countries cc ON cc.country_id = fct.country_id
        WHERE 
            UPPER(rrd.direction) = UPPER($1)
            AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)
    ) t
    WHERE rn = 1
) ranked
GROUP BY date, country
ORDER BY value DESC
LIMIT $2
    "#;

    const GET_OPERATOR_METRICS_TOP_QUERY: &str = r#"
SELECT date, operator, SUM(value)::bigint AS value
FROM (
    SELECT 
        date_str AS date,
        operator_name AS operator,
        value
    FROM (
        SELECT 
            rd.date_str, 
            co.operator_name,
            fct.value,
            ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.operator_id ORDER BY fct.batch_id DESC) as rn
        FROM 
            trx_metrics_operator fct
        JOIN 
            ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN 
            ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN 
            ref_dates rd ON rd.date_id = fct.date_id
        JOIN 
            cfg_operators co ON co.operator_id = fct.operator_id
        JOIN 
            cfg_countries cc ON cc.country_id = co.country_id
        WHERE 
            UPPER(rrd.direction) = UPPER($1)
            AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_operator)
            -- COUNTRY_FILTER_PLACEHOLDER --
    ) t
    WHERE rn = 1
) ranked
GROUP BY date, operator
ORDER BY value DESC
LIMIT $2
    "#;

    const GET_SUBSCRIBER_METRICS_TOP_QUERY: &str = r#"
SELECT date, imsi, msisdn, value
FROM (
    SELECT 
        date_str AS date,
        imsi AS imsi,
        msisdn AS msisdn,
        value
    FROM (
        SELECT 
            rd.date_str, 
            cs.imsi,
            cs.msisdn,
            fct.value,
            ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.subscriber_id ORDER BY fct.batch_id DESC) as rn
        FROM 
            trx_metrics_subscriber fct
        JOIN 
            ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN 
            ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN 
            ref_dates rd ON rd.date_id = fct.date_id
        JOIN 
            cfg_subscribers cs ON cs.subscriber_id = fct.subscriber_id
        JOIN 
            cfg_operators co ON co.operator_id = fct.operator_id
        JOIN 
            cfg_countries cc ON cc.country_id = fct.country_id
        WHERE 
            UPPER(rrd.direction) = UPPER($1)
            AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_subscriber)
            -- COUNTRY_FILTER_PLACEHOLDER --
            -- OPERATOR_FILTER_PLACEHOLDER --
    ) t
    WHERE rn = 1
) ranked
ORDER BY value DESC
LIMIT $2
    "#;

    const GET_SOR_PERFORMANCE_QUERY: &str = r#"
SELECT 
    tpo.perf_id,
    tpo.batch_id,
    rd.date_str AS date,
    cc.country_name AS country,
    co.operator_name AS operator,
    tpo.country_count::INT AS country_count,
    tpo.operator_count::INT AS operator_count,
    COALESCE(tpo.target_percentage, 0) AS target_percentage,
    COALESCE(tpo.actual_percentage, 0) AS actual_percentage,
    COALESCE(tpo.is_outside_tolerance, false) AS is_outside_tolerance
FROM 
    trx_perf_out tpo
JOIN 
    batch_execs be ON tpo.batch_id = be.batch_id
JOIN 
    ref_dates rd ON tpo.date_id = rd.date_id
LEFT JOIN 
    cfg_countries cc ON tpo.country_id = cc.country_id
LEFT JOIN 
    cfg_operators co ON tpo.operator_id = co.operator_id
WHERE 
    1=1
    -- DATE_FILTER_PLACEHOLDER --
    -- COUNTRY_FILTER_PLACEHOLDER --
    -- OPERATOR_FILTER_PLACEHOLDER --
ORDER BY 
    rd.date_str DESC
    "#;

    const GET_NOTIF_DET_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, fct.message AS value
        FROM trx_notifications fct
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        WHERE fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)
    "#;

    const GET_NOTIF_SUM_METRICS_QUERY: &str = r#"
        SELECT rd.date_str AS date, COUNT(*)::text AS value
        FROM trx_notifications fct
        JOIN ref_dates rd ON rd.date_id = fct.date_id
        WHERE fct.date_id = (SELECT MAX(date_id) FROM trx_notifications)
        GROUP BY rd.date_str
    "#;

    // Main dispatcher
    pub async fn get_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        // Debug: Print the incoming request
        println!("Received metrics request: {:?}", req);

        match req.dimension.to_lowercase().as_str() {
            "global" => Self::get_global_metrics(pool, req).await,
            "country" => Self::get_country_metrics(pool, req).await,
            "operator" => Self::get_operator_metrics(pool, req).await,
            "subscriber" => Self::get_subscriber_metrics(pool, req).await,
            "sor_performance" => Self::get_sor_performance_metrics(pool, req).await,
            "notification" => Self::get_notif_metrics(pool, req).await,
            _ => Err(AppError::BadRequest("Invalid dimension".to_string())),
        }
    }

    // ------------------- Global -------------------
    async fn get_global_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing global metrics request: {:?}", req);

        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;

        let date_filter = match aggregation.as_str() {
            "latest" => "AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_global)",
            "history" => &format!("AND rd.date >= CURRENT_DATE - INTERVAL '{} days'", size),
            _ => {
                return Err(AppError::BadRequest(
                    "Aggregation 'latest' or 'history' is required".to_string(),
                ));
            }
        };

        let query =
            Self::GET_GLOBAL_METRICS_QUERY.replace("-- DATE_FILTER_PLACEHOLDER --", date_filter);
        println!("Executing global query: {}", query);

        let rows = sqlx::query(&query)
            .bind(direction)
            .fetch_all(pool)
            .await
            .map_err(AppError::Sqlx)?;

        let mut metrics = Vec::new();
        for row in rows {
            let date: String = row.try_get("date")?;
            let value: i64 = row.try_get("value")?;
            metrics.push(json!({ "date": date, "value": value }));
        }

        println!("Global metrics result: {} records", metrics.len());
        Ok(json!({ "data": metrics, "status": "success" }))
    }

    // ------------------- Country -------------------
    async fn get_country_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing country metrics request: {:?}", req);

        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let country = get_country_from_filters(req.filter.as_ref());

        match aggregation.as_str() {
            "latest" | "history" => {
                let mut query = Self::GET_COUNTRY_METRICS_QUERY.to_string();

                // Replace country filter placeholder
                let country_filter = if !country.is_empty() {
                    " AND UPPER(cc.country_name) = UPPER($2)"
                } else {
                    ""
                };
                query = query.replace("-- COUNTRY_FILTER_PLACEHOLDER --", country_filter);

                // Replace date filter placeholder
                let date_filter = match aggregation.as_str() {
                    "latest" => " AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)",
                    "history" => {
                        &format!(" AND rd.date >= CURRENT_DATE - INTERVAL '{} days'", size)
                    }
                    _ => unreachable!(),
                };
                query = query.replace("-- DATE_FILTER_PLACEHOLDER --", date_filter);

                println!("Executing country query: {}", query);

                let mut q = sqlx::query(&query).bind(direction.clone());
                if !country.is_empty() {
                    q = q.bind(country);
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let country: String = row.try_get("country")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "country": country, "value": value }));
                }

                println!("Country metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            "top" => {
                println!("Executing country top query with size: {}", size);
                let rows = sqlx::query(Self::GET_COUNTRY_METRICS_TOP_QUERY)
                    .bind(direction)
                    .bind(size)
                    .fetch_all(pool)
                    .await
                    .map_err(AppError::Sqlx)?;

                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let country: String = row.try_get("country")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "country": country, "value": value }));
                }

                println!("Country top metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            _ => Err(AppError::BadRequest(
                "Aggregation 'latest', 'history' or 'top' is required".to_string(),
            )),
        }
    }

    // ------------------- Operator -------------------
    async fn get_operator_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing operator metrics request: {:?}", req);

        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let operator = get_operator_from_filters(req.filter.as_ref());
        let country = get_country_from_filters(req.filter.as_ref());

        match aggregation.as_str() {
            "latest" | "history" => {
                let mut query = Self::GET_OPERATOR_METRICS_QUERY.to_string();

                // Replace operator filter placeholder
                let operator_filter = if !operator.is_empty() {
                    " AND UPPER(co.operator_name) = UPPER($2)"
                } else {
                    ""
                };
                query = query.replace("-- OPERATOR_FILTER_PLACEHOLDER --", operator_filter);

                // Replace country filter placeholder (for operator queries)
                let country_filter = if !country.is_empty() {
                    " AND UPPER(cc.country_name) = UPPER($3)"
                } else {
                    ""
                };
                query = query.replace("-- COUNTRY_FILTER_PLACEHOLDER --", country_filter);

                // Replace date filter placeholder
                let date_filter = match aggregation.as_str() {
                    "latest" => {
                        " AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_operator)"
                    }
                    "history" => {
                        &format!(" AND rd.date >= CURRENT_DATE - INTERVAL '{} days'", size)
                    }
                    _ => unreachable!(),
                };
                query = query.replace("-- DATE_FILTER_PLACEHOLDER --", date_filter);

                // Add JOIN for country if country filter is applied
                if !country.is_empty() {
                    query = query.replace(
                        "JOIN cfg_operators co ON co.operator_id = fct.operator_id",
                        "JOIN cfg_operators co ON co.operator_id = fct.operator_id\n        JOIN cfg_countries cc ON cc.country_id = co.country_id"
                    );
                }

                println!("Executing operator query: {}", query);

                let mut q = sqlx::query(&query).bind(direction.clone());
                if !operator.is_empty() {
                    q = q.bind(operator.clone());
                }
                if !country.is_empty() {
                    q = q.bind(country);
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let operator: String = row.try_get("operator")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "operator": operator, "value": value }));
                }

                println!("Operator metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            "top" => {
                println!("Executing operator top query with size: {}", size);

                let mut query = Self::GET_OPERATOR_METRICS_TOP_QUERY.to_string();

                // Replace country filter placeholder for top query
                let country_filter = if !country.is_empty() {
                    " AND UPPER(cc.country_name) = UPPER($3)"
                } else {
                    ""
                };
                query = query.replace("-- COUNTRY_FILTER_PLACEHOLDER --", country_filter);

                println!("Executing operator top query: {}", query);

                let mut q = sqlx::query(&query).bind(direction.clone()).bind(size);
                if !country.is_empty() {
                    q = q.bind(country);
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let operator: String = row.try_get("operator")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({ "date": date, "operator": operator, "value": value }));
                }

                println!("Operator top metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            _ => Err(AppError::BadRequest(
                "Aggregation 'latest', 'history' or 'top' is required".to_string(),
            )),
        }
    }

    // ------------------- Subscriber -------------------
    async fn get_subscriber_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing subscriber metrics request: {:?}", req);

        let direction = get_direction_from_filters(req.filter.as_ref())?;
        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let subscriber = get_subscriber_from_filters(req.filter.as_ref());
        let country = get_country_from_filters(req.filter.as_ref());
        let operator = get_operator_from_filters(req.filter.as_ref());

        match aggregation.as_str() {
            "latest" | "history" => {
                // Build WHERE clause dynamically based on which filters are present
                let mut where_parts = Vec::new();
                let mut params: Vec<String> = Vec::new();

                // Always add direction as first parameter
                params.push(direction.clone());
                where_parts.push("UPPER(rrd.direction) = UPPER($1)".to_string());

                // Track parameter index
                let mut param_index = 2;

                // Add subscriber filter if present
                if !subscriber.is_empty() {
                    where_parts.push(format!("UPPER(cs.imsi) = UPPER(${})", param_index));
                    params.push(subscriber.clone());
                    param_index += 1;
                }

                // Add country filter if present
                if !country.is_empty() {
                    where_parts.push(format!("UPPER(cc.country_name) = UPPER(${})", param_index));
                    params.push(country.clone());
                    param_index += 1;
                }

                // Add operator filter if present
                if !operator.is_empty() {
                    where_parts.push(format!("UPPER(co.operator_name) = UPPER(${})", param_index));
                    params.push(operator.clone());
                    param_index += 1;
                }

                // Add date filter (no parameter needed for this one)
                let date_filter = match aggregation.as_str() {
                    "latest" => "fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_subscriber)"
                        .to_string(),
                    "history" => format!("rd.date >= CURRENT_DATE - INTERVAL '{} days'", size),
                    _ => unreachable!(),
                };
                where_parts.push(date_filter);

                let where_clause = where_parts.join(" AND ");

                // Build the complete query
                let query = format!(
                    r#"
SELECT 
    date_str AS date, 
    imsi AS imsi,
    msisdn AS msisdn,
    value
FROM (
    SELECT 
        rd.date_str, 
        cs.imsi,
        cs.msisdn,
        fct.value,
        ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.subscriber_id ORDER BY fct.batch_id DESC) as rn
    FROM 
        trx_metrics_subscriber fct
    JOIN 
        ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
    JOIN 
        ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
    JOIN 
        ref_dates rd ON rd.date_id = fct.date_id
    JOIN 
        cfg_subscribers cs ON cs.subscriber_id = fct.subscriber_id
    JOIN 
        cfg_operators co ON co.operator_id = fct.operator_id
    JOIN 
        cfg_countries cc ON cc.country_id = fct.country_id
    WHERE 
        {}
) t
WHERE rn = 1
ORDER BY 
    date_str
                "#,
                    where_clause
                );

                println!("Executing subscriber query: {}", query);
                println!("Parameters: {:?}", params);

                let mut q = sqlx::query(&query);
                for param in params {
                    q = q.bind(param);
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let imsi: String = row.try_get("imsi")?;
                    let msisdn: String = row.try_get("msisdn")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({
                        "date": date,
                        "imsi": imsi,
                        "msisdn": msisdn,
                        "value": value
                    }));
                }

                println!("Subscriber metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            "top" => {
                println!("Executing subscriber top query with size: {}", size);

                // Build WHERE clause dynamically for top query
                let mut where_parts = Vec::new();
                let mut params: Vec<String> = Vec::new();

                // Always add direction as first parameter
                params.push(direction.clone());
                where_parts.push("UPPER(rrd.direction) = UPPER($1)".to_string());
                where_parts.push(
                    "fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_subscriber)".to_string(),
                );

                // Track parameter index
                let mut param_index = 2;

                // Add country filter if present
                if !country.is_empty() {
                    where_parts.push(format!("UPPER(cc.country_name) = UPPER(${})", param_index));
                    params.push(country.clone());
                    param_index += 1;
                }

                // Add operator filter if present
                if !operator.is_empty() {
                    where_parts.push(format!("UPPER(co.operator_name) = UPPER(${})", param_index));
                    params.push(operator.clone());
                }

                let where_clause = where_parts.join(" AND ");

                // Build the complete top query
                let query = format!(
                    r#"
SELECT date, imsi, msisdn, value
FROM (
    SELECT 
        date_str AS date,
        imsi AS imsi,
        msisdn AS msisdn,
        value
    FROM (
        SELECT 
            rd.date_str, 
            cs.imsi,
            cs.msisdn,
            fct.value,
            ROW_NUMBER() OVER (PARTITION BY fct.date_id, fct.subscriber_id ORDER BY fct.batch_id DESC) as rn
        FROM 
            trx_metrics_subscriber fct
        JOIN 
            ref_metric_definitions rmd ON rmd.metric_definition_id = fct.metric_definition_id
        JOIN 
            ref_roam_directions rrd ON rrd.roam_direction_id = rmd.roam_direction_id
        JOIN 
            ref_dates rd ON rd.date_id = fct.date_id
        JOIN 
            cfg_subscribers cs ON cs.subscriber_id = fct.subscriber_id
        JOIN 
            cfg_operators co ON co.operator_id = fct.operator_id
        JOIN 
            cfg_countries cc ON cc.country_id = fct.country_id
        WHERE 
            {}
    ) t
    WHERE rn = 1
) ranked
ORDER BY value DESC
LIMIT $2
                "#,
                    where_clause
                );

                println!("Executing subscriber top query: {}", query);
                println!(
                    "Parameters: direction={}, size={}, country={:?}, operator={:?}",
                    direction, size, country, operator
                );

                let mut q = sqlx::query(&query).bind(direction.clone()).bind(size);
                if !country.is_empty() {
                    q = q.bind(country.clone());
                }
                if !operator.is_empty() {
                    q = q.bind(operator.clone());
                }

                let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
                let mut metrics = Vec::new();
                for row in rows {
                    let date: String = row.try_get("date")?;
                    let imsi: String = row.try_get("imsi")?;
                    let msisdn: String = row.try_get("msisdn")?;
                    let value: i64 = row.try_get("value")?;
                    metrics.push(json!({
                        "date": date,
                        "imsi": imsi,
                        "msisdn": msisdn,
                        "value": value
                    }));
                }

                println!("Subscriber top metrics result: {} records", metrics.len());
                Ok(json!({ "data": metrics, "status": "success" }))
            }

            _ => Err(AppError::BadRequest(
                "Aggregation 'latest', 'history' or 'top' is required".to_string(),
            )),
        }
    }

    // ------------------- SoR Performance -------------------
    async fn get_sor_performance_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing SoR performance metrics request: {:?}", req);

        let aggregation = &req.aggregation;
        let size = get_size_for_aggregation(aggregation, req.size)?;
        let country = get_country_from_filters(req.filter.as_ref());
        let operator = get_operator_from_filters(req.filter.as_ref());

        let mut query = Self::GET_SOR_PERFORMANCE_QUERY.to_string();

        // Replace date filter placeholder
        let date_filter = match aggregation.as_str() {
            "latest" => "AND tpo.date_id = (SELECT MAX(date_id) FROM trx_perf_out)",
            "history" => &format!("AND rd.date >= CURRENT_DATE - INTERVAL '{} days'", size),
            _ => {
                return Err(AppError::BadRequest(
                    "Aggregation 'latest' or 'history' is required".to_string(),
                ));
            }
        };
        query = query.replace("-- DATE_FILTER_PLACEHOLDER --", date_filter);

        // Replace country filter placeholder
        let country_filter = if !country.is_empty() {
            " AND UPPER(cc.country_name) = UPPER($1)"
        } else {
            ""
        };
        query = query.replace("-- COUNTRY_FILTER_PLACEHOLDER --", country_filter);

        // Replace operator filter placeholder
        let operator_filter = if !operator.is_empty() {
            " AND UPPER(co.operator_name) = UPPER($2)"
        } else {
            ""
        };
        query = query.replace("-- OPERATOR_FILTER_PLACEHOLDER --", operator_filter);

        println!("Executing SoR performance query: {}", query);

        let mut q = sqlx::query(&query);
        if !country.is_empty() {
            q = q.bind(country.clone());
        }
        if !operator.is_empty() {
            q = q.bind(operator);
        }

        let rows = q.fetch_all(pool).await.map_err(AppError::Sqlx)?;
        let mut metrics = Vec::new();
        for row in rows {
            let perf_id: i32 = row.try_get("perf_id")?;
            let batch_id: i32 = row.try_get("batch_id")?;
            let date: String = row.try_get("date")?;
            let country: Option<String> = row.try_get("country")?;
            let operator: Option<String> = row.try_get("operator")?;
            let country_count: i32 = row.try_get("country_count")?;
            let operator_count: i32 = row.try_get("operator_count")?;

            // These are INT8 in database, so use i64
            let target_percentage_raw: i64 = row.try_get("target_percentage")?;
            let actual_percentage_raw: i64 = row.try_get("actual_percentage")?;

            // Convert to f64 for calculations
            let target_percentage = target_percentage_raw as f64;
            let actual_percentage = actual_percentage_raw as f64;

            let is_outside_tolerance: bool = row.try_get("is_outside_tolerance")?;

            metrics.push(json!({
                "perf_id": perf_id,
                "batch_id": batch_id,
                "date": date,
                "country": country,
                "operator": operator,
                "country_count": country_count,
                "operator_count": operator_count,
                "target_percentage": target_percentage,
                "actual_percentage": actual_percentage,
                "is_outside_tolerance": is_outside_tolerance,
                "success_rate": actual_percentage,
                "variance": (actual_percentage - target_percentage).abs()
            }));
        }

        println!("SoR performance metrics result: {} records", metrics.len());
        Ok(json!({ "data": metrics, "status": "success" }))
    }

    // ------------------- Notification -------------------
    async fn get_notif_metrics(
        pool: &PgPool,
        req: &ValidatedMetricsRequest,
    ) -> Result<serde_json::Value, AppError> {
        println!("Processing notification metrics request: {:?}", req);

        let aggregation = &req.aggregation;
        let query = match aggregation.as_str() {
            "summary" => Self::GET_NOTIF_SUM_METRICS_QUERY,
            "detail" => Self::GET_NOTIF_DET_METRICS_QUERY,
            _ => {
                return Err(AppError::BadRequest(
                    "Aggregation 'summary' or 'detail' is required".to_string(),
                ));
            }
        };

        println!("Executing notification query: {}", query);

        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await
            .map_err(AppError::Sqlx)?;
        let mut metrics = Vec::new();
        for row in rows {
            let date: String = row.try_get("date")?;
            let value: String = row.try_get("value")?;
            metrics.push(json!({ "date": date, "value": value }));
        }

        println!("Notification metrics result: {} records", metrics.len());
        Ok(json!({ "data": metrics, "status": "success" }))
    }
}

// ------------------- Filter Helpers -------------------
fn get_direction_from_filters(filters: Option<&Vec<Filter>>) -> Result<String, AppError> {
    if let Some(filters) = filters {
        let dir = filters
            .iter()
            .find_map(|f| {
                if f.key.to_lowercase() == "direction" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .ok_or(AppError::BadRequest("Direction is required".to_string()))?;

        if ["in", "out"].contains(&dir.to_lowercase().as_str()) {
            Ok(dir)
        } else {
            Err(AppError::BadRequest(
                "Direction must be 'in' or 'out'".to_string(),
            ))
        }
    } else {
        Err(AppError::BadRequest("Direction is required".to_string()))
    }
}

fn get_size_for_aggregation(aggregation: &str, size: Option<u32>) -> Result<i32, AppError> {
    match aggregation {
        "history" => Ok(size.map(|s| s as i32).unwrap_or(30)),
        "top" => Ok(size.map(|s| s as i32).unwrap_or(5)),
        _ => Ok(5),
    }
}

fn get_country_from_filters(filters: Option<&Vec<Filter>>) -> String {
    if let Some(filters) = filters {
        filters
            .iter()
            .find_map(|f| {
                if f.key.to_lowercase() == "country" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        "".to_string()
    }
}

fn get_operator_from_filters(filters: Option<&Vec<Filter>>) -> String {
    if let Some(filters) = filters {
        filters
            .iter()
            .find_map(|f| {
                if f.key.to_lowercase() == "operator" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        "".to_string()
    }
}

fn get_subscriber_from_filters(filters: Option<&Vec<Filter>>) -> String {
    if let Some(filters) = filters {
        filters
            .iter()
            .find_map(|f| {
                if f.key.to_lowercase() == "subscriber" || f.key.to_lowercase() == "imsi" {
                    Some(f.value.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    } else {
        "".to_string()
    }
}
