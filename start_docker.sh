#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p data
mkdir -p target/doc
touch target/doc/.some_data
mkdir -p docker/data
touch docker/data/.some_data

mkdir -p data/
if ! [[ -d data/hoover-testdata ]]; then
    echo "Downloading hoover-testdata"
    git clone https://github.com/liquidinvestigations/hoover-testdata data/hoover-testdata
fi

if ! [[ -d docker/signoz ]]; then
    echo "Downloading signoz"
    git clone https://github.com/SigNoz/signoz.git docker/signoz
fi

( cd docker/signoz && git checkout v0.73.0 && cd deploy/docker && docker compose up -d --remove-orphans)


cd docker
bash up.sh
