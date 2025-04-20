// better-logger/src/settings.rs

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub terminal_logs: bool,
    pub terminal_log_lvl: String,
    pub file_logs: bool,
    pub file_log_lvl: String,
    pub log_file_path: String,
    pub debug_extra: bool,
    pub async_logging: bool,
}