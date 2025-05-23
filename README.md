docker exec -it database psql -U myuser -d roamdb

curl http://127.0.0.1:3000/api/v1/countries

curl -X POST http://127.0.0.1:3000/api/v1/countries -H "Content-Type: application/json"   -d '{"country_name": "France", "iso": "FR"}'