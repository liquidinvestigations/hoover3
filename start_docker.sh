#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p data
mkdir -p target/doc
mkdir -p docker/data
touch docker/data/.some_data


cd docker
bash up.sh