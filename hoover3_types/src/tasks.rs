
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct UiWorkflowStatus {
    pub workflow_id: String,
    pub task_name: String,
    pub queue_name: String,
    pub task_status: UiWorkflowStatusCode,
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
