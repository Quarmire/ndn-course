# m02-varint — the NDN variable-length integer (from scratch)

**Module:** M2 · Bits, bytes, numbers · **Species:** from-scratch
**You build:** `encode_varu64` and `decode_varu64` in `src/lib.rs`.

Every TLV element in NDN begins with a *VAR-NUMBER* — a variable-length
encoding of an unsigned integer used for both the Type and the Length. It is
the atom under everything you will parse for the rest of this course. After
this exercise you will read `../ndn-rs/crates/ndn-tlv`'s version of the same
thing and write three observations about how the professionals did it.

## The encoding

A value `v: u64` encodes to one of four shapes, chosen by its magnitude:

| Range of `v` | Encoding | Total bytes |
|---|---|---|
| `0 ..= 252` | the value itself as one byte | 1 |
| `253 ..= 65535` | `0xFD` then `v` as **u16 big-endian** | 3 |
| `65536 ..= 4294967295` | `0xFE` then `v` as **u32 big-endian** | 5 |
| `4294967296 ..= u64::MAX` | `0xFF` then `v` as **u64 big-endian** | 9 |

Note the first-byte values `0xFD`, `0xFE`, `0xFF` are exactly the ones a
one-byte encoding never uses — that's how a decoder knows which shape follows.

## Contract

```rust
pub fn encode_varu64(value: u64, out: &mut Vec<u8>);
pub fn decode_varu64(input: &[u8]) -> Result<(u64, usize), VarintError>;
```

- `encode_varu64` **appends** the encoding of `value` to `out` (it must not
  clear or otherwise disturb bytes already there) and always produces the
  **shortest** shape that can represent the value.
- `decode_varu64` reads one VAR-NUMBER from the **front** of `input` and
  returns `(value, bytes_consumed)`. Bytes after the number are none of its
  business — callers handle those.
- Decoding errors:
  - `VarintError::UnexpectedEnd` — the input ends before the shape it promised
    is complete (including an empty input).
  - `VarintError::NonMinimal` — the input uses a longer shape than the value
    requires (e.g. `FD 00 01` for the value 1). A strict decoder rejects these
    so that every value has exactly one encoding — you'll meet the reason
    (canonical bytes under signatures) in Phase 4.

## Worked examples

| Value | Bytes |
|---|---|
| `0` | `00` |
| `252` | `FC` |
| `253` | `FD 00 FD` |
| `65535` | `FD FF FF` |
| `65536` | `FE 00 01 00 00` |
| `u64::MAX` | `FF FF FF FF FF FF FF FF FF` |

## Done means

`./course check m02-varint` green: the witness suite in `tests/witness.rs`
(boundary vectors, 10 000 pseudo-random round-trips, truncation, non-minimal
rejection), clippy with `-D warnings`, and fmt. Then `./course submit` and ask
the tutor for the gate-2 review.

## Rules of engagement

No dependencies, no `unsafe`, no peeking at `ndn-tlv` until after your first
submit (the compare-and-critique step needs your design to exist first). The
tutor will explain any *concept* at any depth — shifts, endianness, slices —
but won't write these two functions for you.
