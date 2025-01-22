use crate::client::get_client;
use crate::client::TemporalioClient;
use futures::Future;
use prost_wkt_types::Duration as ProstDuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use temporal_client::WorkflowClientTrait;
use temporal_client::WorkflowOptions;
use temporal_sdk::ActivityError;
use temporal_sdk::Worker;
use temporal_sdk::ActContext;
use temporal_sdk::{ActivityOptions, WfContext, WfExitValue, WorkflowResult};
use temporal_sdk_core::protos::coresdk::activity_result::{
    activity_resolution::Status::Completed, ActivityResolution,
};
use temporal_sdk_core::protos::temporal::api::common::v1::RetryPolicy;
use temporal_sdk_core::{init_worker, CoreRuntime};
use temporal_sdk_core_api::{telemetry::TelemetryOptionsBuilder, worker::WorkerConfigBuilder};
use temporal_sdk_core_protos::coresdk::AsJsonPayloadExt;

/// Global name for this Temporalio thing (activity, workflow)
pub trait TemporalioDescriptorName {
    fn name() -> &'static str;
}

/// Trait to register a Temporalio thing (activity, workflow) into a worker
pub trait TemporalioDescriptorRegister {
    fn register(worker: &mut Worker) -> anyhow::Result<()>;
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
    TemporalioDescriptorName + TemporalioDescriptorRegister
{
    type Arg: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize;
    type Ret: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize;

    fn func(arg: Self::Arg) -> impl Future<Output = Result<Self::Ret, anyhow::Error>> + Send;

    fn register(worker: &mut Worker) -> anyhow::Result<()> {
        let n = Self::name();
        let act_fn = move |_ctx: ActContext, arg: Self::Arg| async move {
            Ok(Self::func(arg).await.map_err(|e| ActivityError::from(e))?)
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
                input: input,
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
                if let Some(Completed(result)) = &resolution.status {
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
macro_rules! make_activity {
    ($id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _activity>];
            impl TemporalioDescriptorName for [<$id _activity>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl TemporalioDescriptorRegister for [<$id _activity>] {
                fn register(worker: &mut Worker) -> anyhow::Result<()> {
                    <Self as TemporalioActivityDescriptor>::register(worker)
                }
            }
            impl TemporalioActivityDescriptor for [<$id _activity>] {
                type Arg = $arg;
                type Ret = $ret;

                async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
                    tokio::task::spawn( async move { $id(arg).await }).await?
                }
            }
        }
    };
}

/// Create an activity descriptor struct called $id_activity
macro_rules! make_activity_sync {
    ($id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _activity>];
            impl TemporalioDescriptorName for [<$id _activity>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl TemporalioDescriptorRegister for [<$id _activity>] {
                fn register(worker: &mut Worker) -> anyhow::Result<()> {
                    <Self as TemporalioActivityDescriptor>::register(worker)
                }
            }
            impl TemporalioActivityDescriptor for [<$id _activity>] {
                type Arg = $arg;
                type Ret = $ret;

                async fn func(arg: Self::Arg) -> Result<Self::Ret, anyhow::Error> {
                    tokio::task::spawn_blocking(move || $id(arg)).await?
                }
            }
        }
    };
}

use temporal_sdk_core_protos::temporal::api::workflowservice::v1::StartWorkflowExecutionResponse;

/// implemented by the `make_workflow` macro
pub trait TemporalioWorkflowDescriptor:
    TemporalioDescriptorName + TemporalioDescriptorRegister
{
    type Arg: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize;
    type Ret: Send + Sync + 'static + for<'de> Deserialize<'de> + Serialize;

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

    fn client_start(
        task_queue: &str,
        workflow_id: &str,
        arg: Self::Arg,
    ) -> impl Future<Output = Result<StartWorkflowExecutionResponse, anyhow::Error>> {
        async move {
            let input = vec![arg.as_json_payload()?.into()];
            let client = get_client().await?;

            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdConflictPolicy;
            use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowIdReusePolicy;
            let _handle1: StartWorkflowExecutionResponse = client
                .start_workflow(
                    input,
                    task_queue.to_owned(),   // task queue
                    workflow_id.to_string(), // workflow id
                    Self::name().to_owned(), // workflow type
                    None,
                    WorkflowOptions {
                        id_reuse_policy: WorkflowIdReusePolicy::AllowDuplicateFailedOnly,
                        id_conflict_policy: WorkflowIdConflictPolicy::UseExisting,
                        ..Default::default()
                    },
                )
                .await?;
            Ok(_handle1)
        }
    }

    fn client_wait_for_completion(
        workflow_id: &str,
        run_id: &str,
    ) -> impl Future<Output = Result<Self::Ret, anyhow::Error>> {
        async move {
            let mut status = query_workflow_execution_status(workflow_id, run_id).await?;
            while status == WorkflowExecutionStatus::Running {
                tokio::time::sleep(Duration::from_secs_f32(0.1)).await;
                status = query_workflow_execution_status(workflow_id, run_id).await?;
            }
            if status != WorkflowExecutionStatus::Completed {
                anyhow::bail!("Workflow execution failed with status={:?}", status);
            }
            let result = query_workflow_execution_result(workflow_id, run_id).await?;
            let result: Self::Ret = serde_json::from_slice(&result)?;
            Ok(result)
        }
    }
}

/// Create a workflow descriptor struct called $id_workflow
macro_rules! make_workflow {
    ($id:ident,$arg:ty,$ret:ty) => {
        ::paste::paste! {
            #[allow(non_camel_case_types)]
            pub struct [<$id _workflow>];
            impl TemporalioDescriptorName for [<$id _workflow>] {
                fn name() -> &'static str { stringify!($id) }
            }
            impl TemporalioDescriptorRegister for [<$id _workflow>] {
                fn register(worker: &mut Worker) -> anyhow::Result<()> {
                    <Self as TemporalioWorkflowDescriptor>::register(worker)
                }
            }
            impl TemporalioWorkflowDescriptor for [<$id _workflow>] {
                type Arg = $arg;
                type Ret = $ret;

                async fn wf_func(ctx: WfContext, arg: Self::Arg) -> WorkflowResult<Self::Ret> {
                    $id(ctx, arg).await
                }
            }
        }
    };
}

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
pub fn create_worker<T: TemporalioDescriptorRegister>(
    client: TemporalioClient,
    task_queue: &str,
) -> anyhow::Result<Worker> {
    let telemetry_options = TelemetryOptionsBuilder::default().build()?;
    let runtime = CoreRuntime::new_assume_tokio(telemetry_options)?;
    let worker_build_id = format!("{task_queue}__{}", build_id());
    println!("build_id: {:?}", worker_build_id);

    let worker_config = WorkerConfigBuilder::default()
        .namespace("default")
        .task_queue(task_queue)
        .worker_build_id(worker_build_id)
        .build()?;

    let core_worker = init_worker(&runtime, worker_config, client)?;
    let mut worker = Worker::new_from_core(Arc::new(core_worker), task_queue);
    T::register(&mut worker)?;
    Ok(worker)
}

use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowExecutionStatus;
/// fetch status from client
pub async fn query_workflow_execution_status(
    workflow_id: &str,
    run_id: &str,
) -> anyhow::Result<WorkflowExecutionStatus> {
    let client = get_client().await?;
    let describe = client
        .describe_workflow_execution(workflow_id.to_string(), Some(run_id.to_string()))
        .await?;
    use anyhow::Context;
    Ok(describe
        .workflow_execution_info
        .context("no execution info")?
        .status())
}

/// fetch history from client and return last result payload
pub async fn query_workflow_execution_result(
    workflow_id: &str,
    run_id: &str,
) -> anyhow::Result<Vec<u8>> {
    let client = get_client().await?;
    use temporal_sdk_core_protos::temporal::api::enums::v1::EventType::WorkflowExecutionCompleted;
    use temporal_sdk_core_protos::temporal::api::history::v1::history_event::Attributes::WorkflowExecutionCompletedEventAttributes;
    let wf_result = client
        .get_workflow_execution_history(workflow_id.to_string(), Some(run_id.to_string()), vec![])
        .await?;
    if let Some(history) = wf_result.history {
        for event in history.events.iter().rev() {
            if event.event_type() == WorkflowExecutionCompleted {
                if let Some(attrib) = event.attributes.as_ref() {
                    if let WorkflowExecutionCompletedEventAttributes(attrib) = attrib {
                        if let Some(payloads) = &attrib.result {
                            if payloads.payloads.len() > 0 {
                                return Ok(payloads.payloads[0].data.clone());
                            }
                        }
                    }
                }
            }
        }
    };
    anyhow::bail!("Workflow execution result not found");
}

/// single-threaded worker, for testing
pub fn spawn_worker_on_thread<T: TemporalioDescriptorRegister>(task_queue: &str) {
    use tokio::task::LocalSet;
    let task_queue = task_queue.to_string();
    println!(
        "_run_on_new_thread_forever OUTSIDE  T={:?}",
        std::thread::current().id()
    );
    std::thread::spawn(move || {
        println!(
            "_run_on_new_thread_forever INSIDE  T={:?}",
            std::thread::current().id()
        );
        use tokio::runtime::Builder;
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let local = LocalSet::new();

        let task_queue = task_queue.to_string();
        local.spawn_local(async move {
            println!(
                "_run_on_new_thread_forever inside spawn_local  T={:?}",
                std::thread::current().id()
            );
            let client = get_client().await.unwrap();
            let mut worker = create_worker::<T>(client, &task_queue).unwrap();
            worker.run().await.unwrap();
        });

        // This will return once all senders are dropped and all
        // spawned tasks have returned.
        rt.block_on(local);
    });
}

/// for execution directly in main thread; cannot be passed to tokio::task::spawn
pub async fn run_worker<T: TemporalioDescriptorRegister>(task_queue: &str) -> anyhow::Result<()> {
    let client = get_client().await?;
    let mut worker = create_worker::<T>(client, task_queue)?;
    worker.run().await?;
    Ok(())
}

pub mod test {
    use super::*;

    make_activity!(test_function_async, u32, u32);
    async fn test_function_async(_payload: u32) -> Result<u32, anyhow::Error> {
        println!("test_function_async T={:?}", std::thread::current().id());
        Ok(_payload)
    }

    make_activity_sync!(test_function_sync, u32, u32);
    fn test_function_sync(_payload: u32) -> Result<u32, anyhow::Error> {
        println!("test_function_sync T={:?}", std::thread::current().id());
        Ok(_payload)
    }

    make_workflow!(sample_workflow2, u32, u32);
    pub async fn sample_workflow2(ctx: WfContext, arg: u32) -> WorkflowResult<u32> {
        println!("sample_workflow T={:?}", std::thread::current().id());
        test_function_async(arg).await?;
        let act1 = test_function_async_activity::run(&ctx, arg).await?;
        let act2 = test_function_sync_activity::run(&ctx, arg).await?;
        println!("sample_workflow 2  T={:?}", std::thread::current().id());
        Ok(WfExitValue::Normal(act1 + act2))
    }

    #[tokio::test]
    async fn test_client_works() {
        let _client = get_client().await.unwrap();
        let _namespaces = _client.list_namespaces().await.unwrap();
        // println!("namespaces: {:#?}", _namespaces.len());
    }

    #[tokio::test]
    async fn test_client_and_worker() -> anyhow::Result<()> {
        let x = 4_u32;
        let task_queue = "test_client_and_worker";

        println!("running on main thread 1");
        spawn_worker_on_thread::<(
            test_function_async_activity,
            test_function_sync_activity,
            sample_workflow2_workflow,
        )>(task_queue);
        println!("{}", test_function_async_activity::name());

        let workflow_id = format!(
            "test_client_and_worker_workflow_id_1_{:?}",
            std::time::SystemTime::now()
        );
        let handle2 = sample_workflow2_workflow::client_start(task_queue, &workflow_id, x).await?;
        let rv2 =
            sample_workflow2_workflow::client_wait_for_completion(&workflow_id, &handle2.run_id)
                .await?;
        assert!(rv2 == x + x);
        Ok(())
    }
}
