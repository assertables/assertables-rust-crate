# Comparisons: assert_matches and Assertables

<https://crates.io/crates/assert_matches>

The `assert_matches` crate provides the `assert_matches!` macro for asserting that an expression matches a given pattern, with optional guards, bindings, and trailing blocks.

The `assertables` crate provides a deliberately-compatible `assert_matches!` macro plus a complementary `assert_not_matches!`. For the common cases the syntax is identical, so migration is mostly a dependency swap.

The rest of this page is information about how you could migrate from `assert_matches` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
assert_matches = "1.5.0"
```

With:

```toml
[dev-dependencies]
assertables = "10.0.0"
```

In your test files, replace:

```rust
use assert_matches::assert_matches;
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `assert_matches` (old)                                 | `assertables` (new)                       | Notes                                          |
| ------------------------------------------------------ | ----------------------------------------- | ---------------------------------------------- |
| `assert_matches!(expr, pat)`                           | `assert_matches!(expr, pat)`              | Identical syntax.                              |
| `assert_matches!(expr, pat if cond)`                   | `assert_matches!(expr, pat if cond)`      | Guard clauses are supported.                   |
| `assert_matches!(expr, pat, "msg {}", arg)`            | `assert_matches!(expr, pat, "msg {}", arg)` | Custom message form is supported.            |
| `debug_assert_matches!(expr, pat)`                     | `debug_assert_matches!(expr, pat)`        | Debug-only variant.                            |
| *(no equivalent)*                                      | `assert_not_matches!(expr, pat)`          | Inverse — asserts the expression does **not** match the pattern. |
| *(no equivalent)*                                      | `assert_matches_as_result!(expr, pat)`    | Logic form — returns `Result<(), String>`.     |

## Examples

### Basic pattern matching

Before:

```rust
use assert_matches::assert_matches;

let value = Ok::<_, ()>(42);
assert_matches!(value, Ok(_));
```

After:

```rust
use assertables::*;

let value = Ok::<_, ()>(42);
assert_matches!(value, Ok(_));
```

### Guard clauses

```rust
use assertables::*;

let value = Some(5);
assert_matches!(value, Some(n) if n > 0);
```

### Inverse (new capability)

```rust
use assertables::*;

let value: Result<i32, &str> = Err("oops");
assert_not_matches!(value, Ok(_));
```

### Logic form for non-panic flows (new capability)

```rust
use assertables::*;

let value = Some(1);
let result = assert_matches_as_result!(value, Some(_));
assert!(result.is_ok());
```

## Capturing bindings: `assert_matches!` vs `let` pattern

The `assert_matches` crate supports a trailing block form for binding extraction:

```rust
// assert_matches crate
assert_matches!(value, Ok(n) => {
    assert_eq!(n, 42);
});
```

In assertables, the idiomatic equivalent is to combine `assert_matches!` with Rust's normal pattern bindings:

```rust
use assertables::*;

let value = Ok::<_, ()>(42);
assert_matches!(value, Ok(_));
if let Ok(n) = value {
    assert_eq!(n, 42);
}
```

Or, when you only need to extract from a wrapper type, the dedicated wrapper macros return the inner value:

```rust
use assertables::*;

let value: Result<i32, &str> = Ok(42);
let inner: i32 = assert_ok!(value); // assertables wrapper macros return the inner value
assert_eq!(inner, 42);
```

## When to keep using `assert_matches`

If your codebase relies heavily on the trailing-block `=> { ... }` capture syntax, the `assert_matches` crate is a great focused dependency and we recommend trying it. It is small, stable, and complementary to assertables.

You can use both crates together — assertables is `assert_*` macros, and `assert_matches` is one macro that focuses purely on pattern matching ergonomics.

## Why migrate

- **One dependency** for matching plus dozens of other assertion patterns (`assert_ok!`, `assert_some!`, `assert_starts_with!`, `assert_in_delta!`, …).
- **Inverse macro** `assert_not_matches!` is built in.
- **Logic / panic / debug forms** for every macro.
- **Same surface syntax** for the common matching cases.

## See also

- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/)
- [`assert_not_matches`](https://docs.rs/assertables/latest/assertables/assert_not_matches/)
- [`assert_is_match`](https://docs.rs/assertables/latest/assertables/assert_is_match/) — regex-based matching
- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/), [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/) — wrapper-aware alternatives
