//! Hoover3 server library - re-exports

/// Re-export the function to migrate all databases;
pub use hoover3_database::migrate::migrate_all;


// ===================
// ===== PLUGINS =====
// ===================

/// Re-export the filesystem scanner plugin;
pub use hoover3_filesystem_scanner;

/// Re-export the data access plugin;
pub use hoover3_data_access;

/// Re-export the database operations plugin;
pub use hoover3_database_operations;