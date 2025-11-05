// better-logger/src/interface/settings.rs

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub enum NetworkFormat {
    PlainText,
    JsonText { field: String },
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub struct LoggerSettings {
    pub terminal_logs: bool,
    pub terminal_log_lvl: String,
    pub wasm_logging: bool,
    pub file_logs: bool,
    pub file_log_lvl: String,
    pub log_file_path: String,
    pub network_logs: bool,
    pub network_log_lvl: String,
    pub network_endpoint_url: String,
    pub network_format: NetworkFormat,
    pub debug_extra: bool,
    pub async_logging: bool,
}

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[cfg(any(feature = "native", feature = "wasm"))]
pub(crate) static RUNNING_SETTINGS: std::sync::OnceLock<RunningSettings> = std::sync::OnceLock::new();

#[cfg(any(feature = "native", feature = "wasm"))]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RunningSettings {
    pub(crate) terminal_logs: bool,
    pub(crate) terminal_log_lvl: String,
    pub(crate) wasm_logging: bool,
    pub(crate) file_logs: bool,
    pub(crate) file_log_lvl: String,
    pub(crate) network_logs: bool,
    pub(crate) network_log_lvl: String,
    pub(crate) network_endpoint_url: String,
    pub(crate) network_format: NetworkFormat,
    pub(crate) debug_extra: bool,
    pub(crate) async_logging: bool,
}

#[cfg(feature = "native")]
pub(crate) static LOG_FILE: std::sync::OnceLock<std::sync::Mutex<std::fs::File>> = std::sync::OnceLock::new();

#[cfg(feature = "native")]
pub(crate) static CLIENT: std::sync::LazyLock<ureq::Agent> = std::sync::LazyLock::new(|| ureq::agent());