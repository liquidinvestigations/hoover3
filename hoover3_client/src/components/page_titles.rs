//! Front-end component for displaying page titles.

use dioxus::prelude::*;

/// Component that displays a page title. Level is 0 for no wrapper, 1 for h1, 2 for h2, etc.
/// Subtitle is rendered as a small gray text to the left of the title.
#[component]
fn PageTitle(level: u8, subtitle: String, title: String) -> Element {
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

/// Create a page title element. This is a helper function for use in other components.
pub fn make_page_title(level: u8, subtitle: &str, title: &str) -> Element {
    rsx! {
        PageTitle {
            subtitle: subtitle.to_string(),
            title: title.to_string(),
            level
        }
    }
}
