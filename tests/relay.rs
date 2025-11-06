// better-logger/tests/relay.rs

// cargo build --features relay
// cargo test --features relay test_relay -- --nocapture
// Each test has to be run individually

#[cfg(feature = "relay")]
use better_logger::NetworkFormat;
#[cfg(feature = "relay")]
use better_logger::RelaySettings;
#[cfg(feature = "relay")]
use better_logger::relay;

#[cfg(feature = "relay")]
#[tokio::test]
async fn test_relay() {
    let relay_settings = RelaySettings {
        listen_address: "0.0.0.0:8070".to_string(),
        output_format: NetworkFormat::JsonText { field: "text".into() },
        output_url: "".to_string(),
        cors_allowed_origins: vec!["*".into()],
        actix_workers: 1,
    };

    match relay::start(relay_settings).await {
        Ok(_) => println!("Exit Ok"),
        Err(error) => println!("Exit Err: {:?}", error),
    }
}