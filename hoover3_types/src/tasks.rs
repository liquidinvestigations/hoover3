use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct UiWorkflowStatus {
    pub workflow_id: String,
    pub task_name: String,
    pub queue_name: String,
    pub task_status: UiWorkflowStatusCode,
    pub status_tree: TemporalioWorkflowStatusTree,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiWorkflowStatusCode {
    Unspecified = 0,
    Running = 1,
    Completed = 2,
    Failed = 3,
    Canceled = 4,
    Terminated = 5,
    ContinuedAsNew = 6,
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
            "{:?}",
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TemporalioWorkflowStatusTree {
    pub root_workflow_id: String,
    pub nodes: BTreeMap<String, UiWorkflowStatusCode>,
    pub parent: BTreeMap<String, String>,
    pub children: BTreeMap<String, Vec<String>>,
    pub counts: BTreeMap<String, BTreeMap<UiWorkflowStatusCode, i64>>,
    pub total_counts: BTreeMap<UiWorkflowStatusCode, i64>,
}
