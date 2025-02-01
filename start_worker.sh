#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

# cargo watch --delay 2 -x run --workdir tasks/filesystem_scanner
(
    cd tasks/filesystem_scanner
    cargo run
)
