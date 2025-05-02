// better-logger/src/lib.rs

pub mod logger;

pub(crate) mod shared;
pub(crate) mod native;
pub(crate) mod wasm;

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[derive(Clone, Debug, PartialEq)]
pub struct LoggerSettings {
    pub terminal_logs: bool,
    pub terminal_log_lvl: String,
    pub wasm_logging: bool,
    pub file_logs: bool,
    pub file_log_lvl: String,
    pub log_file_path: String,
    pub debug_extra: bool,
    pub async_logging: bool,
}

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[doc(hidden)]
pub fn is_async() -> bool {
    let running_settings = crate::auxiliary::running_settings::RUNNING_SETTINGS.get().unwrap();
    if running_settings.async_logging == true {
        return true;
    }
    else {
        return false
    }
}

#[doc(hidden)]
pub fn log_async(level: &str, msg: &str) {
    #[cfg(feature = "native")]
    crate::core::log::native_log_async(level, msg);

    #[cfg(feature = "wasm")]
    crate::core::log::wasm_log_async(level, msg);
}

#[doc(hidden)]
pub fn log_sync(level: &str, msg: &str) {
    #[cfg(feature = "native")]
    crate::core::log::native_log_sync(level, msg);

    #[cfg(feature = "wasm")]
    crate::core::log::wasm_log_sync(level, msg);
}

#[doc(hidden)]
pub fn debug_extra_enabled() -> bool {
    let running_settings = crate::auxiliary::running_settings::RUNNING_SETTINGS.get().unwrap();

    if running_settings.debug_extra == true {
        return true;
    }
    else {
        return false;
    }
}

///0
///1
///2
///3
///4
///5
///6
///7
///8
///9

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("trace", &message);
            } 
            else {
                $crate::log_sync("trace", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("debug", &message);
            } 
            else {
                $crate::log_sync("debug", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! debugx {
    ($($arg:tt)*) => {
        {
            if $crate::debug_extra_enabled() == true {
                let message = format!($($arg)*);
                if $crate::is_async() == true {
                    $crate::log_async("debug", &message);
                } else {
                    $crate::log_sync("debug", &message);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("info", &message);
            } 
            else {
                $crate::log_sync("info", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("warn", &message);
            } 
            else {
                $crate::log_sync("warn", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("error", &message);
            } 
            else {
                $crate::log_sync("error", &message);
            }
        }
    };
}