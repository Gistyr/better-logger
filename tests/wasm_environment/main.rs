// better-logger/tests/wasm_environment/main.rs

// Edit Trunk.toml "address" and "port" to something that works for you
// cargo build --target wasm32-unknown-unknown --bin wasm-test --features testing-wasm
// trunk build
// trunk serve

#[cfg(feature = "testing-wasm")]
use better_logger::LoggerSettings;
#[cfg(feature = "testing-wasm")]
use better_logger::NetworkFormat;
#[cfg(feature = "testing-wasm")]
use better_logger::NetworkEndpointUrl;
#[cfg(feature = "testing-wasm")]
use better_logger::SingleNet;
#[cfg(feature = "testing-wasm")]
use better_logger::logger::*;
#[cfg(feature = "testing-wasm")]
use better_logger::logger;
#[cfg(feature = "testing-wasm")]
use better_logger::RequestSigner;
#[cfg(feature = "testing-wasm")]
use better_logger::SignableRequest;

// The signer adds X-Better-Logger-* headers to every network log attempt (retries included)
// Set "request_signer: None" to test without it
#[cfg(feature = "testing-wasm")]
fn test_signer(signable_request: &SignableRequest<'_>) -> Result<Vec<(String, String)>, String> {
    let attempt_time = match js_sys::Date::new_0().to_iso_string().as_string() {
        Some(value) => value,
        None => return Err("could not read the time".to_string()),
    };
    return Ok(vec![
        ("X-Better-Logger-Signed".to_string(), "true".to_string()),
        ("X-Better-Logger-Body-Length".to_string(), signable_request.body.len().to_string()),
        ("X-Better-Logger-Attempt-Time".to_string(), attempt_time),
    ]);
}

#[cfg(feature = "testing-wasm")]
fn main() {
    let settings = LoggerSettings {
        terminal_logs: true,
        terminal_log_lvl: "trace".to_string(),
        wasm_logging: true,
        file_logs: false,
        file_log_lvl: "error".to_string(),
        log_file_path: "null".to_string(),
        network_logs: true,
        network_log_lvl: "trace".to_string(),
        network_endpoint_url: NetworkEndpointUrl::Single(SingleNet { url: "http://127.0.0.1:8070/".to_string() }),
        network_format: NetworkFormat::PlainText, 
        use_bearer_auth: false,
        bearer_auth_key: "".to_string(),
        request_signer: Some(RequestSigner(test_signer)),
        debug_extra: true,
        async_logging: true,
    };

    if let Err(error) = logger::init(settings) {
        web_sys::console::error_1(&format!("{:?}", error).as_str().into());
        std::process::exit(1);
    }

    yew::Renderer::<App>::new().render();
}

#[cfg(feature = "testing-wasm")]
#[yew::function_component(App)]
pub fn app() -> yew::Html {
    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    trace!("WASM TEST: Manually change the settings");
    trace!("TRACE: WASM test");
    debug!("{}: WASM test", debug);
    debugx!("{}: WASM test", debugx);
    info!("INFO: WASM test");
    warn!("WARN: WASM test");
    error!(r#"{}: WASM test"#, error);

    return yew::html! {
        <>
            {"TEST WASM"}
        </>
    }
}