use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::{collection::CollectionUiRow, identifier::CollectionId};

use crate::api::*;
use crate::components::{DataRowDisplay, HtmlTable, InfoCard};
use crate::errors::AnyhowErrorDioxusExt;
use crate::routes::Route;
use crate::routes::UrlParam;

impl DataRowDisplay for CollectionUiRow {
    fn get_headers() -> Vec<&'static str> {
        vec![
            "Collection Name",
            "Collection Title",
            "Collection Description",
            "Time Created",
            "Time Modified",
        ]
    }

    fn render_cell(&self, header_name: &str) -> Element {
        let c = self.clone();
        match header_name {
            "Collection Name" => rsx! {
                Link {
                    to: Route::CollectionAdminDetailsPage {
                        collection_id : self.collection_id.clone()
                    },
                    "{c.collection_id}"
                }
            },
            "Collection Title" => rsx! {"{self.collection_title}"},
            "Collection Description" => rsx! {"{self.collection_description}"},
            "Time Created" => rsx! {"{self.time_created}"},
            "Time Modified" => rsx! {"{self.time_modified}"},
            _x => panic!("unknown {_x}"),
        }
    }

    fn can_edit(_header_name: &str) -> bool {
        match _header_name {
            "Collection Title" | "Collection Description" => true,
            _ => false,
        }
    }
    fn get_editable_fields(&self) -> std::collections::HashMap<String, String> {
        let mut h = HashMap::new();
        h.insert(
            "Collection Title".to_string(),
            self.collection_title.clone(),
        );
        h.insert(
            "Collection Description".to_string(),
            self.collection_description.clone(),
        );
        h
    }
    fn set_editable_fields(&mut self, _h: HashMap<String, String>) {
        self.collection_title = _h.get(&"Collection Title".to_string()).unwrap().to_string();
        self.collection_description = _h
            .get(&"Collection Description".to_string())
            .unwrap()
            .to_string();
    }
}

#[component]
pub fn CollectionsAdminListPage() -> Element {
    let mut c_list = use_resource(move || async move { get_all_collections(()).await });
    let collections = use_memo(move || {
        if let Some(Ok(v)) = c_list.read().as_ref() {
            v.clone()
        } else {
            vec![]
        }
    });

    rsx! {
        HtmlTable {
            data: collections,
            title: "Collections",
            extra: Some(("Actions", Callback::new(move |c:CollectionUiRow| {rsx!{
                ButtonGroup {
                    c: c.collection_id.clone(),
                    do_refresh: move || c_list.restart()
                }
                }
            })))
        }


        CollectionsCreateWidget {
            cb: Callback::new(move |c: CollectionId| {
                spawn(async move {
                    let c_ = c.clone();
                    let _ = create_new_collection(c_).await;
                    // c_list.restart();
                    navigator().push(
                        Route::CollectionAdminDetailsPage {
                            collection_id: c.name().clone() });
                });
            }),
        }
    }
}

#[component]
fn ButtonGroup(c: String, do_refresh: Callback) -> Element {
    let c = CollectionId::new(&c).throw()?;
    let c2 = c.clone();
    rsx! {
        div {
            role: "group",
            button {
                onclick: move |_| {
                    navigator().push(
                        Route::CollectionAdminDetailsPage {
                            collection_id: c2.name()
                        }
                    );
                },
                "OPEN"
            }
            button {
                class: "secondary",
                onclick: move |_| {
                    let c2 = c.clone();
                    async move {
                        let _ = drop_collection(c2).await;
                        do_refresh.call(());
                    }
                },
                "DROP"
            }
        }
    }
}

#[component]
fn CollectionsCreateWidget(cb: Callback<CollectionId>) -> Element {
    let mut val = use_signal(move || "".to_string());
    rsx! {
        article { class: "grid",
            h1 { "New Collection" }
            div { role: "group",
                input {
                    placeholder: "new collection name...",
                    value: "{val}",
                    oninput: move |_ev| {
                        let v = _ev.value();
                        let v = v.replace(" ", "_").replace("-", "_").replace(".", "_").to_lowercase();
                        val.set(v);
                    },
                }
                button {
                    onclick: move |_| {
                        let v2 = val.read().clone();
                        if let Ok(c) = CollectionId::new(&v2) {
                            val.set("".to_string());
                            cb.call(c);
                        }
                    },
                    disabled: !CollectionId::new(&val.read().clone()).is_ok(),
                    "Create Collection"
                }
            }
        }
    }
}

#[component]
pub fn CollectionAdminDetailsPage(collection_id: String) -> Element {
    let c = CollectionId::new(&collection_id).throw()?;
    rsx! {
        CollectionInfoCard {c: c.clone()}
        CollectionDatasourceListCard { c:  c.clone() }
    }
}

#[component]
fn CollectionInfoCard(c: CollectionId) -> Element {
    let c_title = format!("Collection `{}`", c.name());
    let c2 = c.clone();
    let mut info_res = use_resource(move || crate::api::get_single_collection(c2.clone()));
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
        InfoCard<CollectionUiRow> {
            data: info,
            title: c_title,
            edited_cb: Some(Callback::new(move |i:CollectionUiRow| {
                spawn(async move {
                    info!("sending modify request for {:#?}", i);
                    if let Ok(_v) = crate::api::update_collection(i).await {
                        info_res.restart();
                    }
                });
            }))
        }
    }
}

impl DataRowDisplay for DatasourceUiRow {
    fn get_headers() -> Vec<&'static str> {
        vec!["Name", "Type", "Settings", "Time Created", "Time Modified"]
    }

    fn render_cell(&self, header_name: &str) -> Element {
        match header_name {
            "Name" => rsx! { Link {
                to: Route::DatasourceAdminDetailsPage {
                    collection_id: self.collection_id.clone(),
                    datasource_id: self.datasource_id.clone()
                },
                "{self.datasource_id}"
            }},
            "Type" => rsx! { "{self.datasource_type}" },
            "Settings" => rsx! { "{self.datasource_settings}" },
            "Time Created" => rsx! { "{self.time_created}" },
            "Time Modified" => rsx! { "{self.time_modified}" },
            _ => panic!("unknown {header_name}"),
        }
    }
}

#[component]
fn CollectionDatasourceListCard(c: CollectionId) -> Element {
    let c2 = c.clone();
    let c3 = c.clone();
    let c4 = c.clone();
    let res = use_resource(move || crate::api::get_all_datasources(c2.clone()));
    let sources = use_memo(move || {
        if let Some(Ok(r)) = res.read().as_ref() {
            r.clone()
        } else {
            vec![]
        }
    });
    rsx! {
        HtmlTable {
            title: "Data Sources",
            data: sources,
            extra_buttons: Some(Callback::new(move |_| {
                let c3 = c3.clone();
                rsx!{
                button {
                    onclick: move |_| {
                        let _ = navigator().push(Route::NewDatasourceFormPage {
                            collection_id: c3.name().clone(),
                            current_path: UrlParam::new(std::path::PathBuf::from("."))
                        });
                    },
                    "Add Datasource"
                }
            }})),
            extra: Some(("Actions", Callback::new(move |row:DatasourceUiRow| {
                let c4 = c4.clone(); rsx!{
                button {
                    onclick: move |_| {
                        let c4 = c4.clone();
                        let _ = navigator().push(Route::DatasourceAdminDetailsPage {
                            collection_id: c4.name().clone(),
                            datasource_id: row.datasource_id.clone()
                        });
                    },
                    "View"
                }
                }
            })))
        }
    }
}
