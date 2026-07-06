# m07-data-view — decode nothing until you must

**Module:** M7 · Zero-copy parsing: lifetimes in anger · **Species:** template-assisted
**You write:** `DataView::find`, `content_as_text`, `to_owned_content`.

M4 taught you to hand back a borrow instead of a copy. This module takes it to a
whole packet and adds the two ways *out* of the borrow when you need them. A Data
packet is a TLV whose value is a run of sub-elements. The naive parser walks it
once and copies every field into owned buffers. A `DataView<'a>` copies *nothing*
at parse time and only touches a field when you ask — and even then, hands you a
window onto the original bytes.

## The three ways to hold bytes

This is the module's real content — knowing which to reach for:

- **`&'a [u8]`** — borrowed. Zero cost, but tied to the buffer's lifetime `'a`; it
  cannot outlive the bytes it points at. This is what `find`/`name`/`content` return.
- **`Cow<'a, [u8]>` / `Cow<'a, str>`** — *maybe* owned. Borrows when it can, allocates
  only when it must transform. The perfect return type for "usually free, sometimes not."
- **`bytes::Bytes`** — owned and shareable. A cheap-to-clone handle over a heap
  buffer (a fat pointer with a refcount, not the data inline). Reach for it when the
  data must *outlive* the buffer it came from — the copy is the price of freedom.

## What you write

- **`find(&self, type_num) -> Option<&'a [u8]>`** — the lazy heart. Walk the inner
  sub-elements (`decode_varu64` for each type and length, provided), and return the
  VALUE of the first one whose type matches, **borrowed**. Copy nothing; scan only
  as far as needed. `name`/`content` are given, built on this — that's the pattern.
  Mind the lifetime: the returned slice borrows from the *buffer* (`'a`), not from
  `&self` — same move you made in M4 (copy `self.inner` into a local first).

- **`content_as_text(&self) -> Option<Cow<'a, str>>`** — the `Cow` lesson. The
  standard library already gives you a function over `&[u8]` that returns exactly
  this `Cow`: borrowed for valid UTF-8 (no allocation), owned only when it had to
  substitute replacement characters. Find it, and see that a `Cow` is a decision the
  callee makes for the caller.

- **`to_owned_content(&self) -> Option<Bytes>`** — the `Bytes` lesson. Copy the
  borrowed content into an owned `Bytes` so it can outlive this `DataView`. One call.

## The refactor lens (bring to your submission)

Picture the owned parser you're *not* writing: `struct OwnedData { name: Vec<u8>,
content: Vec<u8> }`, filled by copying both fields at parse time. Two heap
allocations and two copies, every packet, whether or not anyone reads them. Yours
does zero until `to_owned_content`. In your journal, argue where each design wins:
a forwarder that inspects the name of a million packets and stores none of them,
versus a repo that keeps every Content it sees. (Optional Machine strand: add a
`criterion` bench of owned-parse-all vs lazy-view and report the numbers.)

## Done means

`./course check m07-data-view` green: the witness (borrowing proven by pointer
identity, `Cow` borrowed-vs-owned, `Bytes` round-trip, missing fields), clippy
`-D warnings`, and fmt. Then `./course submit`.

## Rules of engagement

No `unsafe`. `find` must not allocate or copy the value (no `.to_vec()`). Afterward,
read how `../ndn-rs/crates/core/ndn-packet` lazily decodes a real Interest/Data —
note one field it decodes eagerly and one it defers, and why the split makes sense.
