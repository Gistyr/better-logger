// better-logger/src/wasm/auxiliary.rs

#[cfg(feature = "wasm")]
pub(crate) fn initialize_wasm_logger(terminal_log_lvl: &str) -> Result<(), String> {
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
            return Err(format!(r#"better-logger (initialize_wasm_logger): The "terminal_log_lvl" setting must match: "trace", "debug", "info", "warn", or "error""#));
        }
    }
}