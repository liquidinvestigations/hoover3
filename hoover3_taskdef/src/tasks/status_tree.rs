use anyhow::Context;
use hoover3_types::tasks::TemporalioWorkflowStatusTree;
use hoover3_types::tasks::UiWorkflowStatusCode;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use temporal_client::WorkflowService;
use temporal_sdk_core_protos::temporal::api::workflowservice::v1::CountWorkflowExecutionsRequest;
use temporal_sdk_core_protos::temporal::api::workflowservice::v1::CountWorkflowExecutionsResponse;
use temporal_sdk_core_protos::temporal::api::workflowservice::v1::ListWorkflowExecutionsRequest;
use temporal_sdk_core_protos::temporal::api::workflowservice::v1::ListWorkflowExecutionsResponse;

use super::convert_status;
use super::query_workflow_execution_status;
const TREE_NODE_LIMIT: usize = 24;

pub async fn get_workflow_status_tree(
    workflow_id: String,
) -> anyhow::Result<TemporalioWorkflowStatusTree> {
    hoover3_database::db_management::with_redis_cache(
        "temporalio_get_workflow_status_tree",
        5,
        _temporalio_get_workflow_status_tree,
        &(workflow_id.to_string()),
    )
    .await
}

async fn _temporalio_get_workflow_status_tree(
    workflow_id: String,
) -> anyhow::Result<TemporalioWorkflowStatusTree> {
    let root_status = convert_status(query_workflow_execution_status(&workflow_id).await?);

    let client = crate::get_client().await?;
    let mut client = (*client).clone();

    let mut open = BTreeSet::new();
    open.insert(workflow_id.to_string());
    let mut popcount = 0;

    let mut tree = TemporalioWorkflowStatusTree {
        root_workflow_id: workflow_id.to_string(),
        nodes: BTreeMap::new(),
        parent: BTreeMap::new(),
        children: BTreeMap::new(),
        counts: BTreeMap::new(),
        total_counts: BTreeMap::new(),
        root_status: root_status.clone(),
    };
    tree.nodes.insert(workflow_id.clone(), root_status.clone());
    let count0 = BTreeMap::from([(root_status, 1)]);
    tree.total_counts = count0.clone();
    tree.counts.insert(workflow_id.clone(), count0);
    while popcount < TREE_NODE_LIMIT && !open.is_empty() {
        let parent_id = open.pop_first().context("empty open set")?;
        popcount += 1;
        if let Ok((count, list)) = temporalio_list_children(&mut client, &parent_id).await {
            let mut current_children = vec![];
            for child_info in list.executions.iter() {
                let child_id = child_info
                    .execution
                    .clone()
                    .context("no id")?
                    .workflow_id
                    .clone();
                let child_status = super::convert_status(child_info.status());
                tree.nodes.insert(child_id.clone(), child_status.clone());
                open.insert(child_id.clone());
                tree.parent.insert(child_id.clone(), parent_id.clone());
                current_children.push(child_id);

                let root_count = tree.total_counts.get(&child_status).unwrap_or(&0);
                let new_count = root_count + 1;
                tree.total_counts.insert(child_status.clone(), new_count);
            }
            tree.children.insert(parent_id.clone(), current_children);
            let mut count_map = BTreeMap::new();
            for group in count.groups.iter() {
                let status = group.group_values.first().context("no status in group")?;
                let status: String = serde_json::from_slice(status.data.as_slice())?;
                use std::str::FromStr;
                let status = UiWorkflowStatusCode::from_str(&status)
                    .unwrap_or(UiWorkflowStatusCode::Unspecified);
                let count = group.count;
                count_map.insert(status.clone(), count);

                let root_count = tree.total_counts.get(&status).unwrap_or(&0);
                let new_count = root_count + count;
                tree.total_counts.insert(status.clone(), new_count);
            }
            tree.counts.insert(parent_id, count_map);
        }
    }

    Ok(tree)
}

pub async fn temporalio_list_children(
    client: &mut temporal_client::RetryClient<temporal_client::Client>,
    workflow_id: &str,
) -> anyhow::Result<(
    CountWorkflowExecutionsResponse,
    ListWorkflowExecutionsResponse,
)> {
    let list_query = format!("ParentWorkflowId=\"{}\" ", workflow_id);
    let count_query = format!(
        "ParentWorkflowId=\"{}\" GROUP BY ExecutionStatus ",
        workflow_id
    );
    let count = client
        .count_workflow_executions(CountWorkflowExecutionsRequest {
            namespace: "default".to_string(),
            query: count_query.to_string(),
        })
        .await?
        .get_ref()
        .clone();
    let list = client
        .list_workflow_executions(ListWorkflowExecutionsRequest {
            namespace: "default".to_string(),
            page_size: 10,
            next_page_token: vec![],
            query: list_query,
        })
        .await?;
    Ok((count, list.get_ref().clone()))
}
