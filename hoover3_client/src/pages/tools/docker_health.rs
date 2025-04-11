use crate::{
    api::get_container_status,
    components::table::{DataRowDisplay, HtmlTable},
};
use dioxus::prelude::*;
use hoover3_types::docker_health::ContainerHealthUi;

impl DataRowDisplay for ContainerHealthUi {}

/// Page that displays the health of Docker containers.
#[component]
pub fn DockerHealthPage() -> Element {
    let container_data = use_resource(move || async move { get_container_status(()).await });
    let memo_data = use_memo(move || {
        if let Some(Ok(v)) = container_data.read().as_ref() {
            v.clone()
        } else {
            vec![]
        }
    });
    rsx! {
        HtmlTable {
            data: memo_data,
            title: "Docker Containers",
        }
    }
}
