//! Assert a number is within delta of another.
//!
//! Pseudocode:<br>
//! | a - b | ≤ Δ
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: i8 = 10;
//! let b: i8 = 11;
//! let delta: i8 = 1;
//! assert_in_delta!(a, b, delta);
//! ```
//!
//! ## Comparisons
//!
//! This crate provides macro groups that test approximations and nearness:
//!
//! * [`assert_approx_eq`](macro@crate::assert_approx_eq) and
//!   [`assert_approx_ne`](macro@crate::assert_approx_ne) test the approximate
//!   equality within 1e-6. The macro name and the approximate value are chosen
//!   to be similar to the longtime popular rust crate `assert_approx_eq`.
//!
//! * [`assert_in_delta`](macro@crate::assert_in_delta) tests the absolute error
//!   (i.e. delta). This is the magnitude of the difference between the exact
//!   value and the approximation.
//!
//! * [`assert_in_epsilon`](macro@crate::assert_in_epsilon) tests the relative
//!   error (i.e. epsilon). This is the absolute error divided by the magnitude
//!   of the exact value. This can be used to compare approximations of numbers
//!   of wildly differing size.
//!
//! Examples:
//!
//! * Approximating the number 100 and 103 has an absolute error (delta) of 3
//!   and a relative error (epsilon) of 0.03.
//!
//! * Approximating the number 1,000,000 and 1,000,003 has an absolute error
//!   (delta) of 3, and a relative error (epsilon) of 0.000003.
//!
//! * For many kinds of applications, the relative error is more important than
//!   the absolute error.
//!
//! ## Thanks
//!
//! * Thanks to [Ashley Williams](https://github.com/ashleygwilliams) for
//!   creating and maintaining the `assert_approx_eq` crate.
//!
//! * Thanks to [Ryan Davis](https://github.com/zenspider) and Ruby minitest for
//!   creating and maintaining `assert_in_delta` and `assert_in_epsilon` code.
//!
//! # Module macros
//!
//! * [`assert_in_delta`](macro@crate::assert_in_delta)
//! * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
//! * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)

/// Assert a number is within delta of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ Δ
///
/// * If true, return Result `Ok((lhs, rhs))`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro provides the same statements as [`assert_`](macro.assert_.html), except this macro
/// returns a Result, rather than doing a panic.
///
/// This macro is useful for runtime checks, such as checking parameters, or
/// sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! assert_in_delta_as_result {
    ($a:expr, $b:expr, $delta:expr $(,)?) => {
        match (&$a, &$b, &$delta) {
            (a, b, delta) => {
                let abs_diff = if (a >= b) { a - b } else { b - a };
                if abs_diff <= *delta {
                    Ok((abs_diff, *delta))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_in_delta!(a, b, Δ)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_delta.html\n",
                            "       a label: `{}`,\n",
                            "       a debug: `{:?}`,\n",
                            "       b label: `{}`,\n",
                            "       b debug: `{:?}`,\n",
                            "       Δ label: `{}`,\n",
                            "       Δ debug: `{:?}`,\n",
                            "     | a - b |: `{:?}`,\n",
                            " | a - b | ≤ Δ: {}"
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b,
                        stringify!($delta),
                        delta,
                        abs_diff,
                        false
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_in_delta_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 11;
        let delta: i8 = 1;
        for _ in 0..1 {
            let actual = assert_in_delta_as_result!(a, b, delta);
            assert_eq!(actual.unwrap(), (1 as i8, 1 as i8));
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> i8 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            10
        }

        static B: Once = Once::new();
        fn b() -> i8 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            11
        }

        static DELTA: Once = Once::new();
        fn delta() -> i8 {
            if DELTA.is_completed() {
                panic!("DELTA.is_completed()")
            } else {
                DELTA.call_once(|| {})
            }
            1
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        assert_eq!(DELTA.is_completed(), false);
        let result = assert_in_delta_as_result!(a(), b(), delta());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        assert_eq!(DELTA.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 12;
        let delta: i8 = 1;
        let actual = assert_in_delta_as_result!(a, b, delta);
        let message = concat!(
            "assertion failed: `assert_in_delta!(a, b, Δ)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_delta.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `12`,\n",
            "       Δ label: `delta`,\n",
            "       Δ debug: `1`,\n",
            "     | a - b |: `2`,\n",
            " | a - b | ≤ Δ: false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a number is within delta of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ Δ
///
/// * If true, return `(lhs, rhs)`.
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
/// let a: i8 = 10;
/// let b: i8 = 11;
/// let delta: i8 = 1;
/// assert_in_delta!(a, b, delta);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: i8 = 10;
/// let b: i8 = 12;
/// let delta: i8 = 1;
/// assert_in_delta!(a, b, delta);
/// # });
/// // assertion failed: `assert_in_delta!(a, b, Δ)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_in_delta.html
/// //        a label: `a`,
/// //        a debug: `10`,
/// //        b label: `b`,
/// //        b debug: `12`,
/// //        Δ label: `delta`,
/// //        Δ debug: `1`,
/// //      | a - b |: `2`,
/// //  | a - b | ≤ Δ: false
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_in_delta!(a, b, Δ)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_delta.html\n",
/// #     "       a label: `a`,\n",
/// #     "       a debug: `10`,\n",
/// #     "       b label: `b`,\n",
/// #     "       b debug: `12`,\n",
/// #     "       Δ label: `delta`,\n",
/// #     "       Δ debug: `1`,\n",
/// #     "     | a - b |: `2`,\n",
/// #     " | a - b | ≤ Δ: false",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// The macros `assert_in_delta` and `assert_in_epsilon` can test
/// approximations:
///
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation. For this,
///  use the macro
///
/// * For an approximation, the relative error (i.e. epsilon) is the absolute
///   error divided by the magnitude of the exact value. This can be used to
///   compare approximations of numbers of wildly differing size.
///
/// * For example, approximating the number 1,000 with an absolute error of 3
///   is, in most applications, much worse than approximating the number
///   1,000,000 with an absolute error of 3; in the first case the relative
///   error is 0.003 and in the second it is only 0.000003.
///
/// * Thanks to Ruby minitest for the example and documentation.
///
/// # Module macros
///
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta_as_result`](macro@crate::assert_in_delta_as_result)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! assert_in_delta {
    ($a:expr, $b:expr, $delta:expr $(,)?) => {
        match $crate::assert_in_delta_as_result!($a, $b, $delta) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $delta:expr, $($message:tt)+) => {
        match $crate::assert_in_delta_as_result!($a, $b, $delta) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_in_delta {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 11;
        let delta: i8 = 1;
        for _ in 0..1 {
            let actual = assert_in_delta!(a, b, delta);
            assert_eq!(actual, (1 as i8, 1 as i8));
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 12;
        let delta: i8 = 1;
        let result = panic::catch_unwind(|| {
            let _actual = assert_in_delta!(a, b, delta);
        });
        let message = concat!(
            "assertion failed: `assert_in_delta!(a, b, Δ)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_delta.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `12`,\n",
            "       Δ label: `delta`,\n",
            "       Δ debug: `1`,\n",
            "     | a - b |: `2`,\n",
            " | a - b | ≤ Δ: false"
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

/// Assert a number is within delta of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ Δ
///
/// This macro provides the same statements as [`assert_in_delta`](macro.assert_in_delta.html),
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
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`assert_in_delta`](macro@crate::assert_in_delta)
/// * [`debug_assert_in_delta`](macro@crate::debug_assert_in_delta)
///
#[macro_export]
macro_rules! debug_assert_in_delta {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_in_delta!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_in_delta {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 11;
        let delta: i8 = 1;
        for _ in 0..1 {
            let _actual = debug_assert_in_delta!(a, b, delta);
            // assert_eq!(actual, (1 as i8, 1 as i8));
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 12;
        let delta: i8 = 1;
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_in_delta!(a, b, delta);
        });
        let message = concat!(
            "assertion failed: `assert_in_delta!(a, b, Δ)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_delta.html\n",
            "       a label: `a`,\n",
            "       a debug: `10`,\n",
            "       b label: `b`,\n",
            "       b debug: `12`,\n",
            "       Δ label: `delta`,\n",
            "       Δ debug: `1`,\n",
            "     | a - b |: `2`,\n",
            " | a - b | ≤ Δ: false"
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
