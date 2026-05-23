# Changes

Changes are summarized here as highlights for recent major versions.

## Version 10.x

- 10.0.0 Create new Assertables organization for code repositories. Mirror to Codeberg.org, GitHub.com, GitLab.com. Upgrade Rust edition from 2021 to 2024. 

## Version 9.x

- 9.10.0 Add `assert_approx_eq_with_absolute_error_*` and `assert_approx_eq_with_relative_error_*`.

- 9.9.0 Add `assert_all_*` and `assert_any_*`.

- 9.8.6 Fix `debug_assert_*` edge cases with better test coverage.

- 9.8.5 Fix `debug_assert_*`.

- 9.8.4 Fix `assert_all` and `assert_any` code from `into_iter` into `iter`.

- 9.8.3 Fix `assert_matchs` and `assert_not_matches` with custom message.

- 9.8.2 Fix `assert_bag_impl_prep` and `assert_set_impl_prep` crate path.

- 9.8.1 Fix docs link version.

- 9.8.0 Add float macros: `assert_{f32,f64}_{eq,ne,lt,le,gt,ge}`.

- 9.7.1 Fix `assert_contains` for methods that require `Borrow`.

- 9.7.0 Add float macros: `assert_eq_f32` and `assert_eq_f64`.

- 9.6.3 Fix `assert_contains` for methods that require `Borrow` (backport from 9.7.1).

- 9.6.1-9.6.2 Fix pass-by-reference tests by adding more tests, more references, more loops.

- 9.6.0 Add `assert_email_address`

- 9.5.8 Fix `assert_contains` for methods that require `Borrow` (backport from 9.7.1.).

- 9.5.2-9.5.7 Fix idempotent access testing via once.

- 9.5.1 Fix assert_in_range for range syntax `a..=b`.

- 9.5.0 Add difference comparison macros: `assert_diff_*_x`, `assert_abs_diff_*_x`.

- 9.4.0 Add `assert_in`, `assert_in_range`.

- 9.3.0 Add status macros: `assert_status_success`, `assert_status_success_false`.
  ge

- 9.2.0 Add status comparison macros: `assert_status_code_value*`.

- 9.1.0 Add absolute difference macros: `assert_abs_diff*`.

- 9.0.0 Breaking change: many macros now return data upon success. Breaking change: ename macros from `assert_*_expr` into `assert_*_x`.

## Version 8.x

- Add matches macros: `assert_matches`, `assert_not_matches`.

- Add iterator macros: `assert_iter_all`, `assert_iter_any`, `assert_iter_eq`, etc.

- Add result macros: `assert_ok`, `assert_err`, etc.

- Add option macros: `assert_some`, `assert_none`, etc.

- Add poll macros: `assert_ready`, `assert_pending`, etc.

- Add list macros: `assert_len`, `assert_count`, `assert_is_empty`, etc.

- Add read macros: `assert_fs_read_to_string`, `assert_io_read_to_string`, etc.

- Add approx macros: `assert_approx_eq`, `assert_approx_ne`.

- Breaking change: change from `assert_read_to_string_*`. to `assert_io_read_to_string_*`.

## Version 7.x

- Add `assert_in_delta`, `assert_in_epsilon`.

- Add `assert_fn_*` macros with multiple arities.

- Add `cargo release` for optimized tagged releases.

## Version 6.x

- Add `assert_starts_with`, `assert_ends_with`, `assert_contains`, `assert_is_match`.

- Add `debug_assert_*` macros everywhere.

- Add `GPL-3.0` license.
