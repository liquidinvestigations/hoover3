#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p data

(
    cd docker
    bash wipe.sh
)

(
    ./start_docker.sh
)

(
    ./migrate.sh
)