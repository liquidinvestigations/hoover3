use crate::api::get_datasource;
use crate::errors::AnyhowErrorDioxusExt;
use dioxus::prelude::*;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};

#[component]
pub fn DatasourceAdminDetailsPage(collection_id: String, datasource_id: String) -> Element {
    let c_id = CollectionId::new(&collection_id).throw()?;
    let d_id = DatabaseIdentifier::new(&datasource_id).throw()?;
    let datasource = use_resource(move || {
        let collection_id = c_id.clone();
        let datasource_id = d_id.clone();
        async move { get_datasource((collection_id, datasource_id)).await }
    });
    rsx! {
        article {
            h1 {
                "Collection: {collection_id}"
            }
            h2 {
                "Datasource: {datasource_id}"
            }
            pre {
                "{datasource:#?}"
            }
        }
    }
}
