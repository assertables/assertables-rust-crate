//! Assert a number is within epsilon of another.
//!
//! Pseudocode:<br>
//! | a - b | ≤ ε * min(a, b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: i8 = 10;
//! let b: i8 = 20;
//! let epsilon: i8 = 1;
//! assert_in_epsilon!(a, b, epsilon);
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
//! * [`assert_in_epsilon`](macro@crate::assert_in_epsilon)
//! * [`assert_in_epsilon_as_result`](macro@crate::assert_in_epsilon_as_result)
//! * [`debug_assert_in_epsilon`](macro@crate::debug_assert_in_epsilon)

/// Assert a number is within epsilon of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ ε * min(a, b)
///
/// * If true, return Result `Ok((lhs, rhs))`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_in_epsilon`](macro@crate::assert_in_epsilon)
/// * [`assert_in_epsilon_as_result`](macro@crate::assert_in_epsilon_as_result)
/// * [`debug_assert_in_epsilon`](macro@crate::debug_assert_in_epsilon)
///
#[macro_export]
macro_rules! assert_in_epsilon_as_result {
    ($a:expr, $b:expr, $epsilon:expr $(,)?) => {
        match (&$a, &$b, &$epsilon) {
            (a, b, epsilon) => {
                let abs_diff = if (a >= b) { a - b } else { b - a };
                let min = if (a < b) { a } else { b };
                let rhs = *epsilon * min;
                if abs_diff <= rhs {
                    Ok((abs_diff, rhs))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_in_epsilon!(a, b, ε)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_epsilon.html\n",
                            "                   a label: `{}`,\n",
                            "                   a debug: `{:?}`,\n",
                            "                   b label: `{}`,\n",
                            "                   b debug: `{:?}`,\n",
                            "                   ε label: `{}`,\n",
                            "                   ε debug: `{:?}`,\n",
                            "                 | a - b |: `{:?}`,\n",
                            "             ε * min(a, b): `{:?}`,\n",
                            " | a - b | ≤ ε * min(a, b): {}",
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
mod test_assert_in_epsilon_as_result {
    use std::sync::Once;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 20;
        let epsilon: i8 = 1;
        for _ in 0..1 {
            let actual = assert_in_epsilon_as_result!(a, b, epsilon);
            assert_eq!(actual.unwrap(), (10, 10));
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

        static EPSILON: Once = Once::new();
        fn epsilon() -> i8 {
            if EPSILON.is_completed() {
                panic!("EPSILON.is_completed()")
            } else {
                EPSILON.call_once(|| {})
            }
            1
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        assert_eq!(EPSILON.is_completed(), false);
        let result = assert_in_epsilon_as_result!(a(), b(), epsilon());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
        assert_eq!(EPSILON.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 30;
        let epsilon: i8 = 1;
        let actual = assert_in_epsilon_as_result!(a, b, epsilon);
        let message = concat!(
            "assertion failed: `assert_in_epsilon!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_epsilon.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `10`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `30`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `1`,\n",
            "                 | a - b |: `20`,\n",
            "             ε * min(a, b): `10`,\n",
            " | a - b | ≤ ε * min(a, b): false"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a number is within epsilon of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ ε * min(a, b)
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
/// let b: i8 = 20;
/// let epsilon: i8 = 1;
/// assert_in_epsilon!(a, b, epsilon);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: i8 = 10;
/// let b: i8 = 30;
/// let epsilon: i8 = 1;
/// assert_in_epsilon!(a, b, epsilon);
/// # });
/// // assertion failed: `assert_in_epsilon!(a, b, epsilon)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_in_epsilon.html
/// //                    a label: `a`,
/// //                    a debug: `10`,
/// //                    b label: `b`,
/// //                    b debug: `30`,
/// //                    ε label: `epsilon`,
/// //                    ε debug: `1`,
/// //                  | a - b |: `20`,
/// //              ε * min(a, b): `10`,\n",
/// //  | a - b | ≤ ε * min(a, b): false"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_in_epsilon!(a, b, ε)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_epsilon.html\n",
/// #     "                   a label: `a`,\n",
/// #     "                   a debug: `10`,\n",
/// #     "                   b label: `b`,\n",
/// #     "                   b debug: `30`,\n",
/// #     "                   ε label: `epsilon`,\n",
/// #     "                   ε debug: `1`,\n",
/// #     "                 | a - b |: `20`,\n",
/// #     "             ε * min(a, b): `10`,\n",
/// #     " | a - b | ≤ ε * min(a, b): false"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// The macros `assert_in_delta` and `assert_in_epsilon` can test
/// approximations:
///
/// * For an approximation, the absolute error (i.e. delta) is the magnitude of
///   the difference between the exact value and the approximation.
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
/// * [`assert_in_epsilon`](macro@crate::assert_in_epsilon)
/// * [`assert_in_epsilon_as_result`](macro@crate::assert_in_epsilon_as_result)
/// * [`debug_assert_in_epsilon`](macro@crate::debug_assert_in_epsilon)
///
#[macro_export]
macro_rules! assert_in_epsilon {
    ($a:expr, $b:expr, $epsilon:expr $(,)?) => {
        match $crate::assert_in_epsilon_as_result!($a, $b, $epsilon) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $epsilon:expr, $($message:tt)+) => {
        match $crate::assert_in_epsilon_as_result!($a, $b, $epsilon) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_in_epsilon {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 20;
        let epsilon: i8 = 1;
        for _ in 0..1 {
            let actual = assert_in_epsilon!(a, b, epsilon);
            assert_eq!(actual, (10, 10));
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 30;
        let epsilon: i8 = 1;
        let result = panic::catch_unwind(|| {
            let _actual = assert_in_epsilon!(a, b, epsilon);
        });
        let message = concat!(
            "assertion failed: `assert_in_epsilon!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_epsilon.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `10`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `30`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `1`,\n",
            "                 | a - b |: `20`,\n",
            "             ε * min(a, b): `10`,\n",
            " | a - b | ≤ ε * min(a, b): false"
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

/// Assert a number is within epsilon of another.
///
/// Pseudocode:<br>
/// | a - b | ≤ ε * min(a, b)
///
/// This macro provides the same statements as [`assert_in_epsilon`](macro.assert_in_epsilon.html),
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
/// * [`assert_in_epsilon`](macro@crate::assert_in_epsilon)
/// * [`assert_in_epsilon`](macro@crate::assert_in_epsilon)
/// * [`debug_assert_in_epsilon`](macro@crate::debug_assert_in_epsilon)
///
#[macro_export]
macro_rules! debug_assert_in_epsilon {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_in_epsilon!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_in_epsilon {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 10;
        let b: i8 = 20;
        let epsilon: i8 = 1;
        for _ in 0..1 {
            let _actual = debug_assert_in_epsilon!(a, b, epsilon);
            // assert_eq!(actual, (10, 10));
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 10;
        let b: i8 = 30;
        let epsilon: i8 = 1;
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_in_epsilon!(a, b, epsilon);
        });
        let message = concat!(
            "assertion failed: `assert_in_epsilon!(a, b, ε)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_in_epsilon.html\n",
            "                   a label: `a`,\n",
            "                   a debug: `10`,\n",
            "                   b label: `b`,\n",
            "                   b debug: `30`,\n",
            "                   ε label: `epsilon`,\n",
            "                   ε debug: `1`,\n",
            "                 | a - b |: `20`,\n",
            "             ε * min(a, b): `10`,\n",
            " | a - b | ≤ ε * min(a, b): false"
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
