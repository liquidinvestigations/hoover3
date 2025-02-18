//! The Hoover3 client - the website frontend entrypoint.

/// Main dioxus Entrypoint. Sets up launch configurations for Dioxus, as well as server backend main function.
pub fn main() {
    hoover3_tracing::init_tracing();

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
                hoover3_database::migrate::migrate_all().await.unwrap();

                // start workers
                // hoover3_taskdef::spawn_worker_on_thread::<hoover3_filesystem_scanner::AllTasks>();

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
}
