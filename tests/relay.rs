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
#[test]
fn test_relay() {

}