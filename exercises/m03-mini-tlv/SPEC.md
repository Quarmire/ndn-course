# m03-mini-tlv — the reader/writer pattern (from scratch)

**Module:** M3 · The reader/writer pattern · **Species:** from-scratch
**You build:** `TlvReader` and `TlvWriter` (and the shape of `TlvError`) in `src/lib.rs`.

In M2 you built the VAR-NUMBER — the variable-length integer under everything.
Now you build the thing made *of* VAR-NUMBERs: the **TLV element**, NDN's one
and only wire shape. A packet, a name, a signature — all of it is TLV, nested.
After this you'll read `../ndn-rs/crates/core/ndn-tlv`'s `TlvReader`/`TlvWriter`
and see how close you got.

## The shape

Every element on the wire is three fields back to back:

```
┌──────────┬──────────┬─────────────────────┐
│  TYPE    │  LENGTH  │  VALUE               │
│ VAR-NUM  │ VAR-NUM  │  exactly LENGTH bytes│
└──────────┴──────────┴─────────────────────┘
```

TYPE and LENGTH are VAR-NUMBERs (the M2 codec — **provided** for you in
`src/lib.rs` as `encode_varu64`/`decode_varu64`; build on them). VALUE is
`LENGTH` raw bytes — and those bytes are often *themselves* a sequence of TLV
elements. That nesting is how a whole Data packet is one outer element.

## Contract

```rust
pub enum TlvError { UnexpectedEnd, NonMinimal, UnexpectedType { expected: u64, found: u64 } }

pub struct TlvReader<'a> { /* provided: new, is_empty */ }
impl<'a> TlvReader<'a> {
    pub fn new(input: &'a [u8]) -> Self;                         // provided
    pub fn is_empty(&self) -> bool;                             // provided
    pub fn read(&mut self) -> Result<(u64, &'a [u8]), TlvError>;      // YOU
    pub fn read_type(&mut self, expected: u64) -> Result<&'a [u8], TlvError>; // YOU
}

pub struct TlvWriter { /* provided: new, into_bytes, Default */ }
impl TlvWriter {
    pub fn new() -> Self;                    // provided
    pub fn write(&mut self, type_num: u64, value: &[u8]);   // YOU
    pub fn into_bytes(self) -> Vec<u8>;      // provided
}
```

- **`read`** decodes one element from the front and **advances** the cursor past
  it, returning `(type_num, value)`. The value is a slice that **borrows from the
  original input** — that's what the `'a` is for. Do not allocate or copy it.
- **`read_type`** reads one element and requires its type to equal `expected`,
  returning just the value. On a type mismatch it consumes nothing and returns
  `UnexpectedType { expected, found }`.
- **`write`** appends `type_num ‖ len(value) ‖ value` and must not disturb bytes
  already in the buffer.
- Errors `read` can raise: `UnexpectedEnd` (input ends before an element is
  whole — mid-type, mid-length, or fewer than `LENGTH` value bytes remain) and
  `NonMinimal` (a type or length VAR-NUMBER is over-long — the provided decoder
  already reports this; let it propagate with `?`).

## Worked examples

One element — type `0x08` (an NDN *name component*), value `"ndn"`:

```
08 03 6E 64 6E
└� T┘ └ L┘ └── "ndn" ──┘        TlvWriter::write(0x08, b"ndn")
```

Nesting — a `Name` (type `0x07`) whose value is two components:

```
07 0D  08 03 6E 64 6E  08 06 63 6F 75 72 73 65
└T┘└L┘ └── "ndn" ────┘ └──── "course" ───────┘
        outer value (0x0D = 13 bytes) is itself two TLV elements
```

Reading the outer with `read()` gives `(0x07, &[08 03 .. 65])`; feed *that value
slice* to a second `TlvReader` and `read()` twice for the components. (That's the
`/ndn/course` name on the course landing page — you're decoding the hero packet.)

## Done means

`./course check m03-mini-tlv` green: the witness suite in `tests/witness.rs`
(round-trips across every VAR-NUMBER shape, nesting, truncation → `UnexpectedEnd`,
non-minimal rejection, `read_type` matching and mismatching), clippy with
`-D warnings`, and fmt. Then `./course submit` and ask the tutor for review.

## Rules of engagement

No dependencies, no `unsafe`. Don't read `../ndn-rs/crates/core/ndn-tlv` until
after your first submit — the compare-and-critique step needs your own design to
exist first. Two things this exercise is really about, and the rubric grades:

- **What to make `pub`.** You're designing an API, not a script. The four public
  methods above are the surface; everything else (helpers, fields) stays private.
  A caller should be unable to construct a reader in a broken state.
- **Borrow, don't copy.** `read` returns `&'a [u8]`, not `Vec<u8>`. The value
  lives in the caller's buffer; you hand out a window onto it. If you find
  yourself writing `.to_vec()`, stop — the lifetime is telling you something.
