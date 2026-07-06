# Rubric — m02-varint

Gate 2 review. Score each criterion 0–2 with one line-anchored observation.
Pass = no zeros and total ≥ 10/14. Note: this is the student's *first* code in
the course — grade honestly, but calibrate tone to a first exercise.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Shape selection | A single clear four-way decision (match with range patterns or equivalent), not a cascade of nested ifs duplicating boundaries |
| 2 | Byte production | `to_be_bytes` / `from_be_bytes` + `extend_from_slice` rather than manual shift-and-mask loops (manual shifting is fine *only if* commented as a deliberate learning detour) |
| 3 | Decode bounds discipline | Uses `get(..)` / slice patterns so short input becomes `UnexpectedEnd` naturally — no indexing that can panic |
| 4 | Non-minimal check | One boundary comparison per shape, after the value exists; correct at exactly 252 / 65 535 / u32::MAX |
| 5 | Comments (standing) | The *why* of the sentinel bytes (0xFD/FE/FF) or the minimality rule is captured somewhere a future reader will find it; zero narration comments |
| 6 | Student-added tests | At least one test they wrote themselves (e.g. consumed-length on every shape, or an encode-then-mutate probe) |
| 7 | Simplicity | Two functions and an enum; no speculative generality (no traits, no generics, no "VarintCodec" struct) |

## Reflection prompts (gate 3, pick 2–3)

- Why do the sentinel values start at 0xFD and not, say, 0x80? What property
  of the one-byte range makes this work?
- Your decoder rejects non-minimal encodings. What breaks in a *signed* data
  format if two byte strings can mean the same value?
- If this had to run on a chip with no allocator (`no_std`, no `Vec`), which
  of your two functions changes, and what would its signature become?
