# Comparisons: claims and related crates

<https://crates.io/crates/claims>

There are various Rust crates that provide assert macros:

* [`claims`](https://crates.io/crates/claims)

* [`claim`](https://crates.io/crates/claim) which was forked to create `claims`.

Each of these crates are doing the same surface-level thing as Assertables, by adding new assert macros.

Assertables has two major differences:

* More assertions

* More logic leverage


## Assertables has more assertions

If there's an assertion from any of those crates that you would like us to add to Assertables, then let us know, or create a merge request, and we'll add it.

<table>

<thead>

<tr>
<th>Category</th>
<th>Assertables</th>
<th>claims</th>
</tr>

</thead>

<tbody>

<tr>
<td>Version</td>
<td>8.4</td>
<td>0.7</td>
</tr>

<tr>
<td>Updated</td>
<td>2024</td>
<td>2022</td>
</tr>

<tr>
<td>
Compare
</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/macro.assert_lt.html">assert_lt</a><br>
<a href="https://docs.rs/assertables/latest/assertables/macro.assert_le.html">assert_le</a><br>
<a href="https://docs.rs/assertables/latest/assertables/macro.assert_gt.html">assert_gt</a><br>
<a href="https://docs.rs/assertables/latest/assertables/macro.assert_ge.html">assert_ge</a>
</td>
<td>
<a href="https://docs.rs/claims/latest/claims/macro.assert_lt.html">assert_lt</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ok.html">assert_le</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ok.html">assert_gt</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ok.html">assert_ge</a>
</td>
</tr>

<tr>
<td>Nearness</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_in_delta">assert_in_delta</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_in_epsilon">assert_in_epsilon</a>
</td>
<td>
</td>
</tr>

<tr>
<td>
Match
</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_is_match">assert_is_match</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_not_match">assert_not_match</a>
</td>
<td>
<a href="https://docs.rs/claims/latest/claims/macro.assert_matches.html">assert_matches</a><br>
&nbsp;
</td>
</tr>

<tr>
<td>Contains</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_contains">assert_contains</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_not_contains">assert_not_contains</a>
</td>
<td>
</td>
</tr>

<tr>
<td>Starts With</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_starts_with">assert_starts_with</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_not_starts_with">assert_not_starts_with</a>
</td>
<td>
</td>
</tr>

<tr>
<td>Ends With</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_ends_with">assert_ends_with</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_not_ends_with">assert_not_ends_with</a>
</td>
<td>
</td>
</tr>

<tr>
<td>Result </td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/macro.assert_err.html">assert_ok</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_result/assert_ok_eq">assert_ok_eq</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_result/assert_ok_ne">assert_ok_ne</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_result/assert_err">assert_err</a>
</td>
<td>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ok.html">assert_ok</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ok_eq.html">assert_ok_eq</a><br>
-<br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_err.html">assert_err</a>
</td>
</tr>

<tr>
<td>Option</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_option/assert_some">assert_some</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_option/assert_some_eq">assert_some_eq</a>&nbsp;(eta&nbsp;v8.5)<br>
<a href="https://docs.rs/assertables/latest/assertables/assert_option/assert_some_ne">assert_some_ne</a>&nbsp;(eta&nbsp;v8.5)<br>
<a href="https://docs.rs/assertables/latest/assertables/assert_option/assert_none">assert_none</a>
</td>
<td>
<a href="https://docs.rs/claims/latest/claims/macro.assert_some.html">assert_some</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_some_eq.html">assert_some_eq</a><br>
-<br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_none.html">assert_none</a>
</td>
</tr>

<tr>
<td>Poll</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_ready">assert_ready</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_ready_eq">assert_ready_eq</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_ready_ne">assert_ready_ne</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_ready_ok">assert_ready_ok</a>(eta 8.7)<br>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_ready_err">assert_ready_err</a>(eta 8.7)<br>
<a href="https://docs.rs/assertables/latest/assertables/assert_poll/assert_pending">assert_pending</a></td>
</td>
<td>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ready.html">assert_ready</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ready_eq.html">assert_ready_eq</a><br>
-<br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ready_ok.html">assert_ready_ok</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_ready_err.html">assert_ready_err</a><br>
<a href="https://docs.rs/claims/latest/claims/macro.assert_pending.html">assert_pending</a><br>
</td>
</tr>

<tr>
<td>Readers</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_fs_read_to_string">assert_fs_read_to_string_*</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_io_read_to_string">assert_io_read_to_string_*</a>
<td>
</td>
</tr>

<tr>
<td>Commands</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_command">assert_command_*</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_program_args">assert_program_args_*</a></td>
<td>
</td>
</tr>

<tr>
<td>Collections</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_set">assert_set_*</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_bag">assert_bag_*</a>
</td>
<td>
</td>
</tr>

<tr>
<td>Functions</td>
<td>
<a href="https://docs.rs/assertables/latest/assertables/assert_fn">assert_fn_*</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_fn_ok">assert_fn_ok_*</a><br>
<a href="https://docs.rs/assertables/latest/assertables/assert_fn_err">assert_fn_err_*</a></td>
<td>
</td>
</tr>

</tbody>
</table>


## Assertables has more logic leverage

Assertables makes deliberate design decisions to implement each concept as three macros:

* The logic macro. This returns a Result and is the most important of the three macros.

* The panic macro. This is what a typical cargo test uses.

* The debug macro. This is what a typical runtime debug config uses.

Assertables puts all the logic in the logic macro, and developers can use the same logic anywhere they want, even for totally different purposes:

* Runtime production analysis using a Result. This works without triggering a panic, and without needing any debug config.

* Chaos engineering where logic macros can detect dirty input, or missing files, or bad data. This well for UI interactions with users, with fallback files, and with data sanitization.

* Custom macro wrapping where developers prefer to write their own syntax for their tests. This works well because the new syntax is just a surface-level addition, and can delegate to the logic macro.


## Compare a macro with various implementations

You can see the difference for yourself, such as in these two source code files:

* [`assert_gt`](https://github.com/assertables/assertables-rust-crate/blob/main/src/assert_gt.rs) by Assertables.

* [`assert_gt`](https://crates.io/crates/rust-claim) by rust-claim.

You can see `assertables` provides three macros:

* The logic macro is `assert_gt_as_result`. It does the comparison and returns a Result and possible error message.

* The panic macro is `assert_gt`. It is a thin wrapper.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.

You can see `claim` provides two macros:

* The panic macro is `assert_gt`. It contains the logic, which means the logic can't be reused independently.

* The debug macro is `debug_assert_gt`. It is a thin wrapper.
