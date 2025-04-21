# HOW TO USE
## ONE: Settings
```rust
use better_logger::LoggerSettings;

let log_settings = LoggerSettings {
    terminal_logs: true,
    terminal_log_lvl: "trace".to_string(),
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
    logger::init(log_settings);
}
```
## THREE: Log
```rust
use better_logger::logger::*;

fn my_function() {
    let debug: &str = "DEBUG";
    let debugx: String = format!("DEBUGX");
    let error: &str = r#"ERROR"#;

    trace!("TRACE: hello");
    debug!("{}: world", debug);
    debugx!("{}: hello", debugx);
    info!("INFO: world");
    warn!("WARN: hello");
    error!(r#"{}: "world""#, error);
}
```
| SETTING           | DESCRIPTION                | 
|-------------------|----------------------------|
| terminal_logs     | Log to terminal            |
| terminal_log_lvl  | Minimum log to display     |
| file_logs         | Log to file                |
| file_log_lvl      | Minimum log to write       |
| log_file_path     | Path to log file           |
| debug_extra       | debugx logs are displayed  |
| async_logging     | Enable async logging       |
# INFORMATION
- Console logging uses env_logger: https://crates.io/crates/env_logger
- File logging uses the same format as the console logs
- trace -> (debug, debugx) -> info -> warn -> error
- better-logger will automatically create the path and file if not already created
- File logs are overwritten not appended
- Uses the "fire and forget" method to handle async logging
- Spawns a new asynchronous task on the existing Tokio runtime for each log message
- better-logger's macros are using the format! macro under the hood, so you can use any string type
#### Settings Alternate Namespace
*Same exact settings, just a semantic difference*
```
use better_logger::settings::Settings;

let log_settings = Settings {
    /* Same as above */
};
```
#### What is DEBUGX?
It is just a second debug, the DEBUGX logs will be labeled as DEBUG when they print
#### Why would I want to use DEBUGX?
Example: You are in development, so you want to see all your debug logs. However, some of your debug logs print massive amounts of text that overrun your terminal.                                      
You can mark those massive logs as DEBUGX, and set debug_extra to false, which clears up your terminal.                                      
Then if you are troubleshooting, or have a reason to look at the massive logs, set debug_extra to true and you can see your extra debug logs!                  
# Future Plans
- Append Setting
- WASM Logging
- Network Logging
