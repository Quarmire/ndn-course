# m04-name-view — borrow, don't own

**Module:** M4 · Ownership and memory beyond trivial · **Species:** from-scratch + signature-locked
**You build:** `longest_component`, `NameView::iter`, `NameComponents::next`, and `common_prefix_len`.

The last three modules, you owned your bytes — `Vec<u8>` in, `Vec<u8>` out. That's
the safe default, and it copies. This module is about the *other* discipline: when
the data already lives somewhere, hand out a **borrow** instead of a copy, and let
the lifetime prove it's safe. It's the skill that makes zero-copy parsers possible
— and the one that feels hardest until it clicks.

## The clone-ectomy, made concrete

A name is a run of TLV components. The naive representation copies:

```rust
struct OwnedName { components: Vec<Vec<u8>> } // one heap allocation PER component,
                                              // and the bytes now live twice.
```

You'll build the version that copies nothing:

```rust
struct NameView<'a> { bytes: &'a [u8] }       // owns nothing; every component it
                                              // yields borrows from `bytes`.
```

`size_of::<NameView>()` is 16 — a pointer and a length, a *fat pointer*, no matter
how long the name is. `size_of::<OwnedName>()` is 24 for the `Vec` alone, plus a
heap allocation per component. Measure both (M1 taught you how) and put the two
numbers in your journal.

## Contract

```rust
// Part A — warmup. The `&'a` return means you can't clone-and-return.
pub fn longest_component<'a>(components: &[&'a [u8]]) -> Option<&'a [u8]>;

// Part B — the view.
pub struct NameView<'a> { /* bytes: &'a [u8] */ }
impl<'a> NameView<'a> {
    pub fn new(bytes: &'a [u8]) -> Self;               // provided
    pub fn iter(&self) -> NameComponents<'a>;          // YOU
    pub fn get(&self, i: usize) -> Option<&'a [u8]>;   // provided (uses iter)
    pub fn len(&self) -> usize;                        // provided (uses iter)
    pub fn is_empty(&self) -> bool;                    // provided
}
pub struct NameComponents<'a> { /* rest: &'a [u8] */ }
impl<'a> Iterator for NameComponents<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<&'a [u8]>;            // YOU — the heart of it
}

// Part C — use the view.
pub fn common_prefix_len(a: &NameView, b: &NameView) -> usize;   // YOU
```

- **`next`** decodes one component (`TYPE ‖ LENGTH ‖ VALUE`, using the provided
  `decode_varu64`) off the front of `rest`, returns its VALUE **borrowed for `'a`**,
  and advances `rest`. It returns `None` to stop — at the end, or on a malformed
  element (this view is lenient: it stops rather than panicking). This is M3's
  `read`, re-shaped as an iterator: the returned slice must borrow from the
  original bytes, never from `&mut self`.
- **`iter`** just hands the iterator the bytes to walk.
- **`common_prefix_len`** counts the leading components two names share — build it
  by *using* your iterators, allocating nothing.
- **`longest_component`** is the warmup: the return type is a borrow, so the
  compiler will not let you build a `Vec` and return it. You have to return one of
  the input slices.

## On `Drop`

`NameView` owns no heap memory, so its `Drop` is trivial — nothing to free, because
nothing was allocated. That is the quiet payoff of borrowing: fewer allocations,
and nothing to clean up. Contrast the `OwnedName`, whose drop frees every `Vec`.

## Done means

`./course check m04-name-view` green: the witness (round-trips, borrowing proven by
pointer identity, common-prefix cases, lenient stop on truncation), clippy
`-D warnings`, and fmt. Then `./course submit` and ask the tutor — bring the two
`size_of` numbers.

## Rules of engagement

No dependencies, no `unsafe`, no `.to_vec()`/`.to_owned()`/`clone()` on the
component bytes — if you reach for one, the lifetime is trying to teach you
something. Afterward, read how `../ndn-rs/crates/core/ndn-packet` decodes a real
`Name` lazily, and note in your journal one thing it does that you didn't.
