#!/bin/bash
set -ex
cd "$(dirname "${BASH_SOURCE[0]}")"

if ! docker ps | grep -q hoover3-worker; then
(
    cd docker/hoover3_worker
    docker build -t hoover3-worker .
)
(
    mkdir -p target/hoover3_worker
    mkdir -p .worker_temp
    docker run -d \
        --user "$(id -u):$(id -g)" \
        --name hoover3-worker \
        -v "$(pwd):/app:ro" \
        -v "$(pwd)/target/hoover3_worker:/app/target" \
        -v "$(pwd)/.worker_temp:/temp_disk_big" \
        --tmpfs /temp_ramdisk_small:noexec,nosuid,nodev,size=4g,uid=$(id -u),gid=$(id -g) \
        --tmpfs /.gradle:size=4g,uid=$(id -u),gid=$(id -g) \
        -w /app \
        -e HOOVER3_WORKER_TEMP_DISK_BIG=/temp_disk_big \
        -e HOOVER3_WORKER_TEMP_RAMDISK_SMALL=/temp_ramdisk_small \
        --net host \
        hoover3-worker \
        sleep infinity
)
fi
docker exec -it hoover3-worker bash -ex /app/docker/hoover3_worker/__start_worker_in_container.sh