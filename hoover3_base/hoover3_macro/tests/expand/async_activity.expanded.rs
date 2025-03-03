use hoover3_macro::activity;
use hoover3_taskdef::declare_task_queue;
///Task queue for
///taskdef_test_macro_task_queue
pub struct TestQueue3;
impl ::hoover3_taskdef::task_inventory::TaskQueueConst for TestQueue3 {
    const QUEUE_NAME: &'static str = "taskdef_test_macro_task_queue";
}
impl ::hoover3_taskdef::task_inventory::TaskQueue for TestQueue3 {
    fn queue_name(&self) -> &'static str {
        "taskdef_test_macro_task_queue"
    }
    fn struct_name(&self) -> &'static str {
        "TestQueue3"
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
                queue_name: "taskdef_test_macro_task_queue",
                struct_name: "TestQueue3",
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
///Macro-generated unit struct that holds our
///test_macro_activity
/// activity name, input/output types, and worker registration.
#[allow(non_camel_case_types)]
pub struct test_macro_activity_activity;
impl ::hoover3_taskdef::TemporalioDescriptorName for test_macro_activity_activity {
    fn name() -> &'static str {
        "test_macro_activity"
    }
}
impl ::hoover3_taskdef::TemporalioDescriptorRegister for test_macro_activity_activity {
    fn register(worker: &mut ::hoover3_taskdef::Worker) -> anyhow::Result<()> {
        <Self as ::hoover3_taskdef::TemporalioActivityDescriptor>::register(worker)
    }
    fn queue_name() -> &'static str {
        <TestQueue3 as ::hoover3_taskdef::task_inventory::TaskQueueConst>::QUEUE_NAME
    }
}
impl ::hoover3_taskdef::TemporalioDescriptorValueTypes for test_macro_activity_activity {
    type Arg = u32;
    type Ret = u32;
}
impl ::hoover3_taskdef::TemporalioActivityDescriptor for test_macro_activity_activity {
    async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
        use futures::FutureExt;
        test_macro_activity(arg).boxed().await
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            ::hoover3_taskdef::task_inventory::ActivityDefinitionStatic {
                name: "test_macro_activity",
                queue_name: <TestQueue3 as ::hoover3_taskdef::task_inventory::TaskQueueConst>::QUEUE_NAME,
                register_fn: <test_macro_activity_activity as ::hoover3_taskdef::TemporalioDescriptorRegister>::register,
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
async fn test_macro_activity(x: u32) -> anyhow::Result<u32> {
    Ok(x)
}
