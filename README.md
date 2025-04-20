# How to Use
## Step One: Build the Settings
```
use better_logger::LoggerSettings;

let log_settings = LoggerSettings {
    terminal_logs: true,
    terminal_log_lvl: "trace".to_string(),
    file_logs: true,
    file_log_lvl: "trace".to_string(),
    log_file_path: "tests/logs/test_six_sync.log".to_string(),
    debug_extra: true,
    async_logging: false,
};
```
#### terminal_logs: A value of true will turn terminal logging on 
- Uses env_logger.
#### terminal_log_lvl: Logs of this level and above will print 
- trace -> (debug, debugx) -> info -> warn -> error.
#### file_logs: A value of true will turn file logging on
#### file_log_lvl: Same concept as terminal_log_lvl, except for the file logger 
- terminal_log_lvl and file_log_lvl do not have to be the same value . 
#### log_file_path: The absolute or relative path of the log file
- Path to the file, not directory.
- The file can be named anything.
- better-logger will automatically create the path and file if not already created.
- File logs are overwritten not appended, meaning better-logger will delete the existing content and write fresh logs.
#### debug_extra: A value of true means that DEBUGX logs will print
- **What is DEBUGX?**
- It is just a second debug, the DEBUGX logs will be labeled as DEBUG when they print.    
- **Then why would I want to use it?**
- For example, you are in development, so you want to see all your debug logs. However, some of your debug logs print massive amounts of text that overrun your terminal.
- You can mark those massive logs as DEBUGX, and set debug_extra to false, which clears up your terminal.
- And then if you are troubleshooting, or have a reason to look at the massive logs, then set debug_extra to true and you can see your extra debug logs!    
#### async_logging: A value of true will turn on async logging
- better-logger uses the "fire and forget" method to handle async logging.
- Spawns a new asynchronous task on the existing Tokio runtime for each log message.
#### Settings alternate namespace
- Same exact settings, just a semantic difference.
```
use better_logger::settings::Settings;

let log_settings = Settings {
    /* Same as above */
};
```
## Step Two: Initialize better-logger
```
use better_logger::logger;

fn main() {
    logger::init(log_settings);
}
```
#### Call the init function to start better-logger
- Typically at the top of your main function
## Step Three: Write your log statements
- better-logger's macros are using the format!() macro under the hood, so you can use any string type.
```
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
# Notes
- If all you need is simple logging to the terminal/console, then use env_logger: https://crates.io/crates/env_logger. This crate uses the env_logger crate for that purpose.
- The functionality that this crate provides in addition to env_logger is: DEBUGX, file logging, and async support.
# Future Plans
#### WASM Logging
#### Network Logging
#### Append option