docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

RUST_LOG=debug cargo run

### --- HEALTH
curl -X GET http://localhost:3000/api/v1/health                    




curl -X POST http://localhost:3000/api/v1/analytics -H "Content-Type: application/json" -d '{
  "dimension": "notification",
  "aggregation": "summary"
}'