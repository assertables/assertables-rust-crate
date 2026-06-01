//! Assert two expressions are Ok and their values are equal.
//!
//! Pseudocode:<br>
//! (a ⇒ Ok(a1) ⇒ a1) = (b ⇒ Ok(b1) ⇒ b1)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: Result<i8, i8> = Ok(1);
//! let b: Result<i8, i8> = Ok(1);
//! assert_ok_eq!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_ok_eq`](macro@crate::assert_ok_eq)
//! * [`assert_ok_eq_as_result`](macro@crate::assert_ok_eq_as_result)
//! * [`debug_assert_ok_eq`](macro@crate::debug_assert_ok_eq)

/// Assert two expressions are Ok and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = (b ⇒ Ok(b1) ⇒ b1)
///
/// * If true, return Result `Ok((a1, b1))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ok_eq`](macro@crate::assert_ok_eq)
/// * [`assert_ok_eq_as_result`](macro@crate::assert_ok_eq_as_result)
/// * [`debug_assert_ok_eq`](macro@crate::debug_assert_ok_eq)
///
#[macro_export]
macro_rules! assert_ok_eq_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            // Match by move here because we return the values back to the caller
            (Ok(a), Ok(b)) if a == b => Ok((a, b)),
            // Match by borrow here because we need to reference both the inner and outer values.
            // The outer value would be inaccessible in the format invocation if we moved.
            (ref a, ref b) => match (a, b) {
                (Ok(a1), Ok(b1)) => Err(format!(
                    concat!(
                        "assertion failed: `assert_ok_eq!(a, b)`\n",
                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`,\n",
                        " a inner: `{:?}`,\n",
                        " b label: `{}`,\n",
                        " b debug: `{:?}`,\n",
                        " b inner: `{:?}`"
                    ),
                    stringify!($a),
                    a,
                    a1,
                    stringify!($b),
                    b,
                    b1
                )),
                _ => Err(format!(
                    concat!(
                        "assertion failed: `assert_ok_eq!(a, b)`\n",
                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`,\n",
                        " b label: `{}`,\n",
                        " b debug: `{:?}`",
                    ),
                    stringify!($a),
                    a,
                    stringify!($b),
                    b
                )),
            },
        }
    };
}

#[cfg(test)]
mod test_assert_ok_eq_as_result {
    use std::sync::Once;

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(1);
        for _ in 0..1 {
            let actual = assert_ok_eq_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 1));
        }
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> Result<i8, i8> {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            Ok(1)
        }

        static B: Once = Once::new();
        fn b() -> Result<i8, i8> {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            Ok(1)
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_ok_eq_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn ne() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(2);
        let actual = assert_ok_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(2)`,\n",
            " b inner: `2`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn not_ok() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Ok(1);
        let actual = assert_ok_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(1)`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn move_semantics() {
        #[derive(Debug, PartialEq)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Ok(NoCopy);
        let b: Result<NoCopy, NoCopy> = Ok(NoCopy);

        let actual = assert_ok_eq_as_result!(a, b);
        assert!(actual.is_ok())
    }
}

/// Assert two expressions are Ok and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = (b ⇒ Ok(b1) ⇒ b1)
///
/// * If true, return `(a1, b1)`.
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
/// let a: Result<i8, i8> = Ok(1);
/// let b: Result<i8, i8> = Ok(1);
/// assert_ok_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Result<i8, i8> = Ok(1);
/// let b: Result<i8, i8> = Ok(2);
/// assert_ok_eq!(a, b);
/// # });
/// // assertion failed: `assert_ok_eq!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_ok_eq.html
/// //  a label: `a`,
/// //  a debug: `Ok(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `Ok(2)`,
/// //  b inner: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_ok_eq!(a, b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ok(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Ok(2)`,\n",
/// #     " b inner: `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ok_eq`](macro@crate::assert_ok_eq)
/// * [`assert_ok_eq_as_result`](macro@crate::assert_ok_eq_as_result)
/// * [`debug_assert_ok_eq`](macro@crate::debug_assert_ok_eq)
///
#[macro_export]
macro_rules! assert_ok_eq {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_ok_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_ok_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_ok_eq {
    use std::panic;

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(1);
        for _ in 0..1 {
            let actual = assert_ok_eq!(a, b);
            assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn ne() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(2);
        let result = panic::catch_unwind(|| {
            let _actual = assert_ok_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(2)`,\n",
            " b inner: `2`",
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
    fn not_ok() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Ok(1);
        let result = panic::catch_unwind(|| {
            let _actual = assert_ok_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(1)`",
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
        #[derive(Debug, PartialEq)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Ok(NoCopy);
        let b: Result<NoCopy, NoCopy> = Ok(NoCopy);

        assert_ok_eq!(a, b);
    }
}

/// Assert two expressions are Ok and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ok(a1) ⇒ a1) = (b ⇒ Ok(b1) ⇒ b1)
///
/// This macro provides the same statements as [`assert_ok_eq`](macro.assert_ok_eq.html),
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
/// * [`assert_ok_eq`](macro@crate::assert_ok_eq)
/// * [`assert_ok_eq`](macro@crate::assert_ok_eq)
/// * [`debug_assert_ok_eq`](macro@crate::debug_assert_ok_eq)
///
#[macro_export]
macro_rules! debug_assert_ok_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ok_eq!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_ok_eq {
    use std::panic;

    #[test]
    fn eq() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(1);
        for _ in 0..1 {
            let _actual = debug_assert_ok_eq!(a, b);
            // assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn ne() {
        let a: Result<i8, i8> = Ok(1);
        let b: Result<i8, i8> = Ok(2);
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_ok_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ok(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(2)`,\n",
            " b inner: `2`",
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
    fn not_ok() {
        let a: Result<i8, i8> = Err(1);
        let b: Result<i8, i8> = Ok(1);
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_ok_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ok_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Err(1)`,\n",
            " b label: `b`,\n",
            " b debug: `Ok(1)`",
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
        #[derive(Debug, PartialEq)]
        struct NoCopy;
        let a: Result<NoCopy, NoCopy> = Ok(NoCopy);
        let b: Result<NoCopy, NoCopy> = Ok(NoCopy);

        debug_assert_ok_eq!(a, b);
    }
}
