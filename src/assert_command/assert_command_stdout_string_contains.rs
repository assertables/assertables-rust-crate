//! Assert a command stdout string contains a given containee.
//!
//! Pseudocode:<br>
//! (command ⇒ stdout ⇒ string) contains (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut command = Command::new("bin/printf-stdout");
//! command.args(["%s", "alfa"]);
//! let containee = "lf";
//! assert_command_stdout_string_contains!(command, containee);
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stdout_string_contains`](macro@crate::assert_command_stdout_string_contains)
//! * [`assert_command_stdout_string_contains_as_result`](macro@crate::assert_command_stdout_string_contains_as_result)
//! * [`debug_assert_command_stdout_string_contains`](macro@crate::debug_assert_command_stdout_string_contains)

/// Assert a command stdout string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) contains (expr into string)
///
/// * If true, return Result `Ok(command ⇒ stdout ⇒ string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_string_contains`](macro@crate::assert_command_stdout_string_contains)
/// * [`assert_command_stdout_string_contains_as_result`](macro@crate::assert_command_stdout_string_contains_as_result)
/// * [`debug_assert_command_stdout_string_contains`](macro@crate::debug_assert_command_stdout_string_contains)
///
#[macro_export]
macro_rules! assert_command_stdout_string_contains_as_result {
    ($command:expr, $containee:expr $(,)?) => {
        match ($command.output(), $containee) {
            (Ok(a), containee) => {
                let a = String::from_utf8(a.stdout).unwrap();
                if a.contains(containee) {
                    Ok(a)
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
                                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
                                "   command label: `{}`,\n",
                                "   command debug: `{:?}`,\n",
                                "   command value: `{:?}`,\n",
                                " containee label: `{}`,\n",
                                " containee debug: `{:?}`,\n",
                                " containee value: `{:?}`"
                            ),
                            stringify!($command),
                            $command,
                            a,
                            stringify!($containee),
                            $containee,
                            containee
                        )
                    )
                }
            },
            (a, containee) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
                            "   command label: `{}`,\n",
                            "   command debug: `{:?}`,\n",
                            "   command value: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`,\n",
                            " containee value: `{:?}`",
                        ),
                        stringify!($command),
                        $command,
                        a,
                        stringify!($containee),
                        $containee,
                        containee
                )
                )
            }
        }
    };
}

#[cfg(test)]
mod test_assert_command_stdout_string_contains_as_result {
    use std::process::Command;
    use std::sync::Once;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = "lf";
        for _ in 0..1 {
            let actual = assert_command_stdout_string_contains_as_result!(a, b);
            assert_eq!(actual.unwrap(), "alfa");
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> Command {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            a
        }

        static B: Once = Once::new();
        fn b() -> &'static str {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "lf"
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_command_stdout_string_contains_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = "zz";
        let actual = assert_command_stdout_string_contains_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            "   command value: `\"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            " containee value: `\"zz\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command stdout string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) contains (expr into string)
///
/// * If true, return (command ⇒ stdout ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// This uses [`::std::String`](https://doc.rust-lang.org/std/string/struct.String.html) method `contains`.
///
/// * The containee can be a &str, char, a slice of chars, or a function or
/// closure that determines if a character contains.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::process::Command;
///
/// # fn main() {
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "alfa"]);
/// let containee = "lf";
/// assert_command_stdout_string_contains!(command, containee);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "alfa"]);
/// let containee = "zz";
/// assert_command_stdout_string_contains!(command, containee);
/// # });
/// // assertion failed: `assert_command_stdout_string_contains!(command, containee)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_command_stdout_string_contains.html
/// //    command label: `command`,
/// //    command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,
/// //    command value: `\"alfa\"`,
/// //  containee label: `containee`,
/// //  containee debug: `\"zz\"`,
/// //  containee value: `\"zz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
/// #     "   command label: `command`,\n",
/// #     "   command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
/// #     "   command value: `\"alfa\"`,\n",
/// #     " containee label: `containee`,\n",
/// #     " containee debug: `\"zz\"`,\n",
/// #     " containee value: `\"zz\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_string_contains`](macro@crate::assert_command_stdout_string_contains)
/// * [`assert_command_stdout_string_contains_as_result`](macro@crate::assert_command_stdout_string_contains_as_result)
/// * [`debug_assert_command_stdout_string_contains`](macro@crate::debug_assert_command_stdout_string_contains)
///
#[macro_export]
macro_rules! assert_command_stdout_string_contains {
    ($command:expr, $containee:expr $(,)?) => {
        match $crate::assert_command_stdout_string_contains_as_result!($command, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($command:expr, $containee:expr, $($message:tt)+) => {
        match $crate::assert_command_stdout_string_contains_as_result!($command, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_command_stdout_string_contains {
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = "lf";
        for _ in 0..1 {
            let actual = assert_command_stdout_string_contains!(a, b);
            assert_eq!(actual, "alfa");
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = "zz";
            let _actual = assert_command_stdout_string_contains!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            "   command value: `\"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            " containee value: `\"zz\"`",
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}

/// Assert a command stdout string contains a given containee.
///
/// Pseudocode:<br>
/// (command ⇒ stdout ⇒ string) contains (expr into string)
///
/// This macro provides the same statements as [`assert_command_stdout_string_contains`](macro.assert_command_stdout_string_contains.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a program in an inconsistent state to
/// keep running, which might have unexpected consequences but does not
/// introduce unsafety as long as this only happens in safe code. The
/// performance cost of assertions, however, is not measurable in general.
/// Replacing `assert*!` with `debug_assert*!` is thus only encouraged
/// after thorough profiling, and more importantly, only in safe code!
///
/// This macro is intended to work in a similar way to
/// [`::std::debug_assert`](https://doc.rust-lang.org/std/macro.debug_assert.html).
///
/// # Module macros
///
/// * [`assert_command_stdout_string_contains`](macro@crate::assert_command_stdout_string_contains)
/// * [`assert_command_stdout_string_contains`](macro@crate::assert_command_stdout_string_contains)
/// * [`debug_assert_command_stdout_string_contains`](macro@crate::debug_assert_command_stdout_string_contains)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_string_contains {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_command_stdout_string_contains!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_command_stdout_string_contains {
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = "lf";
        for _ in 0..1 {
            let _actual = debug_assert_command_stdout_string_contains!(a, b);
            // assert_eq!(actual, "alfa");
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = "zz";
            let _actual = debug_assert_command_stdout_string_contains!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_string_contains!(command, containee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_string_contains.html\n",
            "   command label: `a`,\n",
            "   command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            "   command value: `\"alfa\"`,\n",
            " containee label: `b`,\n",
            " containee debug: `\"zz\"`,\n",
            " containee value: `\"zz\"`",
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast::<String>()
                .unwrap()
                .to_string(),
            message
        );
    }
}
