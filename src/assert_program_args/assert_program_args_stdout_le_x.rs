//! Assert a command (built with program and args) stdout is less than or equal to an expression.
//!
//! Pseudocode:<br>
//! (a_program + a_args ⇒ command ⇒ stdout) ≤ (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let program = "bin/printf-stdout";
//! let args = ["%s", "alfa"];
//! let bytes = vec![b'z', b'z'];
//! assert_program_args_stdout_le_x!(program, args, bytes);
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_le_x`](macro@crate::assert_program_args_stdout_le_x)
//! * [`assert_program_args_stdout_le_x_as_result`](macro@crate::assert_program_args_stdout_le_x_as_result)
//! * [`debug_assert_program_args_stdout_le_x`](macro@crate::debug_assert_program_args_stdout_le_x)

/// Assert a command (built with program and args) stdout is less than or equal to an expression.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout) ≤ (expr into string)
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
/// * [`assert_program_args_stdout_le_x`](macro@crate::assert_program_args_stdout_le_x)
/// * [`assert_program_args_stdout_le_x_as_result`](macro@crate::assert_program_args_stdout_le_x_as_result)
/// * [`debug_assert_program_args_stdout_le_x`](macro@crate::debug_assert_program_args_stdout_le_x)
///
#[macro_export]
macro_rules! assert_program_args_stdout_le_x_as_result {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {
        match (&$a_program, &$a_args, &$b_expr) {
            (a_program, a_args, b_expr) => {
                match assert_program_args_impl_prep!(a_program, a_args) {
                    Ok(a_output) => {
                        let a = a_output.stdout;
                        if a.le(b_expr) {
                            Ok(a)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
                                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        "    b_expr label: `{}`,\n",
                                        "    b_expr debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($b_expr),
                                    $b_expr,
                                    a,
                                    b_expr
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
                                    "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    "    b_expr label: `{}`,\n",
                                    "    b_expr debug: `{:?}`,\n",
                                    "             err: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($b_expr),
                                $b_expr,
                                err
                            )
                        )
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_program_args_stdout_le_x_as_result {
    use std::sync::Once;

    #[test]
    fn lt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'z', b'z'];
        for _ in 0..1 {
            let actual = assert_program_args_stdout_le_x_as_result!(a_program, a_args, b);
            assert_eq!(actual.unwrap(), vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn lt_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "bin/printf-stdout"
        }

        static A_ARGS: Once = Once::new();
        fn a_args() -> [&'static str; 2] {
            if A_ARGS.is_completed() {
                panic!("A_ARGS.is_completed()")
            } else {
                A_ARGS.call_once(|| {})
            }
            ["%s", "alfa"]
        }

        static B: Once = Once::new();
        fn b() -> Vec<u8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            vec![b'z', b'z']
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(A_ARGS.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_program_args_stdout_le_x_as_result!(a(), a_args(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(A_ARGS.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'l', b'f', b'a'];
        for _ in 0..1 {
            let actual = assert_program_args_stdout_le_x_as_result!(a_program, a_args, b);
            assert_eq!(actual.unwrap(), vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "bin/printf-stdout"
        }

        static A_ARGS: Once = Once::new();
        fn a_args() -> [&'static str; 2] {
            if A_ARGS.is_completed() {
                panic!("A_ARGS.is_completed()")
            } else {
                A_ARGS.call_once(|| {})
            }
            ["%s", "alfa"]
        }

        static B: Once = Once::new();
        fn b() -> Vec<u8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            vec![b'a', b'l', b'f', b'a']
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(A_ARGS.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_program_args_stdout_le_x_as_result!(a(), a_args(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(A_ARGS.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn gt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'a'];
        let actual = assert_program_args_stdout_le_x_as_result!(a_program, a_args, b);
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            "    b_expr label: `b`,\n",
            "    b_expr debug: `[97, 97]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 97]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command (built with program and args) stdout is less than or equal to an expression.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout) ≤ (expr into string)
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
///
/// # fn main() {
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let bytes = vec![b'z', b'z'];
/// assert_program_args_stdout_le_x!(program, args, bytes);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let bytes = vec![b'a', b'a'];
/// assert_program_args_stdout_le_x!(program, args, bytes);
/// # });
/// // assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_program_args_stdout_le_x.html
/// //  a_program label: `program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //     b_expr label: `bytes`,
/// //     b_expr debug: `[97, 97]`,
/// //                a: `[97, 108, 102, 97]`,
/// //                b: `[97, 97]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
/// #     " a_program label: `program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     "    b_expr label: `bytes`,\n",
/// #     "    b_expr debug: `[97, 97]`,\n",
/// #     "               a: `[97, 108, 102, 97]`,\n",
/// #     "               b: `[97, 97]`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_le_x`](macro@crate::assert_program_args_stdout_le_x)
/// * [`assert_program_args_stdout_le_x_as_result`](macro@crate::assert_program_args_stdout_le_x_as_result)
/// * [`debug_assert_program_args_stdout_le_x`](macro@crate::debug_assert_program_args_stdout_le_x)
///
#[macro_export]
macro_rules! assert_program_args_stdout_le_x {
    ($a_program:expr, $a_args:expr, $b_expr:expr $(,)?) => {
        match $crate::assert_program_args_stdout_le_x_as_result!($a_program, $a_args, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_program:expr, $a_args:expr, $b_expr:expr, $($message:tt)+) => {
        match $crate::assert_program_args_stdout_le_x_as_result!($a_program, $a_args, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_program_args_stdout_le_x {
    use std::panic;

    #[test]
    fn lt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'z', b'z'];
        for _ in 0..1 {
            let actual = assert_program_args_stdout_le_x!(a_program, a_args, b);
            assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'l', b'f', b'a'];
        for _ in 0..1 {
            let actual = assert_program_args_stdout_le_x!(a_program, a_args, b);
            assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn gt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'a'];
        let result = panic::catch_unwind(|| {
            let _actual = assert_program_args_stdout_le_x!(a_program, a_args, b);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            "    b_expr label: `b`,\n",
            "    b_expr debug: `[97, 97]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 97]`"
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

/// Assert a command (built with program and args) stdout is less than or equal to an expression.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout) ≤ (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stdout_le_x`](macro.assert_program_args_stdout_le_x.html),
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
/// * [`assert_program_args_stdout_le_x`](macro@crate::assert_program_args_stdout_le_x)
/// * [`assert_program_args_stdout_le_x`](macro@crate::assert_program_args_stdout_le_x)
/// * [`debug_assert_program_args_stdout_le_x`](macro@crate::debug_assert_program_args_stdout_le_x)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_le_x {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_le_x!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_program_args_stdout_le_x {
    use std::panic;

    #[test]
    fn lt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'z', b'z'];
        for _ in 0..1 {
            let _actual = debug_assert_program_args_stdout_le_x!(a_program, a_args, b);
            // assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'l', b'f', b'a'];
        for _ in 0..1 {
            let _actual = debug_assert_program_args_stdout_le_x!(a_program, a_args, b);
            // assert_eq!(actual, vec![b'a', b'l', b'f', b'a']);
        }
    }

    #[test]
    fn gt() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = vec![b'a', b'a'];
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_program_args_stdout_le_x!(a_program, a_args, b);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_le_x!(a_program, a_args, b_expr)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stdout_le_x.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            "    b_expr label: `b`,\n",
            "    b_expr debug: `[97, 97]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 97]`"
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
