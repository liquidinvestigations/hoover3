//! App component - the whole website as a single component.
//!
//! This module contains the root dioxus component, root html/head elements with extra CSS assets, as well as global context providers.

use std::collections::{BTreeMap, HashSet, VecDeque};

use crate::{api::ServerCallEvent, routes::Route};
use dioxus::prelude::*;
use dioxus_logger::tracing::info;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const PICO_CSS: Asset = asset!("/assets/libs/pico.min.css");
// const MAIN_CSS: Asset = asset!("/assets/main.css");

/// The main app component that loads extra CSS and the router.
#[component]
pub fn App() -> Element {
    info!("dioxus App()...");

    let mut global_app_url: Signal<Option<String>> = use_signal(move || None);
    use_context_provider(|| GlobalUrlContext {
        global_app_url: global_app_url.into(),
    });
    // use coroutine to avoid "writing to a signal during component rendering" warning spamming up the console.
    let coro_set_global = use_coroutine(move |mut rx| async move {
        use futures_util::StreamExt;
        while let Some(x) = rx.next().await {
            global_app_url.set(Some(x));
        }
    });

    // workaround for browser back button: manually set the PopState callback
    // https://github.com/DioxusLabs/dioxus/issues/1657
    let mut _callback_keep_alive_forever_pls: Signal<Option<Box<web_sys::wasm_bindgen::JsValue>>> =
        use_signal(|| None);
    // web_sys functions crash if called directly; must use async for any of these.
    #[cfg(feature = "web")]
    use_future(move || async move {
        use web_sys::wasm_bindgen::prelude::*;
        // allow some time for the browser to load the page
        info!("Setting up browser back button callback...");
        let callback = Closure::wrap(Box::new(move |_e: web_sys::PopStateEvent| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let url = document.location().unwrap().href().unwrap();
            info!("Back button pressed: {:?}", url);
            coro_set_global.send(url);
        }) as Box<dyn FnMut(_)>);
        let window = web_sys::window().expect("no global `window` exists");
        window.set_onpopstate(Some(callback.as_ref().unchecked_ref()));
        // if the callback handle is dropped, the callback will not be called, and we will get an error instead.
        // so we need to keep the handle alive for a long time, so we put it into a signal.
        let callback_ptr = Box::new(callback.into_js_value());
        _callback_keep_alive_forever_pls.set(Some(callback_ptr));
    });

    use_init_server_call_history();
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
        Router::<Route> {
            config: move || RouterConfig::default().on_update(move |state|{
                let current_url: Route = state.current();
                // tracing::info!("Routing to URL: {}", current_url.to_string());
                coro_set_global.send(current_url.to_string());
                None
            })
        }
    }
}

/// Context containing the current app URL as string.
///
/// Useful for triggering actions when the URL changes.
/// Unlike `use_route()`, this works outside the router component.
#[derive(Clone, Debug)]
pub struct GlobalUrlContext {
    /// The new URL as string.
    pub global_app_url: ReadOnlySignal<Option<String>>,
}

fn use_init_server_call_history() {
    let GlobalUrlContext { global_app_url, .. } = use_context();
    let mut currently_loading = use_signal(HashSet::new);
    let mut show_pic = use_signal(|| false);
    let mut show_pic_debounce =
        dioxus_sdk::utils::timing::use_debounce(std::time::Duration::from_millis(150), move |b| {
            // info!("show_pic_debounce: {:?}", b);
            show_pic.set(b);
        });
    use_effect(move || {
        show_pic_debounce.action(!currently_loading.read().is_empty());
    });
    let loading_count = use_memo(move || currently_loading.read().len());

    let mut hist = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        BTreeMap<String, VecDeque<ServerCallEvent>>,
    >("BTreeMap_ServerCallEvent".to_string(), || {
        BTreeMap::<String, VecDeque<ServerCallEvent>>::new()
    });

    use_effect(move || {
        // we can't use `use_router::Route` because we want this to live in the App root component.
        let _route = global_app_url.read().clone();
        // info!("Router URL Changed, dropping loading state: {:?}", route);
        currently_loading.write().clear();
        show_pic.set(false);
    });

    use_context_provider(|| ServerCallHistory {
        cb: Callback::new(move |event: ServerCallEvent| {
            if event.is_finished {
                currently_loading
                    .write()
                    .remove(&(event.function.clone(), event.arg.clone()));
                let mut h = hist
                    .peek()
                    .get(&event.function)
                    .unwrap_or(&VecDeque::new())
                    .clone();
                h.push_front(event.clone());
                if h.len() > 10 {
                    h.pop_back();
                }
                hist.write().insert(event.function, h);
            } else {
                currently_loading
                    .write()
                    .insert((event.function, event.arg));
            }
        }),
        hist: ReadOnlySignal::new(hist),
        show_pic: show_pic.into(),
        loading_count: loading_count.into(),
    });
}

/// Context provider for server call history and global loading spinner.
#[derive(Clone, Debug)]
pub struct ServerCallHistory {
    /// Callback to push a server call event to the server call history.
    pub cb: Callback<ServerCallEvent>,
    /// Signal for reading server call history.
    pub hist: ReadOnlySignal<BTreeMap<String, VecDeque<ServerCallEvent>>>,
    /// Signal for showing the global loading spinner.
    pub show_pic: ReadOnlySignal<bool>,
    /// Signal for the loading count (the current number of concurrent server calls).
    pub loading_count: ReadOnlySignal<usize>,
}

/// Push a server call event to the server call history.
pub fn nav_push_server_call_event(event: ServerCallEvent) {
    let ServerCallHistory { cb, .. } = use_context();
    cb.call(event);
}

/// Read the server call history.
pub fn read_server_call_history() -> ReadOnlySignal<BTreeMap<String, VecDeque<ServerCallEvent>>> {
    let ServerCallHistory { hist, .. } = use_context();
    hist
}
