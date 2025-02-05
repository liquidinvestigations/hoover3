#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

mkdir -p data
mkdir -p target/doc

cd docker
bash up.sh