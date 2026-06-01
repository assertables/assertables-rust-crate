# Comparisons: cool_asserts and Assertables

<https://crates.io/crates/cool_asserts>

The `cool_asserts` crate is a collection of useful assertions and utilities for writing tests in Rust. Its headline macro is `assert_matches!`, which supports rich pattern-bound blocks for follow-up assertions.

The `assertables` crate provides equivalents for the everyday `cool_asserts` macros (`assert_matches!` in particular). `assertables` does not currently ship a built-in `assert_panics!`, but the migration patterns below cover that gap.

The rest of this page is information about how you could migrate from `cool_asserts` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
cool_asserts = "2.0.4"
```

With:

```toml
[dev-dependencies]
assertables = "10.1.0"
```

In your test files, replace:

```rust
use cool_asserts::{assert_matches, assert_panics, assertion_failure};
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `cool_asserts` (old)                              | `assertables` (new)                                          | Notes                                                       |
| ------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------- |
| `assert_matches!(value, pattern)`                 | `assert_matches!(value, pattern)`                            | Identical syntax.                                           |
| `assert_matches!(value, pattern if cond)`         | `assert_matches!(value, pattern if cond)`                    | Guard clauses are supported.                                |
| `assert_matches!(value, pattern => { … })`        | `assert_matches!(value, pattern); if let pattern = value { … }` | No trailing-block form; use a follow-up `if let`.   |
| *(none)*                                          | `assert_not_matches!(value, pattern)`                        | Inverse — value does **not** match the pattern.             |
| `assert_panics!(expr)`                            | See "Panic checking" below.                                  | No direct equivalent.                                       |
| `assertion_failure!("msg")`                       | `panic!("msg")` or `assert!(false, "msg")`                   | Use plain `panic!` / `assert!`.                             |
| `get_panic_message(&payload)`                     | *(use standard `std::panic` API)*                            | Not provided.                                               |

## Examples

### Matching

Before:

```rust
use cool_asserts::assert_matches;

let v: Result<i32, &str> = Ok(42);
assert_matches!(v, Ok(_));
```

After:

```rust
use assertables::*;

let v: Result<i32, &str> = Ok(42);
assert_matches!(v, Ok(_));
```

### Matching with follow-up assertions

Before:

```rust
use cool_asserts::assert_matches;

let v: Result<(i32, &str), ()> = Ok((42, "hello"));
assert_matches!(v, Ok((n, s)) => {
    assert_eq!(n, 42);
    assert_eq!(s, "hello");
});
```

After:

```rust
use assertables::*;

let v: Result<(i32, &str), ()> = Ok((42, "hello"));
assert_matches!(v, Ok(_));
let Ok((n, s)) = v else { unreachable!() };
assert_eq!(n, 42);
assert_eq!(s, "hello");
```

When the wrapper happens to be `Result` or `Option`, the dedicated `assert_ok!` / `assert_some!` macros are even simpler because they return the inner value:

```rust
use assertables::*;

let v: Result<(i32, &str), ()> = Ok((42, "hello"));
let (n, s) = assert_ok!(v);
assert_eq!(n, 42);
assert_eq!(s, "hello");
```

## Panic checking

`cool_asserts::assert_panics!` lets you assert that an expression panics, and optionally inspect the panic payload. Assertables does not currently ship a built-in equivalent. The idiomatic Rust replacement is `std::panic::catch_unwind`:

Before:

```rust
use cool_asserts::assert_panics;

assert_panics!(panic!("boom"));

assert_panics!(do_thing(), includes("specific text"));
```

After:

```rust
let result = std::panic::catch_unwind(|| {
    panic!("boom");
});
assert!(result.is_err());

let result = std::panic::catch_unwind(|| do_thing());
let payload = result.expect_err("expected a panic");
let message = payload
    .downcast_ref::<&str>().map(|s| s.to_string())
    .or_else(|| payload.downcast_ref::<String>().cloned())
    .unwrap_or_default();
assertables::assert_contains!(message, "specific text");
```

If panic-assertion ergonomics are central to your test suite, it is fine to keep `cool_asserts` alongside `assertables` — they don't conflict.

## Why migrate

- **One dependency** for matching plus dozens of other assertion patterns (comparisons, collections, strings, regex, fs, io, commands, polls, etc.).
- **Inverse macro** `assert_not_matches!` is built in.
- **Logic / panic / debug forms** for every macro.
- **Same surface syntax** for the common `assert_matches!` cases.

## When to keep using `cool_asserts`

If your tests rely heavily on `assert_panics!` with its payload-matching mini-DSL, `cool_asserts` is a reasonable focused dependency to keep — and it composes fine with `assertables`.

## See also

- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/)
- [`assert_not_matches`](https://docs.rs/assertables/latest/assertables/assert_not_matches/)
- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/), [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/) — for wrapper extraction
- [`assert_contains`](https://docs.rs/assertables/latest/assertables/assert_contains/) — useful when inspecting panic payload strings
