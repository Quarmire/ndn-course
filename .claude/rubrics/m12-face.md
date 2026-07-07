# Rubric — m12-face

Gate 2 review. The tests prove the face works; this module is really about the
*craft of contributing*, so weight the journal heavily. Score each 0–2; pass = no
zeros and ≥ 10/14. Comment quality and observability are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Transport impl | `send_frame` maps a dead channel to `Closed` (no panic, no unwrap); `recv_frame` is non-blocking (`try_recv`); `id` is trivial and correct |
| 2 | Framing, layered right | `send_packet` tags with the sender's id and prepends 4 BE bytes; `recv_packet` splits header/payload; the id logic lives in the LinkService, NOT the transport |
| 3 | The split understood | The student can explain what each layer owns (bytes vs framed-bytes+metadata) and why keeping them separate lets one LinkService ride any transport |
| 4 | Object-safety connection | Read the real traits: can say why the async `Transport` isn't object-safe (RPIT) and needs `ErasedTransport`, while `LinkService` deliberately is — the M8 tension in production |
| 5 | Feature-flag reasoning | Journal names the feature its transport would live behind and why faces are opt-in; distinguishes `dep:`-gating a crate from gating a module |
| 6 | Scope + dep direction | Journal places the face in a real scope bucket (spec/extension/…) and states what the `draft → … → spec` rule permits it to depend on |
| 7 | PR narrative + honesty | The submission reads like a reviewable PR — what/why/tested + one journaled bug; a known limitation (if any) is stated plainly, not hidden |

## Reflection prompts (gate 3, pick 2–3)

- Your `LinkService` is generic over `T: Transport`; the real one holds
  `Arc<dyn ErasedTransport>`. What does each choice buy, and when would the engine
  *need* the `dyn` version?
- New faces are gated behind cargo features, off by default in some builds. What
  goes wrong if every transport were always compiled in?
- A `spec` crate may not depend on a `draft` crate. Give a concrete reason that rule
  protects the release, and a case where you'd be tempted to break it.
