use crate::client::get_client;
pub use anyhow;
pub use futures::Future;
pub use serde;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

pub use crate::client::TemporalioClient;
pub use prost_wkt_types::Duration as ProstDuration;
pub use temporal_client::WorkflowClientTrait;
pub use temporal_client::WorkflowOptions;
pub use temporal_sdk::ActContext;
pub use temporal_sdk::ActivityError;
pub use temporal_sdk::StartedChildWorkflow;
pub use temporal_sdk::Worker;
pub use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
pub use temporal_sdk_core::protos::coresdk::activity_result::activity_resolution::Status;
pub use temporal_sdk_core::protos::coresdk::activity_result::ActivityResolution;
pub use temporal_sdk_core::protos::temporal::api::common::v1::RetryPolicy;
pub use temporal_sdk_core::{init_worker, CoreRuntime};
pub use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;
pub use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowExecutionStatus;
pub use temporal_sdk_core_protos::temporal::api::workflowservice::v1::StartWorkflowExecutionResponse;

pub const TEMPORALIO_NAMESPACE: &str = "default";
/// Global name for this Temporalio thing (activity, workflow)
pub trait TemporalioDescriptorName {
    fn name() -> &'static str;
}

/// Trait to register a Temporalio thing (activity, workflow) into a worker
pub trait TemporalioDescriptorRegister {
    fn queue_name() -> &'static str;
    fn register(worker: &mut Worker) -> anyhow::Result<()>;
}

pub trait TemporalioDescriptorValueTypes {
    type Arg: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize + Clone;
    type Ret: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize;
}

/// If T1 and T2 can be registered, then so can (T1,T2) and (T1, (T2, T3))
macro_rules! impl_tuple_register {
    ($($T:ident),+) => {
        impl<$($T: TemporalioDescriptorRegister),+> TemporalioDescriptorRegister for ($($T,)+) {
            fn register(worker: &mut Worker) -> anyhow::Result<()> {
                $(
                    $T::register(worker)?;
                )+
                Ok(())
            }
            fn queue_name() -> &'static str {
                let mut q = vec![
                    $(
                        $T::queue_name(),
                    )+
                ];
                q.dedup();
                assert!(q.len() == 1,
                    "all activities and workflows under a worker must have the same queue name");
                q[0]
            }
        }
    };
}
impl_tuple_register!(T1);
impl_tuple_register!(T1, T2);
impl_tuple_register!(T1, T2, T3);
impl_tuple_register!(T1, T2, T3, T4);
impl_tuple_register!(T1, T2, T3, T4, T5);
impl_tuple_register!(T1, T2, T3, T4, T5, T6);
impl_tuple_register!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_register!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_register!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_register!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

/// implemented by the `make_activity` macro
pub trait TemporalioActivityDescriptor:
    TemporalioDescriptorName + TemporalioDescriptorRegister + TemporalioDescriptorValueTypes
{
    fn func(arg: Self::Arg) -> impl Future<Output = Result<Self::Ret, anyhow::Error>> + Send;

    fn register(worker: &mut Worker) -> anyhow::Result<()> {
        let n = Self::name();
        let act_fn = move |_ctx: ActContext, arg: Self::Arg| async move {
            Self::func(arg).await.map_err(ActivityError::from)
        };
        worker.register_activity(n, act_fn);
        Ok(())
    }

    fn run(
        wf_ctx: &WfContext,
        arg: Self::Arg,
    ) -> impl Future<Output = Result<Self::Ret, anyhow::Error>> {
        let wf_ctx = wf_ctx.clone();
        async move {
            let Ok(input) = arg.as_json_payload() else {
                anyhow::bail!("Error serializing argument for activity {}", Self::name());
            };
            let opt = ActivityOptions {
                activity_type: Self::name().to_string(),
                input,
                task_queue: Some(Self::queue_name().to_string()),
                retry_policy: Some(RetryPolicy {
                    initial_interval: Some(ProstDuration {
                        seconds: 1,
                        nanos: 50_000_000, // 50ms
                    }),
                    maximum_attempts: 2,
                    ..Default::default()
                }),
                start_to_close_timeout: Some(Duration::from_secs(30)),
                ..Default::default()
            };

            let resolution: ActivityResolution = wf_ctx.activity(opt).await;
            if resolution.completed_ok() {
                if let Some(Status::Completed(result)) = &resolution.status {
                    if let Some(payload) = &result.result {
                        let result: Self::Ret = serde_json::from_slice(&payload.data)?;
                        return Ok(result);
                    }
                } else {
                    anyhow::bail!("Activity failed with status={:?}", resolution.status);
                }
            }
            anyhow::bail!("Activity did not complete");
        }
    }
}

/// Create an activity descriptor struct called $id_activity
#[macro_export]
macro_rules! make_activity {
    ($queue_name:expr,$id:ident,$arg:ty,$ret:ty) => {
        $crate::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _activity>];
            impl $crate::TemporalioDescriptorName for [<$id _activity>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl $crate::TemporalioDescriptorRegister for [<$id _activity>] {
                fn register(worker: &mut $crate::Worker) -> anyhow::Result<()> {
                    <Self as $crate::TemporalioActivityDescriptor>::register(worker)
                }
                fn queue_name() -> &'static str { $queue_name }
            }
            impl $crate::TemporalioDescriptorValueTypes for [<$id _activity>] {
                type Arg = $arg;
                type Ret = $ret;
            }
            impl $crate::TemporalioActivityDescriptor for [<$id _activity>] {
                async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
                    $id(arg).await
                }
            }
        }
    };
}
pub use make_activity;

/// Create an activity descriptor struct called $id_activity
#[macro_export]
macro_rules! make_activity_sync {
    ($queue_name:expr,$id:ident,$arg:ty,$ret:ty) => {
        $crate::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _activity>];
            impl $crate::TemporalioDescriptorName for [<$id _activity>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl $crate::TemporalioDescriptorRegister for [<$id _activity>] {
                fn register(worker: &mut $crate::Worker) -> anyhow::Result<()> {
                    <Self as $crate::TemporalioActivityDescriptor>::register(worker)
                }
                fn queue_name() -> &'static str { $queue_name }
            }
            impl $crate::TemporalioDescriptorValueTypes for [<$id _activity>] {
                type Arg = $arg;
                type Ret = $ret;
            }
            impl $crate::TemporalioActivityDescriptor for [<$id _activity>] {

                async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
                    tokio::task::spawn_blocking(move || $id(arg)).await?
                }
            }
        }
    };
}
pub use make_activity_sync;

pub enum ChildWorkflowFuture<T: Sized + TemporalioWorkflowDescriptor> {
    Running(StartedChildWorkflow),
    AlreadyCompleted(T::Arg, PhantomData<T>),
}

impl<T: Sized + TemporalioWorkflowDescriptor> ChildWorkflowFuture<T> {
    pub fn result(self) -> impl Future<Output = Result<T::Ret, anyhow::Error>> {
        use anyhow::Context;
        use temporal_sdk_core_protos::coresdk::child_workflow::child_workflow_result::Status as ChildWorkflowStatus;

        async move {
            match self {
                ChildWorkflowFuture::Running(started) => {
                    let child_result = started.result().await;
                    let child_result_status = child_result.status
                        .as_ref()
                        .context("no child result status")?;
                    match child_result_status {
                        ChildWorkflowStatus::Completed(r) => {
                            let result_payload = r.result.as_ref().context("no result payload")?;
                            let result: T::Ret = serde_json::from_slice(&result_payload.data)?;
                            Ok(result)
                        }
                        _ => anyhow::bail!("child workflow failed: {:?}", child_result_status),
                    }
                }
                ChildWorkflowFuture::AlreadyCompleted(arg, _) => {
                    let wf_id = T::workflow_id(&arg);
                    let payload = query_workflow_execution_result(&wf_id).await?;
                    let result: T::Ret = serde_json::from_slice(&payload)?;
                    Ok(result)
                }
            }
        }
    }
}

/// implemented by the `make_workflow` macro
pub trait TemporalioWorkflowDescriptor:
    TemporalioDescriptorName + TemporalioDescriptorRegister + TemporalioDescriptorValueTypes
{
    fn wf_func(
        ctx: WfContext,
        arg: Self::Arg,
    ) -> impl Future<Output = WorkflowResult<Self::Ret>> + Send;

    fn register(worker: &mut Worker) -> anyhow::Result<()> {
        let n = Self::name();
        let wf_fn = move |ctx: WfContext| async move {
            let arg: Self::Arg = serde_json::from_slice(&ctx.get_args()[0].data)?;
            Self::wf_func(ctx, arg).await
        };
        worker.register_wf(n, wf_fn);
        Ok(())
    }

    fn workflow_id(arg: &Self::Arg) -> String {
        format!(
            "{}_{}",
            Self::name(),
            hoover3_types::stable_hash::stable_hash(arg).unwrap()
        )
    }

    fn client_start(arg: &Self::Arg) -> impl Future<Output = Result<(), anyhow::Error>> {
        async move {
            let workflow_id = Self::workflow_id(arg);
            let input = vec![arg.as_json_payload()?];
            let client = get_client().await?;

            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdConflictPolicy;
            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
            let _handle1 = client
                .start_workflow(
                    input,
                    Self::queue_name().to_owned(), // task queue
                    workflow_id.to_string(),       // workflow id
                    Self::name().to_owned(),       // workflow type
                    None,
                    WorkflowOptions {
                        id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicateFailedOnly,
                        id_conflict_policy: WorkflowIdConflictPolicy::UseExisting,
                        ..Default::default()
                    },
                )
                .await;
            match _handle1 {
                Ok(_resp) => {
                    Ok(())
                }
                Err(e) => {
                    if e.code() == tonic::Code::AlreadyExists {
                        return Ok(());
                    }
                    warn!("error starting workflow {:?}", e);
                    anyhow::bail!("error starting workflow {:?}", e);
                }
            }
        }
    }
    fn run_as_child(
        wf_ctx: &WfContext,
        arg: Self::Arg,
    ) -> impl Future<Output = Result<Self::Ret, anyhow::Error>>
    where
        Self: Sized,
    {
        async move { Self::start_as_child(wf_ctx, arg).await?.result().await }
    }

    fn run_parallel(
        wf_ctx: &WfContext,
        args: Vec<Self::Arg>,
    ) -> impl Future<Output = anyhow::Result<Vec<(Self::Arg, anyhow::Result<Self::Ret>)>>>
    where
        Self: Sized,
    {
        use futures::StreamExt;
        async move {
            let mut fut_1 = futures::stream::FuturesUnordered::new();
            for arg in args.into_iter() {
                fut_1.push(async move { (arg.clone(), Self::start_as_child(wf_ctx, arg).await) });
            }

            let mut fut_2 = futures::stream::FuturesUnordered::new();
            while let Some((arg, res)) = fut_1.next().await {
                fut_2.push(async move {
                    (
                        arg,
                        match res {
                            Ok(res) => res.result().await,
                            Err(e) => Err(e),
                        },
                    )
                });
            }

            let mut results = Vec::new();
            while let Some((arg, res)) = fut_2.next().await {
                results.push((arg, res));
            }
            Ok(results)
        }
    }

    fn start_as_child(
        wf_ctx: &WfContext,
        arg: Self::Arg,
    ) -> impl Future<Output = Result<ChildWorkflowFuture<Self>, anyhow::Error>>
    where
        Self: Sized,
    {
        let arg = arg.clone();
        let wf_ctx = wf_ctx.clone();
        async move {
            let arg = arg.clone();
            let wf_ctx = wf_ctx.clone();

            let workflow_id = Self::workflow_id(&arg);
            let input = vec![arg.as_json_payload()?];

            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdConflictPolicy;
            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
            use temporal_sdk::ChildWorkflowOptions;
            use temporal_sdk_core_protos::coresdk::workflow_activation::resolve_child_workflow_execution_start::Status as ChildWorkflowStartStatus;

            use temporal_sdk_core_protos::coresdk::child_workflow::StartChildWorkflowExecutionFailedCause;

            let child_wf = wf_ctx.child_workflow(ChildWorkflowOptions {
                workflow_id: workflow_id.to_string(),
                workflow_type: Self::name().to_string(),
                task_queue: None, // inherit
                input,
                options: WorkflowOptions {
                    id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicateFailedOnly,
                    id_conflict_policy: WorkflowIdConflictPolicy::UseExisting,
                    ..Default::default()
                },
                ..Default::default()
            });
            let start_result = child_wf.start(&wf_ctx).await;
            match &start_result.status {
                ChildWorkflowStartStatus::Succeeded(_run_id) => {
                    Ok(ChildWorkflowFuture::Running(
                        start_result.into_started().unwrap(),
                    ))
                }
                ChildWorkflowStartStatus::Failed(s) => match s.cause() {
                    StartChildWorkflowExecutionFailedCause::WorkflowAlreadyExists => {
                        let arg: Self::Arg = arg.clone();
                        Ok(ChildWorkflowFuture::AlreadyCompleted(arg, PhantomData))
                    }
                    _e => anyhow::bail!("child workflow start failed: {:?} : {:?}", _e, s),
                },
                _ => {
                    anyhow::bail!("child workflow start failed: {:?}", start_result.status);
                }
            }
        }
    }

    fn client_wait_for_completion(
        arg: &Self::Arg,
    ) -> impl Future<Output = Result<Self::Ret, anyhow::Error>> {
        async move {
            let workflow_id = Self::workflow_id(arg);
            let mut status = query_workflow_execution_status(&workflow_id).await?;
            let mut dt = 0.1;
            while status == WorkflowExecutionStatus::Running {
                tokio::time::sleep(Duration::from_secs_f32(dt)).await;
                dt = dt * 1.1 + 0.1;
                status = query_workflow_execution_status(&workflow_id).await?;
            }
            if status != WorkflowExecutionStatus::Completed {
                anyhow::bail!("Workflow execution failed with status={:?}", status);
            }
            let result = query_workflow_execution_result(&workflow_id).await?;
            let result: Self::Ret = serde_json::from_slice(&result)?;
            Ok(result)
        }
    }
}

/// Create a workflow descriptor struct called $id_workflow
#[macro_export]
macro_rules! make_workflow {
    ($queue_name:expr,$id:ident,$arg:ty,$ret:ty) => {
        $crate::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _workflow>];
            impl $crate::TemporalioDescriptorName for [<$id _workflow>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl $crate::TemporalioDescriptorRegister for [<$id _workflow>] {
                fn register(worker: &mut $crate::Worker) -> $crate::anyhow::Result<()> {
                    <Self as $crate::TemporalioWorkflowDescriptor>::register(worker)
                }
                fn queue_name() -> &'static str { $queue_name }
            }
            impl $crate::TemporalioDescriptorValueTypes for [<$id _workflow>] {
                type Arg = $arg;
                type Ret = $ret;
            }
            impl $crate::TemporalioWorkflowDescriptor for [<$id _workflow>] {
                async fn wf_func(ctx: $crate::WfContext, arg: Self::Arg) -> $crate::WorkflowResult<Self::Ret> {
                    $id(ctx, arg).await
                }
            }
        }
    };
}
pub use make_workflow;

fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

fn build_id() -> String {
    bytes_to_hex_string(buildid::build_id().unwrap_or(b"unknown"))
}

/// build new worker using given activity list, workflow , client, etc.
fn create_worker<T: TemporalioDescriptorRegister>(
    client: TemporalioClient,
) -> anyhow::Result<Worker> {
    use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
    let queue_name = T::queue_name();
    let worker_build_id = format!("{queue_name}__{}", build_id());
    info!("worker build_id: {:?}", worker_build_id);

    let worker_config = WorkerConfigBuilder::default()
        .namespace(TEMPORALIO_NAMESPACE)
        .task_queue(queue_name)
        .worker_build_id(worker_build_id)
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), queue_name);
    T::register(&mut worker)?;
    Ok(worker)
}

/// fetch status from client
async fn query_workflow_execution_status(
    workflow_id: &str,
) -> anyhow::Result<WorkflowExecutionStatus> {
    let client = get_client().await?;
    let describe = client
        .describe_workflow_execution(workflow_id.to_string(), None)
        .await?;
    use anyhow::Context;
    Ok(describe
        .workflow_execution_info
        .context("no execution info")?
        .status())
}

/// fetch history from client and return last result payload
async fn query_workflow_execution_result(workflow_id: &str) -> anyhow::Result<Vec<u8>> {
    let client = get_client().await?;
    use temporal_sdk_core_protos::temporal::api::enums::v1::EventType::WorkflowExecutionCompleted;
    use temporal_sdk_core_protos::temporal::api::history::v1::history_event::Attributes::WorkflowExecutionCompletedEventAttributes;
    let wf_result = client
        .get_workflow_execution_history(workflow_id.to_string(), None, vec![])
        .await?;
    if let Some(history) = wf_result.history {
        for event in history.events.iter().rev() {
            if event.event_type() == WorkflowExecutionCompleted {
                if let Some(WorkflowExecutionCompletedEventAttributes(attrib)) = event.attributes.as_ref() {
                    if let Some(payloads) = &attrib.result {
                        if !payloads.payloads.is_empty() {
                            return Ok(payloads.payloads[0].data.clone());
                        }
                    }
                }
            }
        }
    };
    anyhow::bail!("Workflow execution result not found");
}

pub fn spawn_worker_on_thread<T: TemporalioDescriptorRegister>() -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        tracing::info!(
            "_run_on_new_thread_forever INSIDE  T={:?}",
            std::thread::current().id()
        );
        use tokio::runtime::Builder;
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            run_worker::<T>().await.unwrap();
        });
    })
}

/// for execution directly in main thread; cannot be passed to tokio::task::spawn
pub async fn run_worker<T: TemporalioDescriptorRegister>() -> anyhow::Result<()> {
    let client = get_client().await?;
    let mut worker = create_worker::<T>(client)?;
    worker.run().await?;
    Ok(())
}

pub mod test {
    use super::*;

    make_activity!("taskdef_test_internal", test_function_async, u32, u32);
    async fn test_function_async(_payload: u32) -> Result<u32, anyhow::Error> {
        println!("test_function_async T={:?}", std::thread::current().id());
        Ok(_payload)
    }

    make_activity_sync!("taskdef_test_internal", test_function_sync, u32, u32);
    fn test_function_sync(_payload: u32) -> Result<u32, anyhow::Error> {
        println!("test_function_sync T={:?}", std::thread::current().id());
        Ok(_payload)
    }

    make_workflow!("taskdef_test_internal", sample_workflow2, u32, u32);
    pub async fn sample_workflow2(ctx: WfContext, arg: u32) -> WorkflowResult<u32> {
        println!("sample_workflow 1 T={:?}", std::thread::current().id());
        let act1 = test_function_async_activity::run(&ctx, arg).await?;
        println!("sample_workflow 2 T={:?}", std::thread::current().id());
        let act2 = test_function_sync_activity::run(&ctx, arg).await?;
        println!("sample_workflow 3 T={:?}", std::thread::current().id());
        Ok(WfExitValue::Normal(act1 + act2))
    }

    #[tokio::test]
    async fn test_task_client_works() {
        let _client = get_client().await.unwrap();
        let _namespaces = _client.list_namespaces().await.unwrap();
    }

    #[tokio::test]
    async fn test_task_client_and_worker() -> anyhow::Result<()> {
        let x = 4_u32;

        println!("running on main thread 1");
        spawn_worker_on_thread::<(
            test_function_async_activity,
            test_function_sync_activity,
            sample_workflow2_workflow,
        )>();
        println!("{}", test_function_async_activity::name());

        sample_workflow2_workflow::client_start(&x).await?;
        let rv2 = sample_workflow2_workflow::client_wait_for_completion(&x).await?;
        assert!(rv2 == x + x);

        sample_workflow2_workflow::client_start(&x).await?;
        Ok(())
    }
}
