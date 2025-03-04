//! Task definition inventory - static lists of task queues, activities, workflows.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use crate::Worker;

/// Inventory task queue definition.
pub struct TaskQueueStatic {
    /// Name of the task queue.
    pub queue_name: &'static str,
    /// Name of the struct.
    pub struct_name: &'static str,
    /// Maximum number of concurrent tasks in the queue.
    pub max_concurrency: u32,
    /// Maximum number of blocking threads in the queue.
    pub max_blocking_threads: u32,
    /// Maximum memory limit in MB.
    pub max_memory_mb: u32,
}

inventory::collect!(TaskQueueStatic);

/// Trait for task queues.
pub trait TaskQueue: Send + Sync + 'static {
    /// Name of the task queue.
    fn queue_name(&self) -> &'static str;
    /// Name of the struct.
    fn struct_name(&self) -> &'static str;
    /// Maximum number of concurrent tasks in the queue.
    fn max_concurrency(&self) -> u32;
    /// Maximum number of blocking threads in the queue.
    fn max_blocking_threads(&self) -> u32;
    /// Maximum memory limit in MB.
    fn max_memory_mb(&self) -> u32;
}

/// Trait for task queues with constant values. Useful for macro generation. Get other values from inventory.
pub trait TaskQueueConst {
    /// Queue name as const, used in macro generation.
    const QUEUE_NAME: &'static str;
}

impl TaskQueue for TaskQueueStatic {
    fn queue_name(&self) -> &'static str {
        self.queue_name
    }
    fn struct_name(&self) -> &'static str {
        self.struct_name
    }
    fn max_concurrency(&self) -> u32 {
        self.max_concurrency
    }
    fn max_blocking_threads(&self) -> u32 {
        self.max_blocking_threads
    }
    fn max_memory_mb(&self) -> u32 {
        self.max_memory_mb
    }
}

/// Declare a task queue in the inventory.
/// Arguments:
/// - name of struct (identifier)
/// - name of queue (string literal)
/// - max concurrency (u32 literal) - example: 8
/// - max blocking threads (u32 literal) - example: 64
#[macro_export]
macro_rules! declare_task_queue {
    ($struct_name:ident, $name:expr, $max_concurrency:expr, $max_blocking_threads:expr, $max_memory_mb:expr) => {
        #[doc = "Task queue for "]
        #[doc = $name]
        pub struct $struct_name;
        impl $crate::task_inventory::TaskQueueConst for $struct_name {
            const QUEUE_NAME: &'static str = $name;
        }
        impl $crate::task_inventory::TaskQueue for $struct_name {
            fn queue_name(&self) -> &'static str {
                $name
            }
            fn struct_name(&self) -> &'static str {
                stringify!($struct_name)
            }
            fn max_concurrency(&self) -> u32 {
                $max_concurrency
            }
            fn max_blocking_threads(&self) -> u32 {
                $max_blocking_threads
            }
            fn max_memory_mb(&self) -> u32 {
                $max_memory_mb
            }
        }
        $crate::inventory::submit!($crate::task_inventory::TaskQueueStatic {
            queue_name: $name,
            struct_name: stringify!($struct_name),
            max_concurrency: $max_concurrency,
            max_blocking_threads: $max_blocking_threads,
            max_memory_mb: $max_memory_mb,
        });
    };
}
pub use declare_task_queue;
use hoover3_types::tasks::{
    ActivityDefinition, AllTaskDefinitions, WorkerQueueDefinition, WorkflowDefinition,
};
use tracing::info;

#[cfg(test)]
mod test_declare_task_queue_compiles {
    declare_task_queue!(TestQueue, "test_queue", 8, 64, 1024);
}

/// Inventory activity definition.
pub struct ActivityDefinitionStatic {
    /// Name of the activity.
    pub name: &'static str,
    /// Task queue name.
    pub queue_name: &'static str,
    /// Registration function
    pub register_fn: fn(&mut Worker) -> anyhow::Result<()>,
}

inventory::collect!(ActivityDefinitionStatic);

/// Inventory workflow definition.
pub struct WorkflowDefinitionStatic {
    /// Name of the workflow.
    pub name: &'static str,
    /// Task queue name.
    pub queue_name: &'static str,
    /// Registration function
    pub register_fn: fn(&mut Worker) -> anyhow::Result<()>,
}

inventory::collect!(WorkflowDefinitionStatic);

pub(crate) fn list_task_register_fns_for_queue(
    queue_name: &str,
) -> Vec<(String, fn(&mut Worker) -> anyhow::Result<()>)> {
    let mut result = Vec::new();
    for activity in inventory::iter::<ActivityDefinitionStatic> {
        if activity.queue_name == queue_name {
            result.push((activity.name.to_string(), activity.register_fn));
        }
    }
    for workflow in inventory::iter::<WorkflowDefinitionStatic> {
        if workflow.queue_name == queue_name {
            result.push((workflow.name.to_string(), workflow.register_fn));
        }
    }
    result
}

/// Check all task definitions.
/// This will panic if there are any issues.
pub fn check_task_definitions() {
    info!("checking task definitions...");
    let task_definitions = get_task_definitions_from_inventory();
    info!(
        "found {} worker queues: {:#?}",
        task_definitions.worker_queues.len(),
        task_definitions.worker_queues.keys()
    );
    info!(
        "found {} activities: {:#?}",
        task_definitions.activities.len(),
        task_definitions.activities.keys()
    );
    info!(
        "found {} workflows: {:#?}",
        task_definitions.workflows.len(),
        task_definitions.workflows.keys()
    );
    info!("task definitions ok!");
}

/// Read all task definitions from inventory.
/// Load the results into a global - and only compute them once.
pub fn get_task_definitions_from_inventory() -> Arc<AllTaskDefinitions> {
    TASK_DEFINITIONS.clone()
}

lazy_static::lazy_static! {
    static ref TASK_DEFINITIONS: Arc<AllTaskDefinitions> = Arc::new(read_task_definitions_from_inventory());
}

fn read_task_definitions_from_inventory() -> AllTaskDefinitions {
    let mut worker_queues = BTreeMap::new();
    let mut workflows = BTreeMap::new();
    let mut activities = BTreeMap::new();
    let mut all_names = BTreeSet::new();

    for task_queue in inventory::iter::<TaskQueueStatic> {
        if worker_queues.contains_key(task_queue.queue_name) {
            panic!(
                "Task queue {} already exists, is it defined twice?",
                task_queue.queue_name
            );
        }
        worker_queues.insert(task_queue.queue_name.to_string(), task_queue.into());
        all_names.insert(task_queue.queue_name.to_string());
    }
    for activity in inventory::iter::<ActivityDefinitionStatic> {
        if activities.contains_key(activity.name) {
            panic!(
                "Activity {} already exists, is it defined twice?",
                activity.name
            );
        }
        if all_names.contains(&activity.name.to_string()) {
            panic!(
                "Activity {} is defined as different things (queue, activity, workflow)",
                activity.name
            );
        }
        all_names.insert(activity.name.to_string());
        activities.insert(activity.name.to_string(), activity.into());
    }
    for workflow in inventory::iter::<WorkflowDefinitionStatic> {
        if workflows.contains_key(workflow.name) {
            panic!(
                "Workflow {} already exists, is it defined twice?",
                workflow.name
            );
        }
        if all_names.contains(&workflow.name.to_string()) {
            panic!(
                "Workflow {} is defined as different things (queue, activity, workflow)",
                workflow.name
            );
        }
        all_names.insert(workflow.name.to_string());
        workflows.insert(workflow.name.to_string(), workflow.into());
    }
    AllTaskDefinitions {
        worker_queues,
        workflows,
        activities,
    }
}

impl From<&TaskQueueStatic> for WorkerQueueDefinition {
    fn from(task_queue: &TaskQueueStatic) -> Self {
        WorkerQueueDefinition {
            name: task_queue.queue_name.to_string(),
            max_concurrency: task_queue.max_concurrency,
            max_blocking_threads: task_queue.max_blocking_threads,
            max_memory_mb: task_queue.max_memory_mb,
        }
    }
}

impl From<&ActivityDefinitionStatic> for ActivityDefinition {
    fn from(activity: &ActivityDefinitionStatic) -> Self {
        ActivityDefinition {
            name: activity.name.to_string(),
            queue_name: activity.queue_name.to_string(),
        }
    }
}

impl From<&WorkflowDefinitionStatic> for WorkflowDefinition {
    fn from(workflow: &WorkflowDefinitionStatic) -> Self {
        WorkflowDefinition {
            name: workflow.name.to_string(),
            queue_name: workflow.queue_name.to_string(),
        }
    }
}
