// better-logger/src/wasm/networking.rs

#[cfg(feature = "wasm")]
use crate::interface::settings::{RUNNING_SETTINGS, NetworkFormat, NetworkEndpointUrl};
#[cfg(feature = "wasm")]
use serde_json::json;

#[cfg(feature = "wasm")]
const NETWORK_RETRY_DELAYS_MS: [u32; 2] = [1_000, 3_000];

#[cfg(feature = "wasm")]
enum SendFailure {
    Retryable(String),
    Permanent(String),
}

#[cfg(feature = "wasm")]
fn retry_jitter_ms() -> u32 {
    let roll = js_sys::Math::random();
    if roll < 0.25 {
        return 0;
    } else if roll < 0.5 {
        return 125;
    } else if roll < 0.75 {
        return 250;
    } else {
        return 375;
    }
}

#[cfg(feature = "wasm")]
async fn send_once(url: &str, content_type: &str, bearer_auth_key: Option<&str>, body: &str, format_name: &str) -> Result<(), SendFailure> {
    let mut request = gloo_net::http::Request::post(url)
        .header("Content-Type", content_type);
    if let Some(key) = bearer_auth_key {
        request = request.header("Authorization", &format!("Bearer {}", key));
    }

    let request = match request.body(body.to_owned()) {
        Ok(request) => request,
        Err(error) => return Err(SendFailure::Permanent(format!("build request ({}) failed: {:?}", format_name, error))),
    };

    let response = match request.send().await {
        Ok(response) => response,
        Err(error) => return Err(SendFailure::Retryable(format!("network log POST ({}) failed: {:?}", format_name, error))),
    };

    if response.ok() {
        return Ok(());
    }

    let status = response.status();
    let failure = format!("network log POST ({}) failed: HTTP {} {}", format_name, status, response.status_text());
    if status == 429 || status >= 500 {
        return Err(SendFailure::Retryable(failure));
    }
    return Err(SendFailure::Permanent(failure));
}

#[cfg(feature = "wasm")]
async fn send_with_retry(url: &str, content_type: &str, bearer_auth_key: Option<&str>, body: &str, format_name: &str) -> Result<(), String> {
    let mut last_failure = match send_once(url, content_type, bearer_auth_key, body, format_name).await {
        Ok(()) => return Ok(()),
        Err(SendFailure::Permanent(failure)) => return Err(failure),
        Err(SendFailure::Retryable(failure)) => failure,
    };

    for delay_ms in NETWORK_RETRY_DELAYS_MS {
        gloo_timers::future::TimeoutFuture::new(delay_ms.saturating_add(retry_jitter_ms())).await;
        last_failure = match send_once(url, content_type, bearer_auth_key, body, format_name).await {
            Ok(()) => return Ok(()),
            Err(SendFailure::Permanent(failure)) => return Err(failure),
            Err(SendFailure::Retryable(failure)) => failure,
        };
    }

    return Err(format!("{} (gave up after {} retries)", last_failure, NETWORK_RETRY_DELAYS_MS.len()));
}

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

            let bearer_auth_key: Option<&str> = if running_settings.use_bearer_auth == true {
                Some(running_settings.bearer_auth_key.as_str())
            } else {
                None
            };

            match &running_settings.network_format {
                NetworkFormat::PlainText => {
                    send_with_retry(&url, "text/plain; charset=utf-8", bearer_auth_key, &line, "PlainText").await
                }
                NetworkFormat::JsonText { field } => {
                    let payload = json!({ field: line }).to_string();
                    send_with_retry(&url, "application/json; charset=utf-8", bearer_auth_key, &payload, "JsonText").await
                }
            }
        }.await {
            web_sys::console::error_1(&error.into());
        }
    });

    return Ok(());
}
