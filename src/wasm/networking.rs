// better-logger/src/wasm/networking.rs

#[cfg(feature = "wasm")]
use crate::interface::settings::{RUNNING_SETTINGS, NetworkFormat, NetworkEndpointUrl};
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
            let url: String = {
                match &running_settings.network_endpoint_url {
                    NetworkEndpointUrl::Single(single) => {
                        single.url.clone()
                    }
                    NetworkEndpointUrl::Multiple(multiple) => {
                        match lvl.as_str() {
                            "trace" => multiple.trace.clone(),
                            "debug" => multiple.debug.clone(),
                            "debugx" => multiple.debugx.clone(),
                            "info" => multiple.info.clone(),
                            "warn" => multiple.warn.clone(),
                            "error" => multiple.error.clone(),
                            _ => {
                                return Err(format!(r#"better-logger: Unsupported log level "{}" for network endpoint url"#, lvl));
                            }
                        }
                    }
                }
            };

            let header_level: &str = {
                match lvl.as_str() {
                    "trace" => "TRACE",
                    "debug" => "DEBUG",
                    "debugx" => "DEBUG",
                    "info" => "INFO",
                    "warn" => "WARN",
                    "error" => "ERROR",
                    _ => return Err(format!(r#"better-logger: Unsupported log level "{}" for network formatting"#, lvl)),
                }
            };

            let timestamp = match js_sys::Date::new_0().to_iso_string().as_string() {
                Some(value) => {
                    value
                }
                None => {
                    return Err(format!(r#"better-logger: Failed to format timestamp"#));
                }
            };
            let header: String = format!("[{} {} {}]", timestamp, header_level, trg);
            let line: String   = format!("{} {}", header, msg);

            match &running_settings.network_format {
                NetworkFormat::PlainText => {
                    gloo_net::http::Request::post(&url)
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
                    gloo_net::http::Request::post(&url)
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