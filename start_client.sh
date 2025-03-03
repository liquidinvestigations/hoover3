#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

dx serve --package hoover3_client --platform web --bin hoover3_client_main $@