//! Assert a ::std::fs::read_to_string(path) is a match to a regex.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path) matches expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//! use regex::Regex;
//!
//! let path = "alfa.txt";
//! let matcher = Regex::new(r"lf").expect("regex");
//! assert_fs_read_to_string_is_match!(path, matcher);
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
//! * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
//! * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// * If true, return Result `Ok(path_into_string)`.
///
/// * Otherwise, return Result `Err(message)`.
///
/// This macro is useful for runtime checks, such as checking parameters,
/// or sanitizing inputs, or handling different results in different ways.
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_is_match_as_result {
    ($path:expr, $matcher:expr $(,)?) => {
        match (&$path, &$matcher) {
            (path, matcher) => {
                match (::std::fs::read_to_string(path)) {
                    Ok(string) => {
                        if matcher.is_match(&string) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                                        "    path label: `{}`,\n",
                                        "    path debug: `{:?}`,\n",
                                        " matcher label: `{}`,\n",
                                        " matcher debug: `{:?}`,\n",
                                        "        string: `{:?}`",
                                    ),
                                    stringify!($path),
                                    path,
                                    stringify!($matcher),
                                    matcher,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                                    "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                                    "    path label: `{}`,\n",
                                    "    path debug: `{:?}`,\n",
                                    " matcher label: `{}`,\n",
                                    " matcher debug: `{:?}`,\n",
                                    "           err: `{:?}`"
                                ),
                                stringify!($path),
                                path,
                                stringify!($matcher),
                                matcher,
                                err
                            )
                        )
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_fs_read_to_string_is_match_as_result {
    use regex::Regex;
    use std::path::PathBuf;
    use std::sync::LazyLock;
    use std::sync::Once;

    pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual = assert_fs_read_to_string_is_match_as_result!(path, matcher);
            assert_eq!(actual.unwrap(), String::from("alfa\n"));
        }
    }

    #[test]
    fn success_once() {
        static A: Once = Once::new();
        fn a() -> PathBuf {
            if A.is_completed() {
                panic!("A.is_completed()")
            } else {
                A.call_once(|| {})
            }
            DIR.join("alfa.txt")
        }

        static B: Once = Once::new();
        fn b() -> Regex {
            if B.is_completed() {
                panic!("B.is_completed()")
            } else {
                B.call_once(|| {})
            }
            Regex::new(r"lf").expect("regex")
        }

        assert_eq!(A.is_completed(), false);
        assert_eq!(B.is_completed(), false);
        let result = assert_fs_read_to_string_is_match_as_result!(&a(), &b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zz").expect("regex");
        let actual = assert_fs_read_to_string_is_match_as_result!(path, matcher);
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                "    path label: `path`,\n",
                "    path debug: `{:?}`,\n",
                " matcher label: `matcher`,\n",
                " matcher debug: `Regex(\"zz\")`,\n",
                "        string: `\"alfa\\n\"`",
            ),
            path
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// * If true, return `path_into_string`.
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
/// use regex::Regex;
///
/// # fn main() {
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"lf").expect("regex");
/// assert_fs_read_to_string_is_match!(path, matcher);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let path = "alfa.txt";
/// let matcher = Regex::new(r"zz").expect("regex");
/// assert_fs_read_to_string_is_match!(path, matcher);
/// # });
/// // assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_fs_read_to_string_is_match.html
/// //     path label: `path`,
/// //     path debug: `\"alfa.txt\"`,
/// //  matcher label: `matcher`,
/// //  matcher debug: `Regex(\"zz\")`,
/// //         string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
/// #     "    path label: `path`,\n",
/// #     "    path debug: `\"alfa.txt\"`,\n",
/// #     " matcher label: `matcher`,\n",
/// #     " matcher debug: `Regex(\"zz\")`,\n",
/// #     "        string: `\"alfa\\n\"`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match_as_result`](macro@crate::assert_fs_read_to_string_is_match_as_result)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_is_match {
    ($path:expr, $matcher:expr $(,)?) => {
        match $crate::assert_fs_read_to_string_is_match_as_result!($path, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($path:expr, $matcher:expr, $($message:tt)+) => {
        match $crate::assert_fs_read_to_string_is_match_as_result!($path, $matcher) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_fs_read_to_string_is_match {
    use regex::Regex;
    use std::panic;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let actual = assert_fs_read_to_string_is_match!(path, matcher);
            assert_eq!(actual, String::from("alfa\n"));
        }
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zz").expect("regex");
        let result = panic::catch_unwind(|| {
            let _actual = assert_fs_read_to_string_is_match!(path, matcher);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                "    path label: `path`,\n",
                "    path debug: `{:?}`,\n",
                " matcher label: `matcher`,\n",
                " matcher debug: `Regex(\"zz\")`,\n",
                "        string: `\"alfa\\n\"`",
            ),
            path
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

/// Assert a ::std::fs::read_to_string(path) is a match to a regex.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) matches expr
///
/// This macro provides the same statements as [`assert_fs_read_to_string_is_match`](macro.assert_fs_read_to_string_is_match.html),
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
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`assert_fs_read_to_string_is_match`](macro@crate::assert_fs_read_to_string_is_match)
/// * [`debug_assert_fs_read_to_string_is_match`](macro@crate::debug_assert_fs_read_to_string_is_match)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_is_match {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_fs_read_to_string_is_match!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_fs_read_to_string_is_match {
    use regex::Regex;
    use std::panic;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    pub static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("src")
            .join("std")
            .join("fs")
    });

    #[test]
    fn success() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"lf").expect("regex");
        for _ in 0..1 {
            let _actual = debug_assert_fs_read_to_string_is_match!(path, matcher);
            // assert_eq!(actual, String::from("alfa\n"));
        }
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let matcher = Regex::new(r"zz").expect("regex");
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_fs_read_to_string_is_match!(path, matcher);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_is_match!(path, matcher)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_is_match.html\n",
                "    path label: `path`,\n",
                "    path debug: `{:?}`,\n",
                " matcher label: `matcher`,\n",
                " matcher debug: `Regex(\"zz\")`,\n",
                "        string: `\"alfa\\n\"`",
            ),
            path
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
