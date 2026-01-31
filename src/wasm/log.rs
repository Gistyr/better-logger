// better-logger/src/wasm/log.rs

#[cfg(feature = "wasm")]
use crate::interface::settings::RUNNING_SETTINGS;
#[cfg(feature = "wasm")]
use super::networking::send_log_line;
#[cfg(feature = "wasm")]
use web_sys::console::error_1;

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

#[cfg(feature = "wasm")]
pub(crate) fn wasm_log_async(level: &str, target: &str, msg: &str) {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    if running_settings.terminal_logs == true {
        let terminal_set_log_level: String = running_settings.terminal_log_lvl.clone();
        let given_message_level_one: String = level.to_string();
        let given_message_one: String = msg.to_string();
        let target_one: String = target.to_string();

        wasm_bindgen_futures::spawn_local(async move {
            let terminal_current_settings: u8 = {
                    match terminal_set_log_level.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "debugx" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        error_1(&format!(r#"better-logger (wasm_log_async): "terminal_set_log_level" failed to match"#).as_str().into());
                        panic!();
                    }
                }
            };

            let terminal_requested_message_level: u8 = {
                match given_message_level_one.as_str() {
                    "trace" => 0,
                    "debug" => 1,
                    "debugx" => 1,
                    "info" => 2,
                    "warn" => 3,
                    "error" => 4,
                    _ => { 
                        error_1(&format!(r#"better-logger (1)(wasm_log_async): "given_message_level_one" failed to match"#).as_str().into());
                        panic!();
                    }
                }
            };

            if terminal_requested_message_level >= terminal_current_settings {
                match given_message_level_one.as_str() {
                    "trace" => {
                        log::trace!(target: &target_one, "{}", given_message_one);
                    },
                    "debug" | "debugx" => {
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
                        error_1(&format!(r#"better-logger (2)(wasm_log_async): "given_message_level_one" failed to match"#).as_str().into());
                        panic!();
                    }
                };
            }
        });
    }

    if running_settings.network_logs == true {
        let network_current_settings: u8 = {
            match running_settings.network_log_lvl.as_str() {
                "trace" => 0,
                "debug" => 1,
                "debugx" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    error_1(&format!(r#"better-logger (wasm_log_sync): "running_settings.network_log_lvl" failed to match"#).as_str().into());
                    panic!();
                }
            }
        };

        let network_requested_message_level: u8 = {
            match level {
                "trace" => 0,
                "debug" => 1,
                "debugx" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    error_1(&format!(r#"better-logger (3)(wasm_log_sync): "level" failed to match"#).as_str().into());
                    panic!();
                }
            }
        };

        if network_requested_message_level >= network_current_settings {
            if let Err(error) = send_log_line(level, target, msg) {
                error_1(&error.into());
            }
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

#[cfg(feature = "wasm")]
pub(crate) fn wasm_log_sync(level: &str, target: &str, msg: &str) {
    let running_settings = RUNNING_SETTINGS.get().unwrap();

    if running_settings.terminal_logs == true {
        let terminal_current_settings: u8 = {
                match running_settings.terminal_log_lvl.as_str() {
                "trace" => 0,
                "debug" => 1,
                "debugx" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    error_1(&format!(r#"better-logger (wasm_log_sync): "running_settings.terminal_log_lvl" failed to match"#).as_str().into());
                    panic!();
                }
            }
        };

        let terminal_requested_message_level: u8 = {
            match level {
                "trace" => 0,
                "debug" => 1,
                "debugx" => 1,
                "info" => 2,
                "warn" => 3,
                "error" => 4,
                _ => { 
                    error_1(&format!(r#"better-logger (1)(wasm_log_sync): "level" failed to match"#).as_str().into());
                    panic!();
                }
            }
        };

        if terminal_requested_message_level >= terminal_current_settings {
            match level {
                "trace" => {
                    log::trace!(target: &target, "{}", msg);
                },
                "debug" | "debugx" => {
                    log::debug!(target: &target, "{}", msg);
                },
                "info" => {
                    log::info!(target: &target, "{}", msg);
                },
                "warn" => {
                    log::warn!(target: &target, "{}", msg);
                },
                "error" => {
                    log::error!(target: &target, "{}", msg);
                },
                _ => { 
                    error_1(&format!(r#"better-logger (2)(wasm_log_sync): "level" failed to match"#).as_str().into());
                    panic!();
                }
            };
        }
    }
}