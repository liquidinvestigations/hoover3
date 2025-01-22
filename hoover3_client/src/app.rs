use crate::routes::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const PICO_CSS: Asset = asset!("/assets/libs/pico.min.css");
// const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    tracing::info!("dioxus App()...");
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "preload",
            href: "/assets/libs/pico.min.css",
            r#as: "style",
        }
        document::Link { rel: "preload", href: "/assets/main.css", r#as: "style" }
        // document::Stylesheet { href: "/assets/libs/pico.min.css"  }
        // document::Stylesheet { href: "/assets/main.css"  }
        Router::<Route> {}
    }
}
