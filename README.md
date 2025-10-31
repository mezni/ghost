docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

RUST_LOG=debug cargo run

docker cp file_utf8.dmp roamdb-service:/file_utf8.dmp 
docker exec -it roamdb-service psql -U myuser -d roamdb -f file_utf8.dmp 



curl -X POST http://localhost:8090/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "mohamed",
    "email": "mohamed@example.com",
    "password": "StrongPass123"
  }'




# This will create the user in BOTH your database AND Keycloak automatically
curl -X POST http://localhost:8000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "first_name": "Test",
    "last_name": "User",
    "password": "testpass123"
  }' | jq


  curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpass123"
  }' | jq





curl -X GET http://localhost:3000/api/v1/health


# Create a new user
curl -X POST http://localhost:8000/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe",
    "password": "securepassword123"
  }'

# Login with username and password
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "testpassword"
  }'


