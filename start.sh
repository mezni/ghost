#!/bin/bash

mkdir -p WORK/INPUT/ROUT WORK/ARCHIVE/ROUT WORK/REJECTED/ROUT

docker compose build postgres
docker compose up postgres -d

docker compose build data-generator
docker compose up data-generator

docker compose build loader-service
docker compose up loader-service -d

docker compose build analytics-service
docker compose up analytics-service -d


docker compose build api-service
docker compose up api-service -d