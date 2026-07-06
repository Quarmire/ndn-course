# Rubric — m03-mini-tlv

Gate 2 review. Score each criterion 0–2 with one line-anchored observation.
Pass = no zeros and total ≥ 10/14. This is the student's first *API* (not just
functions), and their first time returning a borrow from `&mut self` — grade the
design thinking, not just the green tests.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Reader decode | `read` is a clean type → length → value → advance; the two `decode_varu64` calls compose with `?`, no re-implementing the codec, no hand-rolled length math off by a byte |
| 2 | Borrow, don't copy | `read` returns the `&'a` slice straight from the input — no `to_vec`/`Vec`/`clone` of the value anywhere; the point of the exercise is honored |
| 3 | The lifetime move | Correctly returns `&'a` out of `&mut self` — copies the `rest` reference into a local (or reslices the `&'a` slice) rather than fighting the borrow checker with owned buffers or `unsafe` |
| 4 | Error handling | Short value → `UnexpectedEnd` via `get(..)`, never a panicking index/`unwrap`; provider errors propagate untouched; `UnexpectedType` carries *both* numbers |
| 5 | `read_type` back-off | On mismatch the cursor is restored (save `rest`, put it back) so nothing is consumed — not "advance then error" |
| 6 | API surface + comments | Only `new`/`is_empty`/`read`/`read_type` (reader) and the writer's three are `pub`; fields/helpers private; a caller can't build a broken reader. The *why* of borrowing (or of the back-off) is captured in a comment; zero narration |
| 7 | Tests + simplicity | At least one self-authored test (e.g. a deeper nesting, or an empty-value round-trip they added); no speculative generality — no traits, no generic `TlvCodec<T>`, no config structs |

## Reflection prompts (gate 3, pick 2–3)

- `read` returns `&[u8]`, not `Vec<u8>`. What does the caller *gain* from the
  borrow, and what constraint do they accept in exchange?
- Your `read_type` puts the cursor back on a mismatch. What parsing situation
  actually needs "try this type, otherwise leave it for someone else to read"?
- Type and length share one VAR-NUMBER codec, and it rejects non-minimal
  encodings. Why does a format that gets *signed* care that each value has
  exactly one byte representation?
