docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

curl -X GET http://localhost:3000/api/v1/countries


curl -X POST http://localhost:3000/api/v1/countries \
     -H "Content-Type: application/json" \
     -d '{
           "iso_code": "US",
           "country_name": "United States",
           "created_by": "admin"
         }'

curl -X PUT http://localhost:3000/api/v1/countries \
     -H "Content-Type: application/json" \
     -d '{
           "country_name": "USA",
           "updated_by": "admin"
         }'


curl -X DELETE http://localhost:3000/api/v1/countries/1

INSERT INTO batch_execs (batch_id, batch_name) VALUES (1,'TEST');

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value)
SELECT 1,1, date_id,9000+ FLOOR(RANDOM()* 1000) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;

INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value)
SELECT 5,1, date_id,12000+ FLOOR(RANDOM()* 2000) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;




INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,76,2500+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;


INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,63,1000+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,138,1000+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,137,1000+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;


INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,137,1000+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;

INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2,1, date_id,66,1000+ FLOOR(RANDOM()* 200) 
FROM dim_dates 
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE;



- last 

curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "IN"
  }'
{"data":[{"date":"2025-10-07","value":9918}],"status":"success"}

curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "OUT"
  }'
{"data":[{"date":"2025-10-07","value":12264}],"status":"success"}


- Trend Roam IN
curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "global",
    "direction": "IN",
    "timeWindow": 30
  }'



curl -X POST http://localhost:3000/api/v1/metrics   -H "Content-Type: application/json"   -d '{
    "metric": "metric",
    "dimension": "country",
    "direction": "IN",
"aggregation": {
    "mesure": "Top",
    "size":
}
  }'




INSERT INTO batch_execs (batch_id, batch_name) VALUES (1,'TEST');

INSERT INTO metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 3, 1, d.date_id, o.country_id, o.operator_id,1000+ FLOOR(RANDOM()* 500)
FROM dim_dates d, dim_operators o
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE
AND o.country_id in (63, 66, 67, 138, 178);


INSERT INTO metrics_operator (metric_definition_id, batch_id, date_id, country_id, operator_id, value)
SELECT 8, 1, d.date_id, o.country_id, o.operator_id,500+ FLOOR(RANDOM()* 100)
FROM dim_dates d, dim_operators o
WHERE date BETWEEN CURRENT_DATE - INTERVAL '30 day' AND CURRENT_DATE
AND o.country_id in (63, 66, 67, 138, 178);


INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 2, batch_id , date_id , country_id, sum(value) 
FROM metrics_operator
WHERE metric_definition_id = 3
GROUP BY batch_id , date_id , country_id;


INSERT INTO metrics_country (metric_definition_id,batch_id,date_id, country_id, value)  
SELECT 7, batch_id , date_id , country_id, sum(value) 
FROM metrics_operator
WHERE metric_definition_id = 8
GROUP BY batch_id , date_id , country_id;


INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value)  
SELECT 1, batch_id , date_id , sum(value) 
FROM metrics_operator
WHERE metric_definition_id = 3
GROUP BY batch_id , date_id , country_id;


INSERT INTO metrics_global (metric_definition_id,batch_id,date_id, value)  
SELECT 6, batch_id , date_id , sum(value) 
FROM metrics_operator
WHERE metric_definition_id = 8
GROUP BY batch_id , date_id , country_id;
