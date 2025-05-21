#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

dx build --package hoover3_client --platform web --bin hoover3_client_main

TIKA_SO2="${PWD}/$(dirname $(find target/server-dev -type f -name 'libtika_native.so' | head -n1))"
LD_LIBRARY_PATH="$TIKA_SO2:$LD_LIBRARY_PATH"
dx serve --package hoover3_client --platform web --bin hoover3_client_main $@