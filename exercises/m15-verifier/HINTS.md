# Hints — m15-verifier

Revealed one rung at a time by `./course hint m15-verifier`.

## Hint 1 — verify is early-returns in order

Each step is a guard that returns on failure; if control reaches the bottom, it's
`Accept`. The shape:

```rust
if action.schema_version != ctx.supported_schema {
    return VerifyResult::Deny(DenyReason::UnsafeSchemaVersion);
}
let grant = match ctx.grants.get(&action.authorizing_grant) {
    Some(g) => g,
    None => return VerifyResult::Unresolved { missing_grant: action.authorizing_grant },
};
// ...revoked, expired, actor, scope, default-refuse...
VerifyResult::Accept
```

`ctx.grants.get(id)` returns an `Option<&Grant>` — the `None` arm is the `Unresolved`
you must *not* turn into a `Deny`.

## Hint 2 — the middle checks

Once you hold `grant`:

```rust
if ctx.revoked.contains(&action.authorizing_grant) { return Deny(GrantRevoked); }
if grant.expiry <= ctx.now                          { return Deny(GrantExpired); }
if action.actor != grant.holder                     { return Deny(ActorMismatch); }
if !covers(&grant.scope_prefix, &action.scope)      { return Deny(ScopeViolation); }
```

`covers` (provided) does the component-wise prefix check. Keep the order — a test
feeds an input where only *one* thing is wrong, so a swapped pair changes the reason.

## Hint 3 — default-refuse is the last gate

The action type must be *explicitly* present, or it's denied:

```rust
if !grant.allowed_actions.contains(&action.action_type) {
    return VerifyResult::Deny(DenyReason::ActionNotPermitted);
}
```

`Vec<String>::contains` takes a `&String`, and `&action.action_type` is one. There is
no "else allow" — the absence *is* the refusal.

## Hint 4 — longest-prefix keyring

Keep the anchors whose prefix covers the name, then take the one with the most
components:

```rust
self.anchors
    .iter()
    .filter(|(prefix, _)| covers(prefix, name))
    .max_by_key(|(prefix, _)| prefix.split('/').filter(|s| !s.is_empty()).count())
    .map(|(_, anchor)| anchor.as_str())
```

`filter` drops the non-matching anchors; `max_by_key` on the component count picks
the most specific; `map` hands back just the anchor. `None` falls out naturally when
nothing matches.
