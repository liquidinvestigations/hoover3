use crate::api::get_datasource;
use crate::api::get_scan_status;
use crate::api::get_workflow_status_tree;
use crate::components::InfoCard;
use crate::errors::AnyhowErrorDioxusExt;
use dioxus::prelude::*;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use hoover3_types::tasks::UiWorkflowStatus;
use hoover3_types::tasks::UiWorkflowStatusCode;
use std::collections::BTreeMap;
use std::future::Future;
#[component]
pub fn DatasourceAdminDetailsPage(collection_id: String, datasource_id: String) -> Element {
    let c = CollectionId::new(&collection_id).throw()?;
    let d = DatabaseIdentifier::new(&datasource_id).throw()?;
    let mut scan_status_res = use_resource(move || {
        let c = c.clone();
        let d = d.clone();
        async move { get_scan_status((c, d)).await }
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
    let c = CollectionId::new(&collection_id).throw()?;
    let d = DatabaseIdentifier::new(&datasource_id).throw()?;

    let mut scan_result_res = use_resource(move || {
        let c = c.clone();
        let d = d.clone();
        async move { crate::api::wait_for_scan_results((c, d)).await }
    });
    let mut scan_result = use_signal(|| None);
    use_effect(move || {
        if let Some(Ok(result)) = scan_result_res.read().as_ref() {
            scan_result.set(Some(result.clone()));
        }
    });

    let c = CollectionId::new(&collection_id).throw()?;
    let d = DatabaseIdentifier::new(&datasource_id).throw()?;
    spawn(async move {
        loop {
            crate::time::sleep(std::time::Duration::from_secs(3)).await;
            if let Some(Ok(status)) = scan_status.peek().as_ref() {
                if status.task_status == UiWorkflowStatusCode::Running {
                    dioxus_logger::tracing::info!("Refreshing scan status");
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
            class: "container",
            h4 {
                "Collection {collection_id}"
            }
            DataousrceInfoCard {c: c.clone(), ds: d.clone()}
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
fn DataousrceInfoCard(c: CollectionId, ds: DatabaseIdentifier) -> Element {
    let c_title = format!("Datasource `{}`", c);
    let c2 = c.clone();
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
            title: c_title,
        }
    }
}

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
            .map(|s| s.as_ref().ok().map(|s| s.workflow_id.clone()))
            .flatten()
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
            .map(|s| s.as_ref().ok().map(|s| s.task_status.clone()))
            .flatten()
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
                    dioxus_logger::tracing::info!("Refreshing scan tree");
                    status_tree_res.restart();
                } else {
                    break;
                }
            }
        }
        tracing::info!("done refreshing scan tree");
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
            total_count.read().clone(),
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
            total_count.read().clone(),
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
            total_count.read().clone(),
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
