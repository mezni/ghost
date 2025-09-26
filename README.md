docker system prune --all --volumes
docker volume rm $(docker volume ls -qf dangling=true)


curl -X GET http://localhost:8000/api/v1/health