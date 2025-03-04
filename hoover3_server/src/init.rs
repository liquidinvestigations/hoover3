//! Code related to server initialization

/// Initialize the server - load all plugins.
///
/// Does not init tracing, use `hoover3_tracing::init_tracing` for that.
pub fn init_server_plugins() -> anyhow::Result<()> {
    // TODO - dynamically loaded plugins from dynamic library file list.
    // Currently, plugins are statically linked.

    // after all plugins are loaded, check the code schema. This loads
    // all the code into the inventory, and checks it's valid.
    hoover3_database::migrate::check_code_schema();
    hoover3_taskdef::task_inventory::check_task_definitions();
    Ok(())
}

#[test]
fn test_init_server_plugins() {
    init_server_plugins().unwrap();
}
