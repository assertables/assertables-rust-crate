# Comparisons: assert2 and Assertables

<https://crates.io/crates/assert2>

The `assert2` crate provides an all-purpose `assert!(...)` and `check!(...)` macro inspired by Catch2, along with a `debug_assert!(...)` variant disabled in optimized builds, and a `let_assert!(...)` macro that lets you test a pattern while capturing parts of it.

`assert2` works by introspecting the expression you pass in: `assert!(a == b)` produces a richer failure message than `std::assert!(a == b)` because the macro can see the `==` operator and print both sides.

The `assertables` crate takes a different design approach: instead of one overloaded macro, it provides many specialized macros (`assert_eq!`, `assert_lt!`, `assert_matches!`, …). Both designs are valid; this guide shows how to migrate from `assert2`'s introspection style to assertables' explicit-name style.

The rest of this page is information about how you could migrate from `assert2` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
assert2 = "0.4.0"
```

With:

```toml
[dev-dependencies]
assertables = "10.0.0"
```

In your test files, replace:

```rust
use assert2::{assert, check, let_assert};
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `assert2` (old)                          | `assertables` (new)                     | Notes                                                              |
| ---------------------------------------- | --------------------------------------- | ------------------------------------------------------------------ |
| `assert!(a == b)`                        | `assert_eq!(a, b)`                      | Use the explicit-name macro for richer diagnostics.                |
| `assert!(a != b)`                        | `assert_ne!(a, b)`                      |                                                                    |
| `assert!(a < b)`                         | `assert_lt!(a, b)`                      |                                                                    |
| `assert!(a <= b)`                        | `assert_le!(a, b)`                      |                                                                    |
| `assert!(a > b)`                         | `assert_gt!(a, b)`                      |                                                                    |
| `assert!(a >= b)`                        | `assert_ge!(a, b)`                      |                                                                    |
| `assert!(matches!(x, pat))`              | `assert_matches!(x, pat)`               |                                                                    |
| `assert!(x.is_ok())`                     | `assert_ok!(x)`                         | Also returns the inner value.                                      |
| `assert!(x.is_err())`                    | `assert_err!(x)`                        |                                                                    |
| `assert!(x.is_some())`                   | `assert_some!(x)`                       | Also returns the inner value.                                      |
| `assert!(x.is_none())`                   | `assert_none!(x)`                       |                                                                    |
| `assert!(s.starts_with("a"))`            | `assert_starts_with!(s, "a")`           |                                                                    |
| `assert!(s.ends_with("z"))`              | `assert_ends_with!(s, "z")`             |                                                                    |
| `assert!(v.contains(&x))`                | `assert_contains!(v, &x)`               |                                                                    |
| `debug_assert!(a == b)`                  | `debug_assert_eq!(a, b)`                | Debug-only variant.                                                |
| `let_assert!(Ok(n) = result)`            | `let n = assert_ok!(result);`           | Wrapper macros return the inner value.                             |
| `let_assert!(Some(n) = option)`          | `let n = assert_some!(option);`         |                                                                    |
| `let_assert!(pat = expr)` (general)      | `assert_matches!(expr, pat); if let pat = expr { … }` | No direct equivalent; use a follow-up `if let` for bindings. |
| `check!(...)`                            | `*_as_result!(...)`                     | Use the logic form to accumulate failures without panicking.       |

## Examples

### Comparison expressions

Before:

```rust
use assert2::assert;

assert!(score == 100);
assert!(count > 0);
```

After:

```rust
use assertables::*;

assert_eq!(score, 100);
assert_gt!(count, 0);
```

### Pattern matching with capture

Before:

```rust
use assert2::let_assert;

let_assert!(Ok(value) = parse("42"));
assert!(value == 42);
```

After:

```rust
use assertables::*;

let value = assert_ok!(parse("42"));
assert_eq!(value, 42);
```

For general patterns where the wrapper macros don't apply:

Before:

```rust
use assert2::let_assert;

let_assert!(Custom { a, b } = make_value());
```

After:

```rust
use assertables::*;

let v = make_value();
assert_matches!(v, Custom { .. });
let Custom { a, b } = v else { unreachable!() };
```

### Non-panicking checks

`assert2::check!` records a failure but lets the test keep running. Assertables doesn't ship a built-in equivalent, but every macro has a `_as_result` form that returns `Result<_, String>`, so you can build the same pattern:

Before:

```rust
use assert2::check;

check!(a == b);
check!(c > 0);
```

After:

```rust
use assertables::*;

let mut errors: Vec<String> = Vec::new();
if let Err(e) = assert_eq_as_result!(a, b) { errors.push(e); }
if let Err(e) = assert_gt_as_result!(c, 0) { errors.push(e); }
assert!(errors.is_empty(), "{}", errors.join("\n"));
```

## Design rationale

| Concern                | `assert2`                                              | `assertables`                                          |
| ---------------------- | ------------------------------------------------------ | ------------------------------------------------------ |
| Surface API            | One overloaded `assert!` that parses expressions       | Many explicit-name macros (`assert_eq!`, `assert_lt!`) |
| Custom failure formats | Generated by macro introspection                       | Tailored per macro for each comparison kind            |
| Domain coverage        | General-purpose                                        | General + collections, IO, commands, polls, fs, …      |
| Result-returning form  | `check!`                                               | `*_as_result!` on every macro                          |

`assert2` is excellent when you want one macro to handle everything. `assertables` is excellent when you want the failure message and the call site to spell out exactly what kind of check is being performed.

## When to keep using `assert2`

If you rely on `let_assert!` for arbitrary pattern destructuring and find it ergonomic, keep `assert2`. The two crates can coexist — assertables won't conflict with `assert2::assert`, `assert2::check`, or `assert2::let_assert` as long as you import them explicitly rather than glob-importing both.

## Why migrate

- **More specialized assertions** — collections, ordering, ranges, approximations, fs, commands, polls, etc.
- **Richer failure output** — each macro formats its own message tuned for the comparison it performs.
- **Logic / panic / debug forms** for every macro.
- **No reliance on macro expression introspection** — what you see at the call site is what runs.

## See also

- [Assertables highlights](https://docs.rs/assertables/) — full module list
- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/)
- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/), [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/) — inner-value returning forms
- [`assert_infix`](https://docs.rs/assertables/latest/assertables/assert_infix/) — if you do want a single overloaded macro
