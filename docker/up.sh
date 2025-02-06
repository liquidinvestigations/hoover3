#!/bin/bash

set -ex

( bash -c "sudo sysctl -w fs.aio-max-nr=1048576" ) || true

time docker compose up -d --remove-orphans