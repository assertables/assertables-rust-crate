# Comparisons: galvanic-assert and Assertables

<https://crates.io/crates/galvanic-assert>

<https://github.com/mindsbackyard/galvanic-assert>

The `galvanic-assert` crate provides new assertion macros (`assert_that!`, `expect_that!`, `get_expectation_for!`) based on matching predicates ("matchers"). It is part of the broader `galvanic` test framework.

Every assertion has the form `assert_that!(value, matcher);`, where `matcher` is something like `less_than(5)`, `eq(42)`, `contains_in_order([1, 2])`, or `is_variant!(Result::Ok)`.

The `assertables` crate takes a different architectural approach: each kind of comparison gets its own explicit macro name. This guide shows how to translate `galvanic-assert` matchers into `assertables` macros.

The rest of this page is information about how you could migrate from `galvanic-assert` to `assertables`.

## Migrate your `Cargo.toml`

Replace:

```toml
[dev-dependencies]
galvanic-assert = "0.8.7"
```

With:

```toml
[dev-dependencies]
assertables = "10.1.0"
```

In your test files, replace:

```rust
#[macro_use] extern crate galvanic_assert;
use galvanic_assert::matchers::*;
```

With:

```rust
use assertables::*;
```

## Macro / matcher migration table

### Equality and ordering

| `galvanic-assert`                              | `assertables`                       |
| ---------------------------------------------- | ----------------------------------- |
| `assert_that!(x, eq(y))`                       | `assert_eq!(x, y)`                  |
| `assert_that!(x, not(eq(y)))`                  | `assert_ne!(x, y)`                  |
| `assert_that!(x, lt(y))` / `less_than(y)`      | `assert_lt!(x, y)`                  |
| `assert_that!(x, leq(y))` / `less_than_or_equal_to(y)` | `assert_le!(x, y)`         |
| `assert_that!(x, gt(y))` / `greater_than(y)`   | `assert_gt!(x, y)`                  |
| `assert_that!(x, geq(y))` / `greater_than_or_equal_to(y)` | `assert_ge!(x, y)`     |
| `assert_that!(x, close_to(y, delta))`          | `assert_in_delta!(x, y, delta)`     |

### Boolean and identity

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `assert_that!(b, is(true))`                      | `assert!(b)`                               |
| `assert_that!(b, is(false))`                     | `assert!(!b)`                              |
| `assert_that!(x, same_object_as(&y))`            | `assert!(std::ptr::eq(&x, &y))`            |

### Collections

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `assert_that!(v, contains(x))`                   | `assert_contains!(v, &x)`                  |
| `assert_that!(v, contains_in_order(items))`      | `assert_iter_eq!(v.iter(), items.iter())`  |
| `assert_that!(v, has_length(n))`                 | `assert_len_eq_x!(v, n)`                   |
| `assert_that!(v, is_empty())`                    | `assert_is_empty!(v)`                      |
| `assert_that!(v, all_elements_satisfy(pred))`    | `assert_all!(v.iter(), pred)`              |
| `assert_that!(v, any_element_satisfies(pred))`   | `assert_any!(v.iter(), pred)`              |

### Variants and patterns

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `assert_that!(x, is_variant!(Result::Ok))`       | `assert_ok!(x)` or `assert_matches!(x, Ok(_))` |
| `assert_that!(x, is_variant!(Result::Err))`      | `assert_err!(x)` or `assert_matches!(x, Err(_))` |
| `assert_that!(x, is_variant!(Option::Some))`     | `assert_some!(x)` or `assert_matches!(x, Some(_))` |
| `assert_that!(x, is_variant!(Option::None))`     | `assert_none!(x)`                          |
| `assert_that!(x, matches_pattern!(pat))`         | `assert_matches!(x, pat)`                  |

### Strings

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `assert_that!(s, contains_string("sub"))`        | `assert_contains!(s, "sub")`               |
| `assert_that!(s, starts_with("a"))`              | `assert_starts_with!(s, "a")`              |
| `assert_that!(s, ends_with("z"))`                | `assert_ends_with!(s, "z")`                |
| `assert_that!(s, matches_regex("…"))`            | `assert_is_match!(Regex::new("…").unwrap(), s)` |

### Composition

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `assert_that!(x, all_of!(eq(y), lt(z)))`         | `assert_eq!(x, y); assert_lt!(x, z);`      |
| `assert_that!(x, any_of!(eq(y), eq(z)))`         | `assert!(x == y \|\| x == z)`              |
| `assert_that!(x, not(matcher))`                  | Use the macro's inverse (`assert_ne!`, `assert_not_matches!`, `assert_not_contains!`, `assert_not_starts_with!`, `assert_not_ends_with!`, …). |

### Non-panicking assertions

| `galvanic-assert`                                | `assertables`                              |
| ------------------------------------------------ | ------------------------------------------ |
| `expect_that!(x, matcher)`                       | `*_as_result!(...)` — every macro has a `_as_result` form that returns `Result<_, String>`. |
| `get_expectation_for!(x, matcher)`               | `*_as_result!(...)` — capture the `Result` instead.   |

## Examples

### Equality

Before:

```rust
#[macro_use] extern crate galvanic_assert;
use galvanic_assert::matchers::*;

assert_that!(42, eq(42));
assert_that!(1, lt(2));
```

After:

```rust
use assertables::*;

assert_eq!(42, 42);
assert_lt!(1, 2);
```

### Result and Option variants

Before:

```rust
assert_that!(value, is_variant!(Result::Ok));
```

After (preferred — also returns the inner value):

```rust
use assertables::*;

let inner = assert_ok!(value);
```

Or, if you prefer the matcher style:

```rust
use assertables::*;

assert_matches!(value, Ok(_));
```

### Composed matchers

Before:

```rust
assert_that!(v, all_of!(has_length(3), contains(1)));
```

After:

```rust
use assertables::*;

assert_len_eq_x!(v, 3);
assert_contains!(v, &1);
```

### Approximations

Before:

```rust
assert_that!(measured, close_to(expected, 0.001));
```

After:

```rust
use assertables::*;

assert_in_delta!(measured, expected, 0.001);
```

### Non-panicking expectations

Before:

```rust
expect_that!(x, eq(y));
expect_that!(x, lt(z));
```

After:

```rust
use assertables::*;

let mut errors: Vec<String> = Vec::new();
if let Err(e) = assert_eq_as_result!(x, y) { errors.push(e); }
if let Err(e) = assert_lt_as_result!(x, z) { errors.push(e); }
assert!(errors.is_empty(), "{}", errors.join("\n"));
```

## Design rationale

| Concern         | `galvanic-assert`                                            | `assertables`                                            |
| --------------- | ------------------------------------------------------------ | -------------------------------------------------------- |
| Surface API     | One `assert_that!` macro plus a matcher DSL                  | Many explicit-name macros                                |
| Extensibility   | Write a new `Matcher` type to extend                         | Write a new `*_as_result!` macro and thin wrappers       |
| Failure output  | Generic, generated by the matcher                            | Tailored per macro                                       |
| Composition     | `all_of!`, `any_of!`, `not(...)`                             | Separate macros + inverse `_ne` / `not_*` forms          |
| Architecture    | Layered on top of the `galvanic` test framework              | Standalone — each module stands on its own               |

`galvanic-assert` is well-suited to teams that want to build their own matcher vocabulary on top of a unified `assert_that!` macro. `assertables` is well-suited to teams that want a large library of ready-made, explicitly-named macros without building a matcher layer.

## When to keep using `galvanic-assert`

If you already have a substantial body of custom `Matcher` implementations, the cost of migrating their semantics may exceed the value. In that case `galvanic-assert` and `assertables` coexist cleanly — there is no namespace collision.

## See also

- [Assertables module index](https://docs.rs/assertables/) — full list of macro modules
- [`assert_matches`](https://docs.rs/assertables/latest/assertables/assert_matches/)
- [`assert_in_delta`](https://docs.rs/assertables/latest/assertables/assert_in/assert_in_delta/)
- [`assert_iter_eq`](https://docs.rs/assertables/latest/assertables/assert_iter/assert_iter_eq/)
- [`assert_all`](https://docs.rs/assertables/latest/assertables/assert_all/), [`assert_any`](https://docs.rs/assertables/latest/assertables/assert_any/)
