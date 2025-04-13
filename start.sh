#!/bin/bash

mkdir -p WORK/INPUT/ROUT WORK/ARCHIVE/ROUT WORK/REJECTED/ROUT

echo "Build"
docker compose build
sleep 5

echo "Start DB"
docker compose up --no-build postgres -d
sleep 5

echo "Start data-generator"
docker compose up --no-build data-generator -d
sleep 5

echo "Start loader"
docker compose up --no-build loader-service -d
sleep 5

echo "Start analytics"
docker compose up --no-build analytics-service -d
sleep 5

echo "Start api"
docker compose up --no-build api-service -d
sleep 5

echo "Start frontend"
docker compose up --no-build frontend -d
sleep 5