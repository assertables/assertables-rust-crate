//! Assert a ::std::io::Read read_to_string() value is equal to an expression.
//!
//! Pseudocode:<br>
//! (reader.read_to_string(a_string) ⇒ a_string) = expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let reader = "alfa".as_bytes();
//! let x = String::from("alfa");
//! assert_io_read_to_string_eq_x!(reader, x);
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_eq_x`](macro@crate::assert_io_read_to_string_eq_x)
//! * [`assert_io_read_to_string_eq_x_as_result`](macro@crate::assert_io_read_to_string_eq_x_as_result)
//! * [`debug_assert_io_read_to_string_eq_x`](macro@crate::debug_assert_io_read_to_string_eq_x)

/// Assert a ::std::io::Read read_to_string() value is equal to an expression.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) = expr
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
/// * [`assert_io_read_to_string_eq_x`](macro@crate::assert_io_read_to_string_eq_x)
/// * [`assert_io_read_to_string_eq_x_as_result`](macro@crate::assert_io_read_to_string_eq_x_as_result)
/// * [`debug_assert_io_read_to_string_eq_x`](macro@crate::debug_assert_io_read_to_string_eq_x)
///
#[macro_export]
macro_rules! assert_io_read_to_string_eq_x_as_result {
    ($a_reader:expr, $b_expr:expr $(,)?) => {
        match (&$b_expr) {
            b_expr => {
                match (std::io::read_to_string($a_reader)) {
                    Ok(a_string) => {
                        if (a_string == *b_expr) {
                            Ok(a_string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
                                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
                                        " a_reader label: `{}`,\n",
                                        " a_reader debug: `{:?}`,\n",
                                        "   b_expr label: `{}`,\n",
                                        "   b_expr debug: `{:?}`,\n",
                                        "              a: `{:?}`",
                                    ),
                                    stringify!($a_reader),
                                    $a_reader,
                                    stringify!($b_expr),
                                    b_expr,
                                    a_string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
                                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
                                    " a_reader label: `{}`,\n",
                                    " a_reader debug: `{:?}`,\n",
                                    "   b_expr label: `{}`,\n",
                                    "   b_expr debug: `{:?}`,\n",
                                    "            err: `{:?}`"
                                ),
                                stringify!($a_reader),
                                $a_reader,
                                stringify!($b_expr),
                                b_expr,
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
mod test_assert_io_read_to_string_eq_x_as_result {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::sync::Once;

    #[test]
    fn eq() {
        let reader = "alfa".as_bytes();
        let x = String::from("alfa");
        for _ in 0..1 {
            let actual = assert_io_read_to_string_eq_x_as_result!(reader, x);
            assert_eq!(actual.unwrap(), String::from("alfa"));
        }
    }

    #[test]
    fn eq_once() {
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
            String::from("alfa")
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_eq_x_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn lt() {
        let reader = "alfa".as_bytes();
        let x = String::from("zz");
        let actual = assert_io_read_to_string_eq_x_as_result!(reader, x);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"zz\"`,\n",
            "              a: `\"alfa\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn gt() {
        let reader = "alfa".as_bytes();
        let x = String::from("aa");
        let actual = assert_io_read_to_string_eq_x_as_result!(reader, x);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"aa\"`,\n",
            "              a: `\"alfa\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::io::Read read_to_string() value is equal to an expression.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) = expr
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
/// let reader = "alfa".as_bytes();
/// let x = String::from("alfa");
/// assert_io_read_to_string_eq_x!(reader, x);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let reader = "alfa".as_bytes();
/// let x = String::from("zz");
/// assert_io_read_to_string_eq_x!(reader, x);
/// # });
/// // assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_io_read_to_string_eq_x.html
/// //  a_reader label: `reader`,
/// //  a_reader debug: `[97, 108, 102, 97]`,
/// //    b_expr label: `x`,
/// //    b_expr debug: `\"zz\"`,
/// //               a: `\"alfa\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
/// #     " a_reader label: `reader`,\n",
/// #     " a_reader debug: `[97, 108, 102, 97]`,\n",
/// #     "   b_expr label: `x`,\n",
/// #     "   b_expr debug: `\"zz\"`,\n",
/// #     "              a: `\"alfa\"`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_eq_x`](macro@crate::assert_io_read_to_string_eq_x)
/// * [`assert_io_read_to_string_eq_x_as_result`](macro@crate::assert_io_read_to_string_eq_x_as_result)
/// * [`debug_assert_io_read_to_string_eq_x`](macro@crate::debug_assert_io_read_to_string_eq_x)
///
#[macro_export]
macro_rules! assert_io_read_to_string_eq_x {
    ($a_reader:expr, $b_expr:expr $(,)?) => {
        match $crate::assert_io_read_to_string_eq_x_as_result!($a_reader, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_reader:expr, $b_expr:expr, $($message:tt)+) => {
        match $crate::assert_io_read_to_string_eq_x_as_result!($a_reader, $b_expr) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_eq_x {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn eq() {
        let reader = "alfa".as_bytes();
        let x = String::from("alfa");
        for _ in 0..1 {
            let actual = assert_io_read_to_string_eq_x!(reader, x);
            assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let x = String::from("zz");
            let _actual = assert_io_read_to_string_eq_x!(reader, x);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"zz\"`,\n",
            "              a: `\"alfa\"`",
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
    fn gt() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let x = String::from("aa");
            let _actual = assert_io_read_to_string_eq_x!(reader, x);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"aa\"`,\n",
            "              a: `\"alfa\"`",
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

/// Assert a ::std::io::Read read_to_string() value is equal to an expression.
///
/// Pseudocode:<br>
/// (reader.read_to_string(a_string) ⇒ a_string) = expr
///
/// This macro provides the same statements as [`assert_io_read_to_string_eq_x`](macro.assert_io_read_to_string_eq_x.html),
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
/// * [`assert_io_read_to_string_eq_x`](macro@crate::assert_io_read_to_string_eq_x)
/// * [`assert_io_read_to_string_eq_x`](macro@crate::assert_io_read_to_string_eq_x)
/// * [`debug_assert_io_read_to_string_eq_x`](macro@crate::debug_assert_io_read_to_string_eq_x)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_eq_x {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_eq_x!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_io_read_to_string_eq_x {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn eq() {
        let reader = "alfa".as_bytes();
        let x = String::from("alfa");
        for _ in 0..1 {
            let _actual = debug_assert_io_read_to_string_eq_x!(reader, x);
            // assert_eq!(actual, String::from("alfa"));
        }
    }

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let x = String::from("zz");
            let _actual = debug_assert_io_read_to_string_eq_x!(reader, x);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"zz\"`,\n",
            "              a: `\"alfa\"`",
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
    fn gt() {
        let result = panic::catch_unwind(|| {
            let reader = "alfa".as_bytes();
            let x = String::from("aa");
            let _actual = debug_assert_io_read_to_string_eq_x!(reader, x);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_eq_x!(a_reader, b_expr)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_io_read_to_string_eq_x.html\n",
            " a_reader label: `reader`,\n",
            " a_reader debug: `[97, 108, 102, 97]`,\n",
            "   b_expr label: `x`,\n",
            "   b_expr debug: `\"aa\"`,\n",
            "              a: `\"alfa\"`",
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
