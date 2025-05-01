/countries
    GET: list last 50 countries
    POST: create a new country


db connection:
- docker exec -it database psql -U myuser -d roamdb



curl -X POST http://localhost:8080/countries \
     -H "Content-Type: application/json" \
     -d '{"name": "France", "iso": "FR"}'

curl -X DELETE http://localhost:8080/countries/252
