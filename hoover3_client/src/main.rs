use dioxus_logger::tracing::{info, Level};
use hoover3_client::app::App;
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("dioxus main()...");
    dioxus::launch(App);
    info!("dioxus main() exit.");
}
