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
use better_logger::Single;
#[cfg(feature = "testing-wasm")]
use better_logger::logger::*;
#[cfg(feature = "testing-wasm")]
use better_logger::logger;

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
        network_endpoint_url: NetworkEndpointUrl::Single(Single { url: "http://127.0.0.1:8070/".to_string() }),
        network_format: NetworkFormat::PlainText, 
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