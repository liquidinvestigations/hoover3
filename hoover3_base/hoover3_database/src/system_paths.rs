//! System path configurations - workspace root, data root, etc.

use std::path::PathBuf;

/// Get the workspace root directory.
pub fn get_workspace_root() -> PathBuf {
    WORKSPACE_ROOT.clone()
}

lazy_static::lazy_static! {
    /// The workspace root directory global.
    pub static ref WORKSPACE_ROOT: PathBuf = get_workspace_root_inner();
}

fn get_workspace_root_inner() -> PathBuf {
    fn is_workspace_root(path: &PathBuf) -> bool {
        path.join("Cargo.toml").exists() && path.join("hoover3_base").exists() && path.join("hoover3_plugins").exists()
    }
    let mut path = std::env::current_dir().unwrap();
    let path0 = path.clone();
    while !is_workspace_root(&path) {
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            panic!("could not find workspace root: {:?}", path0);
        }
    }
    path
}

/// Get the data root directory.
pub fn get_data_root() -> PathBuf {
    get_workspace_root().join("data")
}

/// Get the docker directory.
pub fn get_docker_dir() -> PathBuf {
    get_workspace_root().join("docker")
}

/// Get the database package directory.
pub fn get_db_package_dir() -> PathBuf {
    get_workspace_root().join("hoover3_base").join("hoover3_database")
}
