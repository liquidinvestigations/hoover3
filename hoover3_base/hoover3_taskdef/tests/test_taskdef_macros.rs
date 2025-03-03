//! Integration tests for taskdef macros.

#![allow(unused_crate_dependencies)]
#![allow(unused_extern_crates)]

use hoover3_taskdef::*;

declare_task_queue!(TestQueue2, "taskdef_test_external_task_queue", 2, 4, 256);

#[activity(TestQueue2)]
async fn test_function_async2(_payload: u32) -> anyhow::Result<u32> {
    println!("test_function_async T={:?}", std::thread::current().id());
    Ok(_payload)
}

#[activity(TestQueue2)]
fn test_function_sync2(_payload: u32) -> anyhow::Result<u32> {
    println!("test_function_sync T={:?}", std::thread::current().id());
    Ok(_payload)
}

#[workflow(TestQueue2)]
async fn sample_workflow3(ctx: WfContext, arg: u32) -> WorkflowResult<u32> {
    println!("sample_workflow 1 T={:?}", std::thread::current().id());
    let act1 = test_function_async2_activity::run(&ctx, arg).await?;
    println!("sample_workflow 2 T={:?}", std::thread::current().id());
    let act2 = test_function_sync2_activity::run(&ctx, arg).await?;
    println!("sample_workflow 3 T={:?}", std::thread::current().id());
    Ok(WfExitValue::Normal(act1 + act2))
}

#[tokio::test]
async fn test_task_client_works_integration() {
    let _client = get_client().await.unwrap();
    let _namespaces = _client.list_namespaces().await.unwrap();
}

#[tokio::test]
async fn test_task_client_and_worker_integration() -> anyhow::Result<()> {
    let x = 4_u32;

    println!("running on main thread 1");
    spawn_worker_on_thread(TestQueue2);
    println!("{}", test_function_async2_activity::name());

    sample_workflow3_workflow::client_start(&x).await?;
    let rv2 = sample_workflow3_workflow::client_wait_for_completion(&x).await?;
    assert!(rv2 == x + x);

    sample_workflow3_workflow::client_start(&x).await?;
    Ok(())
}
