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

fn init_private(settings: LoggerSettings) -> Result<String, String> {
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

    let running_settings = RunningSettings {
        terminal_logs: settings.terminal_logs,
        terminal_log_lvl: settings.terminal_log_lvl,
        wasm_logging: settings.wasm_logging,
        file_logs: settings.file_logs,
        file_log_lvl: settings.file_log_lvl,
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