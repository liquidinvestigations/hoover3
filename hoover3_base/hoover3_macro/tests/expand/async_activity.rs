use hoover3_macro::activity;

#[activity("taskdef_test_macro_task_queue")]
async fn test_macro_activity(x: u32) -> anyhow::Result<u32> {
    Ok(x)
}
