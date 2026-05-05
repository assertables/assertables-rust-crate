# Frequently asked questions (FAQ)

Developers ask many questions about Assertables, Rust testing in general, macros in general, and more.


## What are the top benefits?

1. You can write better tests to improve reliability and maintainability.

2. You can handle more corner cases without needing to write custom code.

3. You can troubleshoot faster because error messages show specifics.


## What are the top features?

1. Easy to use: everything is well-documented with runnable examples.

2. More forms: for results, for solo/pair, for success return, etc.

3. Zero overhead: if you don't use a macro, then it's never compiled.


## Why use Assertables instead of just standard Rust macros?

Because Assertables gives you more kinds of tests, stronger semantics, clearer
naming, and better error messages.

Because Assertables has macro forms that do success returns, which enables
you to do subsequent tests. This is especially helpful for asserts that use
Result or Option, as well as for asserts that make external calls such as
reading files or running commands.

Because Assertables has macro forms that do result returns, rather than panic.
This enables production runtime uses such as for validations, verifications,
resilience monitoring, chaos engineering, and more.


## What's an example of more kinds of tests?

Suppose you want to assert that a text file has anything other than "TODO"
Assertables:

```rust
assert_fs_read_to_string_ne!("example.txt", "TODO");
```

Standard Rust:

```rust
assert!(fs::read_to_string("example.txt") == String::from("TODO"));
```

## What's an example of a success return?

Suppose you also want to assert the string length is at least 10.

Assertables:

```rust
let string = assert_fs_read_to_string_ne!("example.txt", "TODO");
assert_len_ge!(string, 10);
```

Standard Rust:

```rust
let string = fs::read_to_string("alfa.txt");
assert!(string != String::from("TODO"));
assert!(string.len() >= 10);
```

## What's an example of a result return?

Suppose you also want to assert during production, without a panic, then trace the result.

Assertables:

```rust
let result = assert_fs_read_to_string_ne_as_result!("alfa.txt", "TODO");
trace!(result);
```

Standard Rust:

```rust
let result = panic::catch_unwind(|| {
   assert!(fs::read_to_string("alfa.txt") != String::from(""));
});
trace!(result);
```


## Why use Assertables instead of more_asserts, cool_asserts, assert2, claims, etc.?

Because Assertables provides all the macros (and more) in all those crates (and more).

Because Assertables provides more specifics in the error messages, to help you troubleshoot.

Because Assertables provides the additional forms for success returns and result returns.


## Are there license reasons to use Assertables instead of other assertion crates?

Yes, for some developers and organizations. Assertables provides more license
choices than each of those crates.

Assertables provides Apache, MIT, GPL, BSD, and custom licenses for custom
needs. See the LICENSE file for specifics.


## Are there assertion crates that are good to use with Assertables?

Yes. There are assertion crates that provide even more functionality, especially
for more-sophisticated needs.

1. The crate `static_assertions` provides static code analysis, even before running `cargo test`.

2. The crate `assert_matches` provides matching macros that have even more functionality than Assertables macros `assert_matches` and `assert_not_matches`.

3. The crate `approx` provides floating point approximation macros that provide even more functionality than Assertables macros `assert_approx_eq`, `assert_in_delta`, `assert_in_epsilon`, etc.


## How do I request a feature, or report a bug, or provide feedback?

If you use GitHub, then you can create a GitHub issue, or create a GitHub pull request:

* [https://github.com/assertables/assertables](https://github.com/assertables/assertables)

If you prefer email, then you can email the maintainer:

* [joel@joelparkerhenderson.com](mailto:joel@joelparkerhenderson.com)

Constructive suggestions are always welcome.
