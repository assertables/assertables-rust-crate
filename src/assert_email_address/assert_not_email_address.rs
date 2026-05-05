//! Assert expression is possibly not an email address.
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let a = "hello*example.com";
//! assert_not_email_address!(a);
//! ```
//!
//! Note: this implementation is relatively basic.
//!
//! * If you want more capabilities, then use an email address parser.
//!
//! * If you want to know for sure, then send an email to the address.
//!
//! # Module macros
//!
//! * [`assert_not_email_address`](macro@crate::assert_not_email_address)
//! * [`assert_not_email_address_as_result`](macro@crate::assert_not_email_address_as_result)
//! * [`debug_assert_not_email_address`](macro@crate::debug_assert_not_email_address)

/// Assert expression is possibly not an email address.
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
/// * [`assert_not_email_address`](macro@crate::assert_not_email_address)
/// * [`assert_not_email_address_as_result`](macro@crate::assert_not_email_address_as_result)
/// * [`debug_assert_not_email_address`](macro@crate::debug_assert_not_email_address)
///
#[macro_export]
macro_rules! assert_not_email_address_as_result {
    ($a:expr $(,)?) => {
        match (&$a) {
            a => {
                if !a.contains("@") {
                    Ok(())
                }
                else {
                    let parts = a.split("@").collect::<Vec<&str>>();
                    match parts.len() {
                        2 => {
                            let (local_part, domain_part) = (parts[0], parts[1]);
                            let local_part_len = local_part.len();
                            let domain_part_len = domain_part.len();
                            if local_part_len < 1 || local_part_len > 64 || domain_part_len < 1 || domain_part_len > 255 {
                                Ok(())
                            }
                            else {
                                Err(
                                    format!(
                                        concat!(
                                            "assertion failed: `assert_not_email_address!(a)`\n",
                                            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_not_email_address.html\n",
                                            " email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.\n",
                                            " a label: `{}`,\n",
                                            " a debug: `{:?}`,\n",
                                            " a: `{}`,\n",
                                            " local part length: {},\n",
                                            " domain part length: {}",
                                        ),
                                        stringify!($a),
                                        $a,
                                        a,
                                        local_part_len,
                                        domain_part_len,
                                    )
                                )
                            }
                        },
                        _ => {
                            Ok(())
                        }
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_not_email_address_as_result {
    use std::sync::Once;

    #[test]
    fn success_because_at_sign_is_absent() {
        let a = "hello*example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address_as_result!(a);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_because_at_sign_is_absent_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "hello*example.com"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_email_address_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn success_because_local_part_is_blank() {
        let a = "@example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address_as_result!(a);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_because_local_part_is_blank_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "@example.com"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_email_address_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn success_because_local_part_is_too_long() {
        let a = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address_as_result!(a);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_because_local_part_is_too_long_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_email_address_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn success_because_domain_part_is_blank() {
        let a = "hello@";
        for _ in 0..1 {
            let actual = assert_not_email_address_as_result!(a);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_because_domain_part_is_blank_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "hello@"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_email_address_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn success_because_domain_part_is_too_long() {
        let a = "hello@xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        for _ in 0..1 {
            let actual = assert_not_email_address_as_result!(a);
            assert_eq!(actual.unwrap(), ());
        }
    }

    #[test]
    fn success_because_domain_part_is_too_long_once() {
        static A: Once = Once::new();
        fn a() -> &'static str {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            "hello@xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        }

        assert_eq!(A.is_completed(), false);
        let result = assert_not_email_address_as_result!(a());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
    }

    #[test]
    fn failure() {
        let a = "hello@example.com";
        let actual = assert_not_email_address_as_result!(a);
        let message = concat!(
            "assertion failed: `assert_not_email_address!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_not_email_address.html\n",
            " email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.\n",
            " a label: `a`,\n",
            " a debug: `\"hello@example.com\"`,\n",
            " a: `hello@example.com`,\n",
            " local part length: 5,\n",
            " domain part length: 11",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert expression is possibly not an email address.
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
/// let a = "hello*example.com";
/// assert_not_email_address!(a);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "hello@example.com";
/// assert_not_email_address!(a);
/// # });
/// // assertion failed: `assert_not_email_address!(a)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_not_email_address.html
/// //  Email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.
/// //  a label: `a`,
/// //  a debug: `\"hello@example.com\"`,
/// //  a: `hello@example.com`,
/// //  local part length: 5,
/// //  domain part length: 11",
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_not_email_address!(a)`\n",
/// #     "https://docs.rs/assertables/10.0.0/assertables/macro.assert_not_email_address.html\n",
/// #     " email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.\n",
/// #     " a label: `a`,\n",
/// #     " a debug: `\"hello@example.com\"`,\n",
/// #     " a: `hello@example.com`,\n",
/// #     " local part length: 5,\n",
/// #     " domain part length: 11",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_not_email_address`](macro@crate::assert_not_email_address)
/// * [`assert_not_email_address_as_result`](macro@crate::assert_not_email_address_as_result)
/// * [`debug_assert_not_email_address`](macro@crate::debug_assert_not_email_address)
///
#[macro_export]
macro_rules! assert_not_email_address {
    ($a:expr $(,)?) => {
        match $crate::assert_not_email_address_as_result!($a) {
            Ok(a) => a,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $($message:tt)+) => {
        match $crate::assert_not_email_address_as_result!($a) {
            Ok(a) => a,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_not_email_address {
    use std::panic;

    #[test]
    fn success_because_at_sign_is_absent() {
        let a = "hello*example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_local_part_is_blank() {
        let a = "@example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_local_part_is_too_long() {
        let a = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com";
        for _ in 0..1 {
            let actual = assert_not_email_address!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_domain_part_is_blank() {
        let a = "hello@";
        for _ in 0..1 {
            let actual = assert_not_email_address!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_domain_part_is_too_long() {
        let a = "hello@xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        for _ in 0..1 {
            let actual = assert_not_email_address!(a);
            assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a = "hello@example.com";
        let result = panic::catch_unwind(|| {
            let _actual = assert_not_email_address!(a);
        });
        let message = concat!(
            "assertion failed: `assert_not_email_address!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_not_email_address.html\n",
            " email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.\n",
            " a label: `a`,\n",
            " a debug: `\"hello@example.com\"`,\n",
            " a: `hello@example.com`,\n",
            " local part length: 5,\n",
            " domain part length: 11"
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

/// Assert expression is possibly not an email address.
///
/// This macro provides the same statements as [`assert_not_email_address`](macro.assert_not_email_address.html),
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
/// * [`assert_not_email_address`](macro@crate::assert_not_email_address)
/// * [`assert_not_email_address_as_result`](macro@crate::assert_not_email_address_as_result)
/// * [`debug_assert_not_email_address`](macro@crate::debug_assert_not_email_address)
///
#[macro_export]
macro_rules! debug_assert_not_email_address {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_not_email_address!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_not_email_address {
    use std::panic;

    #[test]
    fn success_because_at_sign_is_absent() {
        let a = "hello*example.com";
        for _ in 0..1 {
            let _actual = debug_assert_not_email_address!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_local_part_is_blank() {
        let a = "@example.com";
        for _ in 0..1 {
            let _actual = debug_assert_not_email_address!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_local_part_is_too_long() {
        let a = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com";
        for _ in 0..1 {
            let _actual = debug_assert_not_email_address!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_domain_part_is_blank() {
        let a = "hello@";
        for _ in 0..1 {
            let _actual = debug_assert_not_email_address!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn success_because_domain_part_is_too_long() {
        let a = "hello@xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        for _ in 0..1 {
            let _actual = debug_assert_not_email_address!(a);
            // assert_eq!(actual, ());
        }
    }

    #[test]
    fn failure() {
        let a = "hello@example.com";
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_not_email_address!(a);
        });
        let message = concat!(
            "assertion failed: `assert_not_email_address!(a)`\n",
            "https://docs.rs/assertables/10.0.0/assertables/macro.assert_not_email_address.html\n",
            " email address has local part with valid length 1..64, then an '@' sign, then a domain part with valid length 1..255.\n",
            " a label: `a`,\n",
            " a debug: `\"hello@example.com\"`,\n",
            " a: `hello@example.com`,\n",
            " local part length: 5,\n",
            " domain part length: 11"
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
