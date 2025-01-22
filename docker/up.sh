#!/bin/bash

set -ex

sudo sysctl -w fs.aio-max-nr=2560000

time docker compose up -d --remove-orphans