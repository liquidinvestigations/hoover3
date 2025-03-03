use hoover3_macro::workflow;
use hoover3_taskdef::declare_task_queue;
///Task queue for
///taskdef_test_macro_workflow
pub struct TestQueue5;
impl ::hoover3_taskdef::task_inventory::TaskQueueConst for TestQueue5 {
    const QUEUE_NAME: &'static str = "taskdef_test_macro_workflow";
}
impl ::hoover3_taskdef::task_inventory::TaskQueue for TestQueue5 {
    fn queue_name(&self) -> &'static str {
        "taskdef_test_macro_workflow"
    }
    fn struct_name(&self) -> &'static str {
        "TestQueue5"
    }
    fn max_concurrency(&self) -> u32 {
        10
    }
    fn max_blocking_threads(&self) -> u32 {
        10
    }
    fn max_memory_mb(&self) -> u32 {
        1000
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            ::hoover3_taskdef::task_inventory::TaskQueueStatic {
                queue_name: "taskdef_test_macro_workflow",
                struct_name: "TestQueue5",
                max_concurrency: 10,
                max_blocking_threads: 10,
                max_memory_mb: 1000,
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
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
        <TestQueue5 as ::hoover3_taskdef::task_inventory::TaskQueueConst>::QUEUE_NAME
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
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            ::hoover3_taskdef::task_inventory::WorkflowDefinitionStatic {
                name: "test_macro_workflow",
                queue_name: <TestQueue5 as ::hoover3_taskdef::task_inventory::TaskQueueConst>::QUEUE_NAME,
                register_fn: <test_macro_workflow_workflow as ::hoover3_taskdef::TemporalioDescriptorRegister>::register,
            }
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
async fn test_macro_workflow(
    _ctx: hoover3_taskdef::WfContext,
    x: u32,
) -> hoover3_taskdef::WorkflowResult<u32> {
    Ok(hoover3_taskdef::WfExitValue::Normal(x))
}
