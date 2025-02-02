use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use dioxus_logger::tracing;



pub fn init_tracing() {

    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::Registry;

        let layer_config = tracing_wasm::WASMLayerConfigBuilder::new()
            .set_console_config(tracing_wasm::ConsoleConfig::ReportWithoutConsoleColor)
            .set_report_logs_in_timings(true)
            .set_max_level(Level::INFO)
            .build();
        let layer = tracing_wasm::WASMLayer::new(layer_config) ;
        let reg = Registry::default().with(layer);


        console_error_panic_hook::set_once();
        tracing::subscriber::set_global_default(reg).expect("failed to set global default");

        // use tracing_web::{MakeWebConsoleWriter, performance_layer};
        // use tracing_subscriber::fmt::format::Pretty;
        // use tracing_subscriber::prelude::*;
        // let fmt_layer = tracing_subscriber::fmt::layer()
        // .with_ansi(false) // Only partially supported across browsers
        // .without_time()   // std::time is not available in browsers, see note below
        // .with_writer(MakeWebConsoleWriter::new()); // write events to the console
        // let perf_layer = performance_layer()
        //     .with_details_from_fields(Pretty::default());

        // tracing_subscriber::registry()
        //     .with(fmt_layer)
        //     .with(perf_layer)
        //     .init();

            tracing_log::LogTracer::builder().with_max_level(log::LevelFilter::Info).init().expect("failed to init logger x2");
            tracing::info!("wasm tracing init.");
            log::info!("wasm log init.");
        }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // let sub = FmtSubscriber::builder().with_max_level(Level::INFO);
        // let sub = sub.with_target(false).with_file(true).with_line_number(true).without_time().finish();
        // tracing::subscriber::set_global_default(sub).expect("setting default subscriber failed");
        // dioxus_logger::init(Level::INFO).expect("failed to init logger");


        let sub = tracing_subscriber::FmtSubscriber::builder().with_max_level(Level::INFO);


        // todo(jon): this is a small hack to clean up logging when running under the CLI
        // eventually we want to emit everything as json and let the CLI manage the parsing + display
        tracing::subscriber::set_global_default(sub.without_time().with_target(false).finish()).expect("x");

        tracing_log::LogTracer::builder().with_max_level(log::LevelFilter::Info).init().expect("failed to init logger x2");
        tracing::info!("server tracing init.");
        log::info!("server log init.");
    }
}