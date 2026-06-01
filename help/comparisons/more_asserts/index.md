# Comparisons: more_asserts and Assertables

<https://crates.io/crates/more_asserts>

<https://github.com/thomcc/rust-more-asserts>

The `more_asserts` crate provides assertion macros similar to the `{debug_,}assert_{eq,ne}` macros in the standard library: ordering checks (`assert_lt!`, `assert_le!`, `assert_gt!`, `assert_ge!`) and their debug variants.

The `assertables` crate provides every `more_asserts` macro under the same name, with identical syntax. Migration is a one-line dependency change.

The rest of this page is information about how you could migrate from `more_asserts` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
more-asserts = "0.3.1"
```

With:

```toml
[dev-dependencies]
assertables = "10.1.0"
```

In your test files, replace:

```rust
use more_asserts::*;
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `more_asserts` (old)           | `assertables` (new)             | Notes                  |
| ------------------------------ | ------------------------------- | ---------------------- |
| `assert_lt!(a, b)`             | `assert_lt!(a, b)`              | Identical.             |
| `assert_le!(a, b)`             | `assert_le!(a, b)`              | Identical.             |
| `assert_gt!(a, b)`             | `assert_gt!(a, b)`              | Identical.             |
| `assert_ge!(a, b)`             | `assert_ge!(a, b)`              | Identical.             |
| `debug_assert_lt!(a, b)`       | `debug_assert_lt!(a, b)`        | Identical.             |
| `debug_assert_le!(a, b)`       | `debug_assert_le!(a, b)`        | Identical.             |
| `debug_assert_gt!(a, b)`       | `debug_assert_gt!(a, b)`        | Identical.             |
| `debug_assert_ge!(a, b)`       | `debug_assert_ge!(a, b)`        | Identical.             |
| *(none)*                       | `assert_lt_as_result!(a, b)`    | Logic form — returns `Result<_, String>`. |
| *(none)*                       | `assert_le_as_result!(a, b)`    | Logic form.            |
| *(none)*                       | `assert_gt_as_result!(a, b)`    | Logic form.            |
| *(none)*                       | `assert_ge_as_result!(a, b)`    | Logic form.            |

## Examples

### Ordering

Before:

```rust
use more_asserts::*;

assert_lt!(1, 2);
assert_le!(1, 1);
assert_gt!(2, 1);
assert_ge!(1, 1);
```

After (same code, different crate):

```rust
use assertables::*;

assert_lt!(1, 2);
assert_le!(1, 1);
assert_gt!(2, 1);
assert_ge!(1, 1);
```

### Debug variants

Before:

```rust
use more_asserts::*;

debug_assert_lt!(a, b);
```

After:

```rust
use assertables::*;

debug_assert_lt!(a, b);
```

### Logic form for non-panic flows (new capability)

```rust
use assertables::*;

if let Err(message) = assert_lt_as_result!(a, b) {
    log::warn!("validation failed: {}", message);
}
```

## Why migrate

- **Same names, same syntax** — `assert_lt!`, `assert_le!`, `assert_gt!`, `assert_ge!`, plus debug forms.
- **Logic forms** — every comparison macro also has a `*_as_result!` variant that returns `Result<_, String>` for use outside of panicking test paths.
- **Many more macros** — collections, strings, regex, fs, io, commands, polls, approximations, ranges, options, results, infix, etc.
- **One dependency** for general testing instead of stacking several small assert crates.

## When to keep using `more_asserts`

`more_asserts` is tiny, focused, and stable. If you only need the four ordering macros and want the smallest possible dependency footprint, it remains a fine choice. The two crates can coexist — there is no name collision because the macros are exported under each crate's prelude and behave identically.

## Design rationale

Both crates expose the same surface for ordering, so migration is just a name change. The added value in `assertables` is breadth (many more macros) and the three-form architecture per concept (logic / panic / debug). That makes assertables easier to reuse outside of `cargo test` — for example, runtime validation, custom test wrappers, or chaos engineering scenarios.

## See also

- [`assert_lt`](https://docs.rs/assertables/latest/assertables/macro.assert_lt.html)
- [`assert_le`](https://docs.rs/assertables/latest/assertables/macro.assert_le.html)
- [`assert_gt`](https://docs.rs/assertables/latest/assertables/macro.assert_gt.html)
- [`assert_ge`](https://docs.rs/assertables/latest/assertables/macro.assert_ge.html)
- [`assert_infix`](https://docs.rs/assertables/latest/assertables/assert_infix/) — single macro that accepts the comparison operator
- [`assert_in_range`](https://docs.rs/assertables/latest/assertables/assert_in/assert_in_range/) — for testing membership in a range
