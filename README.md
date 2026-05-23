<!--
tags: #assert #assertion #rust #testing #macros #tdd #testdrivendevelopment
-->

# Assertables: assert macros for better testing

Assertables is a Rust crate of assert macros to improve your compile-time tests and run-time reliability.

**[documentation](https://docs.rs/assertables/)**
•
**[llms](https://raw.githubusercontent.com/assertables/assertables-rust-crate/refs/heads/main/llms.txt)**
•
**[email](mailto:joel@joelparkerhenderson.com)**
•
**[crates.io](https://crates.io/crates/assertables)**
•
**[github](https://github.com/assertables/assertables-rust-crate/)**
•
**[gitlab](https://gitlab.com/assertables/assertables-rust-crate/)**
•
**[codeberg](https://codeberg.org/assertables/assertables-rust-crate/)**

## Introduction

The Assertables Rust crate provides many assert macros that can help you
develop, test, and debug.

This documentation is also available as
[llms.txt](https://github.com/assertables/assertables-rust-crate/llms.txt).

- Test values with
  [assert_lt](https://docs.rs/assertables/10.0.0/assertables/assert_lt),
  [assert_gt](https://docs.rs/assertables/10.0.0/assertables/assert_gt),
  [assert_in](https://docs.rs/assertables/10.0.0/assertables/assert_in),
  […](https://docs.rs/assertables)
- Test groups with
  [assert_all](https://docs.rs/assertables/10.0.0/assertables/assert_all),
  [assert_any](https://docs.rs/assertables/10.0.0/assertables/assert_any),
  [assert_iter](https://docs.rs/assertables/10.0.0/assertables/assert_iter),
  […](https://docs.rs/assertables)
- Test wrappers with
  [assert_ok](https://docs.rs/assertables/10.0.0/assertables/assert_ok),
  [assert_some](https://docs.rs/assertables/10.0.0/assertables/assert_some),
  [assert_ready](https://docs.rs/assertables/10.0.0/assertables/assert_ready),
  […](https://docs.rs/assertables)
- Test matching with
  [assert_matches](https://docs.rs/assertables/10.0.0/assertables/assert_matches),
  [assert_is_match](https://docs.rs/assertables/10.0.0/assertables/assert_is_match),
  […](https://docs.rs/assertables)
- Test approximations with
  [assert_approx](https://docs.rs/assertables/10.0.0/assertables/assert_approx),
  [assert_abs_diff](https://docs.rs/assertables/10.0.0/assertables/assert_abs_diff),
  […](https://docs.rs/assertables/)
- Test programs with
  [assert_command](https://docs.rs/assertables/10.0.0/assertables/assert_command),
  [assert_status](https://docs.rs/assertables/10.0.0/assertables/assert_staus),
  […](https://docs.rs/assertables)
- Many more below.

To use this crate, add it to your file `Cargo.toml`:

```toml
assertables = "10.0.0"
```

Benefits:

- You can write better tests to improve reliability and maintainability.
- You can handle more corner cases without needing to write custom code.
- You can troubleshoot faster because error messages show more detail.

Learning:
[FAQ](https://github.com/assertables/assertables-rust-crate/tree/main/help/faq),
[docs](https://docs.rs/assertables/),
[examples](https://github.com/assertables/assertables-rust-crate/blob/main/tests/examples/),
[changes](https://github.com/assertables/assertables-rust-crate/tree/main/CHANGES.md),
[upgrades](https://github.com/assertables/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9),
[developing](https://github.com/assertables/assertables-rust-crate/tree/main/help/developing/).

Comparisons:
[more_asserts](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/more_asserts),
[cool_asserts](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/cool_asserts),
[assert2](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/assert2),
[claims](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/claims),
[etc.](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons)

## Examples

Examples with numbers:

```rust
let i = 1;
assert_lt!(i, 2);
assert_in_range!(&i, 0..2);
```

Examples with strings:

```rust
let s = "hello";
assert_starts_with!(s, "h");
assert_is_match!(Regex::new(r"e.*o").unwrap(), s);
```

Examples with arrays:

```rust
let a = [1, 2, 3];
assert_contains!(a, &2);
assert_all!(a.iter(), |i: i32| i < 4);
```

## Highlights

Values:

- [`assert_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_eq/)
- [`assert_ne!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_ne/)
- [`assert_ge!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_ge/)
- [`assert_gt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_gt/)
- [`assert_le!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_le/)
- [`assert_lt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_lt/)

Floats:

- [`assert_f32_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_f32/assert_f32_eq/)
- [`assert_f64_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_f64/assert_f64_eq/)

Approximations:

- [`assert_approx_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_approx/assert_approx_eq/)
- [`assert_approx_eq_with_absolute_error!(a, b, err)`](https://docs.rs/assertables/10.0.0/assertables/assert_approx/assert_approx_eq_with_absolute_error/)
- [`assert_approx_eq_with_relative_error!(a, b, err)`](https://docs.rs/assertables/10.0.0/assertables/assert_approx/assert_approx_eq_with_relative_error/)

Nearness:

- [`assert_in_delta!(a, b, delta)`](https://docs.rs/assertables/10.0.0/assertables/assert_in/assert_in_delta/)
- [`assert_in_epsilon!(a, b, epsilon)`](https://docs.rs/assertables/10.0.0/assertables/assert_in/assert_in_epsilon/)
- [`assert_in_range!(a, range)`](https://docs.rs/assertables/10.0.0/assertables/assert_in/assert_in_range/)
- [`assert_diff_eq_x!(a, b, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_diff/assert_diff_eq_x/)
- [`assert_abs_diff_eq_x!(a, b, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_abs_diff/assert_abs_diff_eq_x/)

Groups:

- [`assert_all!(group, predicate)`](https://docs.rs/assertables/10.0.0/assertables/assert_all/)
- [`assert_any!(group, predicate)`](https://docs.rs/assertables/10.0.0/assertables/assert_any/)
- [`assert_is_empty!(group)`](https://docs.rs/assertables/10.0.0/assertables/assert_is_empty/assert_is_empty/)
- [`assert_len_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_len/assert_len_eq/)
- [`assert_count_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_count/assert_count_eq/)

Matching:

- [`assert_starts_with!(sequence, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_starts_with/)
- [`assert_ends_with!(sequence, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_ends_with/)
- [`assert_contains!(container, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_contains/)
- [`assert_is_match!(matcher, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_is_match/)
- [`assert_matches!(expr, pattern)`](https://docs.rs/assertables/10.0.0/assertables/assert_matches/)
- [`assert_email_address!(string)`](https://docs.rs/assertables/10.0.0/assertables/assert_email_address/)

Results:

- [`assert_ok!(result)`](https://docs.rs/assertables/10.0.0/assertables/assert_ok/)
- [`assert_ok_eq_x!(result, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_ok/assert_ok_eq_x/)
- [`assert_ok_ne_x!(result, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_ok/assert_ok_ne_x/)
- [`assert_err!(result)`](https://docs.rs/assertables/10.0.0/assertables/assert_err/)

Options:

- [`assert_some!(option)`](https://docs.rs/assertables/10.0.0/assertables/assert_some/)
- [`assert_some_eq_x!(option, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_some/assert_some_eq_x/)
- [`assert_some_ne_x!(option, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_some/assert_some_ne_x/)
- [`assert_none!(option)`](https://docs.rs/assertables/10.0.0/assertables/assert_none/)

Polls:

- [`assert_ready!(poll)`](https://docs.rs/assertables/10.0.0/assertables/assert_ready/)
- [`assert_ready_eq_x!(poll, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_ready/assert_ready_eq_x/)
- [`assert_ready_ne_x!(poll, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_ready/assert_ready_ne_x/)
- [`assert_pending!(poll)`](https://docs.rs/assertables/10.0.0/assertables/assert_pending/)

Readers:

- [`assert_fs_read_to_string_eq_x!(path, x`](https://docs.rs/assertables/10.0.0/assertables/assert_fs_read_to_string/)
- [`assert_io_read_to_string_eq_x!(reader, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_io_read_to_string/)

Iterators:

- [`assert_iter_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_eq/)
- [`assert_iter_ne!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_ne/)
- [`assert_iter_ge!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_ge/)
- [`assert_iter_gt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_gt/)
- [`assert_iter_le!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_le/)
- [`assert_iter_lt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_iter/assert_iter_lt/)

Sets:

- [`assert_set_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_set/assert_set_eq/)
- [`assert_set_ne!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_set/assert_set_ne/)
- [`assert_set_subset!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_set/assert_set_subset/)
- [`assert_set_superset!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_set/assert_set_superset/)

Bags:

- [`assert_bag_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_bag/assert_bag_eq/)
- [`assert_bag_ne!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_bag/assert_bag_ne/)
- [`assert_bag_subbag!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_bag/assert_bag_subbag/)
- [`assert_bag_superbag!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/assert_bag/assert_bag_superbag/)

Commands:

- [`assert_command_stdout_eq_x!(command, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_command/)
- [`assert_program_args_stdout_eq_x!(program, args, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_program_args/)

Status:

- [`assert_status_success!(a)`](https://docs.rs/assertables/10.0.0/assertables/assert_status/assert_status_success/)
- [`assert_status_code_value_eq_x!(a, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_status/assert_status_code_value_eq_x/)
- [`assert_status_code_value_ne_x!(a, x)`](https://docs.rs/assertables/10.0.0/assertables/assert_status/assert_status_code_value_ne_x/)
- [`assert_status_failure!(a)`](https://docs.rs/assertables/10.0.0/assertables/assert_status/assert_status_failure/)

Infix values:

- [`assert_infix!(a == b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a != b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a < b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a <= b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a > b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a >= b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)

Infix logic:

- [`assert_infix!(a & b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a | b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a ^ b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a && b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)
- [`assert_infix!(a || b)`](https://docs.rs/assertables/10.0.0/assertables/assert_infix/)

For a complete list of modules and macros, see the [docs](https://docs.rs/assertables/).

## Forms

All the macros have forms for an optional message:

- [`assert_gt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html) `// default message`
- [`assert_gt!(a, b, "your text")`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html) `// custom message`

All the macros have forms for different outcomes:

- [`assert_gt!(1, 2)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt.html) `// panic`
- [`assert_gt_as_result!(1, 2)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_gt_as_result.html) `return Result`
- [`debug_assert_gt!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/macro.debug_assert_gt.html) `// panic in debug mode`

Many of the macros have a form "compare left item to right item" that compares
items of the same kind, and a form "compare left item to right expression" that
compares one item to any arbitrary expression:

- [`assert_len_eq!(a, b)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq.html) `// a.len() = b.len()`
- [`assert_len_eq_x!(a, x)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok_eq_x.html) `// a.len() = x`

Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.

- [`let inner = assert_ok!(result)`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_ok.html)
- [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_fs_read_to_string_ne.html)
- [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](https://docs.rs/assertables/10.0.0/assertables/macro.assert_command_stdout_gt.html)

## Tracking

- Package: assertables-rust-crate
- Version: 10.0.0
- Created: 2021-03-30T15:47:49Z
- Updated: 2026-05-23T16:03:31Z
- License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
- Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
