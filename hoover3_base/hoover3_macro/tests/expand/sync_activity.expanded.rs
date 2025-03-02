use hoover3_macro::activity;
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
        "taskdef_test_macro_task_queue"
    }
}
impl ::hoover3_taskdef::TemporalioDescriptorValueTypes for test_macro_activity_activity {
    type Arg = u32;
    type Ret = u32;
}
impl ::hoover3_taskdef::TemporalioActivityDescriptor for test_macro_activity_activity {
    async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
        tokio::task::spawn_blocking(move || test_macro_activity(arg)).await?
    }
}
fn test_macro_activity(x: u32) -> anyhow::Result<u32> {
    Ok(x)
}
