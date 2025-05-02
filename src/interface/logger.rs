// better-logger/src/interface/logger.rs

pub use crate::{trace, debug, debugx, info, warn, error};

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

use crate::interface::settings::LoggerSettings;

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

pub fn init(settings: LoggerSettings) -> Result<String, String> {
    match init_private(settings) {
        Ok(msg) => {
            return Ok(msg);
        },
        Err(error) => {
            return Err(error);
        }
    }
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

fn init_private(settings: crate::LoggerSettings) -> Result<String, String> {
    if settings.terminal_logs == true {
        if settings.wasm_logging == false {
            #[cfg(feature = "native")]
            initialize_native_logger(settings.terminal_log_lvl.to_lowercase().as_str())?;
        }
        else {
            #[cfg(feature = "wasm")]
            initialize_wasm_logger(settings.terminal_log_lvl.to_lowercase().as_str())?;
        }
    }

    if settings.file_logs == true {
        if settings.wasm_logging == true {
            return Err(format!(r#"better-logger: file logging is not allowed in a WASM environment"#));
        }

        #[cfg(feature = "native")]
        initialize_file_logging(&settings.file_log_lvl.to_lowercase().as_str(), &settings.log_file_path)?;
    }

    let running_settings = crate::auxiliary::running_settings::RunningSettings {
        terminal_logs: settings.terminal_logs,
        terminal_log_lvl: settings.terminal_log_lvl,
        wasm_logging: settings.wasm_logging,
        file_logs: settings.file_logs,
        file_log_lvl: settings.file_log_lvl,
        debug_extra: settings.debug_extra,
        async_logging: settings.async_logging,
    };

    match crate::auxiliary::running_settings::RUNNING_SETTINGS.set(running_settings) {
        Ok(_ok) => {
            return Ok("better-logger initialized successfully".to_string());
        },
        Err(_error) => {
            return Err(format!(r#"better-logger: RUNNING_SETTINGS have already been initialized"#));
        }
    }
}

#[cfg(feature = "native")]
fn initialize_native_logger(terminal_log_lvl: &str) -> Result<(), String> {
    match terminal_log_lvl {
        "trace" => {
            env_logger::Builder::new().filter_level(log::LevelFilter::Trace).init();
            return Ok(());
        },
        "debug" => {
            env_logger::Builder::new().filter_level(log::LevelFilter::Debug).init();
            return Ok(());
        },
        "info" => {
            env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
            return Ok(());
        },
        "warn" => {
            env_logger::Builder::new().filter_level(log::LevelFilter::Warn).init();
            return Ok(());
        },
        "error" => {
            env_logger::Builder::new().filter_level(log::LevelFilter::Error).init();
            return Ok(());
        },
        _ => {
            return Err(format!(r#"better-logger (native): The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
        }
    }
}

#[cfg(feature = "wasm")]
fn initialize_wasm_logger(terminal_log_lvl: &str) -> Result<(), String> {
    match terminal_log_lvl {
        "trace" => {
            wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
            return Ok(());
        },
        "debug" => {
            wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
            return Ok(());
        },
        "info" => {
            wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
            return Ok(());
        },
        "warn" => {
            wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
            return Ok(());
        },
        "error" => {
            wasm_logger::init(wasm_logger::Config::new(log::Level::Error));
            return Ok(());
        },
        _ => {
            return Err(format!(r#"better-logger (wasm): The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
        }
    }
}

#[cfg(feature = "native")]
fn initialize_file_logging(file_log_lvl: &str, log_file_path: &str) -> Result<(), String> {
    match file_log_lvl {
        "trace" => {},
        "debug" => {},
        "info" => {},
        "warn" => {},
        "error" => {},
        _ => {
            return Err(format!(r#"better-logger: The "file_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
        }
    }

    match create_file(log_file_path) {
        Ok(file) => {
            match crate::auxiliary::running_settings::LOG_FILE.set(Mutex::new(file)) {
                Ok(_ok) => {
                    return Ok(());
                },
                Err(_error) => {
                    return Err(format!(r#"better-logger: Log file has already been initialized as a mutex"#));
                }
            }
        },
        Err(error) => {
            return Err(format!(r#"better-logger: Could not create the log file. Reason: {}"#, error));
        }
    }
}