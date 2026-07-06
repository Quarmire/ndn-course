# Rubric — m06-name-api

Gate 2 review. The oracle proves the ordering is canonical; grade the *design
judgment* and the API-review reasoning. Score each 0–2; pass = no zeros and
≥ 10/14. Comment quality and observability are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Canonical component order | `cmp` is the three-tier type → length → content, chained with `then_with`; not a derive, not content-before-length; matches the oracle everywhere |
| 2 | Name order | Delegates to the component `Vec`'s lexicographic `cmp` (one call), not a hand-rolled loop that reimplements it (and risks the prefix case) |
| 3 | Eq / Ord / Hash consistency | The student can explain *why* the derived `Eq`/`Hash` stay consistent with their hand-written `Ord` (same fields, same tiers) — the `BTreeMap`/`HashMap` agreement argument |
| 4 | has_prefix + Display + From | All correct; `Display` handles the root; `From` skips empty segments and round-trips; no panics on odd input |
| 5 | Small, honest surface | The field stayed private; only what a caller needs is `pub`; no gratuitous getters; `GENERIC`/`generic` used rather than a magic `8` sprinkled around |
| 6 | Comments (standing) | The *why* of length-before-content (or the Eq/Ord contract) is captured in a comment where a maintainer would look; zero narration |
| 7 | API review (journal) | Named two real design choices in `ndn-foundation-types` and reasoned about each against the API Guidelines AND the semver lens (what's a promise to callers, what could change) |

## Reflection prompts (gate 3, pick 2–3)

- Why does canonical order put length before content? What breaks — for signing,
  for routing tables — if two encoders disagree on the order of the same two names?
- You didn't derive `Ord`, but you did derive `Eq` and `Hash`. What would go wrong
  if `Ord` and `Eq` disagreed — give a concrete `BTreeSet` vs `HashSet` symptom.
- The real `Name` hides its `components` field behind `pub(crate)` and accessors.
  What future change does that privacy protect, and what does exposing it cost?
