//! Assert a length is equal to another.
//!
//! Pseudocode:<br>
//! a.len() = b.len()
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "x";
//! let b = "x";
//! assert_len_eq!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_len_eq`](macro@crate::assert_len_eq)
//! * [`assert_len_eq_as_result`](macro@crate::assert_len_eq_as_result)
//! * [`debug_assert_len_eq`](macro@crate::debug_assert_len_eq)

/// Assert a length is equal to another.
///
/// Pseudocode:<br>
/// a.len() = b.len()
///
/// * If true, return Result `Ok((a.len(), b.len()))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_len_eq`](macro@crate::assert_len_eq)
/// * [`assert_len_eq_as_result`](macro@crate::assert_len_eq_as_result)
/// * [`debug_assert_len_eq`](macro@crate::debug_assert_len_eq)
///
#[macro_export]
macro_rules! assert_len_eq_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            (a, b) => {
                let a_len = a.len();
                let b_len = b.len();
                if a_len == b_len {
                    Ok((a_len, b_len))
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_len_eq!(a, b)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " a.len(): `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`\n",
                            " b.len(): `{:?}`",
                        ),
                        stringify!($a),
                        a,
                        a_len,
                        stringify!($b),
                        b,
                        b_len
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_len_eq_as_result {
    use std::sync::Once;

    #[test]
    fn eq() {
        let a = "x";
        let b = "x";
        for _ in 0..1 {
            let actual = assert_len_eq_as_result!(a, b);
            assert_eq!(actual.unwrap(), (1, 1));
        }
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "x"
        }

        static B: Once = Once::new();
        fn b() -> &'static str {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "x"
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_len_eq_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn lt() {
        let a = "x";
        let b = "xx";
        let actual = assert_len_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"x\"`,\n",
            " a.len(): `1`,\n",
            " b label: `b`,\n",
            " b debug: `\"xx\"`\n",
            " b.len(): `2`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }

    #[test]
    fn gt() {
        let a = "xx";
        let b = "x";
        let actual = assert_len_eq_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"xx\"`,\n",
            " a.len(): `2`,\n",
            " b label: `b`,\n",
            " b debug: `\"x\"`\n",
            " b.len(): `1`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a length is equal to another.
///
/// Pseudocode:<br>
/// a.len() = b.len()
///
/// * If true, return `(a.len(), b.len())`.
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
/// let a = "x";
/// let b = "x";
/// assert_len_eq!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "x";
/// let b = "xx";
/// assert_len_eq!(a, b);
/// # });
/// // assertion failed: `assert_len_eq!(a, b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_len_eq.html
/// //  a label: `a`,
/// //  a debug: `\"x\"`,
/// //  a.len(): `1`",
/// //  b label: `b`,
/// //  b debug: `\"xx\"`,
/// //  b.len(): `2`"
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_len_eq!(a, b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"x\"`,\n",
/// #     " a.len(): `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `\"xx\"`\n",
/// #     " b.len(): `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_len_eq`](macro@crate::assert_len_eq)
/// * [`assert_len_eq_as_result`](macro@crate::assert_len_eq_as_result)
/// * [`debug_assert_len_eq`](macro@crate::debug_assert_len_eq)
///
#[macro_export]
macro_rules! assert_len_eq {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_len_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_len_eq_as_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_len_eq {
    use std::panic;

    #[test]
    fn eq() {
        let a = "x";
        let b = "x";
        for _ in 0..1 {
            let actual = assert_len_eq!(a, b);
            assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn lt() {
        let a = "x";
        let b = "xx";
        let result = panic::catch_unwind(|| {
            let _actual = assert_len_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"x\"`,\n",
            " a.len(): `1`,\n",
            " b label: `b`,\n",
            " b debug: `\"xx\"`\n",
            " b.len(): `2`"
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
    fn gt() {
        let a = "xx";
        let b = "x";
        let result = panic::catch_unwind(|| {
            let _actual = assert_len_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"xx\"`,\n",
            " a.len(): `2`,\n",
            " b label: `b`,\n",
            " b debug: `\"x\"`\n",
            " b.len(): `1`"
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

/// Assert a length is equal to another.
///
/// Pseudocode:<br>
/// a.len() = b.len()
///
/// This macro provides the same statements as [`assert_len_eq`](macro.assert_len_eq.html),
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
/// * [`assert_len_eq`](macro@crate::assert_len_eq)
/// * [`assert_len_eq`](macro@crate::assert_len_eq)
/// * [`debug_assert_len_eq`](macro@crate::debug_assert_len_eq)
///
#[macro_export]
macro_rules! debug_assert_len_eq {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_len_eq!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_len_eq {
    use std::panic;

    #[test]
    fn eq() {
        let a = "x";
        let b = "x";
        for _ in 0..1 {
            let _actual = debug_assert_len_eq!(a, b);
            // assert_eq!(actual, (1, 1));
        }
    }

    #[test]
    fn lt() {
        let a = "x";
        let b = "xx";
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_len_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"x\"`,\n",
            " a.len(): `1`,\n",
            " b label: `b`,\n",
            " b debug: `\"xx\"`\n",
            " b.len(): `2`"
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
    fn gt() {
        let a = "xx";
        let b = "x";
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_len_eq!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_len_eq!(a, b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_len_eq.html\n",
            " a label: `a`,\n",
            " a debug: `\"xx\"`,\n",
            " a.len(): `2`,\n",
            " b label: `b`,\n",
            " b debug: `\"x\"`\n",
            " b.len(): `1`"
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
