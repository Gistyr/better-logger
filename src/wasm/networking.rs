// better-logger/src/wasm/networking.rs

#[cfg(feature = "wasm")]
use crate::interface::settings::{RUNNING_SETTINGS, NetworkFormat};
#[cfg(feature = "wasm")]
use serde_json::json;

#[cfg(feature = "wasm")]
pub(super) fn send_log_line(level: &str, target: &str, message: &str) -> Result<(), String> {
    let lvl = level.to_owned();
    let trg = target.to_owned();
    let msg = message.to_owned();

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(error) = async {
            let running_settings = RUNNING_SETTINGS.get().unwrap();
            let url = running_settings.network_endpoint_url.as_str(); 

            let timestamp = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
            let header: String = format!("[{} {} {}]", timestamp, lvl.to_uppercase(), trg);
            let line: String   = format!("{} {}", header, msg);

            match &running_settings.network_format {
                NetworkFormat::PlainText => {
                    gloo_net::http::Request::post(url)
                        .header("Content-Type", "text/plain; charset=utf-8")
                        .body(line)
                        .map_err(|error| format!("build request (PlainText) failed: {:?}", error))?
                        .send()
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("network log POST (PlainText) failed: {:?}", error))
                }
                NetworkFormat::JsonText { field } => {
                    let payload = json!({ field: line }).to_string();
                    gloo_net::http::Request::post(url)
                        .header("Content-Type", "application/json; charset=utf-8")
                        .body(payload)
                        .map_err(|error| format!("build request (JsonText) failed: {:?}", error))?
                        .send()
                        .await
                        .map(|_| ())
                        .map_err(|error| format!("network log POST (JsonText) failed: {:?}", error))
                }
            }
        }.await {
            web_sys::console::error_1(&error.into());
        }
    });

    return Ok(());
}