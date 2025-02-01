use crate::api::get_datasource;
use crate::api::get_scan_status;
use crate::errors::AnyhowErrorDioxusExt;
use dioxus::prelude::*;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};

#[component]
pub fn DatasourceAdminDetailsPage(collection_id: String, datasource_id: String) -> Element {
    let c_id = CollectionId::new(&collection_id).throw()?;
    let d_id = DatabaseIdentifier::new(&datasource_id).throw()?;
    let datasource = use_resource(move || {
        let c_id = c_id.clone();
        let d_id = d_id.clone();
        async move {
            (
                get_datasource((c_id.clone(), d_id.clone())).await,
                get_scan_status((c_id.clone(), d_id.clone())).await,
            )
        }
    });

    let c_id = CollectionId::new(&collection_id).throw()?;
    let d_id = DatabaseIdentifier::new(&datasource_id).throw()?;
    rsx! {
        article {
            h1 {
                "Collection: {collection_id}"
            }
            h2 {
                "Datasource: {datasource_id}"
            }
            button {
                onclick: move |_| {
                    let c_id = c_id.clone();
                    let d_id = d_id.clone();
                    async move {
                    if crate::api::start_scan((c_id.clone(), d_id.clone())).await.is_ok() {

                    }
                }},
                "Start Scan."
            }
            pre {
                "{datasource:#?}"
            }
        }
    }
}
