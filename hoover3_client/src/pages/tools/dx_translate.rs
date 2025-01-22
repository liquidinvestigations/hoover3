use dioxus::prelude::*;

#[component]
pub fn DioxusTranslatePage() -> Element {
    let mut html_code = use_signal(move || "".to_string());
    let dx_res = use_resource(move || {
        let html_code = html_code.read().to_string();
        dx_translate(html_code)
    });
    let mut dx_code = use_signal(move || "".to_string());
    use_effect(move || {
        let v = dx_res.read().clone();
        dx_code.set(match v {
            Some(Ok(r)) => r,
            Some(Err(e)) => format!("err: {e:#?}"),
            None => "".to_string(),
        });
    });
    rsx! {
        textarea {
            placeholder: "<div>...</div>",
            value: "{html_code}",
            oninput: move |_e| {
                html_code.set(_e.value());
            },
        }

        div { class: "grid",
            div {
                h1 { "original html" }
                article {
                    pre { {html_code} }
                }
            }
            div {
                h1 { "dx translate" }
                article {
                    pre { {dx_code} }
                }
            }
        }
    }
}

#[server]
async fn dx_translate(input: String) -> Result<String, ServerFnError> {
    _dx_translate(input).await
}

#[cfg(feature = "server")]
async fn _dx_translate(input: String) -> Result<String, ServerFnError> {
    use tokio::process::Command;
    let mut command = Command::new("dx");
    command.arg("translate");
    command.arg("--raw");
    command.arg(input);

    let output = command.output().await;
    let output = output.map_err(|e| ServerFnError::new(format!("dx translate error: {e:#?}")))?;
    let output = output.stdout;
    let output = String::from_utf8_lossy(&output).trim().to_string();
    Ok(output)
}
