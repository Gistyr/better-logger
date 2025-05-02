# HOW TO USE
## ONE: Settings
```rust
use better_logger::LoggerSettings;

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
## TWO: Initialize
```rust
use better_logger::logger;

fn main() {
    if let Err(error) = logger::init(log_settings) {
        eprintln!("{:?}", error);
        std::process::exit(1);
    }
}
```
## THREE: Log
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
| `wasm_logging`      |   |
| `file_logs`         | Log to file               |
| `file_log_lvl`      | Minimum level to write    |
| `log_file_path`     | Path to log file          |
| `debug_extra`       | Show `debugx!` logs       |
| `async_logging`     | Enable async logging      |
# INFORMATION
- Console logging uses [env_logger](https://crates.io/crates/env_logger)
- File logging uses the same format as the console logs
- `trace!` -> (`debug!`, `debugx!`) -> `info!` -> `warn!` -> `error!`
- better-logger will automatically create the path and file if not already created
- File logs are overwritten not appended
- Async logging uses a “fire and forget” model:
    - It spawns a new async task on the current Tokio runtime for each message
- All macros use format! under the hood, any string-like type is accepted
- Log messages routed through [env_logger](https://crates.io/crates/env_logger) are not written to the file
    - Only messages emitted via better-logger are persisted to the log file
#### Settings Alternate Namespace
*Same exact settings, just a semantic difference*
```
use better_logger::settings::Settings;

let log_settings = Settings {
    /* Same as above */
};
```
#### What is `DEBUGX`?
It is just a second debug, the `debugx!()` logs will be labeled as `DEBUG` when they print
#### Why would I want to use `DEBUGX`?
Let’s say you’re in development, so you want to see all your ``debug`` logs. However, some of your ``debug`` logs are massive and clutter your terminal.                                                                    
You can mark those verbose logs as `debugx!()` and set `debug_extra = false` to hid them.                                      
Later, if you're troubleshooting or need to view them, set `debug_extra = true`, and see your extra debug logs!                  
# Future Plans
- Append Setting
- WASM Logging
- Network Logging