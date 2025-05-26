docker exec -it auth-db psql -U postgres -d authdb

curl -X POST http://localhost:3100/api/v1/register \
     -H "Content-Type: application/json" \
     -d '{"username":"alice","email":"alice@example.com","password":"mypassword123"}'


curl -X POST http://localhost:3100/api/v1/login \
     -H "Content-Type: application/json" \
     -d '{"username":"alice","password":"mypassword123"}'


curl http://localhost:3100/api/v1/metrics
