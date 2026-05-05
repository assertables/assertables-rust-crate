//! Assert an expression is greater than another.
//!
//! Pseudocode:<br>
//! a > b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 2;
//! let b = 1;
//! assert_gt!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_gt`](macro@crate::assert_gt)
//! * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
//! * [`debug_assert_gt`](macro@crate::debug_assert_gt)

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
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
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match (&$a, &$b) {
            (a, b) => {
                if a > b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_gt!(a, b)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_gt_as_result {
    use std::sync::Once;

    mod integer {
        use super::*;

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let actual = assert_gt_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn gt_once() {
            static A: Once = Once::new();
            fn a() -> i8 {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                2
            }

            static B: Once = Once::new();
            fn b() -> i8 {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                1
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_gt_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn eq() {
            let a: i8 = 1;
            let b: i8 = 1;
            let actual = assert_gt_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }

        #[test]
        fn lt() {
            let a: i8 = 1;
            let b: i8 = 2;
            let actual = assert_gt_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
    mod string {
        use super::*;

        #[test]
        fn gt() {
            let a: String = String::from("2");
            let b: String = String::from("1");
            for _ in 0..1 {
                let actual = assert_gt_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn gt_once() {
            static A: Once = Once::new();
            fn a() -> String {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                String::from("2")
            }

            static B: Once = Once::new();
            fn b() -> String {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                String::from("1")
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_gt_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn eq() {
            let a: String = String::from("1");
            let b: String = String::from("1");
            let actual = assert_gt_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"1\"`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }

        #[test]
        fn lt() {
            let a: String = String::from("1");
            let b: String = String::from("2");
            let actual = assert_gt_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"2\"`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
}

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
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
/// let a = 2;
/// let b = 1;
/// assert_gt!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 1;
/// let b = 2;
/// assert_gt!(a, b);
/// # });
/// // assertion failed: `assert_gt!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_gt.html
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_gt!(a, b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt_as_result`](macro@crate::assert_gt_as_result)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_gt_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_gt {
    use std::panic;

    mod integer {
        use super::*;

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let actual = assert_gt!(a, b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let a: i8 = 1;
            let b: i8 = 1;
            let result = panic::catch_unwind(|| {
                let _actual = assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
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
            let a: i8 = 1;
            let b: i8 = 2;
            let result = panic::catch_unwind(|| {
                let _actual = assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`",
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

    mod string {
        use super::*;

        #[test]
        fn gt() {
            let a: String = String::from("2");
            let b: String = String::from("1");
            for _ in 0..1 {
                let actual = assert_gt!(a, b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let a: String = String::from("1");
            let b: String = String::from("1");
            let result = panic::catch_unwind(|| {
                let _actual = assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"1\"`",
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
            let a: String = String::from("1");
            let b: String = String::from("2");
            let result = panic::catch_unwind(|| {
                let _actual = assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"2\"`",
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

/// Assert an expression is greater than another.
///
/// Pseudocode:<br>
/// a > b
///
/// This macro provides the same statements as [`assert_gt`](macro.assert_gt.html),
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
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`assert_gt`](macro@crate::assert_gt)
/// * [`debug_assert_gt`](macro@crate::debug_assert_gt)
///
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_gt!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_gt {
    use std::panic;

    mod integer {
        use super::*;

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let _actual = debug_assert_gt!(a, b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let a: i8 = 1;
            let b: i8 = 1;
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
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
            let a: i8 = 1;
            let b: i8 = 2;
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `2`",
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

    mod string {
        use super::*;

        #[test]
        fn gt() {
            let a: String = String::from("2");
            let b: String = String::from("1");
            for _ in 0..1 {
                let _actual = debug_assert_gt!(a, b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn eq() {
            let a: String = String::from("1");
            let b: String = String::from("1");
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"1\"`",
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
            let a: String = String::from("1");
            let b: String = String::from("2");
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_gt!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_gt!(a, b)`\n",
                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"2\"`",
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
