# m15-verifier — trust and the verifier

**Module:** M15 · Trust and the verifier · **Species:** template-assisted
**You write:** `verify` (the AC.12 pipeline) and `TrustContext::anchor_for` (keyring dispatch).

The last core module, and the one where all the habits pay off: a verifier is where
typed errors, ordering, default-deny, and "I don't know" all have to be *exactly*
right, because getting them wrong is a security bug, not a test failure. You'll build
a faithful (simplified) slice of ndf-core's `verify_action`.

## The pipeline shape

A verifier is a **short-circuiting pipeline**: each step can pass or fail, and the
**first failure decides**, carrying a **typed reason**. Run them in this order —
order is part of the contract (you can't check a grant's scope before you've
confirmed the grant exists):

1. **schema** — `action.schema_version != ctx.supported_schema` → `Deny(UnsafeSchemaVersion)`.
2. **grant fetched?** — if `authorizing_grant` isn't in `ctx.grants`, you *can't
   decide* — return `Unresolved { missing_grant }`. This is M14's fourth verdict in
   the verifier: an unfetched grant is **not** a denial, it's "fetch it and ask
   again." Collapsing it into `Deny` is the classic mistake.
3. **revoked** — grant id in `ctx.revoked` → `Deny(GrantRevoked)`.
4. **expired** — `grant.expiry <= ctx.now` → `Deny(GrantExpired)`.
5. **actor** — `action.actor != grant.holder` → `Deny(ActorMismatch)`.
6. **scope** — the grant's `scope_prefix` must *cover* the action's `scope`
   (component-wise; `covers` is provided) → else `Deny(ScopeViolation)`.
7. **default-refuse** — the action type must be **explicitly** in
   `grant.allowed_actions`; if not → `Deny(ActionNotPermitted)`. Nothing is inferred.

If every step passes → `Accept`.

**Default-refuse is the security posture that matters most.** The verifier never
"probably it's fine" — an intent that isn't listed is refused. Combined with the
typed reasons, a denied action always says *exactly which rule* stopped it, which is
what makes an audit log worth reading.

## The keyring: longest-prefix dispatch

Trust in named data is hierarchical: a key that governs `/alice` implicitly governs
`/alice/photos`, unless a **more specific** key governs `/alice/photos` directly.
`anchor_for(name)` returns the anchor whose prefix **covers `name` and is longest** —
the most specific policy wins. (This is the same longest-prefix match a forwarder's
FIB does for routes; here it decides *which key is allowed to have signed this
name*.) A general root anchor at `/` covers everything as the fallback.

## Crypto hygiene (read; not coded here)

The steps above are *authorization*. Underneath sits *authentication* — the
signature check — and it has its own non-negotiable rules:

- **Constant-time comparison.** Comparing a signature or MAC with `==` leaks *where*
  it first differs through timing, and an attacker can walk a forgery byte-by-byte.
  Real code uses a constant-time equality (e.g. `subtle`), never `==`. A verifier
  that's correct but timing-variable is still broken.
- **Signing/verification end to end.** A `KeyChain` holds signing keys; a
  `TrustContext`/keyring holds the anchors that say *which* key may sign *which*
  names. NDF's **Block-as-Data** rides a single unified NDN signature (M7/M12), so a
  stock NDN verifier authenticates it before this authorization pipeline runs.
- **Ed25519** is the workhorse — small keys, fast verify. Benchmark a verify with
  `criterion` and note the order of magnitude; it explains why a forwarder can afford
  to check every packet.

Read ndf-core's real `DenyReason` (21 step-tagged variants) and `verify.rs`, and the
`security-composition` notes.

## Done means

`./course check m15-verifier` green: every deny reason fires for the right input and
in the right order, `Accept` for a fully-valid action, `Unresolved` for an unfetched
grant, and the keyring picks the longest prefix — plus clippy `-D warnings` and fmt.
Then `./course submit`.

## Rules of engagement

No dependencies. The step ORDER is part of the spec — a test isolates each reason, so
a mis-ordered check (e.g. scope before actor) will surface. Never turn `Unresolved`
into a `Deny`, and never default-*allow* an unlisted action.
