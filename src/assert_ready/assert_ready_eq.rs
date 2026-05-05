//! Assert two expressions are Ready and their values are equal.
//!
//! Pseudocode:<br>
//! (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::task::Poll;
//! use std::task::Poll::*;
//! let a: Poll<i8> = Ready(1);
//! let b: Poll<i8> = Ready(1);
//! assert_ready_eq!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_ready_eq`](macro@crate::assert_ready_eq)
//! * [`assert_ready_eq_as_result`](macro@crate::assert_ready_eq_as_result)
//! * [`debug_assert_ready_eq`](macro@crate::debug_assert_ready_eq)

/// Assert two expressions are Ready and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
///
/// * If true, return Result `Ok((a1, b1))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro provides the same statements as [`assert_ready_eq`](macro.assert_ready_eq.html),
/// except this macro returns a Option, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_ready_eq`](macro@crate::assert_ready_eq)
/// * [`assert_ready_eq_as_result`](macro@crate::assert_ready_eq_as_result)
/// * [`debug_assert_ready_eq`](macro@crate::debug_assert_ready_eq)
///
#[macro_export]
macro_rules! assert_ready_eq_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            (Ready(a1), Ready(b1)) => {
                if a1 == b1 {
                    Ok((a1, b1))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_ready_eq!(a, b)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " a inner: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            " b inner: `{:?}`"
                        ),
                        stringify!($a),
                        $a,
                        a1,
                        stringify!($b),
                        $b,
                        b1
                    ))
                }
            }
            _ => Err(format!(
                concat!(
                    "assertion failed: `assert_ready_eq!(a, b)`\n",
                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`,\n",
                    " b label: `{}`,\n",
                    " b debug: `{:?}`",
                ),
                stringify!($a),
                $a,
                stringify!($b),
                $b
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_ready_eq_as_result {
    // use std::sync::Once;
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn eq() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(1);
        for _ in 0..1 {
            let actual = assert_ready_eq_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 1));
        }
    }

    #[test]
    fn ne() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(2);
        let actual = assert_ready_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ready(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(2)`,\n",
            " b inner: `2`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn failure_because_not_ready() {
        let a: Poll<i8> = Pending;
        let b: Poll<i8> = Ready(1);
        let actual = assert_ready_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Pending`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(1)`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert two expressions are Ready and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
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
/// use std::task::Poll;
/// use std::task::Poll::*;
/// # fn main() {
/// let a: Poll<i8> = Ready(1);
/// let b: Poll<i8> = Ready(1);
/// assert_ready_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: Poll<i8> = Ready(1);
/// let b: Poll<i8> = Ready(2);
/// assert_ready_eq!(a, b);
/// # });
/// // assertion failed: `assert_ready_eq!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_ready_eq.html
/// //  a label: `a`,
/// //  a debug: `Ready(1)`,
/// //  a inner: `1`,
/// //  b label: `b`,
/// //  b debug: `Ready(2)`,
/// //  b inner: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_ready_eq!(a, b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `Ready(1)`,\n",
/// #     " a inner: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `Ready(2)`,\n",
/// #     " b inner: `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_ready_eq`](macro@crate::assert_ready_eq)
/// * [`assert_ready_eq_as_result`](macro@crate::assert_ready_eq_as_result)
/// * [`debug_assert_ready_eq`](macro@crate::debug_assert_ready_eq)
///
#[macro_export]
macro_rules! assert_ready_eq {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_ready_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_ready_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_ready_eq {
    use std::panic;
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn eq() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(1);
        for _ in 0..1 {
            let actual = assert_ready_eq!(a, b);
            assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn ne() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(2);
        let result = panic::catch_unwind(|| {
            let _actual = assert_ready_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ready(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(2)`,\n",
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
    fn not_ready() {
        let a: Poll<i8> = Pending;
        let b: Poll<i8> = Ready(1);
        let result = panic::catch_unwind(|| {
            let _actual = assert_ready_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Pending`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(1)`",
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

/// Assert two expressions are Ready and their values are equal.
///
/// Pseudocode:<br>
/// (a ⇒ Ready(a1) ⇒ a1) = (b ⇒ Ready(b1) ⇒ b1)
///
/// This macro provides the same statements as [`assert_ready_eq`](macro.assert_ready_eq.html),
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
/// * [`assert_ready_eq`](macro@crate::assert_ready_eq)
/// * [`assert_ready_eq`](macro@crate::assert_ready_eq)
/// * [`debug_assert_ready_eq`](macro@crate::debug_assert_ready_eq)
///
#[macro_export]
macro_rules! debug_assert_ready_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_ready_eq!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_ready_eq {
    use std::panic;
    use std::task::Poll;
    use std::task::Poll::*;

    #[test]
    fn eq() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(1);
        for _ in 0..1 {
            let _actual = debug_assert_ready_eq!(a, b);
            // assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn ne() {
        let a: Poll<i8> = Ready(1);
        let b: Poll<i8> = Ready(2);
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_ready_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Ready(1)`,\n",
            " a inner: `1`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(2)`,\n",
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
    fn not_ready() {
        let a: Poll<i8> = Pending;
        let b: Poll<i8> = Ready(1);
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_ready_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_ready_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_ready_eq.html\n",
            " a label: `a`,\n",
            " a debug: `Pending`,\n",
            " b label: `b`,\n",
            " b debug: `Ready(1)`",
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
