//! Task definition inventory - static lists of task queues, activities, workflows.

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
