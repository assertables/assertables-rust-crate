//! Assert an expression (such as a string or array) is not empty.
//!
//! Pseudocode:<br>
//! ¬ a.is_empty()
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "alfa";
//! assert_not_empty!(a);
//! ```
//!
//! # Module macros
//!
//! * [`assert_not_empty`](macro@crate::assert_not_empty)
//! * [`assert_not_empty_as_result`](macro@crate::assert_not_empty_as_result)
//! * [`debug_assert_not_empty`](macro@crate::debug_assert_not_empty)

/// Assert an expression (such as a string or array) is not empty.
///
/// Pseudocode:<br>
/// ¬ a.is_empty()
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
/// * [`assert_not_empty`](macro@crate::assert_not_empty)
/// * [`assert_not_empty_as_result`](macro@crate::assert_not_empty_as_result)
/// * [`debug_assert_not_empty`](macro@crate::debug_assert_not_empty)
///
#[macro_export]
macro_rules! assert_not_empty_as_result {
    ($a:expr $(,)?) => {
        match (&$a) {
            a => {
                if !(a.is_empty()) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_not_empty!(a)`\n",
                            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_not_empty.html\n",
                            " label: `{}`,\n",
                            " debug: `{:?}`"
                        ),
                        stringify!($a),
                        a,
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_not_empty_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a = "alfa";
        for _ in 0..1 {
            let actual = assert_not_empty_as_result!(a);
            assert_eq!(actual.unwrap(), ());
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
            "alfa"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_empty_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a = "";
        let actual = assert_not_empty_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_not_empty!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_not_empty.html\n",
            " label: `a`,\n",
            " debug: `\"\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert an expression (such as a string or array) is not empty.
///
/// Pseudocode:<br>
/// ¬ a.is_empty()
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
/// let a = "alfa";
/// assert_not_empty!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "";
/// assert_not_empty!(a);
/// # });
/// // assertion failed: `assert_not_empty!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_not_empty.html
/// //  label: `a`,
/// //  debug: `\"\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_not_empty!(a)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_not_empty.html\n",
/// #     " label: `a`,\n",
/// #     " debug: `\"\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_not_empty`](macro@crate::assert_not_empty)
/// * [`assert_not_empty_as_result`](macro@crate::assert_not_empty_as_result)
/// * [`debug_assert_not_empty`](macro@crate::debug_assert_not_empty)
///
#[macro_export]
macro_rules! assert_not_empty {
    ($a:expr $(,)?) => {
        match $crate::assert_not_empty_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_not_empty_as_result!($a) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_not_empty {
    use std::panic;

    #[test]
    fn success() {
        let a = "alfa";
        for _ in 0..1 {
            let actual = assert_not_empty!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a = "";
        let result = panic::catch_unwind(|| {
            let _actual = assert_not_empty!(a);
        });
        let message = concat!(
            "assertion failed: `assert_not_empty!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_not_empty.html\n",
            " label: `a`,\n",
            " debug: `\"\"`",
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

/// Assert an expression (such as a string or array) is not empty.
///
/// Pseudocode:<br>
/// ¬ a.is_empty()
///
/// This macro provides the same statements as [`assert_not_empty`](macro.assert_not_empty.html),
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
/// * [`assert_not_empty`](macro@crate::assert_not_empty)
/// * [`assert_not_empty`](macro@crate::assert_not_empty)
/// * [`debug_assert_not_empty`](macro@crate::debug_assert_not_empty)
///
#[macro_export]
macro_rules! debug_assert_not_empty {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_not_empty!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_not_empty {
    use std::panic;

    #[test]
    fn success() {
        let a = "alfa";
        for _ in 0..1 {
            let _actual = debug_assert_not_empty!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a = "";
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_not_empty!(a);
        });
        let message = concat!(
            "assertion failed: `assert_not_empty!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_not_empty.html\n",
            " label: `a`,\n",
            " debug: `\"\"`",
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
