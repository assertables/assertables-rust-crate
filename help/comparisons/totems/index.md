# Comparisons: totems and Assertables

<https://crates.io/crates/totems>

<https://github.com/ObliqueMotion/totems-rs>

The `totems` crate provides extra assertion macros: ordering, `Result`/`Option` checks, collection membership, indexed positional checks (`assert_nth!`), and quantifier checks (`assert_all!`, `assert_any!`).

The `assertables` crate provides direct equivalents for nearly every `totems` macro, plus many more categories.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
totems = "0.2.7"
```

With:

```toml
[dev-dependencies]
assertables = "10.1.0"
```

In your test files, replace:

```rust
use totems::*;
```

With:

```rust
use assertables::*;
```

## Macro migration table

### Ordering

| `totems` (old)            | `assertables` (new)        | Notes      |
| ------------------------- | -------------------------- | ---------- |
| `assert_lt!(x, y)`        | `assert_lt!(x, y)`         | Identical. |
| `assert_le!(x, y)`        | `assert_le!(x, y)`         | Identical. |
| `assert_gt!(x, y)`        | `assert_gt!(x, y)`         | Identical. |
| `assert_ge!(x, y)`        | `assert_ge!(x, y)`         | Identical. |

### Result and Option

| `totems` (old)                | `assertables` (new)              | Notes                                |
| ----------------------------- | -------------------------------- | ------------------------------------ |
| `assert_ok!(r)`               | `assert_ok!(r)`                  | Returns the inner `T`.               |
| `assert_ok!(r, |t| t == 42)`  | `assert_ok_eq_x!(r, 42)` *(common case)* | Or extract and assert manually.|
| `assert_err!(r)`              | `assert_err!(r)`                 | Returns the inner `E`.               |
| `assert_err!(r, |e| …)`       | `let e = assert_err!(r); assert!(…)` | Extract then assert.             |
| `assert_some!(o)`             | `assert_some!(o)`                | Returns the inner `T`.               |
| `assert_some!(o, |t| t == 42)`| `assert_some_eq_x!(o, 42)` *(common case)* | Or extract and assert manually. |
| `assert_none!(o)`             | `assert_none!(o)`                |                                      |

### Collections

| `totems` (old)                       | `assertables` (new)                                                | Notes                                |
| ------------------------------------ | ------------------------------------------------------------------ | ------------------------------------ |
| `assert_contains!(c, x)`             | `assert_contains!(c, &x)`                                          | Borrow the needle when appropriate.  |
| `assert_all!(c, |x| pred)`           | `assert_all!(c.iter(), |x| pred)`                                  | Pass an iterator.                    |
| `assert_any!(c, |x| pred)`           | `assert_any!(c.iter(), |x| pred)`                                  | Pass an iterator.                    |
| `assert_nth!(c, n, expected)`        | `assert_eq!(c[n], expected)` or `assert_eq!(c.iter().nth(n), Some(&expected))` | No dedicated `assert_nth!`. |
| `assert_0th!(c, expected)` … `assert_15th!(c, expected)` | `assert_eq!(c[N], expected)`                          | The indexed shortcuts (0th–15th) are not provided. |

### Quantifiers with extracted lambdas

`totems` quantifier macros take a closure over an element. `assertables::assert_all!` and `assert_any!` work the same way but expect you to pass the iterator explicitly:

Before:

```rust
use totems::*;

let v = vec![1, 2, 3];
assert_all!(v, |x| *x > 0);
assert_any!(v, |x| *x == 2);
```

After:

```rust
use assertables::*;

let v = vec![1, 2, 3];
assert_all!(v.iter(), |x: &i32| *x > 0);
assert_any!(v.iter(), |x: &i32| *x == 2);
```

## Examples

### Result and Option

Before:

```rust
use totems::*;

let r: Result<i32, &str> = Ok(7);
assert_ok!(r);
assert_ok!(r, |t| *t == 7);

let o: Option<i32> = Some(7);
assert_some!(o);
assert_some!(o, |t| *t == 7);
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

### Contains

Before:

```rust
use totems::*;

let v = vec![1, 2, 3];
assert_contains!(v, 2);
```

After:

```rust
use assertables::*;

let v = vec![1, 2, 3];
assert_contains!(v, &2);
```

### Indexed positional checks

`totems` ships shortcut macros `assert_0th!` through `assert_15th!`. Assertables does not provide these; use ordinary indexing or `.nth(...)`:

```rust
use assertables::*;

let v = vec!["a", "b", "c"];
assert_eq!(v[0], "a");                          // direct indexing
assert_eq!(v.iter().nth(2), Some(&"c"));        // for iterators
```

### Quantifiers

Before:

```rust
use totems::*;

let v = vec![1, 2, 3];
assert_all!(v, |x| *x > 0);
assert_any!(v, |x| *x == 2);
```

After:

```rust
use assertables::*;

let v = vec![1, 2, 3];
assert_all!(v.iter(), |x: &i32| *x > 0);
assert_any!(v.iter(), |x: &i32| *x == 2);
```

## Why migrate

- **Same names** for nearly every macro.
- **Active maintenance** — `totems` has not been updated since 2019.
- **More macros** — collections (sets, bags, iterators), strings (`assert_starts_with!`, `assert_ends_with!`, `assert_contains!`), regex (`assert_is_match!`), approximations (`assert_in_delta!`, `assert_in_epsilon!`), fs/io, subprocesses, polls, and more.
- **Three forms per macro** — `assert_*` (panic), `assert_*_as_result` (`Result<_, String>`), `debug_assert_*` (debug-only).
- **Wrapper macros return the inner value** — `let n = assert_ok!(r)` gives you the `Ok` value directly, which removes the need for the `|t| …` closure form.

## Trade-offs

- **No `assert_nth!` family** — replace with `c[n]` or `c.iter().nth(n)`.
- **`assert_all!` / `assert_any!` take iterators** — call `.iter()` (or pass any iterator) instead of the bare collection.

## See also

- [`assert_lt`](https://docs.rs/assertables/latest/assertables/macro.assert_lt.html), [`assert_le`](https://docs.rs/assertables/latest/assertables/macro.assert_le.html), [`assert_gt`](https://docs.rs/assertables/latest/assertables/macro.assert_gt.html), [`assert_ge`](https://docs.rs/assertables/latest/assertables/macro.assert_ge.html)
- [`assert_ok`](https://docs.rs/assertables/latest/assertables/assert_ok/), [`assert_err`](https://docs.rs/assertables/latest/assertables/assert_err/)
- [`assert_some`](https://docs.rs/assertables/latest/assertables/assert_some/), [`assert_none`](https://docs.rs/assertables/latest/assertables/assert_none/)
- [`assert_contains`](https://docs.rs/assertables/latest/assertables/assert_contains/)
- [`assert_all`](https://docs.rs/assertables/latest/assertables/assert_all/), [`assert_any`](https://docs.rs/assertables/latest/assertables/assert_any/)
