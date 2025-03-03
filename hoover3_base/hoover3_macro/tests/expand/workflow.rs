use hoover3_macro::workflow;

use hoover3_taskdef::declare_task_queue;

declare_task_queue!(TestQueue5, "taskdef_test_macro_workflow", 10, 10, 1000);

#[workflow(TestQueue5)]
async fn test_macro_workflow(_ctx: hoover3_taskdef::WfContext, x: u32) -> hoover3_taskdef::WorkflowResult<u32> {
    Ok(hoover3_taskdef::WfExitValue::Normal(x))
}
