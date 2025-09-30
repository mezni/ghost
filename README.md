docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

uvicorn main:app --host 0.0.0.0 --port 8000 --reload



curl -X GET http://0.0.0.0:3000/api/v1/countries

curl -X POST http://0.0.0.0:3000/api/v1/countries   -H "Content-Type: application/json"   -d '{
    "iso_code": "TN",
    "country_name": "Tunisia",
    "created_by": "system"
}'


curl -X GET http://127.0.0.1:3000/api/v1/sor

curl -X POST http://127.0.0.1:3000/api/v1/sor \
     -H "Content-Type: application/json" \
     -d '{
           "country_name": "Algeria",
           "operator_name": "Mobilis",
           "routage_type_name":"Bilateral",
           "barring": "N",
           "rate": "90",
           "created_by": "system"
         }'

curl -X POST http://127.0.0.1:3000/api/v1/sor \
     -H "Content-Type: application/json" \
     -d '{
           "country_name": "Algeria",
           "operator_name": "Djezzy",
           "routage_type_name":"Bilateral",
           "barring": "N",
           "rate": "10",
           "created_by": "system"
         }'

curl -X POST http://127.0.0.1:3000/api/v1/sor \
     -H "Content-Type: application/json" \
     -d '{
           "country_name": "Algeria",
           "operator_name": "Ooredoo",
           "routage_type_name":"Bilateral",
           "barring": "N",
           "rate": "0=",
           "created_by": "system"
         }'         

curl -X PUT http://127.0.0.1:3000/api/v1/sor/9 \
     -H "Content-Type: application/json" \
     -d '{
           "country_name": "Algeria",
           "operator_name": "Ooredoo",
           "routage_type_name":"Bilateral",
           "barring": "N",
           "rate": "0+",
           "updated_by": "system"
         }'


curl -X DELETE http://127.0.0.1:3000/api/v1/sor/1
