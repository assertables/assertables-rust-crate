//! Assert a number is approximately not equal to another by using relative error a.k.a. epsilon.
//!
//! Pseudocode:<br>
//! | a - b | > ε * min(a, b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f32 = 100.0;
//! let b: f32 = 103.0;
//! let epsilon: f32 = 0.02;
//! assert_approx_ne_with_relative_error!(a, b, epsilon);
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
//! * [`assert_approx_eq_with_absolute_error`](macro@crate::assert_approx_eq_with_absolute_error)
//!   tests the absolute error (i.e. delta). This is the magnitude of the
//!   difference between the exact value and the approximation.
//!   This macro is purposefully identical to the macro [`assert_in_delta`](macro@crate::assert_in_delta).
//!
//! * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
//!   tests the relative error (i.e. epsilon). This is the absolute error divided
//!   by the magnitude of the minimum value. This can be used to compare
//!   approximations of numbers of wildly differing size.
//!   This macro is purposefully identical to [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error).
//!
//! ## Absolute error and relative error
//!
//! * For an approximation, the absolute error (i.e. delta) is the magnitude of
//!   the difference between the exact value and the approximation.
//!
//! * For an approximation, the relative error (i.e. epsilon) is the absolute
//!   error divided by the magnitude of the minimum value. This is typically useful
//!   when you want to compare approximations of numbers of wildly differing size.
//!
//! * For many kinds of applications, the relative error is more important than
//!   the absolute error.
//!
//! ## Absolute error and relative error: examples
//!
//! * Approximating the number 100 and 103 has an absolute error (delta) of 3
//!   and a relative error (epsilon) of 0.03.
//!
//! * Approximating the number 1,000,000 and 1,000,003 has an absolute error
//!   (delta) of 3, and a relative error (epsilon) of 0.000003.
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
//! * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
//! * [`assert_approx_ne_with_relative_error_as_result`](macro@crate::assert_approx_ne_with_relative_error_as_result)
//! * [`debug_assert_approx_ne_with_relative_error`](macro@crate::debug_assert_approx_ne_with_relative_error)

/// Assert a number is approximately not equal to another by using relative error a.k.a. epsilon.
///
/// Pseudocode:<br>
/// | a - b | > ε * min(a, b)
///
/// * If true, return Result `Ok((lhs, rhs))`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro is useful for runtime checks, such as checking parameters, or
/// sanitizing inputs, or handling different results in different ways.
/// 
/// ## Absolute error and relative error
///
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation.
///
/// * For an approximation, the relative error (i.e. epsilon) is the absolute
///   error divided by the magnitude of the minimum value. This is typically useful
///   when you want to compare approximations of numbers of wildly differing size.
///
/// * For many kinds of applications, the relative error is more important than
///   the absolute error.
///
/// ## Absolute error and relative error: examples
///
/// * Approximating the number 100 and 103 has an absolute error (delta) of 3
///   and a relative error (epsilon) of 0.03.
///
/// * Approximating the number 1,000,000 and 1,000,003 has an absolute error
///   (delta) of 3, and a relative error (epsilon) of 0.000003.
///
/// ## Thanks
///
/// * Thanks to [Ashley Williams](https://github.com/ashleygwilliams) for
///   creating and maintaining the `assert_approx_eq` crate.
///
/// * Thanks to [Ryan Davis](https://github.com/zenspider) and Ruby minitest for
///   creating and maintaining `assert_in_delta` and `assert_in_epsilon` code.
///
/// # Module macros
///
/// * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
/// * [`assert_approx_ne_with_relative_error_as_result`](macro@crate::assert_approx_ne_with_relative_error_as_result)
/// * [`debug_assert_approx_ne_with_relative_error`](macro@crate::debug_assert_approx_ne_with_relative_error)
///
#[macro_export]
macro_rules! assert_approx_ne_with_relative_error_as_result {
    ($a:expr, $b:expr, $epsilon:expr $(,)?) => {
        match (&$a, &$b, &$epsilon) {
            (a, b, epsilon) => {
                let abs_diff = if (a >= b) { a - b } else { b - a };
                let min = if (a < b) { a } else { b };
                let rhs = *epsilon * min;
                if abs_diff > rhs {
                    Ok((abs_diff, rhs))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_approx_ne_with_relative_error!(a, b, ε)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_approx_ne_with_relative_error.html\n",
                            "                   a label: `{}`,\n",
                            "                   a debug: `{:?}`,\n",
                            "                   b label: `{}`,\n",
                            "                   b debug: `{:?}`,\n",
                            "                   ε label: `{}`,\n",
                            "                   ε debug: `{:?}`,\n",
                            "                 | a - b |: `{:?}`,\n",
                            "             ε * min(a, b): `{:?}`,\n",
                            " | a - b | > ε * min(a, b): {}",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b,
                        stringify!($epsilon),
                        epsilon,
                        abs_diff,
                        rhs,
                        false
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_approx_ne_with_relative_error_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.02;
        for _ in 0..1 {
            let actual = assert_approx_ne_with_relative_error_as_result!(a, b, epsilon);
            assert_eq!(actual.unwrap(), (3.0, 2.0));
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> f32 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            100.0
        }

        static B: Once = Once::new();
        fn b() -> f32 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            103.0
        }

        static EPSILON: Once = Once::new();
        fn epsilon() -> f32 {
            if EPSILON.is_completed() {
                panic!("EPSILON.is_completed()")
            } else {
                EPSILON.call_once(|| {})
            }
            0.02
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        assert_eq!(EPSILON.is_completed(), false);
        let result = assert_approx_ne_with_relative_error_as_result!(a(), b(), epsilon());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        assert_eq!(EPSILON.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.03;
        let actual = assert_approx_ne_with_relative_error_as_result!(a, b, epsilon);
        let message = concat!(
            "assertion failed: `assert_approx_ne_with_relative_error!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_approx_ne_with_relative_error.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `100.0`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `103.0`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `0.03`,\n",
            "                 | a - b |: `3.0`,\n",
            "             ε * min(a, b): `3.0`,\n",
            " | a - b | > ε * min(a, b): false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a number is approximately not equal to another by using relative error a.k.a. epsilon.
///
/// Pseudocode:<br>
/// | a - b | > ε * min(a, b)
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
/// let a: f32 = 100.0;
/// let b: f32 = 103.0;
/// let epsilon: f32 = 0.02;
/// assert_approx_ne_with_relative_error!(a, b, epsilon);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: f32 = 100.0;
/// let b: f32 = 103.0;
/// let epsilon: f32 = 0.03;
/// assert_approx_ne_with_relative_error!(a, b, epsilon);
/// # });
/// // assertion failed: `assert_approx_ne_with_relative_error!(a, b, epsilon)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_approx_ne_with_relative_error.html
/// //                    a label: `a`,
/// //                    a debug: `100.0`,
/// //                    b label: `b`,
/// //                    b debug: `103.0`,
/// //                    ε label: `epsilon`,
/// //                    ε debug: `0.03`,
/// //                  | a - b |: `3.0`,
/// //              ε * min(a, b): `3.0`,\n",
/// //  | a - b | > ε * min(a, b): false"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_approx_ne_with_relative_error!(a, b, ε)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_approx_ne_with_relative_error.html\n",
/// #     "                   a label: `a`,\n",
/// #     "                   a debug: `100.0`,\n",
/// #     "                   b label: `b`,\n",
/// #     "                   b debug: `103.0`,\n",
/// #     "                   ε label: `epsilon`,\n",
/// #     "                   ε debug: `0.03`,\n",
/// #     "                 | a - b |: `3.0`,\n",
/// #     "             ε * min(a, b): `3.0`,\n",
/// #     " | a - b | > ε * min(a, b): false"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// ## Absolute error and relative error
///
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation.
///
/// * For an approximation, the relative error (i.e. epsilon) is the absolute
///   error divided by the magnitude of the minimum value. This is typically useful
///   when you want to compare approximations of numbers of wildly differing size.
///
/// * For many kinds of applications, the relative error is more important than
///   the absolute error.
///
/// ## Absolute error and relative error: examples
///
/// * Approximating the number 100 and 103 has an absolute error (delta) of 3
///   and a relative error (epsilon) of 0.03.
///
/// * Approximating the number 1,000,000 and 1,000,003 has an absolute error
///   (delta) of 3, and a relative error (epsilon) of 0.000003.
///
/// ## Thanks
///
/// * Thanks to [Ashley Williams](https://github.com/ashleygwilliams) for
///   creating and maintaining the `assert_approx_eq` crate.
///
/// * Thanks to [Ryan Davis](https://github.com/zenspider) and Ruby minitest for
///   creating and maintaining `assert_in_delta` and `assert_in_epsilon` code.
///
/// # Module macros
///
/// * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
/// * [`assert_approx_ne_with_relative_error_as_result`](macro@crate::assert_approx_ne_with_relative_error_as_result)
/// * [`debug_assert_approx_ne_with_relative_error`](macro@crate::debug_assert_approx_ne_with_relative_error)
///
#[macro_export]
macro_rules! assert_approx_ne_with_relative_error {
    ($a:expr, $b:expr, $epsilon:expr $(,)?) => {
        match $crate::assert_approx_ne_with_relative_error_as_result!($a, $b, $epsilon) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $epsilon:expr, $($message:tt)+) => {
        match $crate::assert_approx_ne_with_relative_error_as_result!($a, $b, $epsilon) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_approx_ne_with_relative_error {
    use std::panic;

    #[test]
    fn success() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.02;
        for _ in 0..1 {
            let actual = assert_approx_ne_with_relative_error!(a, b, epsilon);
            assert_eq!(actual, (3.0, 2.0));
        }
    }

    #[test]
    fn failure() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.03;
        let result = panic::catch_unwind(|| {
            let _actual = assert_approx_ne_with_relative_error!(a, b, epsilon);
        });
        let message = concat!(
            "assertion failed: `assert_approx_ne_with_relative_error!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_approx_ne_with_relative_error.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `100.0`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `103.0`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `0.03`,\n",
            "                 | a - b |: `3.0`,\n",
            "             ε * min(a, b): `3.0`,\n",
            " | a - b | > ε * min(a, b): false"
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

/// Assert a number is approximately not equal to another by using relative error a.k.a. epsilon.
///
/// Pseudocode:<br>
/// | a - b | > ε * min(a, b)
///
/// This macro provides the same statements as [`assert_approx_ne_with_relative_error`](macro.assert_approx_ne_with_relative_error.html),
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
/// * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
/// * [`assert_approx_ne_with_relative_error`](macro@crate::assert_approx_ne_with_relative_error)
/// * [`debug_assert_approx_ne_with_relative_error`](macro@crate::debug_assert_approx_ne_with_relative_error)
///
#[macro_export]
macro_rules! debug_assert_approx_ne_with_relative_error {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_approx_ne_with_relative_error!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_approx_ne_with_relative_error {
    use std::panic;

    #[test]
    fn success() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.02;
        for _ in 0..1 {
            let _actual = debug_assert_approx_ne_with_relative_error!(a, b, epsilon);
            // assert_eq!(actual, (10, 10));
        }
    }

    #[test]
    fn failure() {
        let a: f32 = 100.0;
        let b: f32 = 103.0;
        let epsilon: f32 = 0.03;
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_approx_ne_with_relative_error!(a, b, epsilon);
        });
        let message = concat!(
            "assertion failed: `assert_approx_ne_with_relative_error!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_approx_ne_with_relative_error.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `100.0`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `103.0`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `0.03`,\n",
            "                 | a - b |: `3.0`,\n",
            "             ε * min(a, b): `3.0`,\n",
            " | a - b | > ε * min(a, b): false"
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
