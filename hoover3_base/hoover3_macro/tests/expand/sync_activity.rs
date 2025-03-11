use hoover3_macro::activity;

use hoover3_taskdef::declare_task_queue;

declare_task_queue!(TestQueue4, "taskdef_test_macro_activity", 10, 10, 1000);

/// Doc
#[activity(TestQueue4)]
fn test_macro_activity(x: u32) -> anyhow::Result<u32> {
    Ok(x)
}
