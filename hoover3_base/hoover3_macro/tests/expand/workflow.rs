use hoover3_macro::workflow;

#[workflow("taskdef_test_macro_task_queue")]
async fn test_macro_workflow(_ctx: hoover3_taskdef::WfContext, x: u32) -> hoover3_taskdef::WorkflowResult<u32> {
    Ok(hoover3_taskdef::WfExitValue::Normal(x))
}
