//! Assert a floating point 32-bit number is equal to another within f32::EPSILON.
//!
//! Pseudocode:<br>
//! a = b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a: f32 = 1.0 / 3.0;
//! let b: f32 = 0.3333333;
//! assert_eq_f32!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_eq_f32`](macro@crate::assert_eq_f32)
//! * [`assert_eq_f32_as_result`](macro@crate::assert_eq_f32_as_result)
//! * [`debug_assert_eq_f32`](macro@crate::debug_assert_eq_f32)

/// Assert a floating point 32-bit number is equal to another within f32::EPSILON.
///
/// Pseudocode:<br>
/// a = b
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
/// * [`assert_eq_f32`](macro@crate::assert_eq_f32)
/// * [`assert_eq_f32_as_result`](macro@crate::assert_eq_f32_as_result)
/// * [`debug_assert_eq_f32`](macro@crate::debug_assert_eq_f32)
///
#[macro_export]
macro_rules! assert_eq_f32_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match (&$a, &$b) {
            (a, b) => {
                if (a >= b && a - b < f32::EPSILON) || (a <= b && b - a < f32::EPSILON) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_eq_f32!(a, b)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_eq_f32.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`,\n",
                            "    diff: `{}`,\n",
                            "       ε: `{}`",
                        ),
                        stringify!($a),
                        a,
                        stringify!($b),
                        b,
                        a - b,
                        f32::EPSILON,
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
pub const EQ: f32 = 1.0 / 3.0;
#[cfg(test)]
pub const EQ_LT: f32 = 0.3333333;
#[cfg(test)]
pub const EQ_GT: f32 = 0.3333334;
#[cfg(test)]
pub const LT: f32 = 0.3333331;
#[cfg(test)]
pub const GT: f32 = 0.3333336;

#[cfg(test)]
mod test_assert_eq_f32_as_result {
    use super::*;
    use std::sync::Once;

    #[test]
    fn eq() {
        let a: f32 = EQ;
        let b: f32 = EQ;
        for _ in 0..1 {
            let actual = assert_eq_f32_as_result!(a, b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> f32 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            EQ
        }

        static B: Once = Once::new();
        fn b() -> f32 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            EQ
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_eq_f32_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq_lt() {
        let a: f32 = EQ;
        let b: f32 = EQ_GT;
        for _ in 0..1 {
            let actual = assert_eq_f32_as_result!(a, b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn eq_lt_once() {
        static A: Once = Once::new();
        fn a() -> f32 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            EQ
        }

        static B: Once = Once::new();
        fn b() -> f32 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            EQ_GT
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_eq_f32_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq_gt() {
        let a: f32 = EQ;
        let b: f32 = EQ_LT;
        for _ in 0..1 {
            let actual = assert_eq_f32_as_result!(a, b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn eq_gt_once() {
        static A: Once = Once::new();
        fn a() -> f32 {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            EQ_LT
        }

        static B: Once = Once::new();
        fn b() -> f32 {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            EQ_LT
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_eq_f32_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn lt() {
        let a: f32 = EQ;
        let b: f32 = GT;
        let actual = assert_eq_f32_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_eq_f32!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_eq_f32.html\n",
            " a label: `a`,\n",
            " a debug: `0.33333334`,\n",
            " b label: `b`,\n",
            " b debug: `0.3333336`,\n",
            "    diff: `-0.0000002682209`,\n",
            "       ε: `0.00000011920929`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn gt() {
        let a: f32 = EQ;
        let b: f32 = LT;
        let actual = assert_eq_f32_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_eq_f32!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_eq_f32.html\n",
            " a label: `a`,\n",
            " a debug: `0.33333334`,\n",
            " b label: `b`,\n",
            " b debug: `0.3333331`,\n",
            "    diff: `0.00000023841858`,\n",
            "       ε: `0.00000011920929`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a floating point 32-bit number is equal to another within f32::EPSILON.
///
/// Pseudocode:<br>
/// a = b
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
/// let a: f32 = 1.0 / 3.0;
/// let b: f32 = 0.3333333;
/// assert_eq_f32!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a: f32 = 1.0 / 3.0;
/// let b: f32 = 0.3333336;
/// assert_eq_f32!(a, b);
/// # });
/// // assertion failed: `assert_eq_f32!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_eq_f32.html
/// //  a label: `a`,
/// //  a debug: `0.33333334`,
/// //  b label: `b`,
/// //  b debug: `0.3333336`,`
/// //     diff: `-0.0000002682209`,
/// //        ε: `0.00000011920929`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_eq_f32!(a, b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_eq_f32.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `0.33333334`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `0.3333336`,\n",
/// #     "    diff: `-0.0000002682209`,\n",
/// #     "       ε: `0.00000011920929`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_eq_f32`](macro@crate::assert_eq_f32)
/// * [`assert_eq_f32_as_result`](macro@crate::assert_eq_f32_as_result)
/// * [`debug_assert_eq_f32`](macro@crate::debug_assert_eq_f32)
///
#[macro_export]
macro_rules! assert_eq_f32 {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_eq_f32_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_eq_f32_as_result!($a, $b) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_eq_f32 {
    use super::*;
    use std::panic;

    #[test]
    fn eq() {
        let a: f32 = EQ;
        let b: f32 = EQ;
        for _ in 0..1 {
            let actual = assert_eq_f32!(a, b);
            assert_eq!(actual, ());
        }
    }
}

/// Assert a floating point 32-bit number is equal to another within f32::EPSILON.
///
/// Pseudocode:<br>
/// a = b
///
///
/// This macro provides the same statements as [`assert_eq_f32`](macro.assert_eq_f32.html),
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
/// * [`assert_eq_f32`](macro@crate::assert_eq_f32)
/// * [`assert_eq_f32`](macro@crate::assert_eq_f32)
/// * [`debug_assert_eq_f32`](macro@crate::debug_assert_eq_f32)
///
#[macro_export]
macro_rules! debug_assert_eq_f32 {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_eq_f32!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_eq_f32 {
    use super::*;
    use std::panic;

    #[test]
    fn eq() {
        let a: f32 = EQ;
        let b: f32 = EQ;
        for _ in 0..1 {
            let _actual = debug_assert_eq_f32!(a, b);
            // assert_eq!(actual, ());
        }
    }
}
