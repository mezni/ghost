#!/bin/bash

mkdir -p WORK/INPUT WORK/ARCHIVE WORK/REJECTED

docker compose build data-generator
docker compose up data-generator
