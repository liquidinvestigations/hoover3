//! A component that renders big menu links that cover whole screen.

use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn FullscreenLinks(links: ReadOnlySignal<Vec<(Route, Element)>>) -> Element {
    rsx! {
        div {
            style: "
                width: 100%;
                height: 100%;
                container-type: size;
                display: flex;
                flex-direction: column;
            ",
            for (route, element) in links.read().iter() {
                FullscreenLink {
                    route: route.clone(),
                    element: element.clone(),
                    link_count: links.read().len()
                }
            }
        }
    }
}

#[component]
fn FullscreenLink(route: Route, element: Element, link_count: usize) -> Element {
    let font_size_cqmin = (1.0 / (link_count as f64 + 1.0) / 2.0 * 100.0).round() as u32;
    rsx! {
        h1 {
            style: "font-size: {font_size_cqmin}cqmin; ",
            Link { to: route.clone(), {element} }
        }
    }
}
