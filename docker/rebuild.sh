#!/bin/bash

set -ex


docker compose down || true
docker rm -f $(docker ps -qa) || true
# ( cd hoover3 && pipenv install && pipenv lock )
docker compose up -d --remove-orphans --build