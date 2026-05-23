# Comparisons: claim and Assertables

<https://crates.io/crates/claim>

The `claim` crate provides assertion macros for `Result`, `Option`, `Poll`, and value comparisons. It was last published in 2021 and has since been forked as [`claims`](https://crates.io/crates/claims).

If you are starting fresh or migrating today, we recommend `assertables`, which provides direct equivalents for every `claim` macro plus many more.

The rest of this page is information about how you could migrate from `claim` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
claim = "0.5.0"
```

With:

```toml
[dev-dependencies]
assertables = "10.0.0"
```

In your test files, replace:

```rust
use claim::*;
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `claim` (old)                     | `assertables` (new)                          | Notes                                         |
| --------------------------------- | -------------------------------------------- | --------------------------------------------- |
| `assert_lt!(a, b)`                | `assert_lt!(a, b)`                           | Identical.                                    |
| `assert_le!(a, b)`                | `assert_le!(a, b)`                           | Identical.                                    |
| `assert_gt!(a, b)`                | `assert_gt!(a, b)`                           | Identical.                                    |
| `assert_ge!(a, b)`                | `assert_ge!(a, b)`                           | Identical.                                    |
| `assert_ok!(result)`              | `assert_ok!(result)`                         | Returns the inner `T`.                        |
| `assert_ok_eq!(result, expected)` | `assert_ok_eq_x!(result, expected)`          | `_x` form compares the inner `T` to a value.  |
| *(no equivalent)*                 | `assert_ok_ne_x!(result, expected)`          | Inverse — inner value is not equal to `x`.    |
| `assert_err!(result)`             | `assert_err!(result)`                        | Returns the inner `E`.                        |
| `assert_some!(option)`            | `assert_some!(option)`                       | Returns the inner `T`.                        |
| `assert_some_eq!(option, expected)` | `assert_some_eq_x!(option, expected)`      | `_x` form compares inner `T` to a value.      |
| *(no equivalent)*                 | `assert_some_ne_x!(option, expected)`        | Inverse.                                      |
| `assert_none!(option)`            | `assert_none!(option)`                       |                                               |
| `assert_ready!(poll)`             | `assert_ready!(poll)`                        | Returns the inner `T`.                        |
| `assert_ready_eq!(poll, x)`       | `assert_ready_eq_x!(poll, x)`                | `_x` form compares inner `T` to a value.      |
| `assert_ready_ok!(poll)`          | `assert_ready_ok!(poll)`                     |                                               |
| `assert_ready_err!(poll)`         | `assert_ready_err!(poll)`                    |                                               |
| `assert_pending!(poll)`           | `assert_pending!(poll)`                      |                                               |
| `assert_matches!(expr, pat)`      | `assert_matches!(expr, pat)`                 |                                               |
| `debug_assert_*!(...)`            | `debug_assert_*!(...)`                       | Every macro has a debug form.                 |

## Examples

### Result and Option

Before:

```rust
use claim::*;

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
let inner = assert_ok!(r);          // returns 7
assert_ok_eq_x!(r, 7);

let o: Option<i32> = Some(7);
let inner = assert_some!(o);        // returns 7
assert_some_eq_x!(o, 7);
```

### Poll

Before:

```rust
use claim::*;
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

### Comparisons

Before / After (identical):

```rust
use assertables::*;

assert_lt!(1, 2);
assert_le!(1, 1);
assert_gt!(2, 1);
assert_ge!(1, 1);
```

## Why migrate

- **Same names for the common cases** — `assert_ok!`, `assert_err!`, `assert_some!`, `assert_none!`, `assert_ready!`, `assert_pending!`, `assert_lt!`, `assert_le!`, `assert_gt!`, `assert_ge!`, `assert_matches!`, `debug_assert_*!`.
- **Active maintenance** — `claim` has not been updated since 2021.
- **Three forms per macro** — `assert_*` (panics), `assert_*_as_result` (returns `Result`), `debug_assert_*` (debug-only).
- **More macros** — collections, iterators, strings, regex, fs, io, commands, ranges, approximations, infix, etc.
- **More inverse forms** — `assert_ok_ne_x!`, `assert_some_ne_x!`, `assert_ready_ne_x!`, etc.

## Naming difference: `_eq` vs `_eq_x`

`claim` writes `assert_ok_eq!(result, value)`. Assertables prefers `assert_ok_eq_x!(result, value)` to make the asymmetry explicit: the left side is wrapped, the right side is a bare value. The plain `assert_ok_eq!(a, b)` form in assertables compares **two** `Result`s — both wrapped. The `_x` suffix marks "compare to an arbitrary expression".

## Compare with `claims`

`claims` is a community fork of `claim` and is more actively maintained. See the [`claims` comparison guide](../claims) for a fuller table including macros that `claims` adds beyond `claim`. Both crates have the same first-level migration paths to assertables.

## See also

- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/) module
- [`assert_err`](https://docs.rs/assertables/latest/assertables/assert_err/) module
- [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/) module
- [`assert_none`](https://docs.rs/assertables/latest/assertables/assert_none/) module
- [`assert_ready`](https://docs.rs/assertables/latest/assertables/assert_ready/) module
- [`assert_pending`](https://docs.rs/assertables/latest/assertables/assert_pending/) module
- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/) module
