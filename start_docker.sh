#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p data
mkdir -p target/doc
mkdir -p docker/data
touch docker/data/.some_data

mkdir -p data/
if ! [[ -d data/hoover-testdata ]]; then
    echo "Downloading hoover-testdata"
    git clone https://github.com/liquidinvestigations/hoover-testdata data/hoover-testdata
fi



cd docker
bash up.sh