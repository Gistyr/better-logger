// better-logger/src/aux/running_settings.rs

pub(crate) static LOG_FILE: once_cell::sync::OnceCell<std::sync::Mutex<std::fs::File>> = once_cell::sync::OnceCell::new();
pub(crate) static ASYNC_LOGGING: once_cell::sync::Lazy<std::sync::atomic::AtomicBool> = once_cell::sync::Lazy::new(|| std::sync::atomic::AtomicBool::new(false));
pub(crate) static RUNNING_SETTINGS: once_cell::sync::OnceCell<RunningSettings> = once_cell::sync::OnceCell::new();

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RunningSettings {
    pub(crate) terminal_logs: bool,
    pub(crate) terminal_log_lvl: String,
    pub(crate) file_logs: bool,
    pub(crate) file_log_lvl: String,
    pub(crate) debug_extra: bool,
}