//! Assert a function Ok(…) is greater than another.
//!
//! Pseudocode:<br>
//! (a_function(a_param) ⇒ Ok(a) ⇒ a) > (b_function(b_param) ⇒ Ok(b) ⇒ b)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! fn f(i: i8) -> Result<String, String> {
//!     match i {
//!         0..=9 => Ok(format!("{}", i)),
//!         _ => Err(format!("{:?} is out of range", i)),
//!     }
//! }
//!
//! let a: i8 = 2;
//! let b: i8 = 1;
//! assert_fn_ok_gt!(f, a, f, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_fn_ok_gt`](macro@crate::assert_fn_ok_gt)
//! * [`assert_fn_ok_gt_as_result`](macro@crate::assert_fn_ok_gt_as_result)
//! * [`debug_assert_fn_ok_gt`](macro@crate::debug_assert_fn_ok_gt)

/// Assert a function Ok(…) is greater than another.
///
/// Pseudocode:<br>
/// (a_function(a_param) ⇒ Ok(a) ⇒ a) > (b_function(b_param) ⇒ Ok(b) ⇒ b)
///
/// * If true, return Result `Ok(a, b)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fn_ok_gt`](macro@crate::assert_fn_ok_gt)
/// * [`assert_fn_ok_gt_as_result`](macro@crate::assert_fn_ok_gt_as_result)
/// * [`debug_assert_fn_ok_gt`](macro@crate::debug_assert_fn_ok_gt)
///
#[macro_export]
macro_rules! assert_fn_ok_gt_as_result {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {
        match (&$a_function, &$a_param, &$b_function, &$b_param) {
            (_a_function, a_param, _b_function, b_param) => {
                match (
                    $a_function($a_param),
                    $b_function($b_param)
                ) {
                    (Ok(a), Ok(b)) => {
                        if a > b {
                            Ok((a, b))
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                                        " a_function label: `{}`,\n",
                                        "    a_param label: `{}`,\n",
                                        "    a_param debug: `{:?}`,\n",
                                        " b_function label: `{}`,\n",
                                        "    b_param label: `{}`,\n",
                                        "    b_param debug: `{:?}`,\n",
                                        "                a: `{:?}`,\n",
                                        "                b: `{:?}`"
                                    ),
                                    stringify!($a_function),
                                    stringify!($a_param),
                                    a_param,
                                    stringify!($b_function),
                                    stringify!($b_param),
                                    b_param,
                                    a,
                                    b
                                )
                            )
                        }
                    },
                    (a, b) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fn_err_gt!(a_function, a_param, b_function, b_param)`\n",
                                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_err_gt.html\n",
                                    " a_function label: `{}`,\n",
                                    "    a_param label: `{}`,\n",
                                    "    a_param debug: `{:?}`,\n",
                                    " b_function label: `{}`,\n",
                                    "    b_param label: `{}`,\n",
                                    "    b_param debug: `{:?}`,\n",
                                    "                a: `{:?}`,\n",
                                    "                b: `{:?}`"
                                ),
                                stringify!($a_function),
                                stringify!($a_param),
                                a_param,
                                stringify!($b_function),
                                stringify!($b_param),
                                b_param,
                                a,
                                b
                            )
                        )
                    }
                }
            }
        }
    };

    //// Arity 0

    ($a_function:path, $b_function:path) => {
        match (
            $a_function(),
            $b_function()
        ) {
            (Ok(a), Ok(b)) => {
                if a > b {
                    Ok((a, b))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                                " a_function label: `{}`,\n",
                                " b_function label: `{}`,\n",
                                "                a: `{:?}`,\n",
                                "                b: `{:?}`"
                            ),
                            stringify!($a_function),
                            stringify!($b_function),
                            a,
                            b
                        )
                    )
                }
            },
            (a, b) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_fn_err_gt!(a_function, b_function)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_err_gt.html\n",
                            " a_function label: `{}`,\n",
                            " b_function label: `{}`,\n",
                            "                a: `{:?}`,\n",
                            "                b: `{:?}`"
                        ),
                        stringify!($a_function),
                        stringify!($b_function),
                        a,
                        b
                    )
                )
            }
        }
    };

}

#[cfg(test)]
mod test_assert_fn_ok_gt_as_result {
    // use std::sync::Once;

    mod arity_1 {

        fn f(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        fn g(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let actual = assert_fn_ok_gt_as_result!(f, a, g, b);
                assert_eq!(actual.unwrap(), (2, 1));
            }
        }

        #[test]
        fn eq() {
            let a: i8 = 1;
            let b: i8 = 1;
            let actual = assert_fn_ok_gt_as_result!(f, a, g, b);
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `1`,\n",
                "                a: `1`,\n",
                "                b: `1`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }

        #[test]
        fn lt() {
            let a: i8 = 1;
            let b: i8 = 2;
            let actual = assert_fn_ok_gt_as_result!(f, a, g, b);
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `2`,\n",
                "                a: `1`,\n",
                "                b: `2`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod arity_0 {

        fn f() -> Result<i8, i8> {
            return Ok(1);
        }

        fn g() -> Result<i8, i8> {
            return Ok(2);
        }

        #[test]
        fn gt() {
            for _ in 0..1 {
                let actual = assert_fn_ok_gt_as_result!(g, f);
                assert_eq!(actual.unwrap(), (2, 1));
            }
        }

        #[test]
        fn eq() {
            let actual = assert_fn_ok_gt_as_result!(f, f);
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `f`,\n",
                "                a: `1`,\n",
                "                b: `1`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }

        #[test]
        fn lt() {
            let actual = assert_fn_ok_gt_as_result!(f, g);
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `g`,\n",
                "                a: `1`,\n",
                "                b: `2`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
}

/// Assert a function Ok(…) is greater than another.
///
/// Pseudocode:<br>
/// (a_function(a_param) ⇒ Ok(a) ⇒ a) > (b_function(b_param) ⇒ Ok(b) ⇒ b)
///
/// * If true, return `(a, b)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// fn f(i: i8) -> Result<String, String> {
///     match i {
///         0..=9 => Ok(format!("{}", i)),
///         _ => Err(format!("{:?} is out of range", i)),
///     }
/// }
///
/// # fn main() {
/// let a: i8 = 2;
/// let b: i8 = 1;
/// assert_fn_ok_gt!(f, a, f, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: i8 = 1;
/// let b: i8 = 2;
/// assert_fn_ok_gt!(f, a, f, b);
/// # });
/// // assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_fn_ok_gt.html
/// //  a_function label: `f`,
/// //     a_param label: `a`,
/// //     a_param debug: `1`,
/// //  b_function label: `f`,
/// //     b_param label: `b`,
/// //     b_param debug: `2`,
/// //                 a: `\"1\"`,
/// //                 b: `\"2\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
/// #     " a_function label: `f`,\n",
/// #     "    a_param label: `a`,\n",
/// #     "    a_param debug: `1`,\n",
/// #     " b_function label: `f`,\n",
/// #     "    b_param label: `b`,\n",
/// #     "    b_param debug: `2`,\n",
/// #     "                a: `\"1\"`,\n",
/// #     "                b: `\"2\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fn_ok_gt`](macro@crate::assert_fn_ok_gt)
/// * [`assert_fn_ok_gt_as_result`](macro@crate::assert_fn_ok_gt_as_result)
/// * [`debug_assert_fn_ok_gt`](macro@crate::debug_assert_fn_ok_gt)
///
#[macro_export]
macro_rules! assert_fn_ok_gt {

    //// Arity 1

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr $(,)?) => {
        match $crate::assert_fn_ok_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };

    ($a_function:path, $a_param:expr, $b_function:path, $b_param:expr, $($message:tt)+) => {
        match $crate::assert_fn_ok_gt_as_result!($a_function, $a_param, $b_function, $b_param) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };

    //// Arity 0

    ($a_function:path, $b_function:path) => {
        match $crate::assert_fn_ok_gt_as_result!($a_function, $b_function) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };

    ($a_function:path, $b_function:path, $($message:tt)+) => {
        match $crate::assert_fn_ok_gt_as_result!($a_function, $b_function) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_fn_ok_gt {
    use std::panic;

    mod arity_1 {
        use super::*;

        fn f(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        fn g(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let actual = assert_fn_ok_gt!(f, a, g, b);
                let expect = (2, 1);
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let result = panic::catch_unwind(|| {
                let a: i8 = 1;
                let b: i8 = 1;
                let _actual = assert_fn_ok_gt!(f, a, g, b);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `1`,\n",
                "                a: `1`,\n",
                "                b: `1`"
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
        fn lt() {
            let result = panic::catch_unwind(|| {
                let a: i8 = 1;
                let b: i8 = 2;
                let _actual = assert_fn_ok_gt!(f, a, g, b);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `2`,\n",
                "                a: `1`,\n",
                "                b: `2`"
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

    mod arity_0 {
        use super::*;

        fn f() -> Result<i8, i8> {
            return Ok(1);
        }

        fn g() -> Result<i8, i8> {
            return Ok(2);
        }

        #[test]
        fn gt() {
            for _ in 0..1 {
                let actual = assert_fn_ok_gt!(g, f);
                let expect = (2, 1);
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let result = panic::catch_unwind(|| {
                let _actual = assert_fn_ok_gt!(f, f);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `f`,\n",
                "                a: `1`,\n",
                "                b: `1`"
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
        fn lt() {
            let result = panic::catch_unwind(|| {
                let _actual = assert_fn_ok_gt!(f, g);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `g`,\n",
                "                a: `1`,\n",
                "                b: `2`"
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
}

/// Assert a function Ok(…) is greater than another.
///
/// Pseudocode:<br>
/// (a_function(a_param) ⇒ Ok(a) ⇒ a) > (b_function(b_param) ⇒ Ok(b) ⇒ b)
///
/// This macro provides the same statements as [`assert_fn_ok_gt`](macro.assert_fn_ok_gt.html),
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
/// * [`assert_fn_ok_gt`](macro@crate::assert_fn_ok_gt)
/// * [`assert_fn_ok_gt`](macro@crate::assert_fn_ok_gt)
/// * [`debug_assert_fn_ok_gt`](macro@crate::debug_assert_fn_ok_gt)
///
#[macro_export]
macro_rules! debug_assert_fn_ok_gt {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_fn_ok_gt!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_fn_ok_gt {
    use std::panic;

    mod arity_1 {
        use super::*;

        fn f(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        fn g(i: i8) -> Result<i8, i8> {
            return Ok(i);
        }

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let _actual = debug_assert_fn_ok_gt!(f, a, g, b);
                let _expect = (2, 1);
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let result = panic::catch_unwind(|| {
                let a: i8 = 1;
                let b: i8 = 1;
                let _actual = debug_assert_fn_ok_gt!(f, a, g, b);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `1`,\n",
                "                a: `1`,\n",
                "                b: `1`"
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
        fn lt() {
            let result = panic::catch_unwind(|| {
                let a: i8 = 1;
                let b: i8 = 2;
                let _actual = debug_assert_fn_ok_gt!(f, a, g, b);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, a_param, b_function, b_param)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                "    a_param label: `a`,\n",
                "    a_param debug: `1`,\n",
                " b_function label: `g`,\n",
                "    b_param label: `b`,\n",
                "    b_param debug: `2`,\n",
                "                a: `1`,\n",
                "                b: `2`"
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

    mod arity_0 {
        use super::*;

        fn f() -> Result<i8, i8> {
            return Ok(1);
        }

        fn g() -> Result<i8, i8> {
            return Ok(2);
        }

        #[test]
        fn gt() {
            for _ in 0..1 {
                let _actual = debug_assert_fn_ok_gt!(g, f);
                let _expect = (2, 1);
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_fn_ok_gt!(f, f);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `f`,\n",
                "                a: `1`,\n",
                "                b: `1`"
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
        fn lt() {
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_fn_ok_gt!(f, g);
            });
            let message = concat!(
                "assertion failed: `assert_fn_ok_gt!(a_function, b_function)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_fn_ok_gt.html\n",
                " a_function label: `f`,\n",
                " b_function label: `g`,\n",
                "                a: `1`,\n",
                "                b: `2`"
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
}
