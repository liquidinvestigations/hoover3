use crate::api::get_processing_status;
use crate::api::get_workflow_status_tree;
use crate::components::page_titles::make_page_title;
use crate::components::table::InfoCard;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_logger::tracing::info;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use hoover3_types::tasks::TemporalioWorkflowStatusTree;
use hoover3_types::tasks::UiWorkflowStatus;
use hoover3_types::tasks::UiWorkflowStatusCode;
use std::collections::BTreeMap;

/// Admin Page that displays the details of a data source.
#[component]
pub fn DatasourceAdminDetailsPage(
    collection_id: CollectionId,
    datasource_id: DatabaseIdentifier,
) -> Element {
    let c = collection_id.clone();
    let d = datasource_id.clone();
    let mut scan_status_res = use_resource(move || {
        let c = c.clone();
        let d = d.clone();
        async move { get_processing_status((c, d)).await }
    });
    let mut scan_status = use_signal(|| None);
    use_effect(move || {
        let new = scan_status_res.read().clone();
        let old = scan_status.peek().clone();
        if new.is_some() && new != old {
            scan_status.set(new);
        }
    });
    // let scan_status: ReadOnlySignal<Option<Result<UiWorkflowStatus, ServerFnError>>> = ReadOnlySignal::new(scan_status);
    let c = collection_id.clone();
    let d = datasource_id.clone();

    let mut scan_result_res = use_resource(move || {
        let c = c.clone();
        let d = d.clone();
        async move { crate::api::wait_for_processing_results((c, d)).await }
    });
    let mut scan_result = use_signal(|| None);
    use_effect(move || {
        if let Some(Ok(result)) = scan_result_res.read().as_ref() {
            scan_result.set(Some(result.clone()));
        }
    });

    let c = collection_id.clone();
    let d = datasource_id.clone();
    spawn(async move {
        loop {
            crate::time::sleep(std::time::Duration::from_secs(3)).await;
            if let Some(Ok(status)) = scan_status.peek().as_ref() {
                if status.task_status == UiWorkflowStatusCode::Running {
                    info!("Refreshing scan status");
                    scan_status_res.restart();
                } else {
                    if !scan_result.peek().is_some() {
                        scan_result_res.restart();
                    }
                    break;
                }
            }
        }
        tracing::info!("done scanning for results");
    });

    rsx! {
        div {
            class: "container-fluid",
            h4 {
                "Collection {collection_id}"
            }
            DatasourceInfoCard {c: c.clone(), ds: d.clone()}
            WorkflowStatusDisplay {
                title: "Scan".to_string(),
                scan_status,
                children: rsx! {
                    if let Some(result) = scan_result.read().as_ref() {
                        pre {
                            "{result:#?}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DatasourceInfoCard(c: CollectionId, ds: DatabaseIdentifier) -> Element {
    let c2 = c.clone();
    let ds2 = ds.clone();
    let info_res = use_resource(move || crate::api::get_datasource((c2.clone(), ds.clone())));
    let mut info = use_signal(|| None);

    use_effect(move || {
        info.set(
            info_res
                .read()
                .as_ref()
                .cloned()
                .map(Result::ok)
                .unwrap_or(None),
        );
    });

    rsx! {
        InfoCard<DatasourceUiRow> {
            data: info,
            title: make_page_title(1, "Datasource", &ds2.to_string()),
        }
    }
}

/// Component that displays the status of a workflow, including self-refreshing progress bar.
#[component]
pub fn WorkflowStatusDisplay(
    title: String,
    scan_status: ReadOnlySignal<Option<Result<UiWorkflowStatus, ServerFnError>>>,
    children: Element,
) -> Element {
    let workflow_id = use_memo(move || {
        scan_status
            .read()
            .as_ref()
            .and_then(|s| s.as_ref().ok().map(|s| s.workflow_id.clone()))
    });
    let workflow_id_str = use_memo(move || {
        workflow_id
            .read()
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone()
    });
    let workflow_status = use_memo(move || {
        scan_status
            .read()
            .as_ref()
            .and_then(|s| s.as_ref().ok().map(|s| s.task_status.clone()))
            .unwrap_or(UiWorkflowStatusCode::Unspecified)
    });
    let mut status_tree_res = use_resource(move || {
        let workflow_id = workflow_id.read().clone();
        async move {
            if let Some(workflow_id) = workflow_id {
                get_workflow_status_tree(workflow_id).await.ok()
            } else {
                None
            }
        }
    });
    let status_tree = use_memo(move || status_tree_res.read().as_ref().cloned().flatten());
    spawn(async move {
        loop {
            crate::time::sleep(std::time::Duration::from_secs(5)).await;
            if let Some(status_tree) = status_tree.peek().as_ref() {
                if status_tree.root_status == UiWorkflowStatusCode::Running {
                    info!("Refreshing scan tree");
                    status_tree_res.restart();
                } else {
                    break;
                }
            }
        }
        info!("done refreshing scan tree");
    });

    let tree_counts = use_memo(move || {
        if let Some(Some(tree)) = status_tree_res.read().as_ref() {
            tree.total_counts.clone()
        } else {
            BTreeMap::new()
        }
    });

    rsx! {
        article {
            h3 {
                "{title}: {workflow_status}"
            }
            code {
                "{workflow_id_str}"
            }
            WorkflowDisplayProgressBar {counts: tree_counts}
            WorkflowDisplayProgressTree {status_tree}
            {children}
        }
    }
}

#[component]
fn WorkflowDisplayProgressBar(
    counts: ReadOnlySignal<BTreeMap<UiWorkflowStatusCode, i64>>,
) -> Element {
    let total_count = use_memo(move || counts.read().values().cloned().sum::<i64>());

    let percent = |a: i64, b: i64| {
        if b == 0 {
            0.0
        } else {
            ((a as f64 / b as f64) * 100.0).floor()
        }
    };

    let running_percent = use_memo(move || {
        percent(
            counts
                .read()
                .iter()
                .filter(|(k, _)| *k == &UiWorkflowStatusCode::Running)
                .map(|(_, v)| *v)
                .sum::<i64>(),
            *total_count.read(),
        )
    });
    let completed_percent = use_memo(move || {
        percent(
            counts
                .read()
                .iter()
                .filter(|(k, _)| *k == &UiWorkflowStatusCode::Completed)
                .map(|(_, v)| *v)
                .sum::<i64>(),
            *total_count.read(),
        )
    });
    let error_percent = use_memo(move || {
        percent(
            counts
                .read()
                .iter()
                .filter(|(k, _)| {
                    *k != &UiWorkflowStatusCode::Running && *k != &UiWorkflowStatusCode::Completed
                })
                .map(|(_, v)| *v)
                .sum::<i64>(),
            *total_count.read(),
        )
    });

    rsx! {
        div {
            class: "debug-border",
            style: "width: 100%; height: 1rem; background-color: #f0f0f0; position: relative; display: flex;",
            div {
                style: "width: {completed_percent}%; background-color: green; height: 100%;"
            }
            div {
                style: "width: {running_percent}%; background-color: #e0e0ff; height: 100%;"
            }
            div {
                style: "width: {error_percent}%; background-color: red; height: 100%;"
            }
        }
        p {
            if *completed_percent.read() > 0.0 {
                "Completed: {completed_percent}%"
            }
            if *error_percent.read() > 0.0 {
                "Error: {error_percent}%"
            }
            ""
        }

    }
}

#[component]
fn WorkflowDisplayProgressTree(
    status_tree: ReadOnlySignal<Option<TemporalioWorkflowStatusTree>>,
) -> Element {
    let Some(status_tree_root) = status_tree
        .read()
        .as_ref()
        .map(|t| t.root_workflow_id.clone())
    else {
        return rsx! {};
    };
    rsx! {
        details {
            "name": "workflow_tree",
            class: "secondary outline",
            style: "border: 1px solid #ccc; border-radius: 4px; padding: 4px;",
            summary {
                "Workflow Details"
            }
            WorkflowDisplayProgressTreeNode {
                status_tree: status_tree,
                current_node: status_tree_root,
            }
        }
    }
}

#[component]
fn WorkflowDisplayProgressTreeNode(
    status_tree: ReadOnlySignal<Option<TemporalioWorkflowStatusTree>>,
    current_node: String,
) -> Element {
    let tree = status_tree.read();
    let Some(Some(current_status)) = tree.as_ref().map(|t| t.nodes.get(&current_node).clone())
    else {
        return rsx! {};
    };
    let current_status = current_status.clone();
    let children = tree
        .as_ref()
        .map(|t| t.children.get(&current_node).cloned())
        .flatten()
        .unwrap_or(vec![]);
    rsx! {
        // div {
            a {
                href: r#"http://localhost:8081/namespaces/default/workflows?query=WorkflowId%3D%22{current_node}%22"#,
                "{current_node}"
            }
            b {
                style: "float: right;",
                " - {current_status}"
            }
            ul {
                style: "
                    padding-top: 0px; margin-top: 0px;
                    padding-bottom: 0px; margin-bottom: 0px;
                ",
                for child in children {
                    li {
                        key: "{child}",
                        style: "
                            margin-bottom: 0; margin-top: 0;
                            padding-bottom: 0; padding-top: 0;
                        ",
                        WorkflowDisplayProgressTreeNode {
                            status_tree: status_tree.clone(),
                            current_node: child.clone(),
                        }
                    }
                }
            }
        // }
    }
}
