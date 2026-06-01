//! Assert an expression is not equal to another.
//!
//! Pseudocode:<br>
//! a ≠ b
//!
//! # Module macro
//!
//! * [`assert_ne_as_result`](macro@crate::assert_ne_as_result)
//!
//! # Rust standard macros
//!
//! * [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html)
//! * [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)

/// Assert an expression is not equal to another.
///
/// Pseudocode:<br>
/// a ≠ b
///
/// * If true, return Result `Ok(())`.
///
/// * When false, return [`Err`] with a message and the values of the
///   expressions with their debug representations.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macro
///
/// * [`assert_ne_as_result`](macro@crate::assert_ne_as_result)
///
/// # Rust standard macros
///
/// * [`assert_ne`](https://doc.rust-lang.org/std/macro.assert_ne.html)
/// * [`debug_assert_ne`](https://doc.rust-lang.org/std/macro.debug_assert_ne.html)
///
#[macro_export]
macro_rules! assert_ne_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match (&$a, &$b) {
            (a, b) => {
                if a != b {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_ne!(a, b)`\n",
                            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_ne.html\n",
                            " a label: `{}`,\n",
                            " a debug: `{:?}`,\n",
                            " b label: `{}`,\n",
                            " b debug: `{:?}`"
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
mod test_assert_ne_as_result {
    use std::sync::Once;

    mod integer {
        use super::*;

        #[test]
        fn lt() {
            let a: i8 = 1;
            let b: i8 = 2;
            for _ in 0..1 {
                let actual = assert_ne_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn lt_once() {
            static A: Once = Once::new();
            fn a() -> i8 {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                1
            }

            static B: Once = Once::new();
            fn b() -> i8 {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                2
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_ne_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn gt() {
            let a: i8 = 2;
            let b: i8 = 1;
            for _ in 0..1 {
                let actual = assert_ne_as_result!(a, b);
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
            let result = assert_ne_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn eq() {
            let a: i8 = 1;
            let b: i8 = 1;
            let actual = assert_ne_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_ne!(a, b)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_ne.html\n",
                " a label: `a`,\n",
                " a debug: `1`,\n",
                " b label: `b`,\n",
                " b debug: `1`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod string {
        use super::*;

        #[test]
        fn lt() {
            let a: String = String::from("1");
            let b: String = String::from("2");
            for _ in 0..1 {
                let actual = assert_ne_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn lt_once() {
            static A: Once = Once::new();
            fn a() -> String {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                String::from("1")
            }

            static B: Once = Once::new();
            fn b() -> String {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                String::from("2")
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_ne_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn gt() {
            let a: String = String::from("2");
            let b: String = String::from("1");
            for _ in 0..1 {
                let actual = assert_ne_as_result!(a, b);
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
            let result = assert_ne_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn eq() {
            let a: String = String::from("1");
            let b: String = String::from("1");
            let actual = assert_ne_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_ne!(a, b)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_ne.html\n",
                " a label: `a`,\n",
                " a debug: `\"1\"`,\n",
                " b label: `b`,\n",
                " b debug: `\"1\"`",
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
}
