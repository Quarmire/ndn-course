# Rubric — m14-verdict

Gate 2 review. The code is small; the **decision note and the architectural
understanding are the weight here** — grade them as heavily as the code. Score each
0–2; pass = no zeros and ≥ 10/14. Comment quality and observability are standing.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | The three zones | `verdict` is the correct inequality check; `>=` on the covered edge; only `>=`/`<`, never float `==` |
| 2 | Unresolved kept first-class | `authorize`'s `Unresolved` arm carries `estimate`/`margin`/`threshold` and is NOT collapsed to `Refused` — the load-bearing test passes for the right reason |
| 3 | Grasps the fourth verdict | Can explain *why* `Unresolved ≠ Deny` — totality over partial knowledge, and that collapsing it hides a choice from the caller |
| 4 | The decision note: form | Uses the house shape — a one-line principle/litmus, a why, what it replaces/rejects, an authority tag (ruling/recommendation/advisory) |
| 5 | The decision note: position | Takes a real, defensible position on the tension (system-default vs caller-chooses) and argues it from the four-verdict thesis, not vibes |
| 6 | The boundary litmus | Can state the waterline rule (mechanism→ndn-workspace, policy→NDF) and place this verdict logic correctly (it's meaning/policy → NDF) |
| 7 | Block-as-Data + honesty | Understands a Block is an NDN Data packet with a unified signature (M7/M12 callback); notices the corpus's own honesty culture (supersession, eviction dates, the gauntlet) |

## Reflection prompts (gate 3, pick 2–3)

- A logging system drops events it "isn't sure" about. A permission check denies
  requests it "isn't sure" about. Both collapse `Unresolved`. What does each lose,
  and which is more dangerous?
- The `unknown()` coverage always returns `Unresolved`. Why is "I measured nothing →
  I don't know" more honest than any crisp answer, and what would go wrong if a
  backend faked a confident estimate?
- NDF gives duplicated infrastructure an "eviction date the day it's written." What
  does that discipline protect, and what's the cost of skipping it?
