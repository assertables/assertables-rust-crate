//! Assert a command stderr string contains a given containee.
//!
//! Deprecated. Please rename from `assert_command_stderr_contains`
//! into `assert_command_stderr_string_contains`.

/// Assert a command stderr string contains a given containee.
///
/// Deprecated. Please rename from `assert_command_stderr_contains_as_result`
/// into `assert_command_stderr_string_contains_as_result`.
///
#[deprecated(
    note = "Please rename from `assert_command_stderr_contains_as_result` into `assert_command_stderr_string_contains_as_result`."
)]
#[macro_export]
macro_rules! assert_command_stderr_contains_as_result {
    ($($arg:tt)*) => {
        $crate::assert_command_stderr_string_contains_as_result!($($arg)*)
    }
}

/// Assert a command stderr string contains a given containee.
///
/// Deprecated. Please rename from `assert_command_stderr_contains` into
/// `assert_command_stderr_string_contains`.
///
#[deprecated(
    note = "Please rename from `assert_command_stderr_contains` into `assert_command_stderr_string_contains`."
)]
#[macro_export]
macro_rules! assert_command_stderr_contains {
    ($($arg:tt)*) => {
        $crate::assert_command_stderr_string_contains!($($arg)*)
    }
}

/// Assert a command stderr string contains a given containee.
///
/// Deprecated. Please rename from `debug_assert_command_stderr_contains` into
/// `debug_assert_command_stderr_string_contains`.
///
#[deprecated(
    note = "Please rename from `debug_assert_command_stderr_contains` into `debug_assert_command_stderr_string_contains`."
)]
#[macro_export]
macro_rules! debug_assert_command_stderr_contains {
    ($($arg:tt)*) => {
        $crate::debug_assert_command_stderr_string_contains!($($arg)*)
    }
}
