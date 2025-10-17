docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

RUST_LOG=debug cargo run

docker cp file_utf8.dmp roamdb-service:/file_utf8.dmp 
docker exec -it roamdb-service psql -U myuser -d roamdb -f file_utf8.dmp 



create or replace view v_metrics_global as
select trx.batch_id, rd.date_str , rrd.direction, rmt.name, value from trx_metrics_global trx 
join ref_dates rd ON trx.date_id = rd.date_id
join ref_metric_definitions rmd on trx.metric_definition_id = rmd.metric_definition_id
join ref_roam_directions rrd on rrd.roam_direction_id = rmd.roam_direction_id
join ref_metric_types rmt on rmd.metric_type_id = rmt.metric_type_id;



select * from trx_metrics_operator limit 5;
 metric_id | metric_definition_id | batch_id | date_id | country_id | operator_id | value 
-----------+----------------------+----------+---------+------------+-------------+-------
         1 |                    3 |        2 |     645 |          2 |           2 |    79
         2 |                    3 |        2 |     645 |          3 |             |     1
         3 |                    3 |        2 |     645 |          6 |             |     4
         4 |                    3 |        2 |     645 |          7 |             |     1
         5 |                    3 |        2 |     645 |          8 |             |     3
(5 rows)


CREATE TABLE IF NOT EXISTS trx_perf_out (
    perf_id SERIAL PRIMARY KEY,
    batch_id INTEGER NOT NULL REFERENCES batch_execs(batch_id), 
    date_id INTEGER NOT NULL REFERENCES ref_dates(date_id),
    country_id INTEGER REFERENCES cfg_countries(country_id),
    operator_id INTEGER REFERENCES cfg_operators(operator_id),
    country_count BIGINT,
    operator_count BIGINT,
    target_percentage BIGINT,
    actual_percentage BIGINT,
    is_outside_tolerance BOOLEAN    
);




select trx.batch_id, trx.date_id, trx.country_id, trx.operator_id , 
from trx_metrics_operator trx 
join ref_dates rd ON trx.date_id = rd.date_id
join ref_metric_definitions rmd on trx.metric_definition_id = rmd.metric_definition_id
join ref_roam_directions rrd on rrd.roam_direction_id = rmd.roam_direction_id
join ref_metric_types rmt on rmd.metric_type_id = rmt.metric_type_id
WHERE rrd.direction = 'OUT'
AND trx.batch_id = 1
;






### --- HEALTH
curl -X GET http://localhost:3000/api/v1/health                    




curl -X POST http://localhost:3000/api/v1/analytics -H "Content-Type: application/json" -d '{
  "dimension": "notification",
  "aggregation": "summary"
}'



CREATE TABLE IF NOT EXISTS cfg_networks (
    network_id SERIAL PRIMARY KEY,
    country_name
    operator_name
    plmn_code VARCHAR(100) NOT NULL,
    plmn VARCHAR(100) NOT NULL,
    mcc VARCHAR(100) NOT NULL,
    mnc VARCHAR(100) NOT NULL,
    tech_2g BOOLEAN DEFAULT FALSE,
    tech_3g BOOLEAN DEFAULT FALSE,
    tech_lte BOOLEAN DEFAULT FALSE,

);