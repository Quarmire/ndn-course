# m06-name-api — designing a type that feels native

**Module:** M6 · Names and API design · **Species:** template-assisted (oracle-graded)
**You write:** `NameComponent::cmp`, `Name::cmp`, `Name::has_prefix`, `Display for Name`, `From<&str> for Name`.

Up to now you've written functions. This module is about designing a *type* —
choosing its traits so it behaves the way the language expects, and so a
`BTreeSet<Name>` or a sorted routing table just works. The struct and its derives
are given; you supply the five behaviors that decide what a name *means*.

Your ordering is graded against the **real `ndn-foundation-types`** as an oracle:
for a spread of inputs, your `cmp` must return the same `Ordering` the production
type does. Match the spec, or the oracle names the exact pair where you diverge.

## The canonical order (the whole point)

An NDN name component is a `(type, bytes)` pair. Its canonical order is:

1. by **type** (the `u64`), then
2. by **length** of the value, then
3. by the value **bytes**, lexicographically.

Length comes *before* content. This is the trap: if you `#[derive(Ord)]` on
`{ typ, value }`, you get type-then-*lexicographic-bytes*, which is **not** the
same. Consider two type-8 components, `[0xFF]` and `[0x00, 0x00]`:

- a derive compares content first: `0xFF > 0x00`, so `[0xFF]` sorts *after*.
- canonical compares length first: `1 < 2`, so `[0xFF]` sorts *before*.

That's why `Ord` here is hand-written, not derived — and why the oracle test
`length_orders_before_content` exists to catch exactly this mistake.

A `Name` orders **lexicographically over its components** (component by component,
the shorter name losing on a shared prefix) — using the component order above.

## What you write

- `NameComponent::cmp` — the three-level canonical order. `Ordering::then_with`
  chains the tiers cleanly.
- `Name::cmp` — lexicographic over the component `Vec` (the standard library
  compares two slices/vecs in one call).
- `Name::has_prefix(&self, prefix)` — a longer prefix is never a prefix; otherwise
  every leading component must match.
- `Display for Name` — `/comp/comp`, root is `/` (the readable subset of an NDN URI).
- `From<&str> for Name` — parse `/a/b/c`, skipping empty segments.

The struct already `#[derive(PartialEq, Eq, Hash)]`. Note the contract you're
keeping: `Eq`, `Ord`, and `Hash` must agree — two values that are `Eq` must compare
`Equal` and hash the same. Structural equality gives you that for free *because*
your `cmp` uses exactly the fields equality does. Break that alignment and a
`BTreeMap` and a `HashMap` will disagree about what's in them.

## Done means

`./course check m06-name-api` green: the oracle tests (component and name ordering
match the real type), the behavior tests (`has_prefix`, `Display`, `From`), clippy
`-D warnings`, and fmt. Then `./course submit`.

## The API review (bring to your submission)

Open `../ndn-rs/crates/core/ndn-foundation-types/src/name.rs` and skim the real
`Name`. In your journal, answer with reference to the
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/): pick two
choices it makes — e.g. `SmallVec<[_; 8]>` instead of `Vec`, the `append_*` builder
methods returning `Self`, `pub(crate)` on the field, the `as_segment`/`as_version`
accessors — and say what each buys, and what it would cost to change it later
(that's the *semver* lens: which parts are promises to callers?).

## Rules of engagement

No `unsafe`. You may not `#[derive(Ord, PartialOrd)]` your way out — the ordering is
the lesson. Keep the public surface small: expose what a caller needs, keep the
rest private (the field is already private for a reason — ask yourself which).
