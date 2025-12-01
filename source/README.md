1- Startup 
docker compose up -d

2- Shutdown
docker compose down

3- Cleanup
docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

4- database conn
docker exec -it roamdb-service psql -U myuser -d roamdb

5- web
http://localhost:8080
