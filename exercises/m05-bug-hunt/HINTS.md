# Hints — m05-bug-hunt

Revealed one rung at a time by `./course hint m05-bug-hunt`. Debugging IS the
skill here — climb slowly, and try the method before the next rung.

## Hint 1 — let the tests triage for you

Run `cargo test -p m05-bug-hunt` and read the failing test *names*, not the code
yet. `first_component_of_empty_name_is_none`, `is_prefix_accepts_a_shorter_prefix`,
`common_prefix_len_when_one_is_a_full_prefix`, `split_components_recovers_each_value`
— each names a function and a case. Group the red by function: four functions, four
bugs. Two of them fail by *panicking* — start there, panics are the easiest to pin.

## Hint 2 — follow the panics to their exact line

`RUST_BACKTRACE=1 cargo test -p m05-bug-hunt`. Two tests panic with index-out-of-
bounds. The first `src/lib.rs` frame in each backtrace is the line. Ask: what index
is used, and what input makes it reach one past the end? An empty name has no
`[0]`. A name that is a *full* prefix of another makes a loop counter reach the
length itself.

## Hint 3 — the two silent (wrong-answer) bugs

`is_prefix` and `split_components` don't panic; they lie. For `is_prefix`, write the
truth table by hand: should a *shorter* prefix be accepted? The first `if` decides
who gets rejected — read it against the doc comment. For `split_components`, print
what it returns for a two-component name (`dbg!(split_components(&bytes));`) — you'll
get too few. The cursor advances at the bottom of the loop; by how much *should* it
move to reach the next element, versus how much does it?

## Hint 4 — the shapes (open only after real attempts)

- `first_component`: `name[0]` panics on an empty name — `slice::first` returns an
  `Option` instead, which is exactly the `None` the contract promises.
- `is_prefix`: a prefix has *no more* components than the name; the length guard's
  comparison is backwards.
- `common_prefix_len`: the loop guard lets `n` reach `a.len()` and then indexes
  `a[n]` — it should stop one earlier.
- `split_components`: after pushing a value, the cursor must jump past the whole
  element, not just past its header.
