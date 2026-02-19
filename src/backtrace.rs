#[cfg(std_backtrace)]
pub(crate) use std::backtrace::{Backtrace, BacktraceStatus};

#[cfg(not(std_backtrace))]
pub(crate) enum Backtrace {}

#[cfg(std_backtrace)]
macro_rules! backtrace {
    () => {
        Some(crate::backtrace::Backtrace::capture())
    };
}

#[cfg(not(std_backtrace))]
macro_rules! backtrace {
    () => {
        None
    };
}

#[cfg(error_generic_member_access)]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        match $crate::nightly::request_ref_backtrace($err as &dyn core::error::Error) {
            Some(_) => None,
            None => backtrace!(),
        }
    };
}

#[cfg(all(
    any(feature = "std", not(anyhow_no_core_error)),
    not(error_generic_member_access),
    std_backtrace
))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        backtrace!()
    };
}

#[cfg(all(any(feature = "std", not(anyhow_no_core_error)), not(std_backtrace)))]
macro_rules! backtrace_if_absent {
    ($err:expr) => {
        None
    };
}

fn _assert_send_sync() {
    fn assert<T: Send + Sync>() {}
    assert::<Backtrace>();
}
