# Rubric — m07-data-view

Gate 2 review. Score each 0–2; pass = no zeros and ≥ 10/14. This module is about
choosing the cheapest byte representation that works — grade whether the student
reaches for it deliberately. Comment quality and observability are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Lazy borrowed find | `find` scans the inner TLVs and returns the value BORROWED (`&'a`), copying nothing; no `.to_vec()`; a truncated element stops the scan via `?` |
| 2 | The lifetime move | Returns `&'a` from `&self` by slicing a local copy of `self.inner` — not fought with owned buffers or `unsafe` |
| 3 | Cow used, not faked | `content_as_text` returns the `from_utf8_lossy` `Cow` — no manual `if valid { Borrowed } else { Owned }`, no unconditional `to_string()` that always allocates |
| 4 | Bytes for the escape | `to_owned_content` copies into `Bytes` exactly once; the student can say why THIS one allocates while `find` does not |
| 5 | Representation judgment | The journal argues borrowed-vs-owned by workload (inspect-and-drop vs keep); the three types are chosen deliberately, not by habit |
| 6 | Comments (standing) | The *why* of laziness / the lifetime is captured where a maintainer looks; the "copy is the price" idea is noted at `to_owned_content` |
| 7 | Simplicity | No eager decoding in `parse`, no caching of fields the caller may never read, no speculative generality |

## Reflection prompts (gate 3, pick 2–3)

- A forwarder reads the name of every packet and stores almost none; an archival
  repo keeps every Content. Which wants `DataView`, which wants an owned parse, why?
- `content_as_text` sometimes allocates and sometimes doesn't, and the caller can't
  tell at the call site. Good API or trap? When would you force the caller to choose?
- `Bytes` is cheap to *clone* but not free to *create* from a slice. What does that
  asymmetry say about where NDN should turn Data into `Bytes` — at parse, or only
  when something decides to keep it?
