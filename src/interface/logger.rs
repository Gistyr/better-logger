// better-logger/src/interface/logger.rs

#[cfg(any(feature = "native", feature = "wasm"))]
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

#[cfg(any(feature = "native", feature = "wasm"))]
use super::settings::{LoggerSettings, RunningSettings, RUNNING_SETTINGS};

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

#[cfg(any(feature = "native", feature = "wasm"))]
fn init_private(settings: LoggerSettings) -> Result<String, String> {
    #[cfg(feature = "native")]
    if settings.async_logging == true {
        if settings.wasm_logging == false {
            if tokio::runtime::Handle::try_current().is_err() {
                return Err(format!(r#"better-logger: "logger::init()" must be called inside a Tokio runtime"#));
            }
        }
    }

    if settings.terminal_logs == true {
        if settings.wasm_logging == false {
            #[cfg(feature = "native")]
            crate::native::auxiliary::initialize_native_logger(settings.terminal_log_lvl.to_lowercase().as_str())?;
        }
        else {
            #[cfg(feature = "wasm")]
            crate::wasm::auxiliary::initialize_wasm_logger(settings.terminal_log_lvl.to_lowercase().as_str())?;
        }
    }

    if settings.file_logs == true {
        if settings.wasm_logging == true {
            return Err(format!(r#"better-logger: file logging is not allowed in a WASM environment"#));
        }

        #[cfg(feature = "native")]
        crate::native::auxiliary::initialize_file_logging(&settings.file_log_lvl.to_lowercase().as_str(), &settings.log_file_path)?;
    }

    if settings.network_logs == true {
        #[cfg(feature = "native")]
        crate::native::auxiliary::initialize_network_logging(&settings.network_log_lvl.to_lowercase().as_str())?;

        #[cfg(feature = "wasm")]
        crate::wasm::auxiliary::initialize_network_logging(&settings.network_log_lvl.to_lowercase().as_str())?;

        #[cfg(feature = "wasm")]
        if settings.async_logging == false {
            return Err(format!(r#"better-logger: when using network logging in a WASM environment, the async_logging setting must be true. Browsers don't allow blocking network I/O on the main thread"#));
        }
    }

    let running_settings = RunningSettings {
        terminal_logs: settings.terminal_logs,
        terminal_log_lvl: settings.terminal_log_lvl,
        wasm_logging: settings.wasm_logging,
        file_logs: settings.file_logs,
        file_log_lvl: settings.file_log_lvl,
        network_logs: settings.network_logs,
        network_log_lvl: settings.network_log_lvl,
        network_endpoint_url: settings.network_endpoint_url,
        debug_extra: settings.debug_extra,
        async_logging: settings.async_logging,
    };

    match RUNNING_SETTINGS.set(running_settings) {
        Ok(_ok) => {
            return Ok("better-logger initialized successfully".to_string());
        },
        Err(_error) => {
            return Err(format!(r#"better-logger: RUNNING_SETTINGS have already been initialized"#));
        }
    }
}