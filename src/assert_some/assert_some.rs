//!Assert expression is Some.
//!
//! Pseudocode:<br>
//! a is Some
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Option<i8> = Option::Some(1);
//! assert_some!(a);
//! ```
//!
//! # Module macros
//!
//! * [`assert_some`](macro@crate::assert_some)
//! * [`assert_some_as_result`](macro@crate::assert_some_as_result)
//! * [`debug_assert_some`](macro@crate::debug_assert_some)

/// Assert an expression.is_some() is true.
///
/// Pseudocode:<br>
/// a is Some(a1)
///
/// * If true, return Result `Ok(a1)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some_as_result`](macro@crate::assert_some_as_result)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
///
#[macro_export]
macro_rules! assert_some_as_result {
    ($a:expr $(,)?) => {
        match ($a) {
            Some(a1) => Ok(a1),
            _ => Err(format!(
                concat!(
                    "assertion failed: `assert_some!(a)`\n",
                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_some.html\n",
                    " option label: `{}`,\n",
                    " option debug: `{:?}`",
                ),
                stringify!($a),
                $a
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_some_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a: Option<i8> = Option::Some(1);
        for _ in 0..1 {
            let actual = assert_some_as_result!(a);
            assert_eq!(actual.unwrap(), 1);
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> Option<i8> {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Option::Some(1)
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_some_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a: Option<i8> = Option::None;
        let actual = assert_some_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_some!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_some.html\n",
            " option label: `a`,\n",
            " option debug: `None`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert expression is Some.
///
/// Pseudocode:<br>
/// a is Some(a1)
///
/// * If true, return `a1`.
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
/// let a: Option<i8> = Option::Some(1);
/// assert_some!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Option<i8> = Option::None;
/// assert_some!(a);
/// # });
/// // assertion failed: `assert_some!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_some.html
/// //  option label: `a`,
/// //  option debug: `None`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_some!(a)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_some.html\n",
/// #     " option label: `a`,\n",
/// #     " option debug: `None`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some_as_result`](macro@crate::assert_some_as_result)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
///
#[macro_export]
macro_rules! assert_some {
    ($a:expr $(,)?) => {
        match $crate::assert_some_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_some_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_some {
    use std::panic;

    #[test]
    fn success() {
        let a: Option<i8> = Option::Some(1);
        for _ in 0..1 {
            let actual = assert_some!(a);
            assert_eq!(actual, 1);
        }
    }

    #[test]
    fn failure() {
        let a: Option<i8> = Option::None;
        let result = panic::catch_unwind(|| {
            let _actual = assert_some!(a);
        });
        let message = concat!(
            "assertion failed: `assert_some!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_some.html\n",
            " option label: `a`,\n",
            " option debug: `None`",
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

/// Assert expression is Some.
///
/// Pseudocode:<br>
/// a is Some
///
/// This macro provides the same statements as [`assert_some`](macro.assert_some.html),
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
/// * [`assert_some`](macro@crate::assert_some)
/// * [`assert_some`](macro@crate::assert_some)
/// * [`debug_assert_some`](macro@crate::debug_assert_some)
///
#[macro_export]
macro_rules! debug_assert_some {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_some!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_some {
    use std::panic;

    #[test]
    fn success() {
        let a: Option<i8> = Option::Some(1);
        for _ in 0..1 {
            let _actual = debug_assert_some!(a);
            // assert_eq!(actual, 1);
        }
    }

    #[test]
    fn failure() {
        let a: Option<i8> = Option::None;
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_some!(a);
        });
        let message = concat!(
            "assertion failed: `assert_some!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_some.html\n",
            " option label: `a`,\n",
            " option debug: `None`",
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
