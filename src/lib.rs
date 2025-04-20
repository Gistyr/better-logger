// better-logger/src/lib.rs

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::logger::is_async() == true {
                $crate::logger::log_async("trace", &message);
            } 
            else {
                $crate::logger::log_sync("trace", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::logger::is_async() == true {
                $crate::logger::log_async("debug", &message);
            } 
            else {
                $crate::logger::log_sync("debug", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! debugx {
    ($($arg:tt)*) => {
        {
            if $crate::logger::debug_extra_enabled() == true {
                let message = format!($($arg)*);
                if $crate::logger::is_async() == true {
                    $crate::logger::log_async("debug", &message);
                } else {
                    $crate::logger::log_sync("debug", &message);
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
            if $crate::logger::is_async() == true {
                $crate::logger::log_async("info", &message);
            } 
            else {
                $crate::logger::log_sync("info", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::logger::is_async() == true {
                $crate::logger::log_async("warn", &message);
            } 
            else {
                $crate::logger::log_sync("warn", &message);
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        {
            let message = format!($($arg)*);
            if $crate::logger::is_async() == true {
                $crate::logger::log_async("error", &message);
            } 
            else {
                $crate::logger::log_sync("error", &message);
            }
        }
    };
}

pub mod settings;
pub mod logger;

pub(crate) mod core;
pub(crate) mod auxiliary;

pub use settings::Settings as LoggerSettings;