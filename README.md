docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)

docker exec -it roamdb-service psql -U myuser -d roamdb

python3 -m http.server 8080

RUST_LOG=debug cargo run

docker cp file_utf8.dmp roamdb-service:/file_utf8.dmp 
docker exec -it roamdb-service psql -U myuser -d roamdb -f file_utf8.dmp 



