use hoover3_types::docker_health::ContainerHealthUi;
use std::path::PathBuf;

use crate::{db_management::redis::with_redis_cache, migrate::get_package_dir};

/// Client API method for generating health check dashboard. Connects to docker using CLI.
pub async fn get_container_status(c: ()) -> anyhow::Result<Vec<ContainerHealthUi>> {
    with_redis_cache("get_container_status", 60, _get_container_status, &c).await
}

async fn _get_container_status(_c: ()) -> anyhow::Result<Vec<ContainerHealthUi>> {
    let list = docker_execute(&["compose", "ps", "-q"]).await?;
    let list = list.trim().lines().map(&str::trim).collect::<Vec<_>>();
    let mut v = vec![];
    for id in list {
        v.push(ContainerHealthUi {
            container_id: id[..12].to_string(),
            container_name: docker_inspect_pattern(id, ".Name").await,
            container_running: docker_inspect_pattern(id, ".State.Status").await,
            container_health: docker_inspect_pattern(id, ".State.Health.Status").await,
        })
    }
    Ok(v)
}

fn get_docker_dir() -> PathBuf {
    get_package_dir()
        .parent()
        .unwrap()
        .join("docker")
        .to_path_buf()
}

async fn docker_inspect_pattern(container_id: &str, pattern: &str) -> String {
    let format_string = format!("{{{{ {} }}}}", pattern);

    docker_execute(&["inspect", "--format", &format_string, container_id])
        .await
        .unwrap_or("-err-".to_string())
}

async fn docker_execute(args: &[&str]) -> anyhow::Result<String> {
    let mut c = tokio::process::Command::new("docker");
    for a in args {
        c.arg(a);
    }
    let b = c.current_dir(get_docker_dir()).output().await?.stdout;
    Ok(String::from_utf8(b)?.trim().to_string())
}

#[tokio::test]
async fn test_container_status_list() {
    let v = get_container_status(()).await.unwrap();
    assert!(v.len() > 0, "we assume some containers exist");
}

#[tokio::test]
async fn test_docker_execute() {
    let v = docker_execute(&["ps", "-qa"]).await.unwrap();
    assert!(v.len() > 0, "we assume some containers exist");
}
