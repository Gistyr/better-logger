// better-logger/src/native/networking.rs

#[cfg(feature = "native")]
use crate::interface::settings::{RUNNING_SETTINGS, CLIENT, NetworkFormat, NetworkEndpointUrl};
#[cfg(feature = "native")]
use serde_json::json;
#[cfg(feature = "native")]
use chrono::Local;

#[cfg(feature = "native")]
pub(super) fn send_log_line(level: &str, target: &str, message: &str) -> Result<(), String> {
    let running_settings = match RUNNING_SETTINGS.get() {
        Some(settings) => settings,
        None => return Err(format!(r#"better-logger: RUNNING_SETTINGS not initialized"#)),
    };

    let url: &str = {
        match &running_settings.network_endpoint_url {
            NetworkEndpointUrl::Single(single) => {
                single.url.as_str()
            }
            NetworkEndpointUrl::Multiple(multiple) => {
                match level {
                    "trace" => multiple.trace.as_str(),
                    "debug" => multiple.debug.as_str(),
                    "debugx" => multiple.debugx.as_str(),
                    "info" => multiple.info.as_str(),
                    "warn" => multiple.warn.as_str(),
                    "error" => multiple.error.as_str(),
                    _ => return Err(format!(r#"better-logger: Unsupported log level "{}" for network endpoint url"#, level)),
                }
            }
        }
    };

    let now = Local::now();
    let timestamp = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));
    let header_level: &str = {
        match level {
            "trace" => "TRACE",
            "debug" => "DEBUG",
            "debugx" => "DEBUG",
            "info" => "INFO",
            "warn" => "WARN",
            "error" => "ERROR",
            _ => return Err(format!(r#"better-logger: Unsupported log level "{}" for network formatting"#, level)),
        }
    };
    let header: String = format!("[{} {} {}]", timestamp, header_level, target);
    let line: String   = format!("{} {}", header, message);

    let request = CLIENT.post(url);

    match &running_settings.network_format {
        NetworkFormat::PlainText => {
            request
                .header("Content-Type", "text/plain; charset=utf-8")
                .send(line) 
                .map(|_| ())
                .map_err(|error| format!("network log POST (PlainText) failed: {:?}", error))
        }
        NetworkFormat::JsonText { field } => {
            let payload = json!({ field: line }).to_string();
            request
                .header("Content-Type", "application/json; charset=utf-8")
                .send(payload)
                .map(|_| ())
                .map_err(|error| format!("network log POST (JsonText) failed: {:?}", error))
        }
    }
}