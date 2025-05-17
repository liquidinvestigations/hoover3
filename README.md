# hoover3 - search tools

## installation

- use linux
- put `fs.aio-max-nr = 1048576` in `/etc/sysctl.conf` and run `sysctl -p`
- install [rustup](https://rustup.rs/) and rust stable (1.81 or later)
- sudo apt-get install clang libmagic1 libmagic-dev zip wget curl libssl-dev pkg-config build-essential  protobuf-compiler
- install [sdkman](https://sdkman.io/) and then `sdk install java 23.0.1-graalce`
- install [docker](https://docs.docker.com/desktop/setup/install/linux/)
- run ./start_docker.sh
- cargo install cargo-binstall
- cargo binstall cargo-watch
- cargo binstall dioxus-cli@0.6.2
- cargo binstall cargo-nextest
- run ./test.sh
- run ./rebuild_docs.sh
- run ./migrate.sh
- run ./start_client.sh
- run ./start_worker.sh

## development

### scripts

This section describes the shell scripts used to run the project.

#### `./test.sh`

Run the test suite. Tests run in parallel. Should take around 20-25 seconds.

All script arguments are passed to `nextest run`. This means you can:

- select tests with `./test.sh PATTERN`, for example `./test.sh regex`
- enable printing and logging using `./test.sh --no-capture`

The test runner is nextest. Running tests with the `cargo test`
command will not work when multiple tests are selected, if those
tests use the same `tokio::sync::OnceCell` global. Nextest will avoid
this by running each test in a separate normal process.

#### `./rebuild_docs.sh`

Build documentation site for all crates in the virtual workspace.

#### `./migrate.sh`

Create / update database schemas for all databases.

Run this script before starting client or worker,
and after editing any database models or functionality.
,
#### `./start_client.sh`

Start a hot-reloading client in the fullstack configuration.

Connect to `http://localhost:8080`.

#### `./start_worker.sh`

Start all workers.

The worker process will crash if it encounters a task that was not registered when it started.

### docker containers

All development docker infrastructure is combined into a single `docker-compose.yml` file.

We deploy the following containers:

- ScyllaDB - main SQL store
- Meilisearch - search engine
- Redis - caching and locking
- MinIO - object storage for testing data loaders
- SeaweedFS - object storage for production data

## to do

### infra
- [ ] [scylla monitoring stack](https://github.com/scylladb/scylla-monitoring.git)
- [ ] signoz + opentelemetry (server + client; client may require proxy)
    - [ ] trace tasks 0.1%
    - [ ] trace api calls 100% with axum integration