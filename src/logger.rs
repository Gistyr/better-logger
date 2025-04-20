// better-logger/src/logger.rs

pub use crate::trace;
pub use crate::debug;
pub use crate::debugx;
pub use crate::info;
pub use crate::warn;
pub use crate::error;

#[doc(hidden)]
pub fn is_async() -> bool {
    if crate::auxiliary::running_settings::ASYNC_LOGGING.load(std::sync::atomic::Ordering::Relaxed) == true {
        return true;
    }
    else {
        return false;
    }
}

#[doc(hidden)]
pub fn debug_extra_enabled() -> bool {
    let running_settings: &crate::auxiliary::running_settings::RunningSettings = {
        match crate::auxiliary::running_settings::RUNNING_SETTINGS.get() {
            Some(settings) => {
                settings
            },
            None => {
                eprintln!(r#"better-logger (1): Running settings mutex was not initialized."#);
                std::process::exit(1);
            }
        }
    };

    if running_settings.debug_extra == true {
        return true;
    }
    else {
        return false;
    }
}

#[doc(hidden)]
pub fn log_async(level: &str, msg: &str) {
    crate::core::log::log_async(level, msg)
}

#[doc(hidden)]
pub fn log_sync(level: &str, msg: &str) {
    crate::core::log::log_sync(level, msg)
}

pub fn init(settings: crate::settings::Settings) {
    if settings.terminal_logs == true {
        match settings.terminal_log_lvl.to_lowercase().as_str() {
            "trace" => {
                env_logger::Builder::new().filter_level(log::LevelFilter::Trace).init();
            },
            "debug" => {
                env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
            },
            "info" => {
                env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
            },
            "warn" => {
                env_logger::Builder::new().filter_level(log::LevelFilter::Warn).init();
            },
            _ => {
                eprintln!(r#"better-logger: The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#);
                std::process::exit(1);
            }
        }
    }

    if settings.file_logs == true {
        match settings.file_log_lvl.to_lowercase().as_str() {
            "trace" => {},
            "debug" => {},
            "info" => {},
            "warn" => {},
            "error" => {},
            _ => {
                eprintln!(r#"better-logger: The "file_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#);
                std::process::exit(1);
            }
        }

        match crate::auxiliary::file_operations::create_file(&settings.log_file_path) {
            Ok(file) => {
                match crate::auxiliary::running_settings::LOG_FILE.set(std::sync::Mutex::new(file)) {
                    Ok(_ok) => {},
                    Err(_error) => {
                        eprintln!(r#"better-logger: Log file has already been initialized as a mutex."#);
                        std::process::exit(1);
                    }
                }
            },
            Err(error) => {
                eprintln!(r#"better-logger: Could not create the log file. Reason: {}"#, error);
                std::process::exit(1);
            }
        }
    }

    if settings.async_logging == true {
        crate::auxiliary::running_settings::ASYNC_LOGGING.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    let running_settings: crate::auxiliary::running_settings::RunningSettings = crate::auxiliary::running_settings::RunningSettings {
        terminal_logs: settings.terminal_logs,
        terminal_log_lvl: settings.terminal_log_lvl,
        file_logs: settings.file_logs,
        file_log_lvl: settings.file_log_lvl,
        debug_extra: settings.debug_extra,
    };

    match crate::auxiliary::running_settings::RUNNING_SETTINGS.set(running_settings) {
        Ok(_ok) => {},
        Err(_error) => {
            eprintln!(r#"better-logger: Running settings have already been initialized as a mutex."#);
            std::process::exit(1);
        }
    }
}