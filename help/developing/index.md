# Developing

Developing Assertables is straightforward. This page explains how to create the code for a typical Assertables macro, in three forms, using the example of the macro `assert_gt`.

We write each macro in three forms:

- `assert_gt_as_result` returns a `Result` as `Ok`, `Err`. This macro contains all the logic, all the error formatting, etc. This macro is especially useful for runtime checks, such as when you want to know success or failure, and you don't want to panic. This macro is called by the other forms below.

- `assert_gt` returns successfully or does a panic. This is the typical macro that most developers will use for testing. This macro wraps `assert_gt_as_result`. This macro provides two arms: one arm is for returning the error message as is, and one arm is for returning a developer's custom error message.

- `debug_assert_gt` returns successfully or does a panic, when the program is running using Rust debug configuration. This macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The sections below show each of the three forms, using our simplest macro group: `assert_gt_as_result`, `assert_gt`, `debug_assert_gt`.

## assert_gt_as_result

The macro `assert_gt_as_result` returns a `Result` as `Ok`, `Err`.

The macro contains all the logic and all the error formatting.

The macro is called by the other forms: directly by `assert_gt` and indirectly by `debug_assert_gt`.

The macros is useful for runtime checks, such as when you want to know success or failure, and you don't want to panic.

Code:

```rust
#[macro_export]
macro_rules! assert_gt_as_result {
    ($a:expr, $b:expr $(,)?) => {
        match ($a, $b) {
            (a, b) => {
                if a > b {
                    Ok(())
                } else {
                    Err(
                        format!(
                            concat!(
                                "assertion failed: `assert_gt!(a, b)`\n",
                                "https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html\n",
                                "  left label: `{}`,\n",
                                "  left debug: `{:?}`,\n",
                                " right label: `{}`,\n",
                                " right debug: `{:?}`"
                            ),
                            stringify!($a),
                            a,
                            stringify!($b),
                            b
                        )
                    )
                }
            }
        }
    };
}
```

## assert_gt

The macro `assert_gt` returns successfully or panics.

The macro is the typical macro that most developers will use for testing.

The macro wraps `assert_gt_as_result`.

The macro provides two arms: one arm is for returning the error message as is, and one arm is for returning a developer's custom error message.

Code:

```rust
#[macro_export]
macro_rules! assert_gt {
    ($a:expr, $b:expr $(,)?) => {
        match $crate::assert_gt_result!($a, $b) {
            Ok(x) => x,
            Err(err) => panic!("{}", err),
        }
    };
    ($a:expr, $b:expr, $($message:tt)+) => ({
        match $crate::assert_gt!($a, $b) {
            Ok(x) => x,
            Err(_err) => panic!("{}", $($message)+),
        }
    };
}
```

## debug_assert_gt

The macro `debug_assert_gt` return `()` or panics.

The macro is the typical macro that most developers will use for runtime debugging during development, and possibly also for runtime debugging during production.

The macro's statements are only enabled in non-optimized builds by default. An optimized build will not execute this macro's statements unless `-C debug-assertions` is passed to the compiler.

The macro wraps `assert_gt`.

Code:

```rust
#[macro_export]
macro_rules! debug_assert_gt {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            $crate::assert_gt!($($arg)*);
        }
    };
}
```
