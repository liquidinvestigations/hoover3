use hoover3_macro::workflow;
#[allow(non_camel_case_types)]
///Macro-generated unit struct that holds our
///test_macro_workflow
/// workflow name, input/output types, and worker registration.
pub struct test_macro_workflow_workflow;
impl ::hoover3_taskdef::TemporalioDescriptorName for test_macro_workflow_workflow {
    fn name() -> &'static str {
        "test_macro_workflow"
    }
}
impl ::hoover3_taskdef::TemporalioDescriptorRegister for test_macro_workflow_workflow {
    fn register(
        worker: &mut ::hoover3_taskdef::Worker,
    ) -> ::hoover3_taskdef::anyhow::Result<()> {
        <Self as ::hoover3_taskdef::TemporalioWorkflowDescriptor>::register(worker)
    }
    fn queue_name() -> &'static str {
        "taskdef_test_macro_task_queue"
    }
}
impl ::hoover3_taskdef::TemporalioDescriptorValueTypes for test_macro_workflow_workflow {
    type Arg = u32;
    type Ret = u32;
}
impl ::hoover3_taskdef::TemporalioWorkflowDescriptor for test_macro_workflow_workflow {
    async fn wf_func(
        ctx: ::hoover3_taskdef::WfContext,
        arg: Self::Arg,
    ) -> ::hoover3_taskdef::WorkflowResult<Self::Ret> {
        test_macro_workflow(ctx, arg).await
    }
}
async fn test_macro_workflow(
    _ctx: hoover3_taskdef::WfContext,
    x: u32,
) -> hoover3_taskdef::WorkflowResult<u32> {
    Ok(hoover3_taskdef::WfExitValue::Normal(x))
}
