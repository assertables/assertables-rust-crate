//! Comparisons: examples of Assertables macros versus standard Rust macros.
//!
//! The Assertables macros can be useful for runtime validation, verification,
//! sanitization, reliability engineering, chaos engineering, and the like.
//!
//! This file has some very simple demonstrations of "before" code that uses
//! just the build-in Rust assert macros, then of "after" code that shows the
//! better Assertables macros.
//!
//! In most of these examples, the real value of the Assertables macros happens
//! when a test fails, because you get better failure messages, which help you
//! find bugs faster, and confirm fixes more easily.

use assertables::*;

#[test]
fn less_than() {
    let a = 1;
    let b = 2;

    // Standard Rust
    assert!(a < b);

    // Assertables using assert_lt
    assert_lt!(a, b);

    // Assertables using assert_infix
    assert_infix!(a < b);
}

/// Are two numbers near each other?
#[test]
fn numbers_nearness() {
    let a = 10;
    let b = 12;
    let delta = 2;

    // Standard Rust
    assert!(i32::abs(a - b) <= delta);

    // Assertables
    assert_in_delta!(a, b, delta);
}

#[test]
fn absolute_difference() {
    let a = 10;
    let b = 13;
    let delta = 4;

    // Standard Rust
    assert!(i32::abs(a - b) < delta);

    // Assertables using assert_lt
    assert_abs_diff_lt_x!(a, b, delta);
}

/// Verify a Result is Ok.
#[test]
fn verify_ok() {
    let a: Result<i8, i8> = Ok(1);

    // Standard Rust
    assert!(a.is_ok());

    // Assertables
    assert_ok!(a);
}

/// Compare a text file to a string.
#[test]
fn compare_text_file_to_string() {
    let path = "alfa.txt";
    let s = "alfa\n";

    // Standard Rust
    assert_eq!(::std::fs::read_to_string(path).unwrap(), s);

    // Assertables
    assert_fs_read_to_string_eq_x!(path, s);
}

/// Verify a command standard output stream contains a target string.
#[test]
fn verify_command_stdout_contains_text() {
    use std::process::Command;
    let mut command = Command::new("echo");
    command.args(["alfa"]);
    let containee = "fa";

    // Standard Rust
    assert!(
        String::from_utf8(command.output().unwrap().stdout)
            .unwrap()
            .contains(containee)
    );

    // Assertables
    assert_command_stdout_string_contains!(command, containee);
}
