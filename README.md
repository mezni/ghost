docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

INSERT INTO batch_execs (batch_id, batch_name) VALUES (1,'TEST');

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value) VALUES 
(1,1,640, 12000),
(1,1,639, 12010),
(1,1,638, 12020),
(1,1,637, 12030),
(1,1,636, 13000),
(1,1,635, 12010),
(1,1,634, 12020),
(1,1,633, 13030),
(1,1,632, 12020),
(1,1,631, 12100);

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value) VALUES 
(5,1,640, 14000),
(5,1,639, 14010),
(5,1,638, 14020),
(5,1,637, 14030),
(5,1,636, 14000),
(5,1,635, 13010),
(5,1,634, 13020),
(5,1,633, 13030),
(5,1,632, 13020),
(5,1,631, 13100);

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value) VALUES 
(1,1,640,63,6000),
(1,1,639,63, 6010),
(1,1,638,63, 6020),
(1,1,637,63, 6030),
(1,1,636,63, 6000),
(1,1,635,63, 6010),
(1,1,634,63, 6020),
(1,1,633,63, 7030),
(1,1,632,63, 7020),
(1,1,631,63, 7100);

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value) VALUES 
(1,1,640,138,6000),
(1,1,639,138, 2010),
(1,1,638,138, 1020),
(1,1,637,138, 1030),
(1,1,636,138, 1000),
(1,1,635,138, 1010),
(1,1,634,138, 2020),
(1,1,633,138, 2030),
(1,1,632,138, 2020),
(1,1,631,138, 2100);


INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value) VALUES 
(5,1,640,63,6000),
(5,1,639,63, 6010),
(5,1,638,63, 6020),
(5,1,637,63, 6030),
(5,1,636,63, 6000),
(5,1,635,63, 6010),
(5,1,634,63, 6020),
(5,1,633,63, 7030),
(5,1,632,63, 7020),
(5,1,631,63, 7100);

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value) VALUES 
(5,1,640,138,6000),
(5,1,639,138, 2010),
(5,1,638,138, 1020),
(5,1,637,138, 1030),
(5,1,636,138, 1000),
(5,1,635,138, 1010),
(5,1,634,138, 2020),
(5,1,633,138, 2030),
(5,1,632,138, 2020),
(5,1,631,138, 2100);


roamdb=# update metrics_country set metric_definition_id =2 where metric_definition_id=1;
UPDATE 20
roamdb=# update metrics_country set metric_definition_id =4 where metric_definition_id=5;
UPDATE 20


SELECT  dd.date_str AS date, dc.country_name, mc.value AS value
FROM metrics_country mc
JOIN dim_dates dd ON mc.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
JOIN dim_countries dc ON dc.country_id = mc.country_id
WHERE crd.direction = $1
AND cmt.name = 'COUNTRY'
AND dd.date = >= CURRENT_DATE - make_interval(days => $2)
ORDER BY mc.date_id;



                    SELECT dd.date_str AS date, mg.value AS value
                    FROM metrics_global mg
                    JOIN dim_dates dd ON mg.date_id = dd.date_id
                    JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
                    JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
                    JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
                    WHERE crd.direction = $1
                    AND cmt.name = 'GLOBAL'
                    AND dd.date >= CURRENT_DATE - make_interval(days => $2)
                    ORDER BY dd.date_str


SELECT dd.date_str, mg.value, cmt.name, crd.direction
FROM metrics_global mg 
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
ORDER BY mg.date_id;



SELECT dd.date_str, mg.value
FROM metrics_global mg 
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
WHERE crd.direction = 'IN'
AND cmt.name = 'GLOBAL'
ORDER BY mg.date_id;





curl -X POST http://localhost:3000/api/v1/metrics \
     -H "Content-Type: application/json" \
     -d '{
           "type": "Metric",
           "dataset": {
             "granularity": "Monthly",
             "aggregation": "Global",
             "direction": "IN"
           },
           "timePeriod": {
             "window": 5,
             "from": "2025-09-01",
             "to": "2025-09-30"
           },
           "filter": {
             "country": "Tunisia",
             "operator": "Orange",
             "subscriber": ""
           }
                        "aggregation": {"Top"}
         }'


"metric": "Metric",
"dimension": "Global",
"direction": "IN",
"timeWindow": 0,
"timePeriod": {
  "start":"2025-10-01",
  "end":"2025-10-01",  
},
"filter": {
    "key": "Tunisia",
    "value": "Orange",
},
"aggregation": {
    "mesure": "Top",
    "size":5
}




curl -X POST http://localhost:3000/api/v1/metrics \
     -H "Content-Type: application/json" \
     -d '{
           "type": "Metric",
           "dataset": {
             "aggregation": "Global",
             "direction": "IN"
           },
           "timePeriod": {},
           "filter": {}
         }'


curl -X POST http://localhost:3000/api/v1/metrics \
     -H "Content-Type: application/json" \
     -d '{
           "type": "Metric",
           "dataset": {
             "aggregation": "Global",
             "direction": "IN"
           },
           "timePeriod": {"window":10},
           "filter": {}
         }'













SELECT dd.date_str AS date, mg.value AS value
FROM metrics_global mg
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
WHERE crd.direction = 'IN'
AND cmt.name = 'GLOBAL'
AND dd.date_id = (SELECT max(date_id) FROM metrics_global)
-- AND dd.date >= CURRENT_DATE - make_interval(days => $2)
ORDER BY dd.date_str;


SELECT dd.date_str AS date, mg.value AS value
FROM metrics_global mg
JOIN dim_dates dd ON mg.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mg.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
WHERE crd.direction = 'IN'
AND cmt.name = 'GLOBAL'
-- AND dd.date >= CURRENT_DATE - make_interval(days => $2)
ORDER BY dd.date_str;




SELECT  DISTINCT dc.country_name AS country
FROM metrics_country mc
JOIN dim_countries dc ON dc.country_id = mc.country_id
ORDER BY dc.country_name;



SELECT  dd.date_str AS date, dc.country_name AS country, mc.value AS value
FROM metrics_country mc
JOIN dim_dates dd ON mc.date_id = dd.date_id
JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
JOIN dim_countries dc ON dc.country_id = mc.country_id
WHERE crd.direction = 'IN'
AND cmt.name = 'COUNTRY'
AND dd.date_id = (SELECT max(date_id) FROM metrics_country)
ORDER BY mc.date_id;






"metric": "Metric",
"dimension": "Global",
"direction": "IN",
"timeWindow": 0,
"timePeriod": {
  "start":"2025-10-01",
  "end":"2025-10-01",  
},
"filter": {
    "key": "Tunisia",
    "value": "Orange",
},
"aggregation": {
    "mesure": "Top",
    "size":5
}


curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "IN"
  }'


curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "OUT"
  }'  


curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "IN",
    "timeWindow": 10
  }'

  
curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "country",
    "direction": "IN"
  }'


curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "country",
    "direction": "IN",
    "timeWindow": 10
  }'


curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "country",
    "direction": "IN",
"aggregation": {
    "mesure": "Top",
    "size":1
}
  }'  


        WITH RankedCountries AS (
            SELECT 
                dd.date_str AS date, 
                dc.country_name, 
                mc.value, 
                ROW_NUMBER() OVER (PARTITION BY dd.date_str ORDER BY mc.value DESC) AS rn
            FROM metrics_country mc
            JOIN dim_dates dd ON mc.date_id = dd.date_id
            JOIN cfg_metric_definitions cmd ON mc.metric_definition_id = cmd.metric_definition_id
            JOIN cfg_metric_types cmt ON cmd.metric_type_id = cmt.metric_type_id
            JOIN cfg_roam_directions crd ON cmd.roam_direction_id = crd.roam_direction_id
            JOIN dim_countries dc ON dc.country_id = mc.country_id
            WHERE crd.direction = 'IN'
            AND cmt.name = 'COUNTRY'
            AND dd.date_id = (SELECT max(date_id) FROM metrics_global)
        )
        SELECT 
            date,CASE
                WHEN rn <= 1 THEN country_name
                ELSE 'Others'
            END AS country_name,
            SUM(value) AS value
        FROM RankedCountries
        GROUP BY 
            date, CASE
                WHEN rn <= 1 THEN country_name
                ELSE 'Others'
            END
        ORDER BY value DESC