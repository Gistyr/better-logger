// better-logger/src/lib.rs

#[cfg(any(feature = "native", feature = "wasm", feature = "relay"))]
pub mod interface;

#[cfg(feature = "native")]
pub(crate) mod native;
#[cfg(feature = "wasm")]
pub(crate) mod wasm;

#[cfg(any(feature = "native", feature = "wasm"))]
pub use interface::settings::LoggerSettings as LoggerSettings;
#[cfg(any(feature = "native", feature = "wasm", feature = "relay"))]
pub use interface::settings::NetworkFormat as NetworkFormat;
#[cfg(any(feature = "native", feature = "wasm"))]
pub use interface::logger as logger;
#[cfg(any(feature = "native", feature = "wasm"))]
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

#[cfg(any(feature = "native", feature = "wasm"))]
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

#[cfg(any(feature = "native", feature = "wasm"))]
#[doc(hidden)]
pub fn debug_extra_enabled() -> bool {
    let running_settings = RUNNING_SETTINGS.get().expect("better-logger: macro called before logger::init()");
    if running_settings.debug_extra == true {
        return true;
    }
    else {
        return false;
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[doc(hidden)]
pub fn is_async() -> bool {
    let running_settings = RUNNING_SETTINGS.get().expect("better-logger: macro called before logger::init()");
    if running_settings.async_logging == true {
        return true;
    }
    else {
        return false
    }
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[doc(hidden)]
pub fn log_async(level: &str, target: &str, msg: &str) {
    #[cfg(feature = "native")]
    crate::native::log::native_log_async(level, target, msg);

    #[cfg(feature = "wasm")]
    crate::wasm::log::wasm_log_async(level, target, msg);
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[doc(hidden)]
pub fn log_sync(level: &str, target: &str, msg: &str) {
    #[cfg(feature = "native")]
    crate::native::log::native_log_sync(level, target, msg);

    #[cfg(feature = "wasm")]
    crate::wasm::log::wasm_log_sync(level, target, msg);
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

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        {   
            let target = module_path!();
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("trace", target, &message);
            } 
            else {
                $crate::log_sync("trace", target, &message);
            }
        }
    };
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {   
            let target = module_path!();
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("debug", target, &message);
            } 
            else {
                $crate::log_sync("debug", target, &message);
            }
        }
    };
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! debugx {
    ($($arg:tt)*) => {
        {   
            if $crate::debug_extra_enabled() == true {
                let target = module_path!();
                let message = format!($($arg)*);
                if $crate::is_async() == true {
                    $crate::log_async("debug", target, &message);
                } else {
                    $crate::log_sync("debug", target, &message);
                }
            }
        }
    };
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            let target = module_path!();
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("info", target, &message);
            } 
            else {
                $crate::log_sync("info", target, &message);
            }
        }
    };
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            let target = module_path!();
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("warn", target, &message);
            } 
            else {
                $crate::log_sync("warn", target, &message);
            }
        }
    };
}

#[cfg(any(feature = "native", feature = "wasm"))]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            let target = module_path!();
            let message = format!($($arg)*);
            if $crate::is_async() == true {
                $crate::log_async("error", target, &message);
            } 
            else {
                $crate::log_sync("error", target, &message);
            }
        }
    };
}