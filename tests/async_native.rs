// better-logger/tests/async_native.rs

// cargo test --features native test_one -- --nocapture
// Each test has to be run individually

#[cfg(feature = "native")]
use better_logger::LoggerSettings;
#[cfg(feature = "native")]
use better_logger::logger::*;
#[cfg(feature = "native")]
use better_logger::logger;
#[cfg(feature = "native")]
use better_logger::NetworkFormat;
#[cfg(feature = "native")]
use better_logger::NetworkEndpointUrl;
#[cfg(feature = "native")]
use better_logger::SingleNet;
#[cfg(feature = "native")]
use better_logger::MultipleNet;

#[cfg(feature = "native")]
#[tokio::test]
async fn test_one() {
    let endpoints = MultipleNet {
        trace: "http://0.0.0.0:8070/trace".to_string(),
        debug: "https://hooks.slack.com/services/TTT/000/XXX".to_string(),
        debugx: "https://hooks.slack.com/services/TTT/000/XXX".to_string(),
        info: "http://127.0.0.1:8090/".to_string(),
        warn: "http://127.0.0.1:8090/".to_string(),
        error: "https://discord.com/api/webhooks/123/abc".to_string(),
    };

    let log_settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "trace".to_string(),
        log_file_path: "tests/logs/test_one_async.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Multiple(endpoints),
        network_format: NetworkFormat::JsonText { field: "text".into() },
        debug_extra: true,
        async_logging: true,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    trace!("Test One: TRACE and above, terminal and file");
    trace!("TRACE: async Test One");
    debug!("{}: async Test One", debug);
    debugx!("{}: async Test One", debugx);
    info!("INFO: async Test One");
    warn!("WARN: async Test One");
    error!(r#"{}: async Test One"#, error);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

#[cfg(feature = "native")]
#[tokio::test]
async fn test_two() {
    let log_settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "warn".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "warn".to_string(),
        log_file_path: "tests/logs/test_two_async.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Single(SingleNet { url: "http://127.0.0.1:8090/".to_string() }),
        network_format: NetworkFormat::PlainText, 
        debug_extra: true,
        async_logging: true,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::warn!("Test Two: WARN and above, terminal and file");
    logger::trace!("TRACE: async Test Two");
    logger::debug!("{}: async Test Two", debug);
    logger::debugx!("{}: async Test Two", debugx);
    logger::info!("INFO: async Test Two");
    logger::warn!("WARN: async Test Two");
    logger::error!(r#"{}: async Test Two"#, error);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

#[cfg(feature = "native")]
#[tokio::test]
async fn test_three() {
    let log_settings: LoggerSettings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "debug".to_string(),
        wasm_logging: false,
        file_logs: false,
        file_log_lvl: "debug".to_string(),
        log_file_path: "tests/logs/test_three_async.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Single(SingleNet { url: "http://127.0.0.1:8090/".to_string() }),
        network_format: NetworkFormat::PlainText, 
        debug_extra: false,
        async_logging: true,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::debug!("Test Three: DEBUG and above (no DEBUGX), terminal only");
    logger::trace!("TRACE: async Test Three");
    logger::debug!("{}: async Test Three", debug);
    logger::debugx!("{}: async Test Three", debugx);
    logger::info!("INFO: async Test Three");
    logger::warn!("WARN: async Test Three");
    logger::error!(r#"{}: async Test Three"#, error);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

#[cfg(feature = "native")]
#[tokio::test]
async fn test_four() {
    let log_settings = LoggerSettings {
        terminal_logs: false,
        terminal_log_lvl: "error".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "error".to_string(),
        log_file_path: "tests/logs/test_four_async.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Single(SingleNet { url: "http://127.0.0.1:8090/".to_string() }),
        network_format: NetworkFormat::PlainText, 
        debug_extra: false,
        async_logging: true,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    logger::error!("Test Four: ERROR and above, file only");
    logger::trace!("TRACE: async Test Four");
    logger::debug!("{}: async Test Four", debug);
    logger::debugx!("{}: async Test Four", debugx);
    logger::info!("INFO: async Test Four");
    logger::warn!("WARN: async Test Four");
    logger::error!(r#"{}: async Test Four"#, error);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

#[cfg(feature = "native")]
#[tokio::test]
async fn test_five() {
    let log_settings: LoggerSettings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: false,
        file_logs: true,
        file_log_lvl: "info".to_string(),
        log_file_path: "tests/logs/test_five_async.log".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Single(SingleNet { url: "http://127.0.0.1:8090/".to_string() }),
        network_format: NetworkFormat::PlainText, 
        debug_extra: true,
        async_logging: true,
    };

    if let Err(error) = init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }

    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    trace!("Test Five: TRACE and above for terminal");
    info!("Test Five: INFO and above for file");
    trace!("TRACE: async Test Five");
    debug!("{}: async Test Five", debug);
    debugx!("{}: async Test Five", debugx);
    info!("INFO: async Test Five");
    warn!("WARN: async Test Five");
    error!(r#"{}: async Test Five"#, error);

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}