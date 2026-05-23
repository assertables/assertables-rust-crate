# Comparisons: assert_approx_eq and Assertables

<https://crates.io/crates/assert_approx_eq>

The `assert_approx_eq` crate provides a single macro for asserting that two floating-point numbers are approximately equal within a delta tolerance.

The `assertables` crate provides the same `assert_approx_eq!` macro name on purpose, plus richer variants for absolute error, relative error, ranges, deltas, and epsilons — so migration is straightforward and unlocks more capability.

The rest of this page is information about how you could migrate from assert_approx_eq to assertables.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
assert_approx_eq = "1.1.0"
```

With:

```toml
[dev-dependencies]
assertables = "10.0.0"
```

In your test files, replace:

```rust
#[macro_use]
extern crate assert_approx_eq;
```

With:

```rust
use assertables::*;
```

## Macro migration table

| `assert_approx_eq` (old)               | `assertables` (new)                                      | Notes                                                  |
| -------------------------------------- | -------------------------------------------------------- | ------------------------------------------------------ |
| `assert_approx_eq!(a, b)`              | `assert_approx_eq!(a, b)`                                | Same name; default tolerance applies.                  |
| `assert_approx_eq!(a, b, delta)`       | `assert_in_delta!(a, b, delta)`                          | Use the explicit name to signal absolute-error intent. |
| `assert_approx_eq!(a, b, delta)`       | `assert_approx_eq_with_absolute_error!(a, b, delta)`     | Alternate explicit form.                               |
| *(no equivalent)*                      | `assert_approx_ne!(a, b)`                                | Inverse of `assert_approx_eq!`.                        |
| *(no equivalent)*                      | `assert_approx_ne_with_absolute_error!(a, b, delta)`     | Inverse, absolute-error form.                          |
| *(no equivalent)*                      | `assert_approx_eq_with_relative_error!(a, b, epsilon)`   | Relative-error form (scales with magnitude).           |
| *(no equivalent)*                      | `assert_approx_ne_with_relative_error!(a, b, epsilon)`   | Inverse, relative-error form.                          |
| *(no equivalent)*                      | `assert_in_epsilon!(a, b, epsilon)`                      | Epsilon-based nearness.                                |
| *(no equivalent)*                      | `assert_in_range!(a, range)`                             | Range-based nearness.                                  |

## Examples

### Default tolerance (1.0e-6)

Before:

```rust
assert_approx_eq!(1.0_f64, 1.0000001_f64);
```

After (same code, different crate):

```rust
use assertables::*;
assert_approx_eq!(1.0_f64, 1.0000001_f64);
```

### Custom absolute-error tolerance

Before:

```rust
assert_approx_eq!(1.0_f64, 1.5_f64, 1.0);
```

After (explicit, preferred):

```rust
use assertables::*;
assert_in_delta!(1.0_f64, 1.5_f64, 1.0);
```

Or, if you prefer the keyword `approx`:

```rust
use assertables::*;
assert_approx_eq_with_absolute_error!(1.0_f64, 1.5_f64, 1.0);
```

### Relative-error tolerance (new capability)

If `assert_approx_eq` is too brittle for values across many orders of magnitude, switch to relative error:

```rust
use assertables::*;
assert_approx_eq_with_relative_error!(1_000_000.0_f64, 1_000_001.0_f64, 1.0e-5);
```

## Why migrate

- **Same macro name** — `assert_approx_eq!(a, b)` works identically, so the most common form needs no code changes.
- **More expressive intent** — `assert_in_delta!`, `assert_approx_eq_with_absolute_error!`, and `assert_approx_eq_with_relative_error!` make tolerance semantics explicit at the call site.
- **More macros** — inverse `_ne` forms, range checks, epsilon checks.
- **Logic / panic / debug forms** — each macro comes in three flavors: `assert_*` (panics), `assert_*_as_result` (returns `Result`), and `debug_assert_*` (debug-only).
- **One dependency** — assertables covers a far wider range of testing needs, so you can drop other small assert crates as well.

## Design rationale

The `assert_approx_eq` crate overloads a single macro with an optional delta argument. Assertables prefers explicit macro names so readers can tell at a glance whether a comparison is using default tolerance, absolute error, relative error, or a range — without having to count macro arguments.

## See also

- [`assert_approx_eq` module docs](https://docs.rs/assertables/latest/assertables/assert_approx/)
- [`assert_in_delta`](https://docs.rs/assertables/latest/assertables/assert_in/assert_in_delta/)
- [`assert_in_epsilon`](https://docs.rs/assertables/latest/assertables/assert_in/assert_in_epsilon/)
- [`assert_approx_eq_with_absolute_error`](https://docs.rs/assertables/latest/assertables/assert_approx/assert_approx_eq_with_absolute_error/)
- [`assert_approx_eq_with_relative_error`](https://docs.rs/assertables/latest/assertables/assert_approx/assert_approx_eq_with_relative_error/)
