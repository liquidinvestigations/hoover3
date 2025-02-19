use dioxus::prelude::*;
use hoover3_types::data_access::DataAccessSettings;
use std::path::PathBuf;

use crate::api::create_or_update_data_access_settings;

#[component]
pub fn ConfigureDataAccessForm(on_close: Callback) -> Element {
    let mut root_path = use_signal(|| "".to_string());
    let mut s3_bucket = use_signal(|| "".to_string());
    let mut s3_access_key = use_signal(|| "".to_string());
    let mut s3_secret_key = use_signal(|| "".to_string());
    let mut webdav_url = use_signal(|| "".to_string());
    let mut webdav_username = use_signal(|| "".to_string());
    let mut webdav_password = use_signal(|| "".to_string());

    rsx! {
        div { class: "modal",
            div { class: "modal-content",
                h2 { "Configure Data Access" }
                form {
                    fieldset {
                        legend { "Filesystem Access" }
                        label { "Root Path" }
                        input {
                            value: "{root_path}",
                            oninput: move |evt| root_path.set(evt.value().clone()),
                        }
                    }
                    fieldset {
                        legend { "S3 Access" }
                        label { "Bucket" }
                        input {
                            value: "{s3_bucket}",
                            oninput: move |evt| s3_bucket.set(evt.value().clone()),
                        }
                        label { "Access Key" }
                        input {
                            value: "{s3_access_key}",
                            oninput: move |evt| s3_access_key.set(evt.value().clone()),
                        }
                        label { "Secret Key" }
                        input {
                            value: "{s3_secret_key}",
                            oninput: move |evt| s3_secret_key.set(evt.value().clone()),
                        }
                    }
                    fieldset {
                        legend { "WebDAV Access" }
                        label { "URL" }
                        input {
                            value: "{webdav_url}",
                            oninput: move |evt| webdav_url.set(evt.value().clone()),
                        }
                        label { "Username" }
                        input {
                            value: "{webdav_username}",
                            oninput: move |evt| webdav_username.set(evt.value().clone()),
                        }
                        label { "Password" }
                        input {
                            value: "{webdav_password}",
                            oninput: move |evt| webdav_password.set(evt.value().clone()),
                        }
                    }
                    button {
                        onclick: move |_e| {
                            _e.prevent_default();
                            let settings = DataAccessSettings::LocalDisk {
                                root_path: PathBuf::from(root_path.read().clone()),
                            };
                            dioxus_logger::tracing::info!("settings: {:?}", settings);
                            spawn(async move {
                                match create_or_update_data_access_settings(settings).await {
                                    Ok(_) => {
                                        dioxus_logger::tracing::info!("Created Data Access Settings");
                                        on_close.call(());
                                    }
                                    Err(_) => {
                                        dioxus_logger::tracing::warn!(
                                            "Failed to create or update data access settings"
                                        );
                                    }
                                }
                            });
                        },
                        "Save"
                    }
                    button {
                        class: "secondary",
                        onclick: move |_| on_close.call(()),
                        "Cancel"
                    }
                }
            }
        }
    }
}
