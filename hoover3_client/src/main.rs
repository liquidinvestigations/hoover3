//! The Hoover3 client - the website frontend entrypoint.
//!
use opentelemetry::global::shutdown_tracer_provider;
use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer},
    Context, Key
};

/// Main dioxus Entrypoint. Sets up launch configurations for Dioxus, as well as server backend main function.
pub fn main() {
    let _ = hoover3_tracing::init_tracing();
    hoover3_tracing::set_process_memory_limit(4096).unwrap();
  let tracer = global::tracer("global_tracer");
    let _cx = Context::new();
  
    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.set_attribute(Key::new("KEY").string("value"));

        span.add_event(
            format!("Operations"),
            vec![
                Key::new("SigNoz is").string("Awesome"),
            ],
        );
    });



    #[cfg(feature = "web")]
    {
        dioxus::web::launch::launch_cfg(
            hoover3_client::app::App,
            dioxus::web::Config::new().hydrate(true),
        );
    }

    #[cfg(feature = "server")]
    {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .max_blocking_threads(64)
            .thread_name("hoover3_client")
            .build()
            .unwrap()
            .block_on(async move {
                use dioxus::prelude::DioxusRouterExt;
                use dioxus::prelude::ServeConfig;
                // migrate
                hoover3_server::init_server_plugins().unwrap();
                hoover3_server::migrate_all().await.unwrap();

                // Start workers. Dioxus doesn't reap threads, so if we use `spawn_worker_on_thread` here,
                //

                // build our application with some routes
                let app = axum::routing::Router::new()
                    // Server side render the application, serve static assets, and register server functions
                    .serve_dioxus_application(
                        ServeConfig::new().unwrap(),
                        hoover3_client::app::App,
                    );

                // serve the app using the address passed by the CLI
                let addr = dioxus::cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }

    // dioxus::launch(App);
    // info!("dioxus main() exit.");
    shutdown_tracer_provider()
}
