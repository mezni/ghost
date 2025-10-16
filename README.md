docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

RUST_LOG=debug cargo run

docker cp file_utf8.dmp roamdb-service:/file_utf8.dmp 
docker exec -it roamdb-service psql -U myuser -d roamdb -f file_utf8.dmp 


















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