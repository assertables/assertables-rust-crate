//! Assert a ::std::io::Read read_to_string() value is greater than or equal to another.
//!
//! Pseudocode:<br>
//! (a_reader.read_to_string(a_string) ⇒ a_string) ≥ (b_reader.read_to_string(b_string) ⇒ b_string)
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::io::Read;
//!
//! let a = "alfa".as_bytes();
//! let b = "aa".as_bytes();
//! assert_io_read_to_string_ge!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_io_read_to_string_ge`](macro@crate::assert_io_read_to_string_ge)
//! * [`assert_io_read_to_string_ge_as_result`](macro@crate::assert_io_read_to_string_ge_as_result)
//! * [`debug_assert_io_read_to_string_ge`](macro@crate::debug_assert_io_read_to_string_ge)

/// Assert a ::std::io::Read read_to_string() value is greater than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≥ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return Result `Ok((a_string, b_string))`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_io_read_to_string_ge`](macro@crate::assert_io_read_to_string_ge)
/// * [`assert_io_read_to_string_ge_as_result`](macro@crate::assert_io_read_to_string_ge_as_result)
/// * [`debug_assert_io_read_to_string_ge`](macro@crate::debug_assert_io_read_to_string_ge)
///
#[macro_export]
macro_rules! assert_io_read_to_string_ge_as_result {
    ($a_reader:expr, $b_reader:expr $(,)?) => {
        match(
            std::io::read_to_string($a_reader),
            std::io::read_to_string($b_reader)
        ) {
            (Ok(a_string), Ok(b_string)) => {
                if a_string >= b_string {
                    Ok((a_string, b_string))
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
                                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
                                " a label: `{}`,\n",
                                " a debug: `{:?}`,\n",
                                " b label: `{}`,\n",
                                " b debug: `{:?}`,\n",
                                "       a: `{:?}`,\n",
                                "       b: `{:?}`"
                            ),
                            stringify!($a_reader),
                            $a_reader,
                            stringify!($b_reader),
                            $b_reader,
                            a_string,
                            b_string
                        )
                    )
                }
            },
            (a_result, b_result) => {
                Err(
                    format!(
                        concat!(
                            "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
                            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
                            "  a label: `{}`,\n",
                            "  a debug: `{:?}`,\n",
                            "  b label: `{}`,\n",
                            "  b debug: `{:?},\n`",
                            " a result: `{:?},\n`",
                            " b result: `{:?}`",
                        ),
                        stringify!($a_reader),
                        $a_reader,
                        stringify!($b_reader),
                        $b_reader,
                        a_result,
                        b_result,
                    )
                )
            }
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_ge_as_result {
    #[allow(unused_imports)]
    use std::io::Read;
    use std::sync::Once;

    #[test]
    fn gt() {
        let a = "alfa".as_bytes();
        let b = "aa".as_bytes();
        let actual = assert_io_read_to_string_ge_as_result!(a, b);
        assert_eq!(actual.unwrap(), (String::from("alfa"), String::from("aa")));
    }

    #[test]
    fn gt_once() {
        static A: Once = Once::new();
        fn a() -> &'static [u8] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        static B: Once = Once::new();
        fn b() -> &'static [u8] {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "aa".as_bytes()
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_ge_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn eq() {
        let a = "alfa".as_bytes();
        let b = "alfa".as_bytes();
        let actual = assert_io_read_to_string_ge_as_result!(a, b);
        assert_eq!(
            actual.unwrap(),
            (String::from("alfa"), String::from("alfa"))
        );
    }

    #[test]
    fn eq_once() {
        static A: Once = Once::new();
        fn a() -> &'static [u8] {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        static B: Once = Once::new();
        fn b() -> &'static [u8] {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            "alfa".as_bytes()
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_io_read_to_string_ge_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn lt() {
        let a = "alfa".as_bytes();
        let b = "zz".as_bytes();
        let actual = assert_io_read_to_string_ge_as_result!(a, b);
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
            " a label: `a`,\n",
            " a debug: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `[122, 122]`,\n",
            "       a: `\"alfa\"`,\n",
            "       b: `\"zz\"`"
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::io::Read read_to_string() value is greater than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≥ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// * If true, return `(a_string, b_string)`.
///
/// * Otherwise, call [`panic!`] with a message and the values of the
///   expressions with their debug representations.
///
/// # Examples
///
/// ```rust
/// use assertables::*;
/// # use std::panic;
/// use std::io::Read;
///
/// # fn main() {
/// let a = "alfa".as_bytes();
/// let b = "aa".as_bytes();
/// assert_io_read_to_string_ge!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "alfa".as_bytes();
/// let b = "zz".as_bytes();
/// assert_io_read_to_string_ge!(a, b);
/// # });
/// // assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_io_read_to_string_ge.html
/// //  a label: `a`,
/// //  a debug: `[97, 108, 102, 97]`,
/// //  b label: `b`,
/// //  b debug: `[122, 122]`,
/// //        a: `\"alfa\"`,
/// //        b: `\"zz\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `[97, 108, 102, 97]`,\n",
/// #     " b label: `b`,\n",
/// #     " b debug: `[122, 122]`,\n",
/// #     "       a: `\"alfa\"`,\n",
/// #     "       b: `\"zz\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_io_read_to_string_ge`](macro@crate::assert_io_read_to_string_ge)
/// * [`assert_io_read_to_string_ge_as_result`](macro@crate::assert_io_read_to_string_ge_as_result)
/// * [`debug_assert_io_read_to_string_ge`](macro@crate::debug_assert_io_read_to_string_ge)
///
#[macro_export]
macro_rules! assert_io_read_to_string_ge {
    ($a_reader:expr, $b:expr $(,)?) => {
        match $crate::assert_io_read_to_string_ge_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a_reader:expr, $b:expr, $($message:tt)+) => {
        match $crate::assert_io_read_to_string_ge_as_result!($a_reader, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_io_read_to_string_ge {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn gt() {
        let a = "alfa".as_bytes();
        let b = "aa".as_bytes();
        for _ in 0..1 {
            let actual = assert_io_read_to_string_ge!(a, b);
            assert_eq!(actual, (String::from("alfa"), String::from("aa")));
        }
    }

    #[test]
    fn eq() {
        let a = "alfa".as_bytes();
        let b = "alfa".as_bytes();
        for _ in 0..1 {
            let actual = assert_io_read_to_string_ge!(a, b);
            assert_eq!(actual, (String::from("alfa"), String::from("alfa")));
        }
    }

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let a = "alfa".as_bytes();
            let b = "zz".as_bytes();
            let _actual = assert_io_read_to_string_ge!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
            " a label: `a`,\n",
            " a debug: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `[122, 122]`,\n",
            "       a: `\"alfa\"`,\n",
            "       b: `\"zz\"`"
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

/// Assert a ::std::io::Read read_to_string() value is greater than or equal to another.
///
/// Pseudocode:<br>
/// (a_reader.read_to_string(a_string) ⇒ a_string) ≥ (b_reader.read_to_string(b_string) ⇒ b_string)
///
/// This macro provides the same statements as [`assert_io_read_to_string_ge`](macro.assert_io_read_to_string_ge.html),
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
/// * [`assert_io_read_to_string_ge`](macro@crate::assert_io_read_to_string_ge)
/// * [`assert_io_read_to_string_ge`](macro@crate::assert_io_read_to_string_ge)
/// * [`debug_assert_io_read_to_string_ge`](macro@crate::debug_assert_io_read_to_string_ge)
///
#[macro_export]
macro_rules! debug_assert_io_read_to_string_ge {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_io_read_to_string_ge!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_io_read_to_string_ge {
    #[allow(unused_imports)]
    use std::panic;

    #[test]
    fn gt() {
        let a = "alfa".as_bytes();
        let b = "aa".as_bytes();
        for _ in 0..1 {
            let _actual = debug_assert_io_read_to_string_ge!(a, b);
            // assert_eq!(actual, (String::from("alfa"), String::from("aa")));
        }
    }

    #[test]
    fn eq() {
        let a = "alfa".as_bytes();
        let b = "alfa".as_bytes();
        for _ in 0..1 {
            let _actual = debug_assert_io_read_to_string_ge!(a, b);
            // assert_eq!(actual, (String::from("alfa"), String::from("alfa")));
        }
    }

    #[test]
    fn lt() {
        let result = panic::catch_unwind(|| {
            let a = "alfa".as_bytes();
            let b = "zz".as_bytes();
            let _actual = debug_assert_io_read_to_string_ge!(a, b);
        });
        let message = concat!(
            "assertion failed: `assert_io_read_to_string_ge!(a_reader, b_reader)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_io_read_to_string_ge.html\n",
            " a label: `a`,\n",
            " a debug: `[97, 108, 102, 97]`,\n",
            " b label: `b`,\n",
            " b debug: `[122, 122]`,\n",
            "       a: `\"alfa\"`,\n",
            "       b: `\"zz\"`"
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
