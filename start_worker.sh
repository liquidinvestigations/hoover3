#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cargo watch -x run --workdir tasks/filesystem_scanner