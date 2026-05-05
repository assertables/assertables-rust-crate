//! Assert a command (built with program and args) stdout into a string is a match to a regex.
//!
//! Pseudocode:<br>
//! (a_program + a_args ⇒ command ⇒ stdout ⇒ string) is match (expr into string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! let program = "bin/printf-stdout";
//! let args = ["%s", "alfa"];
//! let matcher = Regex::new(r"lf").expect("regex");
//! assert_program_args_stdout_string_is_match!(program, args, matcher);
//! ```
//!
//! # Module macros
//!
//! * [`assert_program_args_stdout_string_is_match`](macro@crate::assert_program_args_stdout_string_is_match)
//! * [`assert_program_args_stdout_string_is_match_as_result`](macro@crate::assert_program_args_stdout_string_is_match_as_result)
//! * [`debug_assert_program_args_stdout_string_is_match`](macro@crate::debug_assert_program_args_stdout_string_is_match)

/// Assert a command (built with program and args) stdout into a string is a match to a regex.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout ⇒ string) is match (expr into string)
///
/// * If true, return Result `Ok(a_program + a_args ⇒ command ⇒ stdout ⇒ string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_program_args_stdout_string_is_match`](macro@crate::assert_program_args_stdout_string_is_match)
/// * [`assert_program_args_stdout_string_is_match_as_result`](macro@crate::assert_program_args_stdout_string_is_match_as_result)
/// * [`debug_assert_program_args_stdout_string_is_match`](macro@crate::debug_assert_program_args_stdout_string_is_match)
///
#[macro_export]
macro_rules! assert_program_args_stdout_string_is_match_as_result {
    ($a_program:expr, $a_args:expr, $matcher:expr $(,)?) => {
        match (&$a_program, &$a_args, &$matcher) {
            (a_program, a_args, matcher) => {
                match assert_program_args_impl_prep!(a_program, a_args) {
                    Ok(a_output) => {
                        let a_string = String::from_utf8(a_output.stdout).unwrap();
                        if matcher.is_match(&a_string) {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
                                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
                                        " a_program label: `{}`,\n",
                                        " a_program debug: `{:?}`,\n",
                                        "    a_args label: `{}`,\n",
                                        "    a_args debug: `{:?}`,\n",
                                        " b_matcher label: `{}`,\n",
                                        " b_matcher debug: `{:?}`,\n",
                                        "               a: `{:?}`,\n",
                                        "               b: `{:?}`"
                                    ),
                                    stringify!($a_program),
                                    a_program,
                                    stringify!($a_args),
                                    a_args,
                                    stringify!($matcher),
                                    matcher,
                                    a_string,
                                    $matcher
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
                                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
                                    " a_program label: `{}`,\n",
                                    " a_program debug: `{:?}`,\n",
                                    "    a_args label: `{}`,\n",
                                    "    a_args debug: `{:?}`,\n",
                                    " b_matcher label: `{}`,\n",
                                    " b_matcher debug: `{:?}`,\n",
                                    "        a output: `{:?}`"
                                ),
                                stringify!($a_program),
                                a_program,
                                stringify!($a_args),
                                a_args,
                                stringify!($matcher),
                                matcher,
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
mod test_assert_program_args_stdout_string_is_match_as_result {
    use regex::Regex;
    use std::sync::Once;

    #[test]
    fn success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual =
                assert_program_args_stdout_string_is_match_as_result!(a_program, a_args, b);
            assert_eq!(actual.unwrap(), "alfa");
        }
    }

    #[test]
    fn success_once() {
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
        fn b() -> Regex {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            Regex::new(r"lf").expect("regex")
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(A_ARGS.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_program_args_stdout_string_is_match_as_result!(&a(), &a_args(), &b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(A_ARGS.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"zz").expect("regex");
        let actual = assert_program_args_stdout_string_is_match_as_result!(a_program, a_args, b);
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_matcher label: `b`,\n",
            " b_matcher debug: `Regex(\"zz\")`,\n",
            "               a: `\"alfa\"`,\n",
            "               b: `Regex(\"zz\")`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a command (built with program and args) stdout into a string is a match to a regex.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout ⇒ string) is match (expr into string)
///
/// * If true, return (a_program + a_args ⇒ command ⇒ stdout ⇒ string).
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use regex::Regex;
///
/// # fn main() {
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let matcher = Regex::new(r"lf").expect("regex");
/// assert_program_args_stdout_string_is_match!(program, args, matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let program = "bin/printf-stdout";
/// let args = ["%s", "alfa"];
/// let matcher = Regex::new(r"zz").expect("regex");
/// assert_program_args_stdout_string_is_match!(program, args, matcher);
/// # });
/// // assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_program_args_stdout_string_is_match.html
/// //  a_program label: `program`,
/// //  a_program debug: `\"bin/printf-stdout\"`,
/// //     a_args label: `args`,
/// //     a_args debug: `[\"%s\", \"alfa\"]`,
/// //  b_matcher label: `matcher`,
/// //  b_matcher debug: `Regex(\"zz\")`,
/// //                a: `\"alfa\"`,
/// //                b: `Regex(\"zz\")`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
/// #     " a_program label: `program`,\n",
/// #     " a_program debug: `\"bin/printf-stdout\"`,\n",
/// #     "    a_args label: `args`,\n",
/// #     "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
/// #     " b_matcher label: `matcher`,\n",
/// #     " b_matcher debug: `Regex(\"zz\")`,\n",
/// #     "               a: `\"alfa\"`,\n",
/// #     "               b: `Regex(\"zz\")`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_program_args_stdout_string_is_match`](macro@crate::assert_program_args_stdout_string_is_match)
/// * [`assert_program_args_stdout_string_is_match_as_result`](macro@crate::assert_program_args_stdout_string_is_match_as_result)
/// * [`debug_assert_program_args_stdout_string_is_match`](macro@crate::debug_assert_program_args_stdout_string_is_match)
///
#[macro_export]
macro_rules! assert_program_args_stdout_string_is_match {
    ($a_program:expr, $a_args:expr, $matcher:expr $(,)?) => {
        match $crate::assert_program_args_stdout_string_is_match_as_result!($a_program, $a_args, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_program:expr, $a_args:expr, $matcher:expr, $($message:tt)+) => {
        match $crate::assert_program_args_stdout_string_is_match_as_result!($a_program, $a_args, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_program_args_stdout_string_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual = assert_program_args_stdout_string_is_match!(a_program, a_args, b);
            assert_eq!(actual, "alfa");
        }
    }

    #[test]
    fn failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"zz").expect("regex");
        let result = panic::catch_unwind(|| {
            let _actual = assert_program_args_stdout_string_is_match!(a_program, a_args, b);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_matcher label: `b`,\n",
            " b_matcher debug: `Regex(\"zz\")`,\n",
            "               a: `\"alfa\"`,\n",
            "               b: `Regex(\"zz\")`"
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

/// Assert a command (built with program and args) stdout into a string is a match to a regex.
///
/// Pseudocode:<br>
/// (a_program + a_args ⇒ command ⇒ stdout ⇒ string) is match (expr into string)
///
/// This macro provides the same statements as [`assert_program_args_stdout_string_is_match`](macro.assert_program_args_stdout_string_is_match.html),
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
/// * [`assert_program_args_stdout_string_is_match`](macro@crate::assert_program_args_stdout_string_is_match)
/// * [`assert_program_args_stdout_string_is_match`](macro@crate::assert_program_args_stdout_string_is_match)
/// * [`debug_assert_program_args_stdout_string_is_match`](macro@crate::debug_assert_program_args_stdout_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_program_args_stdout_string_is_match {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_program_args_stdout_string_is_match!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_program_args_stdout_string_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let _actual = debug_assert_program_args_stdout_string_is_match!(a_program, a_args, b);
            // assert_eq!(actual, "alfa");
        }
    }

    #[test]
    fn failure() {
        let a_program = "bin/printf-stdout";
        let a_args = ["%s", "alfa"];
        let b = Regex::new(r"zz").expect("regex");
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_program_args_stdout_string_is_match!(a_program, a_args, b);
        });
        let message = concat!(
            "assertion failed: `assert_program_args_stdout_string_is_match!(a_program, b_matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_program_args_stdout_string_is_match.html\n",
            " a_program label: `a_program`,\n",
            " a_program debug: `\"bin/printf-stdout\"`,\n",
            "    a_args label: `a_args`,\n",
            "    a_args debug: `[\"%s\", \"alfa\"]`,\n",
            " b_matcher label: `b`,\n",
            " b_matcher debug: `Regex(\"zz\")`,\n",
            "               a: `\"alfa\"`,\n",
            "               b: `Regex(\"zz\")`"
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
