//! Assert a ::std::io::Read read_to_string() is a match to a regex.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a_string) ⇒ a_string) matches matcher
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! let reader = "hello".as_bytes();
//! let matcher = Regex::new(r"ell").expect("regex");
//! assert_io_read_to_string_is_match!(reader, matcher);
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
//! * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
//! * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// * If true, return Result `Ok((a_string, b_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_io_read_to_string_is_match_as_result {
    ($reader:expr, $matcher:expr $(,)?) => {
        match (&$matcher) {
            matcher => {
                match (std::io::read_to_string($reader)) {
                    Ok(string) => {
                        if matcher.is_match(&string) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
                                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
                                        "  reader label: `{}`,\n",
                                        "  reader debug: `{:?}`,\n",
                                        " matcher label: `{}`,\n",
                                        " matcher debug: `{:?}`,\n",
                                        " reader string: `{:?}`",
                                    ),
                                    stringify!($reader),
                                    $reader,
                                    stringify!($matcher),
                                    matcher,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
                                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
                                    "  reader label: `{}`,\n",
                                    "  reader debug: `{:?}`,\n",
                                    " matcher label: `{}`,\n",
                                    " matcher debug: `{:?}`,\n",
                                    "           err: `{:?}`"
                                ),
                                stringify!($reader),
                                $reader,
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
mod test_assert_io_read_to_string_is_match_as_result {
    use regex::Regex;
    use std::sync::Once;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual = assert_io_read_to_string_is_match_as_result!(reader, matcher);
            assert_eq!(actual.unwrap(), String::from("alfa"));
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> &'static [u8] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "alfa".as_bytes()
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
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_is_match_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let reader = "alfa".as_bytes();
        let matcher = Regex::new(r"zz").expect("regex");
        let actual = assert_io_read_to_string_is_match_as_result!(reader, matcher);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
            "  reader label: `reader`,\n",
            "  reader debug: `[97, 108, 102, 97]`,\n",
            " matcher label: `matcher`,\n",
            " matcher debug: `Regex(\"zz\")`,\n",
            " reader string: `\"alfa\"`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// * If true, return `(a_string, b_string)`.
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
/// let reader = "hello".as_bytes();
/// let matcher = Regex::new(r"ell").expect("regex");
/// assert_io_read_to_string_is_match!(reader, matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let reader = "hello".as_bytes();
/// let matcher = Regex::new(r"zz").expect("regex");
/// assert_io_read_to_string_is_match!(reader, matcher);
/// # });
/// // assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_io_read_to_string_is_match.html
/// //   reader label: `reader`,
/// //   reader debug: `[104, 101, 108, 108, 111]`,
/// //  matcher label: `matcher`,
/// //  matcher debug: `Regex(\"zz\")`,
/// //  reader string: `\"hello\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
/// #     "  reader label: `reader`,\n",
/// #     "  reader debug: `[104, 101, 108, 108, 111]`,\n",
/// #     " matcher label: `matcher`,\n",
/// #     " matcher debug: `Regex(\"zz\")`,\n",
/// #     " reader string: `\"hello\"`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match_as_result`](macro@crate::assert_io_read_to_string_is_match_as_result)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_io_read_to_string_is_match {
    ($a_reader:expr, $b_matcher:expr $(,)?) => {
        match $crate::assert_io_read_to_string_is_match_as_result!($a_reader, $b_matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_reader:expr, $b_matcher:expr, $($message:tt)+) => {
        match $crate::assert_io_read_to_string_is_match_as_result!($a_reader, $b_matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual = assert_io_read_to_string_is_match!(reader, matcher);
            assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let matcher = Regex::new(r"zz").expect("regex");
            let _actual = assert_io_read_to_string_is_match!(reader, matcher);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
            "  reader label: `reader`,\n",
            "  reader debug: `[97, 108, 102, 97]`,\n",
            " matcher label: `matcher`,\n",
            " matcher debug: `Regex(\"zz\")`,\n",
            " reader string: `\"alfa\"`"
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

/// Assert a ::std::io::Read read_to_string() is a match to a regex.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) matches matcher
///
/// This macro provides the same statements as [`assert_io_read_to_string_is_match`](macro.assert_io_read_to_string_is_match.html),
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
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`assert_io_read_to_string_is_match`](macro@crate::assert_io_read_to_string_is_match)
/// * [`debug_assert_io_read_to_string_is_match`](macro@crate::debug_assert_io_read_to_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_is_match {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_is_match!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_io_read_to_string_is_match {
    use regex::Regex;
    use std::panic;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let _actual = debug_assert_io_read_to_string_is_match!(reader, matcher);
            // assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let matcher = Regex::new(r"zz").expect("regex");
            let _actual = debug_assert_io_read_to_string_is_match!(reader, matcher);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_is_match!(a_reader, &matcher)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_is_match.html\n",
            "  reader label: `reader`,\n",
            "  reader debug: `[97, 108, 102, 97]`,\n",
            " matcher label: `matcher`,\n",
            " matcher debug: `Regex(\"zz\")`,\n",
            " reader string: `\"alfa\"`"
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
