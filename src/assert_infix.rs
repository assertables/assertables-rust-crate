//! Assert a infix operator, such as assert_infix!(a == b).
//!
//! Pseudocode:<br>
//! a infix b
//!
//! Compare values via infix value operator:
//!
//! * `assert_infix!(a == b)` ≈ a == b
//!
//! * `assert_infix!(a != b)` ≈ a ≠ b
//!
//! * `assert_infix!(a < b)` ≈ a < b
//!
//! * `assert_infix!(a <= b)` ≈ a ≤ b
//!
//! * `assert_infix!(a > b)` ≈ a > b
//!
//! * `assert_infix!(a >= b)` ≈ a ≥ b
//!
//! Relate values via infix logical operator:
//!
//! * `assert_infix!(a & b)` ≈ a ∧ b ≈ a AND b
//!
//! * `assert_infix!(a | b)` ≈ a ∨ b ≈ a OR b
//!
//! * `assert_infix!(a ^ b)` ≈ a ⊻ b ≈ a XOR b
//!
//! * `assert_infix!(a && b)` ≈ a …∧ b ≈ a lazy AND b
//!
//! * `assert_infix!(a || b)` ≈ a …∨ b ≈ a lazy OR b
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = 1;
//! let b = 1;
//! assert_infix!(a == b);
//! ```
//!
//! # Infix operators
//!
//! For values:
//!
//! * `==`  equal
//! * `!=`  not equal
//! * `<`	less than
//! * `<=`	less than or equal to
//! * `>`	greater than
//! * `>=`	greater than or equal to
//!
//! For booleans:
//!
//! * `^`	logical XOR
//! * `!`	logical NOT
//! * `&`	logical AND
//! * `|`	logical OR
//! * `&&`	logical lazy AND
//! * `||`	logical lazy OR
//!
//! # Module macros
//!
//! * [`assert_infix`](macro@crate::assert_infix)
//! * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
//! * [`debug_assert_infix`](macro@crate::debug_assert_infix)

/// Assert a infix operator, such as assert_infix!(a == b).
///
/// Pseudocode:<br>
/// a infix b
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
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! assert_infix_as_result {
    ($a:tt $infix:tt $b:tt) => {
        if $a $infix $b {
            Ok(())
        } else {
            Err(
                format!(
                    concat!(
                        "assertion failed: `assert_infix!(a {} b)`\n",
                        "https://docs.rs/assertables/10.0.0/assertables/macro.assert_infix.html\n",
                        " a label: `{}`,\n",
                        " a debug: `{:?}`,\n",
                        " b label: `{}`,\n",
                        " b debug: `{:?}`",
                    ),
                    stringify!($infix),
                    stringify!($a),
                    $a,
                    stringify!($b),
                    $b,
                )
            )
        }
    };
}

#[cfg(test)]
mod test_assert_infix_as_result {
    // use std::sync::Once;

    #[test]
    fn success() {
        let a: i8 = 1;
        let b: i8 = 1;
        for _ in 0..1 {
            let actual = assert_infix_as_result!(a == b);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 1;
        let b: i8 = 2;
        let actual = assert_infix_as_result!(a == b);
        let message = concat!(
            "assertion failed: `assert_infix!(a == b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_infix.html\n",
            " a label: `a`,\n",
            " a debug: `1`,\n",
            " b label: `b`,\n",
            " b debug: `2`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a infix operator, such as assert_infix!(a == b).
///
/// Pseudocode:<br>
/// a infix b
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
/// let a = 1;
/// let b = 1;
/// assert_infix!(a == b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = 1;
/// let b = 2;
/// assert_infix!(a == b);
/// # });
/// // assertion failed: `assert_infix!(a == b)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_infix.html
/// //  a label: `a`,
/// //  a debug: `1`,
/// //  b label: `b`,
/// //  b debug: `2`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_infix!(a == b)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_infix.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `1`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `2`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Infix operators
///
/// For values:
///
/// * `==`  equal
/// * `!=`  not equal
/// * `<`	less than
/// * `<=`	less than or equal to
/// * `>`	greater than
/// * `>=`	greater than or equal to
///
/// For booleans:
///
/// * `^`	logical XOR
/// * `!`	logical NOT
/// * `&`	logical AND
/// * `|`	logical OR
/// * `&&`	logical lazy AND
/// * `||`	logical lazy OR
///
/// # Module macros
///
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix_as_result`](macro@crate::assert_infix_as_result)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! assert_infix {
    ($a:tt $infix:tt $b:tt) => {
        match $crate::assert_infix_as_result!($a $infix $b) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($a:tt $infix:tt $b:tt, $($message:tt)+) => {
        match $crate::assert_infix_as_result!($a $infix $b) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_infix {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 1;
        let b: i8 = 1;
        for _ in 0..1 {
            let actual = assert_infix!(a == b);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 1;
        let b: i8 = 2;
        let result = panic::catch_unwind(|| {
            let _actual = assert_infix!(a == b);
        });
        let message = concat!(
            "assertion failed: `assert_infix!(a == b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_infix.html\n",
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

/// Assert a infix operator, such as assert_infix!(a == b).
///
/// Pseudocode:<br>
/// a infix b
///
/// This macro provides the same statements as [`assert_infix`](macro.assert_infix.html),
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
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`assert_infix`](macro@crate::assert_infix)
/// * [`debug_assert_infix`](macro@crate::debug_assert_infix)
///
#[macro_export]
macro_rules! debug_assert_infix {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_infix!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_infix {
    use std::panic;

    #[test]
    fn success() {
        let a: i8 = 1;
        let b: i8 = 1;
        for _ in 0..1 {
            let _actual = debug_assert_infix!(a == b);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a: i8 = 1;
        let b: i8 = 2;
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_infix!(a == b);
        });
        let message = concat!(
            "assertion failed: `assert_infix!(a == b)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_infix.html\n",
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
