//! Assert a ::std::fs::read_to_string(path) contains a pattern.
//!
//! Pseudocode:<br>
//! std::fs::read_to_string(path) contains expr
//!
//! # Example
//!
//! ```rust
//! use assertables::*;
//!
//! let path = "alfa.txt";
//! let containee = "alfa";
//! assert_fs_read_to_string_contains!(path, containee);
//! ```
//!
//! # Module macros
//!
//! * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
//! * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
//! * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)

/// Assert a ::std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
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
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_contains_as_result {
    ($path:expr, $containee:expr $(,)?) => {
        match (&$path, &$containee) {
            (path, containee) => {
                match (::std::fs::read_to_string(path)) {
                    Ok(string) => {
                        if string.contains(*containee) {
                            Ok(string)
                        } else {
                            Err(
                                format!(
                                    concat!(
                                        "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                                        "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                                        "      path label: `{}`,\n",
                                        "      path debug: `{:?}`,\n",
                                        " containee label: `{}`,\n",
                                        " containee debug: `{:?}`,\n",
                                        "          string: `{:?}`",
                                    ),
                                    stringify!($path),
                                    path,
                                    stringify!($containee),
                                    containee,
                                    string
                                )
                            )
                        }
                    },
                    Err(err) => {
                        Err(
                            format!(
                                concat!(
                                    "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                                    "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                                    "      path label: `{}`,\n",
                                    "      path debug: `{:?}`,\n",
                                    " containee label: `{}`,\n",
                                    " containee debug: `{:?}`,\n",
                                    "        read err: `{:?}`"
                                ),
                                stringify!($path),
                                path,
                                stringify!($containee),
                                containee,
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
mod test_assert_fs_read_to_string_contains_as_result {
    #[allow(unused_imports)]
    use std::io::Read;
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
        let containee = "lf";
        for _ in 0..1 {
            let actual = assert_fs_read_to_string_contains_as_result!(path, containee);
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
        let result = assert_fs_read_to_string_contains_as_result!(a(), b());
        assert!(result.is_ok());
        assert_eq!(A.is_completed(), true);
        assert_eq!(B.is_completed(), true);
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let containee = "zz";
        let actual = assert_fs_read_to_string_contains_as_result!(path, containee);
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                "      path label: `path`,\n",
                "      path debug: `{:?}`,\n",
                " containee label: `containee`,\n",
                " containee debug: `\"zz\"`,\n",
                "          string: `\"alfa\\n\"`",
            ),
            path
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}

/// Assert a ::std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
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
///
/// # fn main() {
/// let path = "alfa.txt";
/// let containee = "alfa";
/// assert_fs_read_to_string_contains!(path, containee);
///
/// # let result = panic::catch_unwind(|| {
/// // This will panic
/// let path = "alfa.txt";
/// let containee = "zz";
/// assert_fs_read_to_string_contains!(path, containee);
/// # });
/// // assertion failed: `assert_fs_read_to_string_contains!(path, containee)`
/// // https://docs.rs/assertables/…/assertables/macro.assert_fs_read_to_string_contains.html
/// //       path label: `path`,
/// //       path debug: `\"alfa.txt\"`,
/// //  containee label: `containee`,
/// //  containee debug: `\"zz\"`,
/// //           string: `\"alfa\\n\"`
/// # let actual = result.unwrap_err().downcast::<String>().unwrap().to_string();
/// # let message = concat!(
/// #     "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
/// #     "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
/// #     "      path label: `path`,\n",
/// #     "      path debug: `\"alfa.txt\"`,\n",
/// #     " containee label: `containee`,\n",
/// #     " containee debug: `\"zz\"`,\n",
/// #     "          string: `\"alfa\\n\"`",
/// # );
/// # assert_eq!(actual, message);
/// # }
/// ```
///
/// # Module macros
///
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains_as_result`](macro@crate::assert_fs_read_to_string_contains_as_result)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! assert_fs_read_to_string_contains {
    ($path:expr, $containee:expr $(,)?) => {
        match $crate::assert_fs_read_to_string_contains_as_result!($path, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($path:expr, $containee:expr, $($message:tt)+) => {
        match $crate::assert_fs_read_to_string_contains_as_result!($path, $containee) {
            Ok(x) => x,
            Err(err) => panic!("{}\n{}", format_args!($($message)+), err),
        }
    };
}

#[cfg(test)]
mod test_assert_fs_read_to_string_contains {
    #[allow(unused_imports)]
    use std::io::Read;
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
        let containee = "alfa";
        for _ in 0..1 {
            let actual = assert_fs_read_to_string_contains!(path, containee);
            assert_eq!(actual, String::from("alfa\n"));
        }
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let containee = "zz";
        let result = panic::catch_unwind(|| {
            let _actual = assert_fs_read_to_string_contains!(path, containee);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                "      path label: `path`,\n",
                "      path debug: `{:?}`,\n",
                " containee label: `containee`,\n",
                " containee debug: `\"zz\"`,\n",
                "          string: `\"alfa\\n\"`",
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

/// Assert a ::std::fs::read_to_string(path) contains a pattern.
///
/// Pseudocode:<br>
/// std::fs::read_to_string(path) contains expr
///
/// This macro provides the same statements as [`assert_fs_read_to_string_contains`](macro.assert_fs_read_to_string_contains.html),
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
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`assert_fs_read_to_string_contains`](macro@crate::assert_fs_read_to_string_contains)
/// * [`debug_assert_fs_read_to_string_contains`](macro@crate::debug_assert_fs_read_to_string_contains)
///
#[macro_export]
macro_rules! debug_assert_fs_read_to_string_contains {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_fs_read_to_string_contains!($($arg)*);
        }
    };
}

#[cfg(test)]
mod test_debug_assert_fs_read_to_string_contains {
    #[allow(unused_imports)]
    use std::io::Read;
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
        let containee = "alfa";
        for _ in 0..1 {
            let _actual = debug_assert_fs_read_to_string_contains!(path, containee);
            // assert_eq!(actual, String::from("alfa\n"));
        }
    }

    #[test]
    fn failure() {
        let path = DIR.join("alfa.txt");
        let containee = "zz";
        let result = panic::catch_unwind(|| {
            let _actual = debug_assert_fs_read_to_string_contains!(path, containee);
        });
        let message = format!(
            concat!(
                "assertion failed: `assert_fs_read_to_string_contains!(path, containee)`\n",
                "https://docs.rs/assertables/10.1.0/assertables/macro.assert_fs_read_to_string_contains.html\n",
                "      path label: `path`,\n",
                "      path debug: `{:?}`,\n",
                " containee label: `containee`,\n",
                " containee debug: `\"zz\"`,\n",
                "          string: `\"alfa\\n\"`",
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
