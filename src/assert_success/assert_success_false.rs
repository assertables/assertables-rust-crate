//! Assert failure.
//!
//! Pseudocode:<br>
//! a.success() = false
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! #[derive(Debug)]
//! struct A;
//! impl A { fn success(&self) -> bool { false }}
//! let a = A{};
//! assert_success_false!(a);
//! ```
//!
//! # Module macros
//!
//! * [`assert_success_false`](macro@crate::assert_success_false)
//! * [`assert_success_false_as_result`](macro@crate::assert_success_false_as_result)
//! * [`debug_assert_success_false`](macro@crate::debug_assert_success_false)

/// Assert a failure method is true.
///
/// Pseudocode:<br>
/// a.success() = false
///
/// * If true, return Result `Ok(true)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_success_false`](macro@crate::assert_success_false)
/// * [`assert_success_false_as_result`](macro@crate::assert_success_false_as_result)
/// * [`debug_assert_success_false`](macro@crate::debug_assert_success_false)
///
#[macro_export]
macro_rules! assert_success_false_as_result {
    ($a:expr $(,)?) => {
        if !$a.success() {
            Ok(true)
        } else {
            Err(format!(
                concat!(
                    "assertion failed: `assert_success_false!(a)`\n",
                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_success_false.html\n",
                    " a label: `{}`,\n",
                    " a debug: `{:?}`",
                ),
                stringify!($a),
                $a,
            ))
        }
    };
}

#[cfg(test)]
mod test_assert_success_false_as_result {
    // use std::sync::Once;

    #[test]
    fn success() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                false
            }
        }
        let a = A {};
        for _ in 0..1 {
            let actual = assert_success_false_as_result!(a);
            assert_eq!(actual.unwrap(), true);
        }
    }

    #[test]
    fn failure() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                true
            }
        }
        let a = A {};
        let actual = assert_success_false_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_success_false!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_success_false.html\n",
            " a label: `a`,\n",
            " a debug: `A`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a failure method is true.
///
/// Pseudocode:<br>
/// a.success() = false
///
/// * If true, return `true`.
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
/// #[derive(Debug)]
/// struct A;
/// impl A { fn success(&self) -> bool { false }}
/// let a = A{};
/// assert_success_false!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// #[derive(Debug)]
/// struct A;
/// impl A { fn success(&self) -> bool { true }}
/// let a = A{};
/// assert_success_false!(a);
/// # });
/// // assertion failed: `assert_success_false!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_success_false.html
/// //  a label: `a`,
/// //  a debug: `A`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_success_false!(a)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_success_false.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `A`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_success_false`](macro@crate::assert_success_false)
/// * [`assert_success_false_as_result`](macro@crate::assert_success_false_as_result)
/// * [`debug_assert_success_false`](macro@crate::debug_assert_success_false)
///
#[macro_export]
macro_rules! assert_success_false {
    ($a:expr $(,)?) => {
        match $crate::assert_success_false_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_success_false_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_success_false {
    use std::panic;

    #[test]
    fn success() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                false
            }
        }
        let a = A {};
        for _ in 0..1 {
            let actual = assert_success_false!(a);
            assert_eq!(actual, true);
        }
    }

    #[test]
    fn failure() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                true
            }
        }
        let a = A {};
        let result = panic::catch_unwind(|| {
            let _actual = assert_success_false!(a);
        });
        let message = concat!(
            "assertion failed: `assert_success_false!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_success_false.html\n",
            " a label: `a`,\n",
            " a debug: `A`",
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

/// Assert a failure method is true.
///
/// Pseudocode:<br>
/// a.success() = false
///
/// This macro provides the same statements as [`assert_success_false`](macro.assert_success_false.html),
/// except this macro's statements are only enabled in non-optimized
/// builds by default. An optimized build will not execute this macro's
/// statements unless `-C debug-assertions` is passed to the compiler.
///
/// This macro is useful for checks that are too expensive to be present
/// in a release build but may be helpful during development.
///
/// The result of expanding this macro is always type checked.
///
/// An unchecked assertion allows a "bin/exit-with-arg" in an inconsistent state to
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
/// * [`assert_success_false`](macro@crate::assert_success_false)
/// * [`assert_success_false`](macro@crate::assert_success_false)
/// * [`debug_assert_success_false`](macro@crate::debug_assert_success_false)
///
#[macro_export]
macro_rules! debug_assert_success_false {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_success_false!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_success_false {
    use std::panic;

    #[test]
    fn success() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                false
            }
        }
        let a = A {};
        for _ in 0..1 {
            let _actual = debug_assert_success_false!(a);
            // assert_eq!(actual, true);
        }
    }

    #[test]
    fn failure() {
        #[derive(Debug)]
        struct A;
        impl A {
            fn success(&self) -> bool {
                true
            }
        }
        let a = A {};
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_success_false!(a);
        });
        let message = concat!(
            "assertion failed: `assert_success_false!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_success_false.html\n",
            " a label: `a`,\n",
            " a debug: `A`",
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
