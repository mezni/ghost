export GOOS=linux
go build -o generator-linux-amd64
export GOOS=darwin

docker build -t mezni/generator:latest .

docker tag mezni/generator generator:latest
docker push mezni/generator:latest
