export GOOS=linux
go build -o generator-linux-amd64
export GOOS=darwin

docker build -t dali/generator .