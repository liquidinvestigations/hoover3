#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cd hoover3_database
cargo run --bin migrate