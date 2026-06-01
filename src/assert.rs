//! Assert a condition is true.
//!
//! Pseudocode:<br>
//! condition
//!
//! * If true, return Result `Ok(())`.
//!
//! * Otherwise, return Result `Err(message)`.
//!
//! This macro provides the same statements as [`assert`](macro@assert),
//! except this macro returns a Result, rather than doing a panic.
//!
//! This macro is useful for runtime checks, such as checking parameters,
//! or sanitizing inputs, or handling different results in different ways.
//!
//! # Module macros
//!
//! * [`assert_as_result`](macro.assert_as_result.html)

/// Assert a condition is true.
///
/// Pseudocode:<br>
/// condition
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
/// * [`assert_as_result`](macro.assert_as_result.html)
///
#[macro_export]
macro_rules! assert_as_result {
    ($a:expr $(,)?) => {
        match ($a) {
            a => {
                if a {
                    Ok(())
                } else {
                    Err(format!(
                        concat!(
                            "assertion failed: `assert!(condition)`\n",
                            "https://docs.rs/assertables/10.1.0/assertables/macro.assert.html\n",
                            " condition label: `{}`,\n",
                            " condition debug: `{:?}`,\n",
                        ),
                        stringify!($a),
                        a,
                    ))
                }
            }
        }
    };
}

#[cfg(test)]
mod test_assert_as_result {
    // use std::sync::Once;

    #[test]
    fn success() {
        let a = true;
        for _ in 0..1 {
            let actual = assert_as_result!(a);
            let expect = ();
            assert_eq!(actual.unwrap(), expect);
        }
    }

    #[test]
    fn failure() {
        let a = false;
        let actual = assert_as_result!(a);
        let message = concat!(
            "assertion failed: `assert!(condition)`\n",
            "https://docs.rs/assertables/10.1.0/assertables/macro.assert.html\n",
            " condition label: `a`,\n",
            " condition debug: `false`,\n",
        );
        assert_eq!(actual.unwrap_err(), message);
    }
}
