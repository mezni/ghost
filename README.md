docker exec -it roam_db psql -U myuser -d roamdb

cleanup 
docker compose stop loader-service
docker compose rm -f loader-service
docker rmi alpha/loader-service:latest
docker compose build loader-service
docker compose up -d loader-service

http-server -p 8080 --cors

NOTES 
remove .env under source