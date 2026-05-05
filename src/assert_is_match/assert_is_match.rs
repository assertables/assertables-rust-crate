//! Assert a matcher is a match for an expression.
//!
//! Pseudocode:<br>
//! a.is_match(b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! let a = Regex::new(r"lf").expect("regex");
//! let b = "alfa";
//! assert_is_match!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_is_match`](macro@crate::assert_is_match)
//! * [`assert_is_match_as_result`](macro@crate::assert_is_match_as_result)
//! * [`debug_assert_is_match`](macro@crate::debug_assert_is_match)

/// Assert an expression (such as a regex) is a match for an expression (such as a string).
///
/// Pseudocode:<br>
/// a.is_match(b)
///
/// * If true, return Result `Ok(())`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_is_match`](macro@crate::assert_is_match)
/// * [`assert_is_match_as_result`](macro@crate::assert_is_match_as_result)
/// * [`debug_assert_is_match`](macro@crate::debug_assert_is_match)
///
#[macro_export]
macro_rules! assert_is_match_as_result {
    ($matcher:expr, $matchee:expr $(,)?) => {
        match (&$matcher, &$matchee) {
            (matcher, matchee) => {
                if matcher.is_match(matchee) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_is_match!(matcher, matchee)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_is_match.html\n",
                            " matcher label: `{}`,\n",
                            " matcher debug: `{:?}`,\n",
                            " matchee label: `{}`,\n",
                            " matchee debug: `{:?}`",
                        ),
                        stringify!($matcher),
                        matcher,
                        stringify!($matchee),
                        matchee,
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_is_match_as_result {
    use regex::Regex;
    use std::sync::Once;

    #[test]
    fn success() {
        let a = Regex::new(r"lf").expect("regex");
        let b = "alfa";
        for _ in 0..1 {
            let actual = assert_is_match_as_result!(a, b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> Regex {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Regex::new(r"lf").expect("regex")
        }

        static B: Once = Once::new();
        fn b() -> &'static str {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "alfa"
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_is_match_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a = Regex::new(r"xx").expect("regex");
        let b = "alfa";
        let actual = assert_is_match_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_is_match!(matcher, matchee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_is_match.html\n",
            " matcher label: `a`,\n",
            " matcher debug: `Regex(\"xx\")`,\n",
            " matchee label: `b`,\n",
            " matchee debug: `\"alfa\"`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a matcher is a match for an expression.
///
/// Pseudocode:<br>
/// a.is_match(b)
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
/// use regex::Regex;
///
/// # fn main() {
/// let a = Regex::new(r"lf").expect("regex");
/// let b = "alfa";
/// assert_is_match!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = Regex::new(r"xx").expect("regex");
/// let b = "alfa";
/// assert_is_match!(a, b);
/// # });
/// // assertion failed: `assert_is_match!(matcher, matchee)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_is_match.html
/// //  matcher label: `a`,
/// //  matcher debug: `Regex(\"xx\")`,
/// //  matchee label: `b`,
/// //  matchee debug: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_is_match!(matcher, matchee)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_is_match.html\n",
/// #     " matcher label: `a`,\n",
/// #     " matcher debug: `Regex(\"xx\")`,\n",
/// #     " matchee label: `b`,\n",
/// #     " matchee debug: `\"alfa\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_is_match`](macro@crate::assert_is_match)
/// * [`assert_is_match_as_result`](macro@crate::assert_is_match_as_result)
/// * [`debug_assert_is_match`](macro@crate::debug_assert_is_match)
///
#[macro_export]
macro_rules! assert_is_match {
    ($matcher:expr, $matchee:expr $(,)?) => {
        match $crate::assert_is_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($matcher:expr, $matchee:expr, $($message:tt)+) => {
        match $crate::assert_is_match_as_result!($matcher, $matchee) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let a = Regex::new(r"lf").expect("regex");
        let b = "alfa";
        for _ in 0..1 {
            let actual = assert_is_match!(a, b);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let a = Regex::new(r"xx").expect("regex");
            let b = "alfa";
            let _actual = assert_is_match!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_is_match!(matcher, matchee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_is_match.html\n",
            " matcher label: `a`,\n",
            " matcher debug: `Regex(\"xx\")`,\n",
            " matchee label: `b`,\n",
            " matchee debug: `\"alfa\"`"
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

/// Assert a matcher is a match for an expression.
///
/// Pseudocode:<br>
/// a.is_match(b)
///
/// This macro provides the same statements as [`assert_is_match`](macro.assert_is_match.html),
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
/// * [`assert_is_match`](macro@crate::assert_is_match)
/// * [`assert_is_match`](macro@crate::assert_is_match)
/// * [`debug_assert_is_match`](macro@crate::debug_assert_is_match)
///
#[macro_export]
macro_rules! debug_assert_is_match {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_is_match!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let a = Regex::new(r"lf").expect("regex");
        let b = "alfa";
        for _ in 0..1 {
            let _actual = debug_assert_is_match!(a, b);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let a = Regex::new(r"xx").expect("regex");
            let b = "alfa";
            let _actual = debug_assert_is_match!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_is_match!(matcher, matchee)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_is_match.html\n",
            " matcher label: `a`,\n",
            " matcher debug: `Regex(\"xx\")`,\n",
            " matchee label: `b`,\n",
            " matchee debug: `\"alfa\"`"
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
