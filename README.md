# BETTER-LOGGER
### Full stack development, one logger to rule them all
✔️ **Native Environment**       
✔️ **WASM Environment**        
✔️ **Terminal Logging**               
✔️ **File Logging**              
✔️ **Network Logging**           
✔️ **Relay Server**               
# HOW TO USE
#### If you want to use the `relay` feature, scroll down to **RELAY SERVER** 
## 😺 ONE: Declare Feature
```rust
/* no default feature enabled (enabling both at once won't compile) */
better-logger = { version = "2.1.5", features = ["native"] }
better-logger = { version = "2.1.5", features = ["wasm"] }
```
## 💻 TWO: Settings
```rust
use better_logger::{LoggerSettings, NetworkFormat};

/* native settings */
let settings = LoggerSettings {
    terminal_logs: true,
    terminal_log_lvl: "trace".to_string(),
    wasm_logging: false, // must be false 
    file_logs: true,
    file_log_lvl: "error".to_string(),
    log_file_path: "/path/to/my/file.log".to_string(),
    network_logs: true,
    network_log_lvl: "warn".to_string(),
    network_endpoint_url: "http://127.0.0.1:8090/".to_string(),
    network_format: NetworkFormat::JsonText { field: "text".into() },
    debug_extra: true,
    async_logging: false,
};

/* wasm settings */
let settings = LoggerSettings {
    terminal_logs: true,
    terminal_log_lvl: "debug".to_string(),
    wasm_logging: true, // must be true
    file_logs: false, // must be false
    file_log_lvl: "".to_string(), // value doesn't matter
    log_file_path: "".to_string(), // value doesn't matter
    network_logs: true,
    network_log_lvl: "trace".to_string(),
    network_endpoint_url: "https://my.domain.com".to_string(),
    network_format: NetworkFormat::PlainText,
    debug_extra: true,
    async_logging: true, // if network_logs is true, async_logging must also be true 
};
```
## 💡 THREE: Initialize
```rust
use better_logger::logger;

fn main() {
    if let Err(err) = logger::init(settings) {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}
```
## ⚠️ FOUR: Log
```rust
use better_logger::logger::*;

fn my_function() {
    let debug_msg: &str = "world";
    let debugx_msg: String = format!(", world");
    let fail: &str = r#""failed""#;

    trace!("hello");
    debug!("{}", debug_msg);
    debugx!("hello{}", debugx_msg);
    info!("hello message");
    warn!("world message");
    error!(r#""hello" "world" {}"#, fail);
}
```
`trace!` ➡️ (`debug!`, `debugx!`) ➡️ `info!` ➡️ `warn!` ➡️ `error!`
| SETTING                  | DESCRIPTION                   |   
|--------------------------|-------------------------------|
| `terminal_logs`          | Log to terminal               |
| `terminal_log_lvl`       | Minimum level to display      |
| `wasm_logging`           | Log to dev tools console      | 
| `file_logs`              | Log to file                   |
| `file_log_lvl`           | Minimum level to write        |
| `log_file_path`          | Path to log file              |
| `network_logs`           | Log to an HTTP endpoint       |
| `network_log_lvl`        | Minimum level to send         |
| `network_format`         | Network message format        |
| `network_endpoint_url`   | URL to send log messages to   |
| `debug_extra`            | Show `debugx!` logs           |
| `async_logging`          | Enable async logging          |
# ℹ️ INFORMATION
- NATIVE console logging uses [env_logger](https://crates.io/crates/env_logger)
- WASM console logging uses [wasm-logger](https://crates.io/crates/wasm-logger)
- Log messages ([log](https://crates.io/crates/log)) routed through [env_logger](https://crates.io/crates/env_logger) and [wasm-logger](https://crates.io/crates/wasm-logger) **are NOT written to the file or sent via HTTP**
    - **Only** messages emitted via [better-logger](https://crates.io/crates/better-logger) are **persisted** to the log file and sent via HTTP 
    - **You can use [better-logger](https://crates.io/crates/better-logger) as your logging facade**
        - You would have to incorporate [better-logger](https://crates.io/crates/better-logger) into your low level crates, but only initialize **ONCE at the highest level**
- `logger::init()` Can only be called ONCE, subsequent calls will cause a `panic!()`
- [better-logger's](https://crates.io/crates/better-logger) NATIVE feature **requires the [tokio](https://crates.io/crates/tokio) runtime**
    - [tokio](https://crates.io/crates/tokio) is only required when `async_logging = true`, if you log synchronously (`async_logging = false`) you don’t need [tokio](https://crates.io/crates/tokio)
    - Many async frameworks will start the [tokio](https://crates.io/crates/tokio) runtime for you
- `File and network logging` uses the same `formatting` as the `NATIVE console logs`
    - `[<RFC 3339 timestamp> <LEVEL> <target>] <message>`
- better-logger will automatically create the path and file if not already created
    - `log_file_path` requires a `local` or `absolute` path, **a file name only will fail** `(E.g. log_file_path: "file.log".to_string())` 
    - File logs are **overwritten, not appended**
- Async logging uses a **“fire and forget”** model:
    - It spawns a new async task on the current ([tokio](https://crates.io/crates/tokio)) runtime for each message
- Network logging uses a **“fire and forget”** model
     - If your HTTP endpoint is down, [better-logger's](https://crates.io/crates/better-logger) will continue to run without issue
- Why is `synchronous` `network logging` NOT allowed `in WASM`? Browsers don’t allow blocking network I/O on the main thread
- Why is `file logging` NOT allowed `in WASM`? Browsers can't talk to your file system
- All macros use `format!()` under the hood, any string-like type is accepted
- The `testing-wasm` and `testing-http` features are for TESTING only
    - The NATIVE tests are `tests/async_native.rs` or `tests/sync_native.rs`
    - The WASM test is `tests/wasm_environment/main.rs`; this is also the `"wasm-test"` binary
    - The `"http-test"` binary is a simple HTTP server that will print log messages it receives
- Instructions for testing are in the comments of these files 
#### What is `DEBUGX`?
It is just a second debug, the `debugx!()` logs will be labeled as `DEBUG` when they print
#### Why would I want to use `DEBUGX`?
Let’s say you’re in development, so you want to see all your ``debug`` logs. However, some of your ``debug`` logs are massive and clutter your terminal.                                                                    
You can mark those verbose logs as `debugx!()` and set `debug_extra = false` to hide them.                                      
Later, if you're troubleshooting or need to view them, set `debug_extra = true` and see your extra debug logs!                             
## How to use `NetworkFormat`     
``` rust
pub enum NetworkFormat {
    PlainText,
    JsonText { field: String },
}
```
- `PlainText` - Sends network logs as `text/plain`
- `JsonText` - Sends network logs as `application/json`
### Integrating with external services
Sending logs to JSON endpoints is easy, just set the expected `field`.                     
- Slack: `NetworkFormat::JsonText { field: "text".into() }`
- Discord: `{ field: "content".into() }`            
- Generic: `{ field: "message".into() }`                            
# RELAY SERVER      
- When using WASM in the browser, CORS will block requests to external domains such as `hooks.slack.com` or `discord.com`.           
- To avoid this, your web client should send logs to a logging server on the same domain, which can then forward those logs to external services like Slack or Discord.
## See the working example: https://github.com/Lozlof/easy-log-forwarding
## 😺 ONE: Declare Feature
```rust
better-logger = { version = "2.1.5", features = ["relay"] }
/* if you want the relay server to also send logs, declare native as well */
/* "relay" cannot run in a WASM environment */
better-logger = { version = "2.1.5", features = ["relay", "native"] }
```
## 💻 TWO: Settings
```rust
    use better_logger::{NetworkFormat, RelaySettings};

    let settings = RelaySettings {
        listen_address: "0.0.0.0:8070".to_string(),
        output_format: NetworkFormat::JsonText { field: "text".into() },
        output_url: "https://logs.mydomain.com".to_string(),
        cors_allowed_origins: vec!["*".into()],
        actix_workers: 1,
    };
```
## 💡 THREE: start
```rust
use better_logger::relay;

#[tokio::main]
async fn main() {
    if let Err(err) = relay::start(settings).await {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}
```
| SETTING                  | DESCRIPTION                   |   
|--------------------------|-------------------------------|
| `listen_address`         | The socket to listen on       |
| `output_format`          | See NetworkFormat above       |
| `output_url`             | Logs are forwarded to here    | 
| `cors_allowed_origins`   | Domains allowed (CORS)        |
| `actix_workers`          | Number of Actix worker threads|
#### NOTE: Send log messages to the relay server as `NetworkFormat::PlainText`, it does not accept incoming `NetworkFormat::JsonText`
It can output as either option           
## More relay server info
- This relay server runs on [actix-web](https://crates.io/crates/actix-web) and requires the [tokio runtime](https://crates.io/crates/tokio)
- It is designed to be a standalone server
- In [actix-web](https://crates.io/crates/actix-web), each worker thread is an independent thread that runs the full Actix runtime.
- `cors_allowed_origins: vec!["*".into()]` allows all origins.
    - Specify a specific URL to restrict access.
# 🎉 Contributing
#### TODO:
- Validate all user settings in the init function
- Formatting options for the log messages
- UDP logging
- Append option for file logs
- Native async logging without Tokio
- Consolidation, optimization
- Add more network formats
- Increase `relay` features
- **This list is not exclusive, all ideas are welcome**
# License
&copy; 2026 Gistyr LLC               
This project, **better-logger**, is dual-licensed under your choice of:
- **Apache License 2.0**  
  See the [LICENSE-APACHE](LICENSE-APACHE) file or view it online at <http://www.apache.org/licenses/LICENSE-2.0>
- **MIT License**  
  See the [LICENSE-MIT](LICENSE-MIT) file or view it online at <http://opensource.org/licenses/MIT>
