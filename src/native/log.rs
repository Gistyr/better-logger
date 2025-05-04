// better-logger/src/native/log.rs

#[cfg(feature = "native")]
use crate::interface::settings::RUNNING_SETTINGS;
#[cfg(feature = "native")]
use super::file::write_log_line;
#[cfg(feature = "native")]
use super::networking::send_log_line;

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
pub(crate) fn native_log_async(level: &str, target: &str, msg: &str) {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    if running_settings.terminal_logs == true {
        let terminal_set_log_level: String = running_settings.terminal_log_lvl.clone();
        let given_message_level_one: String = level.to_string();
        let given_message_one: String = msg.to_string();
        let target_one: String = target.to_string();

        tokio::spawn(async move {
            let terminal_current_settings: u8 = {
                    match terminal_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (native_log_async): "terminal_set_log_level" failed to match"#);
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
                        eprintln!(r#"better-logger (1)(native_log_async): "given_message_level_one" failed to match"#);
                        panic!();
                    }
                }
            };

            if terminal_requested_message_level >= terminal_current_settings {
                match given_message_level_one.as_str() {
                    "trace" => {
                        log::trace!(target: &target_one, "{}", given_message_one);
                    },
                    "debug" => {
                        log::debug!(target: &target_one, "{}", given_message_one);
                    },
                    "info" => {
                        log::info!(target: &target_one, "{}", given_message_one);
                    },
                    "warn" => {
                        log::warn!(target: &target_one, "{}", given_message_one);
                    },
                    "error" => {
                        log::error!(target: &target_one, "{}", given_message_one);
                    },
                    _ => { 
                        eprintln!(r#"better-logger (2)(native_log_async): "given_message_level_one" failed to match"#);
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
        let target_two: String = target.to_string();

        tokio::spawn(async move {
            let file_current_settings: u8 = {
                match file_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (native_log_async): "file_set_log_level" failed to match"#);
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
                        eprintln!(r#"better-logger (1)(native_log_async): "given_message_level_two" failed to match"#);
                        panic!();
                    }
                }
            };

            if file_requested_message_level >= file_current_settings {
                match given_message_level_two.as_str() {
                    "trace" => {
                        if let Err(error) = write_log_line("TRACE", &target_two, &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "debug" => {
                        if let Err(error) = write_log_line("DEBUG", &target_two, &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "info" => {
                        if let Err(error) = write_log_line("INFO", &target_two, &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "warn" => {
                        if let Err(error) = write_log_line("WARN", &target_two, &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    "error" => {
                        if let Err(error) = write_log_line("ERROR", &target_two, &given_message_two) {
                            eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                        }
                    },
                    _ => { 
                        eprintln!(r#"better-logger (2)(native_log_async): "given_message_level_two" failed to match"#);
                        panic!();
                    }
                };
            }
        });
    }

    if running_settings.network_logs == true {
        let network_set_log_level: String = running_settings.network_log_lvl.clone();
        let given_message_level_three: String = level.to_string();
        let given_message_three: String = msg.to_string();
        let target_three: String = target.to_string();

        tokio::task::spawn_blocking(move || {
            let network_current_settings: u8 = {
                match network_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (native_log_async): "network_current_settings" failed to match"#);
                        panic!();
                    }
                }
            };

            let network_requested_message_level: u8 = {
                match given_message_level_three.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        eprintln!(r#"better-logger (native_log_async): "given_message_level_three" failed to match"#);
                        panic!();
                    }
                }
            };

            if network_requested_message_level >= network_current_settings {
                match given_message_level_three.as_str() {
                    "trace" => {
                        if let Err(error) = send_log_line("TRACE", &target_three, &given_message_three) {
                            eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                        }
                    },
                    "debug" => {
                        if let Err(error) = send_log_line("DEBUG", &target_three, &given_message_three) {
                            eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                        }
                    },
                    "info" => {
                        if let Err(error) = send_log_line("INFO", &target_three, &given_message_three) {
                            eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                        }
                    },
                    "warn" => {
                        if let Err(error) = send_log_line("WARN", &target_three, &given_message_three) {
                            eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                        }
                    },
                    "error" => {
                        if let Err(error) = send_log_line("ERROR", &target_three, &given_message_three) {
                            eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                        }
                    },
                    _ => { 
                        eprintln!(r#"better-logger (6)(native_log_sync): "level" failed to match"#);
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
pub(crate) fn native_log_sync(level: &str, target: &str, msg: &str) {
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
                    eprintln!(r#"better-logger (native_log_sync): "running_settings.terminal_log_lvl" failed to match"#);
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
                    eprintln!(r#"better-logger (1)(native_log_sync): "level" failed to match"#);
                    panic!();
                }
            }
        };

        if terminal_requested_message_level >= terminal_current_settings {
            match level {
                "trace" => {
                    log::trace!(target: target, "{}", msg);
                },
                "debug" => {
                    log::debug!(target: target, "{}", msg);
                },
                "info" => {
                    log::info!(target: target, "{}", msg);
                },
                "warn" => {
                    log::warn!(target: target, "{}", msg);
                },
                "error" => {
                    log::error!(target: target, "{}", msg);
                },
                _ => { 
                    eprintln!(r#"better-logger (2)(native_log_sync): "level" failed to match"#);
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
                    eprintln!(r#"better-logger (native_log_sync): "running_settings.file_log_lvl" failed to match"#);
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
                    eprintln!(r#"better-logger (3)(native_log_sync): "level" failed to match"#);
                    panic!();
                }
            }
        };

        if file_requested_message_level >= file_current_settings {
            match level {
                "trace" => {
                    if let Err(error) = write_log_line("TRACE", target, &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "debug" => {
                    if let Err(error) = write_log_line("DEBUG", target, &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "info" => {
                    if let Err(error) = write_log_line("INFO", target, &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "warn" => {
                    if let Err(error) = write_log_line("WARN", target, &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                "error" => {
                    if let Err(error) = write_log_line("ERROR", target, &msg) {
                        eprintln!(r#"better-logger: Failed to write line to log file. Error: {}"#, error);
                    }
                },
                _ => { 
                    eprintln!(r#"better-logger (4)(native_log_sync): "level" failed to match"#);
                    panic!();
                }
            };
        }
    }

    if running_settings.network_logs == true {
        let network_current_settings: u8 = {
            match running_settings.network_log_lvl.as_str() {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (native_log_sync): "running_settings.network_log_lvl" failed to match"#);
                    panic!();
                }
            }
        };

        let network_requested_message_level: u8 = {
            match level {
                "trace" => 0,
                "debug" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    eprintln!(r#"better-logger (5)(native_log_sync): "level" failed to match"#);
                    panic!();
                }
            }
        };

        if network_requested_message_level >= network_current_settings {
            match level {
                "trace" => {
                    if let Err(error) = send_log_line("TRACE", target, &msg) {
                        eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                    }
                },
                "debug" => {
                    if let Err(error) = send_log_line("DEBUG", target, &msg) {
                        eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                    }
                },
                "info" => {
                    if let Err(error) = send_log_line("INFO", target, &msg) {
                        eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                    }
                },
                "warn" => {
                    if let Err(error) = send_log_line("WARN", target, &msg) {
                        eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                    }
                },
                "error" => {
                    if let Err(error) = send_log_line("ERROR", target, &msg) {
                        eprintln!(r#"better-logger: Failed to send line over http. Error: {}"#, error);
                    }
                },
                _ => { 
                    eprintln!(r#"better-logger (6)(native_log_sync): "level" failed to match"#);
                    panic!();
                }
            };
        }
    }
}