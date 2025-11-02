// better-logger/src/native/networking.rs

#[cfg(feature = "native")]
use crate::interface::settings::{RUNNING_SETTINGS, CLIENT, NetworkFormat};
#[cfg(feature = "native")]
use serde_json::json;

#[cfg(feature = "native")]
pub(super) fn send_log_line(level: &str, target: &str, message: &str) -> Result<(), String> {
    let running_settings = RUNNING_SETTINGS.get().unwrap();
    let url = running_settings.network_endpoint_url.as_str();  

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let header: String = format!("[{} {} {}]", timestamp, level.to_uppercase(), target);
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