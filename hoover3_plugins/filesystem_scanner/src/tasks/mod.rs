//! Tasks for the filesystem scanner plugin.
//! Specifically, the shared queue definition.

use hoover3_taskdef::declare_task_queue;

declare_task_queue!(
    FilesystemScannerQueue,
    "filesystem_scanner",
    8,    // 8 concurrent workflows
    64,   // 64 max i/o threads
    2048  // 2048 MB ram worker total
);

pub mod hash_files;
pub mod hash_files_plan;
pub mod process_plan;
pub mod scan_filesystem;
