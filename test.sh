#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

if ! [ -d "data/hoover-testdata/data" ]; then
    echo "data/hoover-testdata/data directory not found!"
    echo "Please run the following command to download the test data:"
    echo "( mkdir -p data && cd data && git clone https://github.com/liquidinvestigations/hoover-testdata )"
    exit 1
fi

if ! [[ $@ == *"--no-capture"* ]]; then
    # export NEXTEST_TEST_THREADS=1
    # export NEXTEST_RETRIES=2
    echo
fi
# export CARGO_TARGET_DIR="target/hoover3_test"
cargo nextest run $@
