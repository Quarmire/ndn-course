# Rubric — Capstone B (the contribution)

Grade this in three sittings: the **design-note gate** (before any code), the
**implementation + review-rounds review**, and the **viva**. Do NOT approve
implementation until the design note passes. Score each criterion 0–2; pass = no zeros
and ≥ 14/20. The contributor *flow* is the deliverable, not just the diff.

## Design-note gate (must pass before building)

| # | Criterion | What "2" looks like |
|---|---|---|
| 1 | Problem framed in the repo's terms | The issue states a real gap the way a maintainer would, and confirms currency (the repo moves) |
| 2 | Design note precedes code | A house-format note naming the approach AND the rejected alternative, with a litmus |
| 3 | Fits the doctrine | Places the change in the right scope bucket; respects the dependency-direction rule; feature-gates a new surface |

## Implementation + review rounds

| # | Criterion | What "2" looks like |
|---|---|---|
| 4 | Witnesses fail-before / pass-after | The audit-witness genuinely fails on the unfixed code and passes after — proof, not decoration |
| 5 | Reads before it writes | The change fits the surrounding code's idiom and conventions (M1); no drive-by reformatting |
| 6 | PR narrative | A reviewer can say yes: what, why, what was tested, and one journaled bug and how it was found |
| 7 | Handled a review round | Responded to real feedback like a colleague — revised, or defended with reasons; the round is graded |

## Viva

| # | Criterion | What "2" looks like |
|---|---|---|
| 8 | Defends the approach | Can argue why this design and not the rejected one, from the repo's constraints |
| 9 | Knows the blast radius | Can say what would break first under change, and who else the change touches |
| 10 | Worked with the agent well | Used the agent to navigate a large unfamiliar codebase, but owns the judgment and can explain it |

## Closing

If they pass, say so and mark it done in `student/progress.json`. If not, name the gate
and criterion and send them back to it. A rejected design note goes back to the design
gate — not to code — because building the wrong change well is still the wrong change.
