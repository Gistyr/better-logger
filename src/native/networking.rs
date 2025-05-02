// better-logger/src/native/networking.rs

#[cfg(feature = "native")]
use crate::interface::settings::{RUNNING_SETTINGS, CLIENT};

#[cfg(feature = "native")]
pub(super) fn send_log_line(level: &str, target: &str, message: &str) -> Result<(), String> {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    let url = running_settings.network_endpoint_url.as_str();  

    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let header: String = format!("[{} {} {}]", timestamp, level.to_uppercase(), target);
    let line: String   = format!("{} {}", header, message);

    CLIENT
    .post(url)
    .header("Content-Type", "text/plain; charset=utf-8")
    .send(line)                     
    .map(|_| ())                      
    .map_err(|error| format!("network log POST failed: {error}"))
}