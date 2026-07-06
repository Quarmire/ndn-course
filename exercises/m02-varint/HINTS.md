# Hints — m02-varint

Revealed one rung at a time by `./course hint m02-varint`. Each rung assumes
you've genuinely tried since the last one.

## Hint 1 — shape selection

Encoding is a four-way decision on magnitude. Rust's `match` with range
patterns reads almost exactly like the table in SPEC.md:
`0..=252`, `253..=65_535`, and so on. Decide the shape first; producing the
bytes comes second.

## Hint 2 — producing big-endian bytes

You never need manual shifting for the multi-byte shapes: `u16`, `u32`, and
`u64` all have `to_be_bytes()`, and `Vec<u8>` has `extend_from_slice`. Cast
with `as u16` / `as u32` only *after* the match arm has proven the value fits.

## Hint 3 — decoding is "peek, then demand"

Look at `input.first()` to learn the shape, which tells you exactly how many
bytes the whole number needs, say `n`. `input.get(1..n)` hands you the payload
slice as an `Option` — `None` is precisely your `UnexpectedEnd`. Then
`u16::from_be_bytes` (etc.) wants a fixed-size array: `try_into()` converts a
slice whose length you've already established.

## Hint 4 — the non-minimal check is one comparison

After decoding a 3-byte shape you hold a value. If that value would have fit
in the *previous* shape (`value <= 252`), the encoding was non-minimal. Same
comparison, one boundary per shape: `<= 252`, `<= 65_535`, `<= u32::MAX as u64`.
Check it *after* you have the value, not while reading bytes.
