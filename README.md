docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

RUST_LOG=debug cargo run


python3 -m http.server 8080

curl -X POST http://localhost:3000/api/v1/analytics \
  -H 'Content-Type: application/json' \
  -d '{
    "dimension": "sor_performance", 
    "aggregation": "history",
    "size": 30,
    "filter": [
      {"key": "country", "value": "France"},
      {"key": "operator", "value": "Orange"}
    ]
  }'


curl -X POST http://localhost:3000/api/v1/analytics \
  -H "Content-Type: application/json" \
  -d '{
    "dimension": "subscriber",
    "aggregation": "latest",
    "filter": [
      {
        "key": "direction",
        "value": "OUT"
      },
      {
        "key": "operator",
        "value": "Orange"
      },
      {
        "key": "country",
        "value": "France"
      }
    ]
  }'


2025-11-02T02:27:41.153237Z  INFO 🔹 Starting API service
2025-11-02T02:27:41.858539Z  INFO ✅ Database pool initialized
2025-11-02T02:27:41.859238Z  INFO 🚀 Starting server on http://0.0.0.0:3000
2025-11-02T02:27:41.860251Z  INFO starting 2 workers
2025-11-02T02:27:41.861215Z  INFO Actix runtime found; starting in Actix runtime
2025-11-02T02:27:41.862024Z  INFO starting service: "actix-web-service-0.0.0.0:3000", workers: 2, listening on: 0.0.0.0:3000
Received metrics request: ValidatedMetricsRequest { dimension: "subscriber", aggregation: "latest", filter: Some([Filter { key: "direction", value: "OUT", operator: None }, Filter { key: "operator", value: "Orange", operator: None }, Filter { key: "country", value: "France", operator: None }]), size: Some(30), period: None }
Processing subscriber metrics request: ValidatedMetricsRequest { dimension: "subscriber", aggregation: "latest", filter: Some([Filter { key: "direction", value: "OUT", operator: None }, Filter { key: "operator", value: "Orange", operator: None }, Filter { key: "country", value: "France", operator: None }]), size: Some(30), period: None }
Executing subscriber query: 
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
        UPPER(rrd.direction) = UPPER('OUT')
         AND UPPER(cc.country_name) = UPPER('France') AND UPPER(co.operator_name) = UPPER('Orange') AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_subscriber)
) t
WHERE rn = 1
ORDER BY 
    date_str;

SELECT 
    date_str AS date, 
    cs.imsi AS imsi,
    cs.msisdn AS msisdn,
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
    WHERE 
        UPPER(rrd.direction) = UPPER('OUT')
         AND UPPER(cc.country_name) = UPPER('France')
         AND UPPER(co.operator_name) = UPPER('Orange')
         AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_subscriber)
) t
WHERE rn = 1
ORDER BY 
    date_str;


UPDATE trx_perf_out
SET is_outside_tolerance = NULL
WHERE is_outside_tolerance IS FALSE;  


SELECT 
    tpo.perf_id,
    tpo.batch_id,
    rd.date_str AS date,
    cc.country_name AS country,
    co.operator_name AS operator,
    tpo.country_count,
    tpo.operator_count,
    tpo.target_percentage,
    tpo.actual_percentage,
    tpo.is_outside_tolerance
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
    (tpo.batch_id, rd.date_str) = (
        SELECT 
            tpo2.batch_id, 
            rd2.date_str
        FROM 
            trx_perf_out tpo2
        JOIN 
            ref_dates rd2 ON tpo2.date_id = rd2.date_id
        ORDER BY 
            rd2.date_str DESC, 
            tpo2.batch_id DESC
        LIMIT 1
    )
ORDER BY 
    rd.date_str DESC;


curl -X POST http://localhost:3000/api/v1/analytics \
  -H 'Content-Type: application/json' \
  -d '{
    "dimension": "operator", 
    "aggregation": "top", 
    "filter": [
      {"key": "direction", "value": "in"},
      {"key": "country", "value": "France"}
    ]
  }'

SELECT 
    date, 
    country, 
    SUM(value)::bigint AS value
FROM (
    SELECT 
        rd.date_str AS date,
        CASE 
            WHEN ROW_NUMBER() OVER (PARTITION BY rd.date_str ORDER BY fct.value DESC) <= CAST (5 AS INT)
            THEN cc.country_name 
            ELSE 'Others'
        END AS country,
        fct.value
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
        (fct.date_id, fct.batch_id) IN (
            SELECT 
                date_id, 
                MAX(batch_id) AS max_batch_id
            FROM 
                trx_metrics_country
            GROUP BY 
                date_id
        )
        AND UPPER(rrd.direction) = UPPER('IN')
        AND fct.date_id = (SELECT MAX(date_id) FROM trx_metrics_country)
) AS ranked
GROUP BY 
    date, 
    country
ORDER BY 
    value DESC



curl -X POST http://localhost:3000/api/v1/analytics \
-H "Content-Type: application/json" \
-d '{
  "dimension": "global",
  "aggregation": "history",
  "size": 7,
  "filter": [
    {"key": "direction", "value": "out"}
  ]
}'




curl -X POST http://localhost:8080/analytics \
-H "Content-Type: application/json" \
-d '{
  "dimension": "global",
  "aggregation": "latest",
  "filter": [
    {"key": "direction", "value": "in"}
  ]
}'



RUST_LOG=debug cargo run

docker cp file_utf8.dmp roamdb-service:/file_utf8.dmp 
docker exec -it roamdb-service psql -U myuser -d roamdb -f file_utf8.dmp 




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
        UPPER(rrd.direction) = UPPER('IN')
        AND rd.date >= CURRENT_DATE - INTERVAL '30 days'
) t
WHERE rn = 1
ORDER BY 
    date_str






curl -X POST http://localhost:8090/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mohamed",
    "email": "mohamed@example.com",
    "password": "StrongPass123"
  }'




# This will create the user in BOTH your database AND Keycloak automatically
curl -X POST http://localhost:8000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "first_name": "Test",
    "last_name": "User",
    "password": "testpass123"
  }' | jq


  curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123"
  }' | jq





curl -X GET http://localhost:3000/api/v1/health


# Create a new user
curl -X POST http://localhost:8000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "password": "securepassword123"
  }'

# Login with username and password
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpassword"
  }'


