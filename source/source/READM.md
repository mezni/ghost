1. Get all countries
curl -X GET http://localhost:3000/api/v1/countries \
     -H "Accept: application/json"

2. Get a country by ID
curl -X GET http://localhost:3000/api/v1/countries/1 \
     -H "Accept: application/json"

3. Create a new country
curl -X POST http://localhost:3000/api/v1/countries \
     -H "Content-Type: application/json" \
     -d '{
           "iso_code": "CA",
           "country_name": "Canada",
           "created_by": "admin"
         }'

4. Update an existing country
curl -X PUT http://localhost:3000/api/v1/countries/1 \
     -H "Content-Type: application/json" \
     -d '{
           "iso_code": "CAN",
           "country_name": "Canada Updated",
           "is_valid": true,
           "updated_by": "editor"
         }'
5. Soft delete a country
curl -X DELETE http://localhost:3000/api/v1/countries/1 \
     -H "Accept: application/json"
