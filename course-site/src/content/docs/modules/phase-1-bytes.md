---
title: Phase 1 — Bytes, memory, bugs
description: Build a codec from scratch, borrow instead of copy, and hunt planted bugs.
---

**Checkpoint: _I can build, test, and debug a codec._**

## M2 · Bits, bytes, numbers

Integer types and casts, shifts and masks, endianness, arrays vs slices — by building
the NDN variable-length integer (`varu64`) from scratch against a property-test suite,
then diffing your version against the real `ndn-tlv`.

## M3 · The reader/writer pattern

Your first real crate: a cursor-over-`&[u8]` `TlvReader` that hands back **borrowed**
value slices, a `TlvWriter` that appends type-length-value, and your own error enum.
The first lesson in _what to make `pub`_.

## M4 · Ownership and lifetimes beyond trivial

The module that turns lifetimes from syntax into a tool. Build a zero-copy
`NameView<'a>` that owns nothing and returns a borrow out of `&mut self` — proven by
pointer identity, so a hidden `.to_vec()` fails the test.

## M5 · Debugging, and code that debugs itself

The debugging covenant, made a module. Four bugs are planted in working-looking code —
a panic-hiding edge case, a backwards comparison, an off-by-one, a slice-advance slip —
and you hunt them with backtraces, minimization, and comparison to known-good, logging
each in your bug journal. From here, comment quality and observability are graded on
every exercise.

Next: [Phase 2 →](/modules/phase-2-types/).
