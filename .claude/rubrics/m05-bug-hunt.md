# Rubric — m05-bug-hunt

Gate 2 review. The witness proves the four bugs are fixed; your job is to grade
*how* they debugged and whether they hardened the code — the bug journal is the
evidence. Score each 0–2; pass = no zeros and ≥ 10/14. **Comment quality and
observability are STANDING criteria from this module forward.**

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Fixes correct and minimal | Each of the four is a one-line, root-cause fix — not a symptom patch, and not a rewrite that "happens to pass" |
| 2 | Bug journal: method | Each entry shows a real diagnosis path — a backtrace line, a minimized input, a comparison to a known-good — not "I noticed it was wrong" |
| 3 | Bug journal: classification | Each bug named by its kind (panic-hiding / logic slip / off-by-one / slice-advance) with the lesson stated — pattern recognition, not just this instance |
| 4 | Prevention added | At least one genuine defense (a `debug_assert!` invariant, `first`/`get` over `[i]`, an exhaustive match) with a note on which bug it would have caught |
| 5 | Comments (standing) | Changed lines explain the *why* / the invariant, not the mechanics; the edge cases that bit are now documented; zero narration |
| 6 | Observability (standing) | Evidence of thinking about self-instrumentation — a `debug_assert!` that would trip in a test, or a note on where a `debug!` belongs and at what level |
| 7 | Didn't break the rest | Minimal fixes leave the passing behavior intact; clippy + fmt still clean; no dead scaffolding left behind |

## Reflection prompts (gate 3, pick 2–3)

- Two bugs panicked; two returned wrong answers silently. Which class is more
  dangerous in a forwarder that runs for weeks, and why?
- Pick one bug: what change to the *design* (a type, a signature, an invariant)
  would have made it unrepresentable, not merely unlikely?
- You fixed `common_prefix_len`, which you also wrote from scratch in M4. Did your
  M4 version have this bug? What does that say about writing versus reviewing?
