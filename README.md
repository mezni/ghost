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
