# HOW TO USE
## 😺 ONE: Declare Feature
```rust
/* no default feature enabled (enabling both at once won't compile) */
better-logger = { version = "1.0.1", features = ["native"] }
better-logger = { version = "1.0.1", features = ["wasm"] }
```
## 💻 TWO: Settings
```rust
use better_logger::LoggerSettings;

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
- HTTP messages are sent as `Content-Type`: `text/plain; charset=utf-8`
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
# 🎉 Contributing
#### TODO:
- Validate all user settings in the init function
- Formatting options for the log messages
- UDP logging
- Append option for file logs
- Native async logging without Tokio
- Consolidation, optimization
- **This list is not exclusive, all ideas are welcome**
# License
&copy; 2025 Gistyr LLC               
This project, **better-logger**, is dual-licensed under your choice of:
- **Apache License 2.0**  
  See the [LICENSE-APACHE](LICENSE-APACHE) file or view it online at <http://www.apache.org/licenses/LICENSE-2.0>
- **MIT License**  
  See the [LICENSE-MIT](LICENSE-MIT) file or view it online at <http://opensource.org/licenses/MIT>
