# Rubric — Capstone A (the application)

Grade this in three sittings: the **proposal gate** (before any code), the **build
review**, and the **viva**. Do NOT approve implementation until the proposal passes.
Score each criterion 0–2; pass = no zeros and ≥ 14/20. The weight is on judgment and
defense, not line count.

## Proposal gate (must pass before building)

| # | Criterion | What "2" looks like |
|---|---|---|
| 1 | Scope is finishable | A crisp what-it-is and an honest out-of-scope list; achievable in the time |
| 2 | Namespace is designed | Names are deliberate and defensible (M6) — components carry meaning, versioning is thought through; not `/data/1` |
| 3 | Decision notes | At least two real notes in house form, each naming the rejected alternative and why |

## Build review

| # | Criterion | What "2" looks like |
|---|---|---|
| 4 | Runs in minutes | The README's quick start works on a fresh checkout — the course's own invariant, honored |
| 5 | Tested behavior | A produce → consume → verify round-trip is tested; not merely "it compiled" |
| 6 | Observability | `tracing` spans on the interesting path; you can watch an Interest go out and Data come back |
| 7 | Signed + clean | Data is signed and the consumer verifies; `clippy -D warnings` + `fmt` green; the bug journal was kept |

## Viva

| # | Criterion | What "2" looks like |
|---|---|---|
| 8 | Defends the design | Justifies the namespace and trust choices, and names one thing they'd design differently |
| 9 | Knows the seams | Can say what breaks first under load or change, and exactly where the app trusts the forwarder |
| 10 | Worked with the agent well | Used the tutor/agent as a tool but owns every judgment — can explain any line, not just paste it |

## Closing

If they pass, say so plainly and mark the capstone done in `student/progress.json`. If
not, be specific about which gate and which criterion, and send them back to that gate —
a capstone that fails the build review returns to build, not to proposal.
