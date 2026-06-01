//! Assert a ::std::io::Read read_to_string() contains a pattern.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a_string) ⇒ a_string) contains (expr ⇒ b_string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let reader = "hello".as_bytes();
//! let containee = "ell";
//! assert_io_read_to_string_contains!(reader, containee);
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
//! * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
//! * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)

/// Assert a ::std::io::Read read_to_string() contains a pattern.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) contains (expr)
///
/// * If true, return Result `Ok(a_string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_io_read_to_string_contains_as_result {
    ($reader:expr, $containee:expr $(,)?) => {
        match (&$containee) {
            containee => {
                match (std::io::read_to_string($reader)) {
                    Ok(string) => {
                        if string.contains(containee) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
                                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
                                        "    reader label: `{}`,\n",
                                        "    reader debug: `{:?}`,\n",
                                        " containee label: `{}`,\n",
                                        " containee debug: `{:?}`,\n",
                                        "          string: `{:?}`",
                                    ),
                                    stringify!($reader),
                                    $reader,
                                    stringify!($containee),
                                    containee,
                                    string,
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
                                    "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
                                    "    reader label: `{}`,\n",
                                    "    reader debug: `{:?}`,\n",
                                    " containee label: `{}`,\n",
                                    " containee debug: `{:?}`,\n",
                                    "             err: `{:?}`"
                                ),
                                stringify!($reader),
                                $reader,
                                stringify!($containee),
                                containee,
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
mod test_assert_io_read_to_string_contains_as_result {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::sync::Once;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let containee = "lf";
        for _ in 0..1 {
            let actual = assert_io_read_to_string_contains_as_result!(reader, containee);
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
        fn b() -> String {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            String::from("lf")
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_contains_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let reader = "alfa".as_bytes();
        let containee = "zz";
        let actual = assert_io_read_to_string_contains_as_result!(reader, containee);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
            "    reader label: `reader`,\n",
            "    reader debug: `[97, 108, 102, 97]`,\n",
            " containee label: `containee`,\n",
            " containee debug: `\"zz\"`,\n",
            "          string: `\"alfa\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::io::Read read_to_string() contains a pattern.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) contains (expr)
///
/// * If true, return `a_string`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::io::Read;
///
/// # fn main() {
/// let reader = "hello".as_bytes();
/// let containee = "ell";
/// assert_io_read_to_string_contains!(reader, containee);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let reader = "hello".as_bytes();
/// let containee = "zz";
/// assert_io_read_to_string_contains!(reader, containee);
/// # });
/// // assertion failed: `assert_io_read_to_string_contains!(reader, containee)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_io_read_to_string_contains.html
/// //     reader label: `&reader`,
/// //     reader debug: `[104, 101, 108, 108, 111]`,
/// //  containee label: `containee`,
/// //  containee debug: `\"zz\"`,
/// //           string: `\"hello\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
/// #     "    reader label: `reader`,\n",
/// #     "    reader debug: `[104, 101, 108, 108, 111]`,\n",
/// #     " containee label: `containee`,\n",
/// #     " containee debug: `\"zz\"`,\n",
/// #     "          string: `\"hello\"`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains_as_result`](macro@crate::assert_io_read_to_string_contains_as_result)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_io_read_to_string_contains {
    ($reader:expr, $containee:expr $(,)?) => {
        match $crate::assert_io_read_to_string_contains_as_result!($reader, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_io_read_to_string_contains_as_result!($reader, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_contains {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let containee = "alfa";
        for _ in 0..1 {
            let actual = assert_io_read_to_string_contains!(reader, containee);
            assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let containee = "zz";
            let _actual = assert_io_read_to_string_contains!(reader, containee);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
            "    reader label: `reader`,\n",
            "    reader debug: `[97, 108, 102, 97]`,\n",
            " containee label: `containee`,\n",
            " containee debug: `\"zz\"`,\n",
            "          string: `\"alfa\"`",
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

/// Assert a ::std::io::Read read_to_string() contains a pattern.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) contains (expr ⇒ b_string)
///
/// This macro provides the same statements as [`assert_io_read_to_string_contains`](macro.assert_io_read_to_string_contains.html),
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
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`assert_io_read_to_string_contains`](macro@crate::assert_io_read_to_string_contains)
/// * [`debug_assert_io_read_to_string_contains`](macro@crate::debug_assert_io_read_to_string_contains)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_contains {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_contains!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_io_read_to_string_contains {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn success() {
        let reader = "alfa".as_bytes();
        let containee = "alfa";
        for _ in 0..1 {
            let _actual = debug_assert_io_read_to_string_contains!(reader, containee);
            // assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let containee = "zz";
            let _actual = debug_assert_io_read_to_string_contains!(reader, containee);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_contains!(reader, containee)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_contains.html\n",
            "    reader label: `reader`,\n",
            "    reader debug: `[97, 108, 102, 97]`,\n",
            " containee label: `containee`,\n",
            " containee debug: `\"zz\"`,\n",
            "          string: `\"alfa\"`",
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
