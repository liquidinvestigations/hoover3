//! This module contains helpers for working with temporal types

use hoover3_types::tasks::UiWorkflowStatusCode;
use temporal_sdk_core_protos::temporal::api::enums::v1::WorkflowExecutionStatus;

use anyhow;
use temporal_client::WorkflowClientTrait;

use crate::get_client;

/// Converts a Temporal workflow execution status to a UI-friendly status code
pub fn convert_status(status: WorkflowExecutionStatus) -> UiWorkflowStatusCode {
    match status {
        WorkflowExecutionStatus::Unspecified => UiWorkflowStatusCode::Unspecified,
        WorkflowExecutionStatus::Running => UiWorkflowStatusCode::Running,
        WorkflowExecutionStatus::Completed => UiWorkflowStatusCode::Completed,
        WorkflowExecutionStatus::Failed => UiWorkflowStatusCode::Failed,
        WorkflowExecutionStatus::Canceled => UiWorkflowStatusCode::Canceled,
        WorkflowExecutionStatus::Terminated => UiWorkflowStatusCode::Terminated,
        WorkflowExecutionStatus::ContinuedAsNew => UiWorkflowStatusCode::ContinuedAsNew,
        WorkflowExecutionStatus::TimedOut => UiWorkflowStatusCode::TimedOut,
    }
}

/// Retrieves the current status of a workflow execution
pub async fn query_workflow_execution_status(
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

/// Retrieves the final result payload from a completed workflow execution
pub async fn query_workflow_execution_result(workflow_id: &str) -> anyhow::Result<Vec<u8>> {
    let client = get_client().await?;
    use temporal_sdk_core_protos::temporal::api::enums::v1::EventType::WorkflowExecutionCompleted;
    use temporal_sdk_core_protos::temporal::api::history::v1::history_event::Attributes::WorkflowExecutionCompletedEventAttributes;
    let wf_result = client
        .get_workflow_execution_history(workflow_id.to_string(), None, vec![])
        .await?;
    if let Some(history) = wf_result.history {
        for event in history.events.iter().rev() {
            if event.event_type() == WorkflowExecutionCompleted {
                if let Some(WorkflowExecutionCompletedEventAttributes(attrib)) =
                    event.attributes.as_ref()
                {
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
