use dioxus::prelude::*;

use crate::{components::page_titles::make_page_title, routes::Route};

/// Render a card that links to a page.
#[component]
pub fn LinkCard(subtitle: String, title: String, link: Route, children: Element) -> Element {
    rsx! {
        Link {
            to: link,
            article {
                style: "max-width:500px; min-height: 200px;",
                {make_page_title(3, &subtitle, &title)}
                {children}
            }
        }
    }
}

/// Render a grid of cards.
#[component]
pub fn CardGridDisplay(children: Element) -> Element {
    rsx! {
        div {
            style: "
                display: grid;
                grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
                gap: 10px;
                min-height: 200px;
            ",
            {children}
        }
    }
}