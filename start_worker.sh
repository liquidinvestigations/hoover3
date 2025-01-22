#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cd hoover3_tasks
cargo watch -x run -w src -w ../hoover3_database/src -w ../hoover3_types/src