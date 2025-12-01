use chrono::Utc;
use core::errors::AppError;
use tokio_postgres::Client;
use tokio_postgres::types::ToSql;

pub async fn get_max_date(client: &Client, direction: &str) -> Result<String, AppError> {
    let query = "
        SELECT MAX(date_str)
        FROM v_metrics
        WHERE metric = $1
    ";

    let row = match client.query_one(query, &[&direction]).await {
        Ok(r) => r,
        Err(_) => {
            let today = Utc::now().date_naive().to_string();
            return Ok(today);
        }
    };

    let max_date: Option<String> = row.get(0);
    Ok(max_date.unwrap_or_else(|| Utc::now().date_naive().to_string()))
}

fn resolve_metric_name(dimension: &str, direction: &str) -> Option<&'static str> {
    match (
        dimension.to_ascii_uppercase().as_str(),
        direction.to_ascii_uppercase().as_str(),
    ) {
        ("GLOBAL", "TOTIN") => Some("number_subscribers_in"),
        ("GLOBAL", "OUT") => Some("number_subscribers_out"),

        ("COUNTRY", "TOTIN") => Some("number_subscribers_in_by_country"),
        ("COUNTRY", "OUT") => Some("number_subscribers_out_by_country"),

        ("OPERATOR", "TOTIN") => Some("number_subscribers_in_by_operator"),
        ("OPERATOR", "OUT") => Some("number_subscribers_out_by_operator"),

        ("GLOBAL", "ACTIN") => Some("number_active_subscribers_in"),
        ("COUNTRY", "ACTIN") => Some("number_active_subscribers_in_by_country"),
        ("OPERATOR", "ACTIN") => Some("number_active_subscribers_in_by_operator"),

        _ => None,
    }
}

pub async fn get_metrics(
    client: &Client,
    direction: &str,
    dimension: &str,
    kind: &str,
    limit: i64,
) -> Result<Vec<(String, Option<String>, Option<String>, i32)>, AppError> {
    let metric_name = resolve_metric_name(dimension, direction)
        .ok_or_else(|| AppError::BadRequest("Invalid metric or dimension".to_string()))?;

    let dim = dimension.to_ascii_uppercase();
    let is_global = dim == "GLOBAL" || dim == "COUNTRY" || dim == "OPERATOR";

    if !is_global {
        return Err(AppError::BadRequest("Invalid dimension".into()));
    }

    let limit_clause = if limit > 0 { " LIMIT $2" } else { "" };

    let (filter_clause, order_clause, query_params): (
        &str,
        String,
        Vec<&(dyn tokio_postgres::types::ToSql + Sync)>,
    ) = match kind.to_ascii_uppercase().as_str() {
        "LATEST" => {
            let filter =
                "AND date_str = (SELECT MAX(date_str) FROM v_metrics WHERE metric_name = $1)";
            let order = match dim.as_str() {
                "GLOBAL" => format!("{}", limit_clause),
                "COUNTRY" | "OPERATOR" => format!("ORDER BY value DESC{}", limit_clause),
                _ => unreachable!(),
            };
            let params: Vec<&(dyn ToSql + Sync)> = if limit > 0 {
                vec![
                    &metric_name as &(dyn ToSql + Sync),
                    &limit as &(dyn ToSql + Sync),
                ]
            } else {
                vec![&metric_name as &(dyn ToSql + Sync)]
            };
            (filter, order, params)
        }

        "HISTORY" => {
            let filter = "
                    AND date_str <= (SELECT MAX(date_str) FROM v_metrics WHERE metric_name = $1)
                    AND date_str > (SELECT to_char((to_date(MAX(date_str),'YYYY-MM-DD')-30),'YYYY-MM-DD') FROM v_metrics WHERE metric_name = $1)
                ";
            let order = match dim.as_str() {
                "GLOBAL" => format!("{}", limit_clause),
                "COUNTRY" | "OPERATOR" => format!("ORDER BY value DESC{}", limit_clause),
                _ => unreachable!(),
            };

            let params: Vec<&(dyn ToSql + Sync)> = if limit > 0 {
                vec![
                    &metric_name as &(dyn ToSql + Sync),
                    &limit as &(dyn ToSql + Sync),
                ]
            } else {
                vec![&metric_name as &(dyn ToSql + Sync)]
            };
            (filter, order, params)
        }

        _ => return Err(AppError::BadRequest("Invalid kind".into())),
    };

    let query = format!(
        "SELECT date_str, country, operator, value
         FROM v_metrics
         WHERE metric_name = $1
         {}
         {}",
        filter_clause, order_clause
    );

    //    println!("{:?}", query.clone());

    let rows = client
        .query(&query, &query_params)
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| {
            (
                row.get::<_, String>(0),
                row.get::<_, Option<String>>(1),
                row.get::<_, Option<String>>(2),
                row.get::<_, i32>(3),
            )
        })
        .collect();

    Ok(result)
}

pub async fn get_notifications_summary(
    client: &Client,
) -> Result<Vec<(String, String, i32)>, AppError> {
    let query = "
        SELECT d.date_str AS date, r.description AS rule, count(*)::int AS count
        FROM notifications n 
        JOIN rules r ON n.rule_id = r.id
        JOIN dates d ON n.date_id = d.date_id
        WHERE n.date_id = (SELECT max(date_id) FROM notifications)
        GROUP BY d.date_str, r.description
    ";

    let rows = client
        .query(query, &[]) // no &
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| {
            (
                row.get::<_, String>(0),
                row.get::<_, String>(1),
                row.get::<_, i32>(2),
            )
        })
        .collect();

    Ok(result)
}

pub async fn get_notifications_details(
    client: &Client,
) -> Result<Vec<(String, String, String)>, AppError> {
    let query = "
        SELECT operator || ' (' || common_name || ')' AS operator, 
               CAST(perct_configure AS TEXT) AS perct_configure,
                 CAST(perct_reel AS TEXT) AS perct_reel
        FROM v_roam_out_perf        
        WHERE date_id = (SELECT max(date_id) FROM v_roam_out_perf)
        ORDER BY common_name, operator
    ";

    let rows = client
        .query(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| {
            (
                row.get::<_, String>(0),
                row.get::<_, String>(1),
                row.get::<_, String>(2),
            )
        })
        .collect();

    Ok(result)
}

pub async fn get_notifications_count(
    client: &Client,
) -> Result<Vec<i32>, AppError> {
    let query = "
        SELECT count(*)::int 
        FROM (
            SELECT r.description AS rule, count(*)
            FROM notifications n 
            JOIN rules r ON n.rule_id = r.id
            WHERE n.date_id = (SELECT max(date_id) FROM notifications)
            GROUP BY r.description
        ) subquery
    ";

    let rows = client
        .query(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| row.get::<_, i32>(0))
        .collect();

    Ok(result)
}


pub async fn get_alerts_count(
    client: &Client,
) -> Result<Vec<i32>, AppError> {
    let query = "
            SELECT  count(*)::int 
            FROM notifications n 
            JOIN rules r ON n.rule_id = r.id
            WHERE n.date_id = (SELECT max(date_id) FROM notifications)
            GROUP BY r.description
    ";

    let rows = client
        .query(query, &[])
        .await
        .map_err(AppError::DatabaseError)?;

    let result = rows
        .into_iter()
        .map(|row| row.get::<_, i32>(0))
        .collect();

    Ok(result)
}