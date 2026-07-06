# Rubric — m00-zero-to-running

Gate 2 review. This is an OBSERVATION lab: the mechanical gate only confirms the
four config facts. Your real job is to check the student actually *ran and
observed* — the journal is the evidence. Score each 0–2; pass = no zeros and
≥ 8/12. (Six criteria, not seven — there's no code to critique yet.)

| # | Criterion | What "2" looks like |
|---|---|---|
| 1 | The facts are right | All four consts match the real default config (the witness confirms). Spot-check that udp and web-socket ports weren't swapped |
| 2 | They ran it | The journal describes running the forwarder — a real log line, an actual command, a concrete error — not a paraphrase of the SPEC |
| 3 | They poked it | Evidence of using at least one tool (ndn-ctl / ndn-peek) against the running forwarder, including whatever friction they hit |
| 4 | Doc-vs-reality catch | They found and named a real mismatch (no-config → zero faces, or the socket-path gap) and said HOW they verified it against code/config |
| 5 | Journal quality | Entry #1 has substance — what surprised them, not just what they did. A future reader learns something |
| 6 | Honest about friction | They wrote down what did NOT work (sockets, unanswered Interests) rather than implying it was smooth. The covenant starts here |

## Reflection prompts (gate 3, pick 2–3)

- The forwarder's first printed line disclaims its own correctness. Why would a
  serious project ship that banner instead of quietly hoping? What does it buy?
- You found a place where a doc and the code disagreed. In a codebase this size,
  which do you trust by default, and how do you confirm which is right?
- With no config, the forwarder opens zero faces. Is that a bug or a good default?
  Argue it either way — the reasoning matters more than the verdict.
