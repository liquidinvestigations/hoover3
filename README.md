# hoover3 - search tools

## installation

- use linux
- put `fs.aio-max-nr = 1048576` in `/etc/sysctl.conf` and run `sysctl -p`
- install rustup and rust stable (1.81)
- sudo apt-get install mold clang
- install docker
- run ./start_docker.sh
- cargo install cargo-binstall
- cargo binstall cargo-watch
- cargo binstall dioxus-cli@0.6.2
- cargo binstall cargo-nextest
- run ./test.sh
- run ./migrate.sh
- run ./start_client.sh




## to do


### basic features
- [ ] admin view of all tables
    - [ ] sort/filter by any column (meilisearch/clickhouse)
    - [ ]
- [ ]


### infra
- [ ] [scylla monitoring stack](https://github.com/scylladb/scylla-monitoring.git)
- [ ] signoz + opentelemetry (server + client; client mamy require proxy)
    - [ ] trace tasks 0.1%
    - [ ] trace api calls 100%