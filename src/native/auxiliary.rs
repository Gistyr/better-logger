// better-logger/src/native/auxiliary.rs

#[cfg(feature = "native")]
use crate::interface::settings::LOG_FILE;

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

#[cfg(feature = "native")]
pub(crate) fn initialize_native_logger(terminal_log_lvl: &str) -> Result<(), String> {
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
            return Err(format!(r#"better-logger (initialize_native_logger): The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
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

#[cfg(feature = "native")]
pub(crate) fn initialize_file_logging(file_log_lvl: &str, log_file_path: &str) -> Result<(), String> {
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

    match super::file::create_file(log_file_path) {
        Ok(file) => {
            match LOG_FILE.set(std::sync::Mutex::new(file)) {
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

#[cfg(feature = "native")]
pub(crate) fn initialize_network_logging(network_log_lvl: &str) -> Result<(), String> {
    match network_log_lvl {
        "trace" => {},
        "debug" => {},
        "info" => {},
        "warn" => {},
        "error" => {},
        _ => {
            return Err(format!(r#"better-logger: The "network_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
        }
    }
    
    return Ok(());
}