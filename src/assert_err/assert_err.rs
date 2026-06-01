//! Assert expression is Err.
//!
//! Pseudocode:<br>
//! a is Err(_)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Result<i8, i8> = Err(1);
//! assert_err!(a);
//! ```
//!
//! # Module macros
//!
//! * [`assert_err`](macro@crate::assert_err)
//! * [`assert_err_as_result`](macro@crate::assert_err_as_result)
//! * [`debug_assert_err`](macro@crate::debug_assert_err)

/// Assert expression is Err.
///
/// Pseudocode:<br>
/// a is Err(a1)
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
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err_as_result`](macro@crate::assert_err_as_result)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! assert_err_as_result {
    ($a:expr $(,)?) => {
        match ($a) {
            a => match (a) {
                Err(a1) => Ok(a1),
                _ => Err(format!(
                    concat!(
                        "assertion failed: `assert_err!(a)`\n",
                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_err.html\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`",
                    ),
                    stringify!($a),
                    a
                )),
            },
        }
    };
}

#[cfg(test)]
mod test_assert_err_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a: Result<i8, i8> = Err(1);
        for _ in 0..1 {
            let actual = assert_err_as_result!(a);
            assert_eq!(actual.unwrap(), 1);
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> Result<i8, i8> {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Err(1)
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_err_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a: Result<i8, i8> = Ok(1);
        let actual = assert_err_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_err!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_err.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn move_semantics() {
        #[derive(Debug)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Err(NoCopy);

        let actual = assert_err_as_result!(a);
        assert!(actual.is_ok());
    }
}

/// Assert expression is Err.
///
/// Pseudocode:<br>
/// a is Err(a1)
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
/// let a: Result<i8, i8> = Err(1);
/// assert_err!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Result<i8, i8> = Ok(1);
/// assert_err!(a);
/// # });
/// // assertion failed: `assert_err!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_err.html
/// //  a label: `a`,
/// //  a debug: `Ok(1)`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_err!(a)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_err.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ok(1)`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err_as_result`](macro@crate::assert_err_as_result)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! assert_err {
    ($a:expr $(,)?) => {
        match $crate::assert_err_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_err_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_err {
    use std::panic;

    #[test]
    fn success() {
        let a: Result<i8, i8> = Err(1);
        for _ in 0..1 {
            let actual = assert_err!(a);
            assert_eq!(actual, 1);
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let a: Result<i8, i8> = Ok(1);
            let _actual = assert_err!(a);
        });
        let message = concat!(
            "assertion failed: `assert_err!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_err.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`",
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
    fn move_semantics() {
        #[derive(Debug)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Err(NoCopy);

        assert_err!(a);
    }
}

/// Assert expression is Err.
///
/// Pseudocode:<br>
/// a is Err(_)
///
/// This macro provides the same statements as [`assert_err`](macro.assert_err.html),
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
/// * [`assert_err`](macro@crate::assert_err)
/// * [`assert_err`](macro@crate::assert_err)
/// * [`debug_assert_err`](macro@crate::debug_assert_err)
///
#[macro_export]
macro_rules! debug_assert_err {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_err!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_err {
    use std::panic;

    #[test]
    fn success() {
        let a: Result<i8, i8> = Err(1);
        for _ in 0..1 {
            let _actual = debug_assert_err!(a);
            // assert_eq!(actual, 1);
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let a: Result<i8, i8> = Ok(1);
            let _actual = debug_assert_err!(a);
        });
        let message = concat!(
            "assertion failed: `assert_err!(a)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_err.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`",
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
    fn move_semantics() {
        #[derive(Debug)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Err(NoCopy);

        debug_assert_err!(a);
    }
}
