// better-logger/src/lib.rs

pub mod interface;

pub(crate) mod native;
pub(crate) mod wasm;

pub use interface::settings::LoggerSettings as LoggerSettings;
pub use interface::logger as logger;
pub use interface::logger::init as init;

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

use crate::interface::settings::RUNNING_SETTINGS;

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
    let running_settings = RUNNING_SETTINGS.get().unwrap();
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
    crate::native::log::native_log_async(level, msg);

    #[cfg(feature = "wasm")]
    crate::wasm::log::wasm_log_async(level, msg);
}

#[doc(hidden)]
pub fn log_sync(level: &str, msg: &str) {
    #[cfg(feature = "native")]
    crate::native::log::native_log_sync(level, msg);

    #[cfg(feature = "wasm")]
    crate::wasm::log::wasm_log_sync(level, msg);
}

#[doc(hidden)]
pub fn debug_extra_enabled() -> bool {
    let running_settings = RUNNING_SETTINGS.get().unwrap();
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