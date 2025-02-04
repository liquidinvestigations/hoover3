#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

export CARGO_TARGET_DIR="target/hoover3_client"
cd hoover3_client
dx serve --package hoover3_client --platform web