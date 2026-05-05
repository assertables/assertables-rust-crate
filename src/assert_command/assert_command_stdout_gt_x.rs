//! Assert a command stdout string is greater than an expression.
//!
//! Pseudocode:<br>
//! (command ⇒ stdout) = (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut command = Command::new("bin/printf-stdout");
//! command.args(["%s", "alfa"]);
//! let bytes = vec![b'a', b'a'];
//! assert_command_stdout_gt_x!(command, bytes);
//! ```
//!
//! # Module macros
//!
//! * [`assert_command_stdout_gt_x`](macro@crate::assert_command_stdout_gt_x)
//! * [`assert_command_stdout_gt_x_as_result`](macro@crate::assert_command_stdout_gt_x_as_result)
//! * [`debug_assert_command_stdout_gt_x`](macro@crate::debug_assert_command_stdout_gt_x)

/// Assert a command stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stdout) = (expr into string)
///
/// * If true, return Result `Ok(stdout)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_command_stdout_gt_x`](macro@crate::assert_command_stdout_gt_x)
/// * [`assert_command_stdout_gt_x_as_result`](macro@crate::assert_command_stdout_gt_x_as_result)
/// * [`debug_assert_command_stdout_gt_x`](macro@crate::debug_assert_command_stdout_gt_x)
///
#[macro_export]
macro_rules! assert_command_stdout_gt_x_as_result {
    ($a_command:expr, $b_expr:expr $(,)?) => {
        match ($a_command.output(), &$b_expr) {
            (Ok(a_output), b_expr) => {
                let a = a_output.stdout;
                if a.gt(b_expr) {
                    Ok(a)
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
                            " command label: `{}`,\n",
                            " command debug: `{:?}`,\n",
                            " command value: `{:?}`,\n",
                            "    expr label: `{}`,\n",
                            "    expr debug: `{:?}`,\n",
                            "    expr value: `{:?}`"
                        ),
                        stringify!($a_command),
                        $a_command,
                        a,
                        stringify!($b_expr),
                        $b_expr,
                        b_expr
                    ))
                }
            }
            (a, b) => Err(format!(
                concat!(
                    "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
                    " command label: `{}`,\n",
                    " command debug: `{:?}`,\n",
                    " command value: `{:?}`,\n",
                    "    expr label: `{}`,\n",
                    "    expr debug: `{:?}`,\n",
                    "    expr value: `{:?}`"
                ),
                stringify!($a_command),
                $a_command,
                a,
                stringify!($b_expr),
                $b_expr,
                b
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_command_stdout_gt_x_as_result {
    use std::process::Command;
    use std::sync::Once;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'a'];
        for _ in 0..1 {
            let actual = assert_command_stdout_gt_x_as_result!(a, b);
            assert_eq!(actual.unwrap(), vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn gt_once() {
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
        fn b() -> Vec<u8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            vec![b'a', b'a']
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_command_stdout_gt_x_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'l', b'f', b'a'];
        let actual = assert_command_stdout_gt_x_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[97, 108, 102, 97]`,\n",
            "    expr value: `[97, 108, 102, 97]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn lt() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = vec![b'z', b'z'];
        let actual = assert_command_stdout_gt_x_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[122, 122]`,\n",
            "    expr value: `[122, 122]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stdout) = (expr into string)
///
/// * If true, return `(stdout)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
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
/// let bytes = vec![b'a', b'a'];
/// assert_command_stdout_gt_x!(command, bytes);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut command = Command::new("bin/printf-stdout");
/// command.args(["%s", "alfa"]);
/// let bytes = vec![b'z', b'z'];
/// assert_command_stdout_gt_x!(command, bytes);
/// # });
/// // assertion failed: `assert_command_stdout_gt_x!(command, expr)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_command_stdout_gt_x.html
/// //  command label: `command`,
/// //  command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,
/// //  command value: `[97, 108, 102, 97]`,
/// //     expr label: `bytes`,
/// //     expr debug: `[122, 122]`,
/// //     expr value: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
/// #     " command label: `command`,\n",
/// #     " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
/// #     " command value: `[97, 108, 102, 97]`,\n",
/// #     "    expr label: `bytes`,\n",
/// #     "    expr debug: `[122, 122]`,\n",
/// #     "    expr value: `[122, 122]`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_command_stdout_gt_x`](macro@crate::assert_command_stdout_gt_x)
/// * [`assert_command_stdout_gt_x_as_result`](macro@crate::assert_command_stdout_gt_x_as_result)
/// * [`debug_assert_command_stdout_gt_x`](macro@crate::debug_assert_command_stdout_gt_x)
///
#[macro_export]
macro_rules! assert_command_stdout_gt_x {
    ($a_command:expr, $b_expr:expr $(,)?) => {
        match $crate::assert_command_stdout_gt_x_as_result!($a_command, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_command:expr, $b_expr:expr, $($message:tt)+) => {
        match $crate::assert_command_stdout_gt_x_as_result!($a_command, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_command_stdout_gt_x {
    use std::panic;
    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'a'];
        for _ in 0..1 {
            let actual = assert_command_stdout_gt_x!(a, b);
            assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn eq() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = vec![b'a', b'l', b'f', b'a'];
            let _actual = assert_command_stdout_gt_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[97, 108, 102, 97]`,\n",
            "    expr value: `[97, 108, 102, 97]`"
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

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = vec![b'z', b'z'];
            let _actual = assert_command_stdout_gt_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[122, 122]`,\n",
            "    expr value: `[122, 122]`"
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

/// Assert a command stdout string is greater than an expression.
///
/// Pseudocode:<br>
/// (command ⇒ stdout) = (expr into string)
///
/// This macro provides the same statements as [`assert_command_stdout_gt_x`](macro.assert_command_stdout_gt_x.html),
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
/// * [`assert_command_stdout_gt_x`](macro@crate::assert_command_stdout_gt_x)
/// * [`assert_command_stdout_gt_x`](macro@crate::assert_command_stdout_gt_x)
/// * [`debug_assert_command_stdout_gt_x`](macro@crate::debug_assert_command_stdout_gt_x)
///
#[macro_export]
macro_rules! debug_assert_command_stdout_gt_x {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_command_stdout_gt_x!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_command_stdout_gt_x {
    use std::panic;
    use std::process::Command;

    #[test]
    fn gt() {
        let mut a = Command::new("bin/printf-stdout");
        a.args(["%s", "alfa"]);
        let b = vec![b'a', b'a'];
        for _ in 0..1 {
            let _actual = debug_assert_command_stdout_gt_x!(a, b);
            // assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn eq() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = vec![b'a', b'l', b'f', b'a'];
            let _actual = debug_assert_command_stdout_gt_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[97, 108, 102, 97]`,\n",
            "    expr value: `[97, 108, 102, 97]`"
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

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/printf-stdout");
            a.args(["%s", "alfa"]);
            let b = vec![b'z', b'z'];
            let _actual = debug_assert_command_stdout_gt_x!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_command_stdout_gt_x!(command, expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt_x.html\n",
            " command label: `a`,\n",
            " command debug: `\"bin/printf-stdout\" \"%s\" \"alfa\"`,\n",
            " command value: `[97, 108, 102, 97]`,\n",
            "    expr label: `b`,\n",
            "    expr debug: `[122, 122]`,\n",
            "    expr value: `[122, 122]`"
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
