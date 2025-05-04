# HOW TO USE
## ONE: Declare Feature
```rust
better-logger = { version = "1.0.0", features = ["native"] }
better-logger = { version = "1.0.0", features = ["wasm"] }
```
## TWO: Settings
```rust
use better_logger::LoggerSettings;

// If using the native feature 
let log_settings = LoggerSettings {
    terminal_logs: true,
    terminal_log_lvl: "trace".to_string(),
    wasm_logging: false,
    file_logs: true,
    file_log_lvl: "error".to_string(),
    log_file_path: "/path/to/my/file.log".to_string(),
    debug_extra: true,
    async_logging: false,
};
```
## THREE: Initialize
```rust
use better_logger::logger;

fn main() {
    if let Err(err) = logger::init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}
```
## FOUR: Log
```rust
use better_logger::logger::*;

fn my_function() {
    let debug_msg: &str = "world";
    let debugx_msg: String = format!(", world");
    let error: &str = r#""failed""#;

    trace!("hello");
    debug!("{}", debug_msg);
    debugx!("hello{}", debugx_msg);
    info!("hello message");
    warn!("world message");
    error!(r#""hello" "world" {}"#, error);
}
```
| SETTING             | DESCRIPTION               | 
|---------------------|---------------------------|
| `terminal_logs`     | Log to terminal           |
| `terminal_log_lvl`  | Minimum level to display  |
| `wasm_logging`      | Log to dev tools console  | 
| `file_logs`         | Log to file               |
| `file_log_lvl`      | Minimum level to write    |
| `log_file_path`     | Path to log file          |
| `debug_extra`       | Show `debugx!` logs       |
| `async_logging`     | Enable async logging      |
# Rules
**better-logger has no default feature:**     
- Using both will fail

**If using the wasm feature:**
- wasm_logging must be true
- file_logs must be false

**If using the native feature**
- wasm_logging must be false
# INFORMATION
- NATIVE console logging uses [env_logger](https://crates.io/crates/env_logger)
- WASM console logging uses [wasm-logger](https://crates.io/crates/wasm-logger)
- File logging uses the same format as the NATIVE console logs
- `trace!` -> (`debug!`, `debugx!`) -> `info!` -> `warn!` -> `error!`
- better-logger will automatically create the path and file if not already created
- File logs are overwritten not appended
- Async logging uses a “fire and forget” model:
    - It spawns a new async task on the current Tokio runtime for each message
- All macros use format! under the hood, any string-like type is accepted
- Log messages ([log](https://crates.io/crates/log)) routed through [env_logger](https://crates.io/crates/env_logger) and [wasm-logger](https://crates.io/crates/wasm-logger) are not written to the file or sent over the network
    - Only messages emitted via [better-logger](https://crates.io/crates/better-logger) are persisted to the log file and sent over the network
    - You can use better-logger as your logging facade 
        - (You would have to incorporate [better-logger](https://crates.io/crates/better-logger) into your low level crates, but only initialize once at the highest level)
#### Possible errors

#### What is `DEBUGX`?
It is just a second debug, the `debugx!()` logs will be labeled as `DEBUG` when they print
#### Why would I want to use `DEBUGX`?
Let’s say you’re in development, so you want to see all your ``debug`` logs. However, some of your ``debug`` logs are massive and clutter your terminal.                                                                    
You can mark those verbose logs as `debugx!()` and set `debug_extra = false` to hid them.                                      
Later, if you're troubleshooting or need to view them, set `debug_extra = true`, and see your extra debug logs!                  
# Contributing
#### TODO:
- Validate all user settings in the init function
- Formatting options for the log messages
- Append option for file logs
- Consolidation, optimization


Browsers don’t allow blocking network I/O on the main thread