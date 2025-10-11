docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080



curl -X GET http://localhost:3000/api/v1/countries

curl -X POST http://localhost:3000/api/v1/countries -H "Content-Type: application/json" -d '{
  "iso_code": "AA",
  "country_name": "TESTAA",
  "created_by": "dali"
}'

"dimension": "global",
"aggregation": "top",   -- latest, history  X
"filter": [
  {
    "key": "direction",
    "value": "in",
  }
],
"size": 30,
"period": {
  "start":"2025-10-01",
  "end":"2025-10-01",  
}

curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "notification",
  "aggregation": "summary"
}'


curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "global",
  "aggregation": "history",
  "filter": [{"key":"direction", "value":"in"}]
}'

body: JSON.stringify({
  dimension: "global",
  aggregation: "history",
  filter: [{ "key": "direction", "value": "in" }]
})



curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "global",
  "filter": [{"key":"direction", "value":"in"}]
}'


curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "global",
  "aggregation": "latest",
  "filter": [{"key":"direction", "value":"in"}]
}'

curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "country",
  "filter": [{"key":"direction", "value":"in"}]
}'

curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "country",
  "aggregation": "latest",
  "filter": [{"key":"direction", "value":"in"}]
}'


curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "country",
  "aggregation": "latest",
  "filter": [{"key":"direction", "value":"in"}, {"key":"country", "value":"France"}]
}'

curl -X POST http://localhost:3000/api/v1/metrics -H "Content-Type: application/json" -d '{
  "dimension": "country",
  "aggregation": "top",
  "filter": [{"key":"direction", "value":"in"}]
}'





                      