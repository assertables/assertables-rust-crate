//! Assert a command (built with program and args) stderr is greater than to another.
//!
//! Pseudocode:<br>
//! (a_program + a_args ⇒ command ⇒ stderr) > (b_program + b_args ⇒ command ⇒ stderr)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a_program = "bin/printf-stderr";
//! let a_args = ["%s", "alfa"];
//! let b_program = "bin/printf-stderr";
//! let b_args = ["%s", "aa"];
//! assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
//! ```
//!
//! /// # Module macros
//!
//! * [`assert_program_args_stderr_gt`](macro@crate::assert_program_args_stderr_gt)
//! * [`assert_program_args_stderr_gt_as_result`](macro@crate::assert_program_args_stderr_gt_as_result)
//! * [`debug_assert_program_args_stderr_gt`](macro@crate::debug_assert_program_args_stderr_gt)

/// Assert a command (built with program and args) stderr is greater than another.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stderr) > (b_program + b_args ⇒ command ⇒ stderr)
///
/// * If true, return Result `Ok(stderr)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_program_args_stderr_gt`](macro@crate::assert_program_args_stderr_gt)
/// * [`assert_program_args_stderr_gt_as_result`](macro@crate::assert_program_args_stderr_gt_as_result)
/// * [`debug_assert_program_args_stderr_gt`](macro@crate::debug_assert_program_args_stderr_gt)
///
#[macro_export]
macro_rules! assert_program_args_stderr_gt_as_result {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {
        match (&$a_program, &$a_args, &$b_program, &$b_args) {
            (a_program, a_args, b_program, b_args) => {
                match (
                    assert_program_args_impl_prep!(a_program, a_args),
                    assert_program_args_impl_prep!(b_program, b_args)
                ) {
                    (Ok(a_output), Ok(b_output)) => {
                        let a = a_output.stderr;
                        let b = b_output.stderr;
                        if a.gt(&b) {
                            Ok((a, b))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
                                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        " b_program label: `{}`,\n",
                                        " b_program debug: `{:?}`,\n",
                                        "    b_args label: `{}`,\n",
                                        "    b_args debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($b_program),
                                    b_program,
                                    stringify!($b_args),
                                    b_args,
                                    a,
                                    b
                                )
                            )
                        }
                    },
                    (a, b) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
                                    "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    " b_program label: `{}`,\n",
                                    " b_program debug: `{:?}`,\n",
                                    "    b_args label: `{}`,\n",
                                    "    b_args debug: `{:?}`,\n",
                                    "               a: `{:?}`,\n",
                                    "               b: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($b_program),
                                b_program,
                                stringify!($b_args),
                                b_args,
                                a,
                                b
                            )
                        )
                    }
                }
            }
        }
    };

}

#[cfg(test)]
mod test_assert_program_args_stderr_gt_as_result {
    use std::sync::Once;

    #[test]
    fn gt() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "aa"];
        let actual = assert_program_args_stderr_gt_as_result!(a_program, a_args, b_program, b_args);
        assert_eq!(
            actual.unwrap(),
            (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a'])
        );
    }

    #[test]
    fn gt_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "bin/printf-stderr"
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
        fn b() -> &'static str {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "bin/printf-stderr"
        }

        static B_ARGS: Once = Once::new();
        fn b_args() -> [&'static str; 2] {
            if B_ARGS.is_completed() {
                panic!("B_ARGS.is_completed()")
            } else {
                B_ARGS.call_once(|| {})
            }
            ["%s", "aa"]
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(A_ARGS.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        assert_eq!(B_ARGS.is_completed(), false);
        let result = assert_program_args_stderr_gt_as_result!(a(), a_args(), b(), b_args());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(A_ARGS.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        assert_eq!(B_ARGS.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "alfa"];
        let actual = assert_program_args_stderr_gt_as_result!(a_program, a_args, b_program, b_args);
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"alfa\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 108, 102, 97]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn lt() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "zz"];
        let actual = assert_program_args_stderr_gt_as_result!(a_program, a_args, b_program, b_args);
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"zz\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[122, 122]`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command (built with program and args) stderr is greater than to another.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stderr) > (b_program + b_args ⇒ command ⇒ stderr)
///
/// * If true, return `()`.
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
/// let a_program = "bin/printf-stderr";
/// let a_args = ["%s", "alfa"];
/// let b_program = "bin/printf-stderr";
/// let b_args = ["%s", "aa"];
/// assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a_program = "bin/printf-stderr";
/// let a_args = ["%s", "alfa"];
/// let b_program = "bin/printf-stderr";
/// let b_args = ["%s", "zz"];
/// assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
/// # });
/// // assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_program_args_stderr_gt.html
/// //  a_program label: `a_program`,
/// //  a_program debug: `\"bin/printf-stderr\"`,
/// //     a_args label: `a_args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //  b_program label: `b_program`,
/// //  b_program debug: `\"bin/printf-stderr\"`,
/// //     b_args label: `b_args`,
/// //     b_args debug: `[\"%s\", \"zz\"]`,
/// //                a: `[97, 108, 102, 97]`,
/// //                b: `[122, 122]`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
/// #     " a_program label: `a_program`,\n",
/// #     " a_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    a_args label: `a_args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     " b_program label: `b_program`,\n",
/// #     " b_program debug: `\"bin/printf-stderr\"`,\n",
/// #     "    b_args label: `b_args`,\n",
/// #     "    b_args debug: `[\"%s\", \"zz\"]`,\n",
/// #     "               a: `[97, 108, 102, 97]`,\n",
/// #     "               b: `[122, 122]`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// /// # Module macros
///
/// * [`assert_program_args_stderr_gt`](macro@crate::assert_program_args_stderr_gt)
/// * [`assert_program_args_stderr_gt_as_result`](macro@crate::assert_program_args_stderr_gt_as_result)
/// * [`debug_assert_program_args_stderr_gt`](macro@crate::debug_assert_program_args_stderr_gt)
///
#[macro_export]
macro_rules! assert_program_args_stderr_gt {
    ($a_program:expr, $a_args:expr, $b_program:expr, $b_args:expr $(,)?) => {
        match $crate::assert_program_args_stderr_gt_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_program:expr, $a_args:expr, $b_program:expr, $($message:tt)+) => {
        match $crate::assert_program_args_stderr_gt_as_result!($a_program, $a_args, $b_program, $b_args) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_program_args_stderr_gt {
    use std::panic;

    #[test]
    fn gt() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "aa"];
        for _ in 0..1 {
            let actual = assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
            assert_eq!(actual, (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a']));
        }
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "alfa"];
        let result = panic::catch_unwind(|| {
            let _actual = assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"alfa\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 108, 102, 97]`"
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
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "zz"];
        let result = panic::catch_unwind(|| {
            let _actual = assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"zz\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[122, 122]`"
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

/// Assert a command (built with program and args) stderr is greater than another.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stderr) > (b_program + b_args ⇒ command ⇒ stderr)
///
/// This macro provides the same statements as [`assert_program_args_stderr_gt`](macro.assert_program_args_stderr_gt.html),
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
/// * [`assert_program_args_stderr_gt`](macro@crate::assert_program_args_stderr_gt)
/// * [`assert_program_args_stderr_gt`](macro@crate::assert_program_args_stderr_gt)
/// * [`debug_assert_program_args_stderr_gt`](macro@crate::debug_assert_program_args_stderr_gt)
///
#[macro_export]
macro_rules! debug_assert_program_args_stderr_gt {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_program_args_stderr_gt!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_program_args_stderr_gt {
    use std::panic;

    #[test]
    fn gt() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "aa"];
        for _ in 0..1 {
            let _actual =
                debug_assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
            // assert_eq!(actual, (vec![b'a', b'l', b'f', b'a'], vec![b'a', b'a']));
        }
    }

    #[test]
    fn eq() {
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "alfa"];
        let result = panic::catch_unwind(|| {
            let _actual =
                debug_assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"alfa\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[97, 108, 102, 97]`"
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
        let a_program = "bin/printf-stderr";
        let a_args = ["%s", "alfa"];
        let b_program = "bin/printf-stderr";
        let b_args = ["%s", "zz"];
        let result = panic::catch_unwind(|| {
            let _actual =
                debug_assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stderr_gt!(a_program, a_args, b_program, b_args)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_program_args_stderr_gt.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stderr\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_program label: `b_program`,\n",
            " b_program debug: `\"bin/printf-stderr\"`,\n",
            "    b_args label: `b_args`,\n",
            "    b_args debug: `[\"%s\", \"zz\"]`,\n",
            "               a: `[97, 108, 102, 97]`,\n",
            "               b: `[122, 122]`"
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
