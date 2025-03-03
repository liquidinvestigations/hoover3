use hoover3_macro::activity;

use hoover3_taskdef::declare_task_queue;

declare_task_queue!(TestQueue3, "taskdef_test_macro_task_queue", 10, 10, 1000);

#[activity(TestQueue3)]
async fn test_macro_activity(x: u32) -> anyhow::Result<u32> {
    Ok(x)
}
