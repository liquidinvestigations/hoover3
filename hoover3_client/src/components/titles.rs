use dioxus::prelude::*;

#[component]
pub fn PageTitle(level: u8, subtitle: String, title: String) -> Element {
    let spans = rsx! {
        span { style: "font-size:50%; color:gray; margin-right:10px; ", {subtitle} }
        span { {title} }
    };
    match level {
        0 => spans,
        1 => {
            rsx! {h1 { {spans} }}
        }
        2 => {
            rsx! {h2 { {spans} }}
        }
        3 => {
            rsx! {h3 { {spans} }}
        }
        4 => {
            rsx! {h4 { {spans} }}
        }
        _ => {
            rsx! {h5 { {spans} }}
        }
    }
}

pub fn make_page_title(level: u8, subtitle: &str, title: &str) -> Element {
    rsx! {
        PageTitle {
            subtitle: subtitle.to_string(),
            title: title.to_string(),
            level
        }
    }
}
