#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

if ! [[ $@ == *"--no-capture"* ]]; then
    # export NEXTEST_TEST_THREADS=1
    # export NEXTEST_RETRIES=2
    echo
fi
# export CARGO_TARGET_DIR="target/hoover3_test"
cargo nextest run $@
