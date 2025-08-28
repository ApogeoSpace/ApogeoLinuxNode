use core::ffi::c_char;

#[repr(C)]
pub enum ShimLogLevel {
    InternalLogLevelTrace = 0,
    InternalLogLevelDebug,
    InternalLogLevelInfo,
    InternalLogLevelNotice,
    InternalLogLevelWarning,
    InternalLogLevelError,
    InternalLogLevelAlert,
    InternalLogLevelEmerg,
}

unsafe extern "C" {
    fn __internal_log_shim(level: ShimLogLevel, msg: *const c_char);
}

pub fn kernel_log(level: ShimLogLevel, msg: &str){
    unsafe { __internal_log_shim(level, msg.as_ptr() as *const c_char) }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)+) => { crate::logging::kernel_log($level, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelTrace, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelDebug, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelInfo, format!($($arg)+).as_str()) }
}


#[macro_export]
macro_rules! log_notice {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelNotice, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelWarning, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelError, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_alert {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelAlert, format!($($arg)+).as_str()) }
}

#[macro_export]
macro_rules! log_emerg {
    ($($arg:tt)+) => { crate::logging::kernel_log(crate::logging::ShimLogLevel::InternalLogLevelEmerg, format!($($arg)+).as_str()) }
}
