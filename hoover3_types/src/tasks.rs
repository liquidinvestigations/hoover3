//! Types and structures related to workflow tasks.

use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

/// Represents the status of a workflow task in the UI
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct UiWorkflowStatus {
    /// Unique identifier of the workflow
    pub workflow_id: String,
    /// Name of the task
    pub task_name: String,
    /// Name of the queue the task belongs to
    pub queue_name: String,
    /// Current status code of the workflow task
    pub task_status: UiWorkflowStatusCode,
}

/// Status codes representing different states of a workflow task
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiWorkflowStatusCode {
    /// Status has not been specified
    Unspecified = 0,
    /// Workflow is currently executing
    Running = 1,
    /// Workflow has finished successfully
    Completed = 2,
    /// Workflow has encountered an error and failed
    Failed = 3,
    /// Workflow was manually canceled
    Canceled = 4,
    /// Workflow was forcefully terminated
    Terminated = 5,
    /// Workflow was continued as a new workflow instance
    ContinuedAsNew = 6,
    /// Workflow exceeded its time limit
    TimedOut = 7,
}

impl FromStr for UiWorkflowStatusCode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Running" => UiWorkflowStatusCode::Running,
            "Completed" => UiWorkflowStatusCode::Completed,
            "Failed" => UiWorkflowStatusCode::Failed,
            "Canceled" => UiWorkflowStatusCode::Canceled,
            "Terminated" => UiWorkflowStatusCode::Terminated,
            "ContinuedAsNew" => UiWorkflowStatusCode::ContinuedAsNew,
            "TimedOut" => UiWorkflowStatusCode::TimedOut,
            _ => anyhow::bail!("invalid status: {}", s),
        })
    }
}

impl Display for UiWorkflowStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UiWorkflowStatusCode::Unspecified => "Unspecified",
                UiWorkflowStatusCode::Running => "Running",
                UiWorkflowStatusCode::Completed => "Completed",
                UiWorkflowStatusCode::Failed => "Failed",
                UiWorkflowStatusCode::Canceled => "Canceled",
                UiWorkflowStatusCode::Terminated => "Terminated",
                UiWorkflowStatusCode::ContinuedAsNew => "ContinuedAsNew",
                UiWorkflowStatusCode::TimedOut => "TimedOut",
            }
        )
    }
}

/// Represents a tree structure of workflow statuses and their relationships
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemporalioWorkflowStatusTree {
    /// ID of the top-level workflow in the tree
    pub root_workflow_id: String,
    /// Status of the root workflow
    pub root_status: UiWorkflowStatusCode,
    /// Map of workflow IDs to their status codes
    pub nodes: BTreeMap<String, UiWorkflowStatusCode>,
    /// Map of workflow IDs to their parent workflow IDs
    pub parent: BTreeMap<String, String>,
    /// Map of workflow IDs to their child workflow IDs
    pub children: BTreeMap<String, Vec<String>>,
    /// Map of workflow IDs to their status code counts
    pub counts: BTreeMap<String, BTreeMap<UiWorkflowStatusCode, i64>>,
    /// Aggregate counts of all status codes across all workflows
    pub total_counts: BTreeMap<UiWorkflowStatusCode, i64>,
}
