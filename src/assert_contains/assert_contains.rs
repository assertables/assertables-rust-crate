//! Assert a container is a match for an expression.
//!
//! Pseudocode:<br>
//! a.contains(b)
//!
//! These macros work with many kinds of Rust types, such as String, Vec, Range, HashSet.
//! The specifics depend on each type's implementation of a method `contains`, and some types
//! require the second argument to be borrowable, so be sure to check the Rust documentation.
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use std::collections::HashSet;
//!
//! // String contains substring
//! let a = "alfa";
//! let b = "lf";
//! assert_contains!(a, b);
//!
//! // Range contains value.
//! // Notice the &b because the macro calls Range.contains(&self, &value).
//! let a = 1..3;
//! let b = 2;
//! assert_contains!(a, &b);
//!
//! // Vector contains element.
//! // Notice the &b because the macro calls Vec.contains(&self, &value).
//! let a = vec![1, 2, 3];
//! let b = 2;
//! assert_contains!(a, &b);
//!
//! // HashSet contains element.
//! // Notice the &b because the macro calls HashSet.contains(&self, &value).
//! let a: HashSet<String> = [String::from("1")].into();
//! let b: String = String::from("1");
//! assert_contains!(a, &b);
//!
//! // HashSet contains element with automatic borrow optimization.
//! // Notice the b because the value type &str is already a reference,
//! // which HashSet.contains knows how to borrow to compare to String.
//! let a: HashSet<String> = [String::from("1")].into();
//! let b = "1";
//! assert_contains!(a, b);
//! ```
//!
//! # Module macros
//!
//! * [`assert_contains`](macro@crate::assert_contains)
//! * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
//! * [`debug_assert_contains`](macro@crate::debug_assert_contains)

/// Assert an expression (such as a string) contains an expression (such as a substring).
///
/// Pseudocode:<br>
/// a.contains(b)
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
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! assert_contains_as_result {
    ($container:expr, $containee:expr $(,)?) => {
        match (&$container, &$containee) {
            (container, containee) => {
                if (container).contains(*containee) {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert_contains!(container, containee)`\n",
                            "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                            " container label: `{}`,\n",
                            " container debug: `{:?}`,\n",
                            " containee label: `{}`,\n",
                            " containee debug: `{:?}`",
                        ),
                        stringify!($container),
                        container,
                        stringify!($containee),
                        containee,
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_contains_as_result {
    use std::panic;
    use std::sync::Once;

    mod str {
        use super::*;

        #[test]
        fn success() {
            let a: &'static str = "alfa";
            let b: &'static str = "lf";
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> &'static str {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                "alfa"
            }

            static B: Once = Once::new();
            fn b() -> &'static str {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                "lf"
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_contains_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: &'static str = "alfa";
            let b: &'static str = "xx";
            let actual = assert_contains_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"alfa\"`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"xx\"`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod range_i32 {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<i32> = 1..3;
            let b: i32 = 2;
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, &b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> Range<i32> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                1..3
            }

            static B: Once = Once::new();
            fn b() -> i32 {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                2
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_contains_as_result!(a(), &b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: Range<i32> = 1..3;
            let b: i32 = 4;
            let actual = assert_contains_as_result!(a, &b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `1..3`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod range_string {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("2");
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, &b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> Range<String> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                String::from("1")..String::from("3")
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
            let result = assert_contains_as_result!(a(), &b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("4");
            let actual = assert_contains_as_result!(a, &b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"1\"..\"3\"`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod vec_i32 {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 2;
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, &b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> Vec<i32> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                vec![1, 2, 3]
            }

            static B: Once = Once::new();
            fn b() -> i32 {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                2
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_contains_as_result!(a(), &b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 4;
            let actual = assert_contains_as_result!(a, &b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[1, 2, 3]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod vec_string {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("2");

            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, &b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> Vec<String> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                vec![String::from("1"), String::from("2"), String::from("3")]
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
            let result = assert_contains_as_result!(a(), &b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("4");
            let actual = assert_contains_as_result!(a, &b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[\"1\", \"2\", \"3\"]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod hashset_string {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("1");
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, &b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> HashSet<String> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                [String::from("1")].into()
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
            let result = assert_contains_as_result!(a(), &b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("2");
            let actual = assert_contains_as_result!(a, &b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"2\"`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }

    mod hashset_string_with_str_automatic_borrow_optimization {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "1";
            for _ in 0..1 {
                let actual = assert_contains_as_result!(a, b);
                assert_eq!(actual.unwrap(), ());
            }
        }

        #[test]
        fn success_once() {
            static A: Once = Once::new();
            fn a() -> HashSet<String> {
                if A.is_completed() {
                    panic!("A.is_completed()")
                } else {
                    A.call_once(|| {})
                }
                [String::from("1")].into()
            }

            static B: Once = Once::new();
            fn b() -> &'static str {
                if B.is_completed() {
                    panic!("B.is_completed()")
                } else {
                    B.call_once(|| {})
                }
                "1"
            }

            assert_eq!(A.is_completed(), false);
            assert_eq!(B.is_completed(), false);
            let result = assert_contains_as_result!(a(), b());
            assert!(result.is_ok());
            assert_eq!(A.is_completed(), true);
            assert_eq!(B.is_completed(), true);
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "2";
            let actual = assert_contains_as_result!(a, b);
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"2\"`"
            );
            assert_eq!(actual.unwrap_err(), message);
        }
    }
}

/// Assert a container is a match for an expression.
///
/// Pseudocode:<br>
/// a.contains(b)
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
/// use std::collections::HashSet;
/// # use std::panic;
///
/// # fn main() {
/// // String contains a substring
/// let a = "alfa";
/// let b = "lf";
/// assert_contains!(a, b);
///
/// // Range contains a value
/// let a = 1..3;
/// let b = 2;
/// assert_contains!(a, &b);
///
/// // Vector contains element
/// let a = vec![1, 2, 3];
/// let b = 2;
/// assert_contains!(a, &b);
///
/// // HashSet contains element.
/// // Notice the &b because the macro calls HashSet.contains(&self, &value).
/// let a: HashSet<String> = [String::from("1")].into();
/// let b: String = String::from("1");
/// assert_contains!(a, &b);
///
/// // HashSet contains element with automatic borrow optimization.
/// // Notice the b because the value type &str is already a reference,
/// // which HashSet.contains knows how to borrow to compare to String.
/// let a: HashSet<String> = [String::from("1")].into();
/// let b = "1";
/// assert_contains!(a, b);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let a = "alfa";
/// let b = "xx";
/// assert_contains!(a, b);
/// # });
/// // assertion failed: `assert_contains!(container, containee)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_contains.html
/// //  container label: `a`,
/// //  container debug: `\"alfa\"`,
/// //  containee label: `b`,
/// //  containee debug: `\"xx\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_contains!(container, containee)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
/// #     " container label: `a`,\n",
/// #     " container debug: `\"alfa\"`,\n",
/// #     " containee label: `b`,\n",
/// #     " containee debug: `\"xx\"`"
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains_as_result`](macro@crate::assert_contains_as_result)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! assert_contains {
    ($container:expr, $containee:expr $(,)?) => {
        match $crate::assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}", err),
        }
    };
    ($container:expr, $containee:expr, $($message:tt)+) => {
        match $crate::assert_contains_as_result!($container, $containee) {
            Ok(()) => (),
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_contains {
    use std::panic;

    mod str {
        use super::*;

        #[test]
        fn success() {
            let a: &'static str = "alfa";
            let b: &'static str = "lf";
            for _ in 0..1 {
                let actual = assert_contains!(a, b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: &'static str = "alfa";
            let b: &'static str = "xx";
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"alfa\"`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"xx\"`"
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

    mod range_i32 {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<i32> = 1..3;
            let b: i32 = 2;
            for _ in 0..1 {
                let actual = assert_contains!(a, &b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Range<i32> = 1..3;
            let b: i32 = 4;
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `1..3`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
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

    mod range_string {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("2");
            for _ in 0..1 {
                let actual = assert_contains!(a, &b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("4");
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"1\"..\"3\"`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
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

    mod vec_i32 {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 2;
            for _ in 0..1 {
                let actual = assert_contains!(a, &b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 4;
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[1, 2, 3]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
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

    mod vec_string {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("2");
            for _ in 0..1 {
                let actual = assert_contains!(a, &b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("4");
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[\"1\", \"2\", \"3\"]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
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

    mod hashset_string {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("1");
            for _ in 0..1 {
                let actual = assert_contains!(a, &b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("2");
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"2\"`"
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

    mod hashset_string_with_str_automatic_borrow_optimization {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "1";
            for _ in 0..1 {
                let actual = assert_contains!(a, b);
                let expect = ();
                assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "2";
            let result = panic::catch_unwind(|| {
                let _actual = assert_contains!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"2\"`"
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

/// Assert a container is a match for an expression.
///
/// Pseudocode:<br>
/// a.contains(b)
///
/// This macro provides the same statements as [`assert_contains`](macro.assert_contains.html),
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
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`assert_contains`](macro@crate::assert_contains)
/// * [`debug_assert_contains`](macro@crate::debug_assert_contains)
///
#[macro_export]
macro_rules! debug_assert_contains {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_contains!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_contains {
    use std::panic;

    mod str {
        use super::*;

        #[test]
        fn success() {
            let a: &'static str = "alfa";
            let b: &'static str = "lf";
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: &'static str = "alfa";
            let b: &'static str = "xx";
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"alfa\"`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"xx\"`"
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

    mod range_i32 {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<i32> = 1..3;
            let b: i32 = 2;
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, &b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Range<i32> = 1..3;
            let b: i32 = 4;
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `1..3`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
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

    mod range_string {
        use super::*;
        use std::ops::Range;

        #[test]
        fn success() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("2");
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, &b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Range<String> = String::from("1")..String::from("3");
            let b: String = String::from("4");
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `\"1\"..\"3\"`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
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

    mod vec_i32 {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 2;
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, &b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: Vec<i32> = vec![1, 2, 3];
            let b: i32 = 4;
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[1, 2, 3]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `4`"
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

    mod vec_string {
        use super::*;

        #[test]
        fn success() {
            let a: Vec<String> = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("2");
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, &b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a = vec![String::from("1"), String::from("2"), String::from("3")];
            let b: String = String::from("4");
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `[\"1\", \"2\", \"3\"]`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"4\"`"
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

    mod hashset_string {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("1");
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, &b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: String = String::from("2");
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, &b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `&b`,\n",
                " containee debug: `\"2\"`"
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

    mod hashset_string_with_str_automatic_borrow_optimization {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn success() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "1";
            for _ in 0..1 {
                let _actual = debug_assert_contains!(a, b);
                let _expect = ();
                // assert_eq!(actual, expect);
            }
        }

        #[test]
        fn failure() {
            let a: HashSet<String> = [String::from("1")].into();
            let b: &'static str = "2";
            let result = panic::catch_unwind(|| {
                let _actual = debug_assert_contains!(a, b);
            });
            let message = concat!(
                "assertion failed: `assert_contains!(container, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_contains.html\n",
                " container label: `a`,\n",
                " container debug: `{\"1\"}`,\n",
                " containee label: `b`,\n",
                " containee debug: `\"2\"`"
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
