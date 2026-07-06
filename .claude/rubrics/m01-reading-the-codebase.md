# Rubric — m01-reading-the-codebase

Gate 2 review. Observation lab: the witness confirms the student LOCATED the code
and MEASURED the sizes. Your job is to check they can READ what they found — the
journal trace is the evidence. Score each 0–2; pass = no zeros and ≥ 8/12. (Six
criteria, not seven — no code of their own to critique yet.)

| # | Criterion | What "2" looks like |
|---|---|---|
| 1 | Found the right places | All three file paths pass the witness, and the student can say in one line what each file's stage does |
| 2 | Understands the PIT | Can explain what a PIT entry IS and why an Interest miss creates one (it aggregates pending downstreams so one Data satisfies many), not just where it lives |
| 3 | Understands the verified gate | Explains that unverified Data is still forwarded to satisfy pending Interests but is NOT admitted to the cache — fail-secure — and why that matters |
| 4 | Pipeline trace | Journal entry #2 traces the Data path through the real stage structs in order, naming decisions — not a paraphrase of the ASCII diagram |
| 5 | Measured and reasoned | Both sizes correct AND the student can say why `NameComponent` exceeds the bytes it names — `Bytes` is a fat handle, the data lives elsewhere |
| 6 | Navigation skill | Evidence they used search/tools (grep, `cargo doc`) to find things, and noticed the flat-vs-grouped path gotcha between the doc and the tree |

## Reflection prompts (gate 3, pick 2–3)

- Unverified Data is forwarded but never cached. What does that closed door stop?
  What would break if the forwarder cached everything it saw?
- `NameComponent` is a fixed ~40 bytes but can name a component of any length.
  Where do the actual name bytes live, and who owns them?
- ARCHITECTURE.md's paths didn't match the tree. If you were tidying this repo,
  would you fix the doc, move the crates, or add a test that keeps them honest?
