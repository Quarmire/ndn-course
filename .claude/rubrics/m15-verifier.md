# Rubric — m15-verifier

Gate 2 review. This is a verifier — correctness *and* the security posture both
count. Score each 0–2; pass = no zeros and ≥ 10/14. Comment quality and observability
are standing criteria.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Pipeline order | The seven checks run in the specified order; each isolated test gets the right reason; no check reads a grant before confirming it exists |
| 2 | Typed reasons | Every denial carries the correct `DenyReason` variant — never a bare bool or a stringly-typed message; the student can say why a typed taxonomy beats a bool for an audit log |
| 3 | Unresolved kept honest | An unfetched grant returns `Unresolved { missing_grant }`, NOT a `Deny`; the student can explain the M14 tie-in (unfetched ≠ refuse) |
| 4 | Default-refuse | The last gate refuses an unlisted action; there is no "else allow" path anywhere; the student can articulate why default-deny is the safe posture |
| 5 | Longest-prefix keyring | `anchor_for` returns the most-specific covering anchor (or `None`); handles the `/` root fallback and component-wise matching (`/alice` ≠ `/alicia`) |
| 6 | Crypto hygiene grasped | Can explain constant-time comparison (why `==` on a signature leaks a forgery path) and where authentication (the signature) sits relative to this authorization pipeline |
| 7 | Comments + no shortcuts | The security-relevant choices (order, default-refuse, Unresolved) are noted where a maintainer looks; no `unwrap` on `get`, no panics on odd input |

## Reflection prompts (gate 3, pick 2–3)

- A denied action must say *which* rule stopped it. Walk through what an operator
  loses if the verifier returned only `true`/`false`, and what a typed reason buys.
- You return `Unresolved` for an unfetched grant instead of `Deny`. Construct the
  concrete harm of getting that wrong — a legitimate action wrongly denied, or a
  bad one wrongly allowed — and say which direction is worse here.
- Authorization (these seven steps) sits above authentication (the signature check).
  Why must the signature be verified *first*, and what breaks if you authorize an
  action whose signature you haven't checked?
