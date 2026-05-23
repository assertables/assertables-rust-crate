//! # Assertables: assert macros for better testing
//!
//! Assertables is a Rust crate that provides many assert macros
//! to improve your compile-time tests and run-time reliability.
//!
//! **[documentation](https://docs.rs/assertables/)**
//! •
//! **[source](https://github.com/assertables/assertables-rust-crate/)**
//! •
//! **[llms.txt](https://raw.githubusercontent.com/assertables/assertables-rust-crate/refs/heads/main/llms.txt)**
//! •
//! **[crate](https://crates.io/crates/assertables)**
//! •
//! **[email](mailto:joel@joelparkerhenderson.com)**
//!
//! ## Introduction
//!
//! The Assertables Rust crate provides many assert macros
//! that can help you develop, test, and debug.
//!
//! * Test values with
//!   [assert_lt](module@crate::assert_lt),
//!   [assert_gt](module@crate::assert_gt),
//!   [assert_in](module@crate::assert_in),
//!   […](https://docs.rs/assertables)
//! * Test groups with
//!   [assert_all](module@crate::assert_all),
//!   [assert_any](module@crate::assert_any),
//!   [assert_iter](module@crate::assert_iter),
//!   […](https://docs.rs/assertables)
//! * Test wrappers with
//!   [assert_ok](module@crate::assert_ok),
//!   [assert_some](module@crate::assert_some),
//!   [assert_ready](module@crate::assert_ready),
//!   […](https://docs.rs/assertables)
//! * Test matching with
//!   [assert_matches](module@crate::assert_matches),
//!   [assert_is_match](module@crate::assert_is_match),
//!   […](https://docs.rs/assertables)
//! * Test nearness with
//!   [assert_approx](module@crate::assert_approx),
//!   [assert_abs_diff](module@crate::assert_abs_diff),
//!   […](https://docs.rs/assertables/)
//! * Test programs with
//!   [assert_command](module@crate::assert_command),
//!   [assert_status](module@crate::assert_status),
//!   […](https://docs.rs/assertables)
//! * Many more below.
//!
//! To use this crate, add it to your file `Cargo.toml`:
//!
//! ```toml
//! assertables = "9.8.4"
//! ```
//!
//! Benefits:
//!
//! * You will write better tests to improve reliability and maintainability.
//! * You will handle more corner cases without needing to write custom code.
//! * You will troubleshoot faster because error messages show more detail.
//!
//! Learning:
//! [FAQ](https://github.com/assertables/assertables-rust-crate/tree/main/help/faq),
//! [docs](https://docs.rs/assertables/),
//! [examples](https://github.com/assertables/assertables-rust-crate/blob/main/tests/examples/),
//! [changes](https://github.com/assertables/assertables-rust-crate/tree/main/CHANGES.md),
//! [upgrades](https://github.com/assertables/assertables-rust-crate/tree/main/help/upgrades/upgrade-from-version-8-to-9),
//! [developing](https://github.com/assertables/assertables-rust-crate/tree/main/help/developing/).
//!
//! Comparisons:
//! [more_asserts](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/more_asserts),
//! [cool_asserts](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/cool_asserts),
//! [assert2](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/assert2),
//! [claims](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons/claims),
//! [etc.](https://github.com/assertables/assertables-rust-crate/tree/main/help/comparisons)
//!
//! ## Examples
//!
//! Examples with numbers:
//!
//! ```rust
//! # use assertables::*;
//! let i = 1;
//! assert_lt!(i, 2);
//! assert_in_range!(i, 0..2);
//! ```
//!
//! Examples with strings:
//!
//! ```rust
//! # use assertables::*;
//! # use regex::Regex;
//! let s = "hello";
//! assert_starts_with!(s, "h");
//! assert_is_match!(Regex::new(r"e.*o").unwrap(), s);
//! ```
//!
//! Examples with arrays:
//!
//! ```rust
//! # use assertables::*;
//! let a = [1, 2, 3];
//! assert_contains!(a, &2);
//! assert_all!(a.into_iter(), |i: i32| i < 4);
//! ```
//!
//! ## Highlights
//!
//! Values:
//!
//! * [`assert_eq!(a, b)`](module@crate::assert_eq)
//! * [`assert_ne!(a, b)`](module@crate::assert_ne)
//! * [`assert_ge!(a, b)`](module@crate::assert_ge)
//! * [`assert_gt!(a, b)`](module@crate::assert_gt)
//! * [`assert_le!(a, b)`](module@crate::assert_le)
//! * [`assert_lt!(a, b)`](module@crate::assert_lt)
//!
//! Floats:
//!
//! * [`assert_f32_eq!(a, b)`](module@crate::assert_f32::assert_f32_eq)
//! * [`assert_f64_eq!(a, b)`](module@crate::assert_f64::assert_f64_eq)
//!
//! Nearness:
//!
//! * [`assert_approx_eq!(a, b)`](module@crate::assert_approx::assert_approx_eq)
//! * [`assert_in_delta!(a, b, delta)`](module@crate::assert_in::assert_in_delta)
//! * [`assert_in_epsilon!(a, b, epsilon)`](module@crate::assert_in::assert_in_epsilon)
//! * [`assert_in_range!(a, range)`](module@crate::assert_in::assert_in_range)
//! * [`assert_diff_eq_x!(a, b, x)`](module@crate::assert_diff::assert_diff_eq_x)
//! * [`assert_abs_diff_eq_x!(a, b, x)`](module@crate::assert_abs_diff::assert_abs_diff_eq_x)
//!
//! Groups:
//!
//! * [`assert_all!(group, predicate)`](module@crate::assert_all)
//! * [`assert_any!(group, predicate)`](module@crate::assert_any)
//! * [`assert_is_empty!(group)`](module@crate::assert_is_empty::assert_is_empty)
//! * [`assert_len_eq!(a, b)`](module@crate::assert_len::assert_len_eq)
//! * [`assert_count_eq!(a, b)`](module@crate::assert_count::assert_count_eq)
//!
//! Matching:
//!
//! * [`assert_starts_with!(sequence, x)`](module@crate::assert_starts_with)
//! * [`assert_ends_with!(sequence, x)`](module@crate::assert_ends_with)
//! * [`assert_contains!(container, x)`](module@crate::assert_contains)
//! * [`assert_is_match!(matcher, x)`](module@crate::assert_is_match)
//! * [`assert_matches!(expr, pattern)`](module@crate::assert_matches)
//! * [`assert_email_address!(string)`](module@crate::assert_email_address)
//!
//! Results:
//!
//! * [`assert_ok!(result)`](module@crate::assert_ok)
//! * [`assert_ok_eq_x!(result, x)`](module@crate::assert_ok::assert_ok_eq_x)
//! * [`assert_ok_ne_x!(result, x)`](module@crate::assert_ok::assert_ok_ne_x)
//! * [`assert_err!(result)`](module@crate::assert_err)
//!
//! Options:
//!
//! * [`assert_some!(option)`](module@crate::assert_some)
//! * [`assert_some_eq_x!(option, x)`](module@crate::assert_some::assert_some_eq_x)
//! * [`assert_some_ne_x!(option, x)`](module@crate::assert_some::assert_some_ne_x)
//! * [`assert_none!(option)`](module@crate::assert_none)
//!
//! Polls:
//!
//! * [`assert_ready!(poll)`](module@crate::assert_ready)
//! * [`assert_ready_eq_x!(poll, x)`](module@crate::assert_ready::assert_ready_eq_x)
//! * [`assert_ready_ne_x!(poll, x)`](module@crate::assert_ready::assert_ready_ne_x)
//! * [`assert_pending!(poll)`](module@crate::assert_pending)
//!
//! Iterators:
//!
//! * [`assert_iter_eq!(a, b)`](module@crate::assert_iter::assert_iter_eq)
//! * [`assert_iter_ne!(a, b)`](module@crate::assert_iter::assert_iter_ne)
//! * [`assert_iter_ge!(a, b)`](module@crate::assert_iter::assert_iter_ge)
//! * [`assert_iter_gt!(a, b)`](module@crate::assert_iter::assert_iter_gt)
//! * [`assert_iter_le!(a, b)`](module@crate::assert_iter::assert_iter_le)
//! * [`assert_iter_lt!(a, b)`](module@crate::assert_iter::assert_iter_lt)
//!
//! Sets:
//!
//! * [`assert_set_eq!(a, b)`](module@crate::assert_set::assert_set_eq)
//! * [`assert_set_ne!(a, b)`](module@crate::assert_set::assert_set_ne)
//! * [`assert_set_subset!(a, b)`](module@crate::assert_set::assert_set_subset)
//! * [`assert_set_superset!(a, b)`](module@crate::assert_set::assert_set_superset)
//!
//! Bags:
//!
//! * [`assert_bag_eq!(a, b)`](module@crate::assert_bag::assert_bag_eq)
//! * [`assert_bag_ne!(a, b)`](module@crate::assert_bag::assert_bag_ne)
//! * [`assert_bag_subbag!(a, b)`](module@crate::assert_bag::assert_bag_subbag)
//! * [`assert_bag_superbag!(a, b)`](module@crate::assert_bag::assert_bag_superbag)
//!
//! Readers:
//!
//! * [`assert_fs_read_to_string_eq_x!(path, x)`](module@crate::assert_fs_read_to_string)
//! * [`assert_io_read_to_string_eq_x!(reader, x)`](module@crate::assert_io_read_to_string)
//!
//! Commands:
//!
//! * [`assert_command_stdout_eq_x!(command, x)`](module@crate::assert_command) `// command ⇒ stdout == x`
//! * [`assert_program_args_stdout_eq_x!(program, args, x)`](module@crate::assert_program_args) `// program.args ⇒ stdout == x`
//!
//! Status:
//!
//! * [`assert_status_success!(a)`](module@crate::assert_status::assert_status_success)
//! * [`assert_status_code_value_eq_x!(a, x)`](module@crate::assert_status::assert_status_code_value_eq_x)
//! * [`assert_status_code_value_ne_x!(a, x)`](module@crate::assert_status::assert_status_code_value_ne_x)
//! * [`assert_status_failure!(a)`](module@crate::assert_status::assert_status_failure)
//!
//! Infix values:
//!
//! * [`assert_infix!(a == b)`](module@crate::assert_infix)
//! * [`assert_infix!(a != b)`](module@crate::assert_infix)
//! * [`assert_infix!(a < b)`](module@crate::assert_infix)
//! * [`assert_infix!(a <= b)`](module@crate::assert_infix)
//! * [`assert_infix!(a > b)`](module@crate::assert_infix)
//! * [`assert_infix!(a >= b)`](module@crate::assert_infix)
//!
//! Infix logic:
//!
//! * [`assert_infix!(a & b)`](module@crate::assert_infix)
//! * [`assert_infix!(a | b)`](module@crate::assert_infix)
//! * [`assert_infix!(a ^ b)`](module@crate::assert_infix)
//! * [`assert_infix!(a && b)`](module@crate::assert_infix)
//! * [`assert_infix!(a || b)`](module@crate::assert_infix)
//!
//! For a complete list of modules and macros, see the
//! [docs](https://docs.rs/assertables/).
//!
//! ## Forms
//!
//! The Assertables macros have a variety of forms to help you write the tests that matter most to you.
//!
//! All the macros have forms for an optional message:
//!
//! * [`assert_gt!(a, b)`](module@crate::assert_gt) `// default message`
//! * [`assert_gt!(a, b, "your text")`](module@crate::assert_gt) `// custom message`
//!
//! All the macros have forms for different outcomes:
//!
//! * [`assert_gt!(1, 2)`](macro@crate::assert_gt) `// panic`
//! * [`assert_gt_as_result!(1, 2)`](macro@crate::assert_gt_as_result) `// return Result`
//! * [`debug_assert_gt!(1, 2)`](macro@crate::debug_assert_gt) `// panic in debug mode`
//!
//! Many of the macros have a form "compare left item to right item" that compares
//! items of the same kind, and a form "compare left item to right expression" that
//! compares one item to any arbitrary expression:
//!
//! * [`assert_len_eq!(a, b)`](module@crate::assert_ok::assert_ok_eq) `// a.len() = b.len()`
//! * [`assert_len_eq_x!(a, x)`](module@crate::assert_ok::assert_ok_eq_x)) `// a.len() = x`
//!
//! Many of the macros has a "success return", which means the macro returns data that you can optionally use for more testing.
//!
//! * [`let inner = assert_ok!(result)`](module@crate::assert_ok::assert_ok)
//! * [`let string = assert_fs_read_to_string_ne!("alfa.txt", "")`](module@crate::assert_fs_read_to_string::assert_fs_read_to_string_ne)
//! * [`let stdout = assert_command_stdout_gt!("ls", vec![b' '])`](module@crate::assert_command::assert_command_stdout_gt)
//!
//! ## Tracking
//!
//! * Package: assertables-rust-crate
//! * Version: 10.0.0
//! * Created: 2021-03-30T15:47:49Z
//! * Updated: 2026-05-23T16:03:31Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

// Each macro group lives in `src/assert_X/` with a `mod.rs` that re-exports
// the same-named macro file (`assert_X/assert_X.rs`); this mirrors the public
// macro names and is intentional.
#![allow(clippy::module_inception)]

// Assert truth
pub mod assert; // (in addition to what's provided by Rust `std`)

// Assert value comparison
pub mod assert_eq; // (in addition to what's provided by Rust `std`)
pub mod assert_ge;
pub mod assert_gt;
pub mod assert_le;
pub mod assert_lt;
pub mod assert_ne; // (in addition to what's provided by Rust `std`)

// Assert value comparison for floats
pub mod assert_eq_f32;
pub mod assert_eq_f64;
pub mod assert_f32;
pub mod assert_f64;

// Assert difference
pub mod assert_abs_diff;
pub mod assert_approx;
pub mod assert_diff;
pub mod assert_in;

// Assert all/any
pub mod assert_all;
pub mod assert_any;

// Infix
pub mod assert_infix;

// Matching
pub mod assert_contains;
pub mod assert_count;
pub mod assert_email_address;
pub mod assert_ends_with;
pub mod assert_is_empty;
pub mod assert_is_match;
pub mod assert_len;
pub mod assert_matches;
pub mod assert_starts_with;

// For Result Ok & Err
pub mod assert_err;
pub mod assert_ok;
pub mod assert_result; // Deprecated

// For Option Some & None
pub mod assert_none;
pub mod assert_option;
pub mod assert_some; // Deprecated

// For Poll Ready & Pending
pub mod assert_pending;
pub mod assert_poll;
pub mod assert_ready; // Deprecated

// For collections
pub mod assert_bag;
pub mod assert_iter;
pub mod assert_set;

// For functions
pub mod assert_fn;
pub mod assert_fn_err;
pub mod assert_fn_ok;

// For reading
pub mod assert_fs_read_to_string;
pub mod assert_io_read_to_string;

// For externals
pub mod assert_command;
pub mod assert_program_args;
pub mod assert_status;

// Misc
pub mod assert_success;
