#!/bin/bash

mkdir -p WORK/INPUT WORK/ARCHIVE WORK/REJECTED

docker compose build postgres
docker compose up postgres -d

#docker compose build data-generator
#docker compose up data-generator

#docker compose build loader-service
#docker compose up loader-service -d