mod collections;
pub use collections::*;

mod datasources;
pub use datasources::*;

mod new_datasource_form;
pub use new_datasource_form::*;

use dioxus::prelude::*;

/// The admin home page.
#[component]
pub fn AdminHomePage() -> Element {
    rsx! {
        "Admin Home Page"
    }
}
