// better-logger/tests/sync_native.rs

// cargo build --features native
// cargo test --features native test_six -- --nocapture
// Each test has to be run individually

#[cfg(feature = "native")]
use better_logger::LoggerSettings;
#[cfg(feature = "native")]
use better_logger::logger;
#[cfg(feature = "native")]
use better_logger::logger::*;

#[cfg(feature = "native")]
#[test]
fn test_six() {
    let log_settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "trace".to_string(),
        log_file_path: "tests/logs/test_six_sync.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
        debug_extra: true,
        async_logging: false,
    };

    if let Err(error) = logger::init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::trace!("Test Six: TRACE and above, terminal and file");
    logger::trace!("TRACE: sync Test Six");
    logger::debug!("{}: async Test Six", debug);
    logger::debugx!("{}: async Test Six", debugx);
    logger::info!("INFO: sync Test Six");
    logger::warn!("WARN: sync Test Six");
    logger::error!(r#"{}: sync Test Six"#, error);
}

#[cfg(feature = "native")]
#[test]
fn test_seven() {
    let log_settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "warn".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "warn".to_string(),
        log_file_path: "tests/logs/test_seven_sync.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
        debug_extra: true,
        async_logging: false,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::warn!("Test Seven: WARN and above, terminal and file");
    logger::trace!("TRACE: sync Test Seven");
    logger::debug!("{}: async Test Seven", debug);
    logger::debugx!("{}: async Test Seven", debugx);
    logger::info!("INFO: sync Test Seven");
    logger::warn!("WARN: sync Test Seven");
    logger::error!(r#"{}: sync Test Seven"#, error);
}

#[cfg(feature = "native")]
#[test]
fn test_eight() {
    let log_settings: LoggerSettings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "debug".to_string(),
        wasm_logging: false,
        file_logs: false,
        file_log_lvl: "debug".to_string(),
        log_file_path: "tests/logs/test_eight_sync.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
        debug_extra: false,
        async_logging: false,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::debug!("Test Eight: DEBUG and above (no DEBUGX), terminal only");
    logger::trace!("TRACE: sync Test Eight");
    logger::debug!("{}: async Test Eight", debug);
    logger::debugx!("{}: async Test Eight", debugx);
    logger::info!("INFO: sync Test Eight");
    logger::warn!("WARN: sync Test Eight");
    logger::error!(r#"{}: sync Test Eight"#, error);
}

#[cfg(feature = "native")]
#[test]
fn test_nine() {
    let log_settings = LoggerSettings {
        terminal_logs: false,
        terminal_log_lvl: "error".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "error".to_string(),
        log_file_path: "tests/logs/test_nine_sync.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
        debug_extra: false,
        async_logging: false,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::error!("Test Nine: ERROR and above, file only");
    logger::trace!("TRACE: sync Test Nine");
    logger::debug!("{}: async Test Nine", debug);
    logger::debugx!("{}: async Test Nine", debugx);
    logger::info!("INFO: sync Test Nine");
    logger::warn!("WARN: sync Test Nine");
    logger::error!(r#"{}: sync Test Nine"#, error);
}

#[cfg(feature = "native")]
#[test]
fn test_ten() {
    let log_settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "info".to_string(),
        log_file_path: "tests/logs/test_ten_sync.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
        debug_extra: true,
        async_logging: false,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    trace!("Test Ten: TRACE and above for terminal");
    info!("Test Ten: INFO and above for file");
    trace!("TRACE: sync Test Ten");
    debug!("{}: async Test Ten", debug);
    debugx!("{}: async Test Ten", debugx);
    info!("INFO: sync Test Ten");
    warn!("WARN: sync Test Ten");
    error!(r#"{}: sync Test Ten"#, error);
}