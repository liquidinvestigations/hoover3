#!/bin/bash

set -ex

docker rm -f $(docker ps -qa) || true
docker volume rm -f $(docker volume ls -q) || true
time docker compose up -d