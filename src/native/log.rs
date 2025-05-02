// better-logger/src/native/log.rs

#[cfg(feature = "native")]
use crate::interface::settings::RUNNING_SETTINGS;
#[cfg(feature = "native")]
use super::file::write_log_line;

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
pub(crate) fn native_log_async(level: &str, msg: &str) {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    if running_settings.terminal_logs == true {
        let terminal_set_log_level: String = running_settings.terminal_log_lvl.clone();
        let given_message_level_one: String = level.to_string();
        let given_message_one: String = msg.to_string();

        tokio::spawn(async move {
            let terminal_current_settings: u8 = {
                    match terminal_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (native_log_async): "terminal_current_settings" failed to match"#);
                        panic!();
                    }
                }
            };

            let terminal_requested_message_level: u8 = {
                match given_message_level_one.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (1): The "level" value passed to "logger_async" must match: "trace", "debug", "info", "warn", or "error". Adjust the public functions that call logger_async"#);
                        panic!();
                    }
                }
            };

            if terminal_requested_message_level >= terminal_current_settings {
                match given_message_level_one.as_str() {
                    "trace" => {
                        log::trace!("{}", given_message_one);
                    },
                    "debug" => {
                        log::debug!("{}", given_message_one);
                    },
                    "info" => {
                        log::info!("{}", given_message_one);
                    },
                    "warn" => {
                        log::warn!("{}", given_message_one);
                    },
                    "error" => {
                        log::error!("{}", given_message_one);
                    },
                    _ => { 
                        eprintln!(r#"better-logger: "given_message_level_one" failed to match"#);
                        panic!();
                    }
                };
            }
        });
    }

    if running_settings.file_logs == true {
        let file_set_log_level: String = running_settings.file_log_lvl.clone();
        let given_message_level_two: String = level.to_string();
        let given_message_two: String = msg.to_string();

        tokio::spawn(async move {
            let file_current_settings: u8 = {
                match file_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (1): The "file_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error". This should have ben caught by the init() function"#);
                        panic!();
                    }
                }
            };

            let file_requested_message_level: u8 = {
                match given_message_level_two.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (2): The "level" value passed to "logger_async" must match: "trace", "debug", "info", "warn", or "error". Adjust the public functions that call logger_async"#);
                        panic!();
                    }
                }
            };

            if file_requested_message_level >= file_current_settings {
                match given_message_level_two.as_str() {
                    "trace" => {
                        if let Err(error) = write_log_line("TRACE", module_path!(), &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "debug" => {
                        if let Err(error) = write_log_line("DEBUG", module_path!(), &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "info" => {
                        if let Err(error) = write_log_line("INFO", module_path!(), &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "warn" => {
                        if let Err(error) = write_log_line("WARN", module_path!(), &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "error" => {
                        if let Err(error) = write_log_line("ERROR", module_path!(), &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    _ => { 
                        eprintln!(r#"better-logger: "given_message_level_one" failed to match"#);
                        panic!();
                    }
                };
            }
        });
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
pub(crate) fn native_log_sync(level: &str, msg: &str) {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    if running_settings.terminal_logs == true {
        let terminal_current_settings: u8 = {
                match running_settings.terminal_log_lvl.as_str() {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (2): The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error". This should have ben caught by the init() function"#);
                    panic!();
                }
            }
        };

        let terminal_requested_message_level: u8 = {
            match level {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (3): The "level" value passed to "logger_async" must match: "trace", "debug", "info", "warn", or "error". Adjust the public functions that call logger_async"#);
                    panic!();
                }
            }
        };

        if terminal_requested_message_level >= terminal_current_settings {
            match level {
                "trace" => {
                    log::trace!("{}", msg);
                },
                "debug" => {
                    log::debug!("{}", msg);
                },
                "info" => {
                    log::info!("{}", msg);
                },
                "warn" => {
                    log::warn!("{}", msg);
                },
                "error" => {
                    log::error!("{}", msg);
                },
                _ => { 
                    eprintln!(r#"better-logger: "given_message_level_one" failed to match"#);
                    panic!();
                }
            };
        }
    }

    if running_settings.file_logs == true {
        let file_current_settings: u8 = {
            match running_settings.file_log_lvl.as_str() {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (2): The "file_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error". This should have ben caught by the init() function"#);
                    panic!();
                }
            }
        };

        let file_requested_message_level: u8 = {
            match level {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (4): The "level" value passed to "logger_async" must match: "trace", "debug", "info", "warn", or "error". Adjust the public functions that call logger_async"#);
                    panic!();
                }
            }
        };

        if file_requested_message_level >= file_current_settings {
            match level {
                "trace" => {
                    if let Err(error) = write_log_line("TRACE", module_path!(), &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "debug" => {
                    if let Err(error) = write_log_line("DEBUG", module_path!(), &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "info" => {
                    if let Err(error) = write_log_line("INFO", module_path!(), &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "warn" => {
                    if let Err(error) = write_log_line("WARN", module_path!(), &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "error" => {
                    if let Err(error) = write_log_line("ERROR", module_path!(), &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                _ => { 
                    eprintln!(r#"better-logger: "given_message_level_one" failed to match"#);
                    panic!();
                }
            };
        }
    }
}