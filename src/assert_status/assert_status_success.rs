//! Assert a status is a success.
//!
//! Pseudocode:<br>
//! a ⇒ status ⇒ success = true
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::process::Command;
//!
//! let mut a = Command::new("bin/exit-with-arg"); a.arg("0");
//! assert_status_success!(a);
//! ```
//!
//! # Module macros
//!
//! * [`assert_status_success`](macro@crate::assert_status_success)
//! * [`assert_status_success_as_result`](macro@crate::assert_status_success_as_result)
//! * [`debug_assert_status_success`](macro@crate::debug_assert_status_success)

/// Assert a status is a success.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ success = true
///
/// * If true, return Result `Ok(a ⇒ status ⇒ code ⇒ value)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_status_success`](macro@crate::assert_status_success)
/// * [`assert_status_success_as_result`](macro@crate::assert_status_success_as_result)
/// * [`debug_assert_status_success`](macro@crate::debug_assert_status_success)
///
#[macro_export]
macro_rules! assert_status_success_as_result {
    ($a:expr $(,)?) => {
        match ($a.status()) {
            Ok(a1) => {
                if a1.success() {
                    Ok(true)
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_status_success!(a)`\n",
                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`",
                        ),
                        stringify!($a),
                        $a,
                    ))
                }
            }
            a_status => Err(format!(
                concat!(
                    "assertion failed: `assert_status_success!(a)`\n",
                    "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
                    "  a label: `{}`,\n",
                    "  a debug: `{:?}`,\n",
                    " a status: `{:?}`",
                ),
                stringify!($a),
                $a,
                a_status
            )),
        }
    };
}

#[cfg(test)]
mod test_assert_status_success_as_result {
    use std::process::Command;
    use std::sync::Once;

    #[test]
    fn success() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("0");
        for _ in 0..1 {
            let actual = assert_status_success_as_result!(a);
            assert_eq!(actual.unwrap(), true);
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> Command {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("0");
            a
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_status_success_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("1");
        let actual = assert_status_success_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_status_success!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/exit-with-arg\" \"1\"`",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a status is a success.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ success = true
///
/// * If true, return `a ⇒ status ⇒ code ⇒ value``.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// use std::process::Command;
/// # use std::panic;
///
/// # fn main() {
/// let mut a = Command::new("bin/exit-with-arg"); a.arg("0");
/// assert_status_success!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let mut a = Command::new("bin/exit-with-arg"); a.arg("1");
/// assert_status_success!(a);
/// # });
/// // assertion failed: `assert_status_success!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_status_success.html
/// //  a label: `a`,
/// //  a debug: `\"bin/exit-with-arg\" \"1\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_status_success!(a)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"bin/exit-with-arg\" \"1\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_status_success`](macro@crate::assert_status_success)
/// * [`assert_status_success_as_result`](macro@crate::assert_status_success_as_result)
/// * [`debug_assert_status_success`](macro@crate::debug_assert_status_success)
///
#[macro_export]
macro_rules! assert_status_success {
    ($a:expr $(,)?) => {
        match $crate::assert_status_success_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_status_success_as_result!($a) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_status_success {
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("0");
        for _ in 0..1 {
            let actual = assert_status_success!(a);
            assert_eq!(actual, true);
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("1");
            let _actual = assert_status_success!(a);
        });
        let message = concat!(
            "assertion failed: `assert_status_success!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/exit-with-arg\" \"1\"`",
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

/// Assert a status is a success.
///
/// Pseudocode:<br>
/// a ⇒ status ⇒ success = true
///
/// This macro provides the same statements as [`assert_status_success`](macro.assert_status_success.html),
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
/// * [`assert_status_success`](macro@crate::assert_status_success)
/// * [`assert_status_success`](macro@crate::assert_status_success)
/// * [`debug_assert_status_success`](macro@crate::debug_assert_status_success)
///
#[macro_export]
macro_rules! debug_assert_status_success {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_status_success!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_status_success {
    use std::panic;
    use std::process::Command;

    #[test]
    fn success() {
        let mut a = Command::new("bin/exit-with-arg");
        a.arg("0");
        for _ in 0..1 {
            let _actual = debug_assert_status_success!(a);
            // assert_eq!(actual, true);
        }
    }

    #[test]
    fn failure() {
        let result = panic::catch_unwind(|| {
            let mut a = Command::new("bin/exit-with-arg");
            a.arg("1");
            let _actual = debug_assert_status_success!(a);
        });
        let message = concat!(
            "assertion failed: `assert_status_success!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_status_success.html\n",
            " a label: `a`,\n",
            " a debug: `\"bin/exit-with-arg\" \"1\"`",
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
