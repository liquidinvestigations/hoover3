use crate::{
    api::get_container_status,
    components::{DataRowDisplay, HtmlTable},
};
use dioxus::prelude::*;
use hoover3_types::docker_health::ContainerHealthUi;

impl DataRowDisplay for ContainerHealthUi {
    // fn get_headers() -> Vec<&'static str> {
    //     vec!["Id", "Name", "Running", "Health"]
    // }

    // fn render_cell(&self, header_name: &str) -> Element {
    //     let x = match header_name {
    //         "Id" => self.container_id[0..12].to_string(),
    //         "Name" => self.container_name.clone(),
    //         "Running" => self.container_running.clone(),
    //         "Health" => self.container_health.clone(),
    //         _ => unreachable!(),
    //     };
    //     rsx!("{x}")
    // }
}

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
