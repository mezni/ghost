docker exec -it database psql -U myuser -d roamdb

curl http://127.0.0.1:3000/api/v1/countries

curl -X POST http://127.0.0.1:3000/api/v1/countries -H "Content-Type: application/json"   -d '{"country_name": "France", "iso": "FR"}'


curl -X POST http://localhost:3000/api/v1/register  -H "Content-Type: application/json"   -d '{
    "name": "Dali",
    "email": "dali2@example.com",
    "password": "securePassword123"
}'


curl -X POST http://localhost:3000/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{
        "email": "dali2@example.com",
        "password": "securePassword123"
      }'

docker exec -it auth-db psql -U myuser -d authdb
