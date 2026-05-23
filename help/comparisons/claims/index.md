# Comparisons: claims and Assertables

<https://crates.io/crates/claims>

The `claims` crate is a community-maintained fork of [`claim`](../claim). It provides assertion macros for `Result`, `Option`, `Poll`, and value comparisons.

The `assertables` crate provides direct equivalents for every `claims` macro, plus many more categories (collections, strings, regex, fs, io, commands, ranges, approximations, infix, …). Migration is mostly a dependency swap and a small number of `_x` suffix additions.

The rest of this page is information about how you could migrate from `claims` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
claims = "0.8.0"
```

With:

```toml
[dev-dependencies]
assertables = "10.0.0"
```

In your test files, replace:

```rust
use claims::*;
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `claims` (old)                                   | `assertables` (new)                          | Notes                                                       |
| ------------------------------------------------ | -------------------------------------------- | ----------------------------------------------------------- |
| **Comparisons**                                  |                                              |                                                             |
| `assert_lt!(a, b)`                               | `assert_lt!(a, b)`                           | Identical.                                                  |
| `assert_le!(a, b)`                               | `assert_le!(a, b)`                           | Identical.                                                  |
| `assert_gt!(a, b)`                               | `assert_gt!(a, b)`                           | Identical.                                                  |
| `assert_ge!(a, b)`                               | `assert_ge!(a, b)`                           | Identical.                                                  |
| **Result**                                       |                                              |                                                             |
| `assert_ok!(result)`                             | `assert_ok!(result)`                         | Returns the inner `T`.                                      |
| `assert_ok_eq!(result, expected)`                | `assert_ok_eq_x!(result, expected)`          | `_x` form compares the inner `T` to a value.                |
| *(none)*                                         | `assert_ok_eq!(a, b)`                        | Compare two `Result::Ok` values.                            |
| *(none)*                                         | `assert_ok_ne_x!(result, expected)` / `assert_ok_ne!(a, b)` | Inverse equality.                            |
| `assert_err!(result)`                            | `assert_err!(result)`                        | Returns the inner `E`.                                      |
| *(none)*                                         | `assert_err_eq_x!`, `assert_err_eq!`, `assert_err_ne_x!`, `assert_err_ne!` | Inner-error comparisons.       |
| **Option**                                       |                                              |                                                             |
| `assert_some!(option)`                           | `assert_some!(option)`                       | Returns the inner `T`.                                      |
| `assert_some_eq!(option, expected)`              | `assert_some_eq_x!(option, expected)`        | `_x` form compares inner `T` to a value.                    |
| *(none)*                                         | `assert_some_ne_x!`, `assert_some_eq!`, `assert_some_ne!` | Inverse and same-kind comparisons.            |
| `assert_none!(option)`                           | `assert_none!(option)`                       |                                                             |
| **Poll**                                         |                                              |                                                             |
| `assert_ready!(poll)`                            | `assert_ready!(poll)`                        | Returns the inner `T`.                                      |
| `assert_ready_eq!(poll, x)`                      | `assert_ready_eq_x!(poll, x)`                | `_x` form.                                                  |
| `assert_ready_ok!(poll)`                         | `assert_ready_ok!(poll)`                     |                                                             |
| `assert_ready_err!(poll)`                        | `assert_ready_err!(poll)`                    |                                                             |
| `assert_pending!(poll)`                          | `assert_pending!(poll)`                      |                                                             |
| **Matching**                                     |                                              |                                                             |
| `assert_matches!(expr, pat)`                     | `assert_matches!(expr, pat)`                 |                                                             |
| **Debug variants**                               |                                              |                                                             |
| `debug_assert_*!(...)`                           | `debug_assert_*!(...)`                       | Every macro has a debug form.                               |

## Examples

### Result and Option

Before:

```rust
use claims::*;

let r: Result<i32, &str> = Ok(7);
assert_ok!(r);
assert_ok_eq!(r, 7);

let o: Option<i32> = Some(7);
assert_some!(o);
assert_some_eq!(o, 7);
```

After:

```rust
use assertables::*;

let r: Result<i32, &str> = Ok(7);
let inner = assert_ok!(r);              // returns 7
assert_ok_eq_x!(r, 7);

let o: Option<i32> = Some(7);
let inner = assert_some!(o);            // returns 7
assert_some_eq_x!(o, 7);
```

### Poll

Before:

```rust
use claims::*;
use std::task::Poll;

let p: Poll<Result<i32, ()>> = Poll::Ready(Ok(1));
assert_ready!(p);
assert_ready_ok!(p);
```

After:

```rust
use assertables::*;
use std::task::Poll;

let p: Poll<Result<i32, ()>> = Poll::Ready(Ok(1));
assert_ready!(p);
assert_ready_ok!(p);
```

### Matching

Before:

```rust
use claims::assert_matches;

let v: Result<i32, &str> = Ok(42);
assert_matches!(v, Ok(_));
```

After:

```rust
use assertables::*;

let v: Result<i32, &str> = Ok(42);
assert_matches!(v, Ok(_));
```

## Naming difference: `_eq` vs `_eq_x`

`claims` writes `assert_ok_eq!(result, value)`. Assertables uses two distinct names:

- `assert_ok_eq!(a, b)` — compares **two** `Result::Ok` values.
- `assert_ok_eq_x!(result, x)` — compares the inner `T` of a single `Result::Ok` to an **arbitrary expression** `x`.

When migrating, change every `claims::assert_ok_eq!(r, v)` to `assertables::assert_ok_eq_x!(r, v)`. The same `_x` suffix rule applies to `assert_some_eq`, `assert_err_eq`, `assert_ready_eq`, and their inverse `_ne` forms.

## Beyond what `claims` offers

Once migrated, you also gain:

- **Inverse forms** for every wrapper macro: `assert_ok_ne_x!`, `assert_err_ne_x!`, `assert_some_ne_x!`, `assert_ready_ne_x!`, …
- **Logic forms** — every macro has `*_as_result!` that returns `Result<_, String>`.
- **Strings and regex** — `assert_starts_with!`, `assert_ends_with!`, `assert_contains!`, `assert_is_match!`, `assert_email_address!`, …
- **Collections** — `assert_set_*`, `assert_bag_*`, `assert_iter_*`, `assert_len_*`, `assert_count_*`, `assert_is_empty!`, `assert_all!`, `assert_any!`.
- **Approximations and ranges** — `assert_approx_*`, `assert_in_delta!`, `assert_in_epsilon!`, `assert_in_range!`, `assert_abs_diff_*`, `assert_diff_*`.
- **Files and IO** — `assert_fs_read_to_string_*`, `assert_io_read_to_string_*`.
- **Subprocesses** — `assert_command_*`, `assert_program_args_*`, `assert_status_*`.
- **Functions** — `assert_fn_*`, `assert_fn_ok_*`, `assert_fn_err_*`.
- **Infix** — `assert_infix!(a == b)`, `assert_infix!(a && b)`, etc.

## Design rationale

Assertables makes a deliberate design decision to implement each concept as three macros:

- The **logic macro** (`*_as_result`) — returns a `Result<_, String>`. This is the most important of the three because it can be reused outside of tests for runtime validation, custom test wrappers, chaos engineering, etc.
- The **panic macro** (`assert_*`) — a thin wrapper used by typical `cargo test`.
- The **debug macro** (`debug_assert_*`) — a thin wrapper used by typical runtime debug configs.

`claims` provides two: the panic macro and the debug macro. The logic lives inside the panic macro, which makes it hard to reuse outside of the panicking test path.

## See also

- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/) module
- [`assert_err`](https://docs.rs/assertables/latest/assertables/assert_err/) module
- [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/) module
- [`assert_none`](https://docs.rs/assertables/latest/assertables/assert_none/) module
- [`assert_ready`](https://docs.rs/assertables/latest/assertables/assert_ready/) module
- [`assert_pending`](https://docs.rs/assertables/latest/assertables/assert_pending/) module
- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/) module
- [`claim` comparison guide](../claim) — the original crate that `claims` forked from
