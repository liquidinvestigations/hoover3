#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

cd hoover3_client
dx serve --package hoover3_client --platform web --addr 192.168.1.147