use crate::api::create_datasource;
use crate::api::list_directory;
use crate::components::DataRowDisplay;
use crate::components::HtmlTable;
use crate::routes::Route;
use crate::routes::UrlParam;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::tasks::DatasourceScanRequest;
use hoover3_types::{filesystem::FsMetadata, identifier::DatabaseIdentifier};
use std::path::PathBuf;
fn display_path(path: &PathBuf) -> String {
    let p = format!("{:?}", path);
    let len = p.len();
    // skip commas
    let p = p.as_str()[1..len - 1].to_string();
    p
}

fn display_file_name(path: &FsMetadata) -> String {
    let icon = if path.is_dir { "📁" } else { "📄" };

    let p = format!("{:?}", path.path.file_name().unwrap_or_default());
    let len = p.len();
    // skip commas
    let p = p.as_str()[1..len - 1].to_string();

    format!("{icon} {p}")
}

fn format_time(time: Option<DateTime<Utc>>) -> String {
    time.map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "-".to_string())
}

impl DataRowDisplay for FsMetadata {
    fn get_headers() -> Vec<&'static str> {
        vec!["Name", "Size", "Modified", "Created"]
    }

    fn render_cell(&self, header_name: &str) -> Element {
        match header_name {
            "Name" => rsx! {"{display_file_name(&self)}"},
            "Size" => rsx! {"{self.size_bytes} bytes"},
            "Created" => rsx! {"{format_time(self.created)}"},
            "Modified" => rsx! {"{format_time(self.modified)}"},
            _ => rsx! {"-"},
        }
    }
}

#[component]
pub fn NewDatasourceFormPage(
    collection_id: ReadOnlySignal<String>,
    current_path: ReadOnlySignal<UrlParam<PathBuf>>,
) -> Element {
    let (path, path_loaded) = UrlParam::convert_signals(current_path);

    rsx! {
        if *path_loaded.read() {
            _NewDatasourceFormPage{collection_id, path}
        }
    }
}

#[component]
fn _NewDatasourceFormPage(
    collection_id: ReadOnlySignal<String>,
    path: ReadOnlySignal<PathBuf>,
) -> Element {
    use crate::errors::AnyhowErrorDioxusExt;
    use hoover3_types::identifier::CollectionId;

    let _collection = CollectionId::new(&collection_id.read().as_str()).throw()?;
    let mut name = use_signal(|| String::new());
    let mut children = use_signal(|| Vec::new());

    let can_create_datasource = use_memo(move || {
        let name = name.read().clone();
        let has_children = !children.read().is_empty();
        let is_valid_path =
            path.read().clone() != PathBuf::from(".") && path.read().clone() != PathBuf::default();
        DatabaseIdentifier::new(&name).is_ok() && has_children && is_valid_path
    });

    let children_res = use_resource(move || {
        let path = path.read().clone();
        async move { list_directory(path).await }
    });
    use_effect(move || {
        children.set(
            children_res
                .read()
                .as_ref()
                .and_then(|x| x.as_ref().ok())
                .cloned()
                .unwrap_or_default(),
        );
    });

    rsx! {
        article { class: "container",
            h1 { "Create New Datasource for {collection_id.read()}" }
            form {
                fieldset { role: "group",
                    h3 {
                        "Name:"
                    }
                    input {
                        value: "{name}",
                        placeholder: "Datasource Name...",
                        oninput: move |evt| name.set(
                            evt.value().clone()
                            .replace(" ", "_")
                            .replace("-", "_")
                            .replace(".", "_")
                            .to_lowercase()),
                    }

                    button {
                        onclick: move |_e| {
                            _e.prevent_default();
                            let name = name.peek().clone();
                            let path = path.peek().clone();
                            let settings = DatasourceSettings::LocalDisk { path };
                            let collection_id = _collection.clone();
                            let collection_id_str = collection_id.name();
                            spawn(async move {
                                if let Ok(d) = DatabaseIdentifier::new(&name) {
                                    if let Ok(r) = create_datasource((collection_id.clone(), d.clone(), settings.clone())).await {
                                        navigator().push(Route::DatasourceAdminDetailsPage {
                                            collection_id: collection_id_str.clone(),
                                            datasource_id: r.datasource_id.to_string()
                                        });
                                    }
                                }
                                dioxus_logger::tracing::warn!("Failed to create datasource");
                            });
                        },
                        disabled: !*can_create_datasource.read(),
                        "Create Datasource",
                    }
                }
            }
        }
        div {
            class: "container",
            DatasourcePathPicker{
                path: path.clone(),
                child_list: children.clone(),
                collection_id: collection_id.read().clone()
            }
        }
    }
}

#[component]
fn DatasourcePathPicker(
    collection_id: ReadOnlySignal<String>,
    path: ReadOnlySignal<PathBuf>,
    child_list: ReadOnlySignal<Vec<FsMetadata>>,
) -> Element {
    let parent_path = use_memo(move || {
        let parent = path
            .read()
            .clone()
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();

        if format!("{:?}", parent) == "\"\"" {
            PathBuf::from(".")
        } else {
            parent
        }
    });
    let is_root = use_memo(move || path.read().as_os_str() == ".");

    rsx! {
        article {
            h2 {"Pick Datasource Folder"}
            HtmlTable{
                title: "{display_path(&path.read())}",
                extra_buttons: Some(Callback::new(move |_| rsx! {
                    button {
                        disabled: *is_root.read(),
                        class:"secondary",
                        onclick: move |_| {
                            navigator().push(Route::NewDatasourceFormPage {
                                collection_id: collection_id.read().clone(),
                                current_path: UrlParam::new(parent_path.peek().clone())});
                        },
                        "Up"
                    }
                })),
                data: child_list,
                extra: Some(("Select", Callback::new(move |child: FsMetadata| rsx! {
                    if child.is_dir {
                        button {
                            style:"min-width: 7rem;",
                            onclick: move |_| {
                                if child.is_dir {
                                    navigator().push(Route::NewDatasourceFormPage {
                                        collection_id: collection_id.read().clone() ,
                                        current_path: UrlParam::new(child.path.clone())});
                                }
                            },
                            "Select"
                        }
                    }
                }))),
            }
        }
    }
}
