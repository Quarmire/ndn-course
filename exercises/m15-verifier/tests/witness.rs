//! Witness suite for m15-verifier. Each test isolates one outcome so a mis-ordered
//! or mis-typed check surfaces. The pipeline's order and default-refuse are the
//! contract.

use std::collections::{HashMap, HashSet};

use m15_verifier::{Action, DenyReason, Grant, TrustContext, VerifierContext, VerifyResult};

const NOW: u64 = 1_700_000_000;
const SCHEMA: u32 = 3;

fn grant(holder: &str, actions: &[&str], prefix: &str, expiry: u64) -> Grant {
    Grant {
        holder: holder.into(),
        allowed_actions: actions.iter().map(|s| s.to_string()).collect(),
        scope_prefix: prefix.into(),
        expiry,
    }
}

fn ctx() -> VerifierContext {
    let mut grants = HashMap::new();
    grants.insert(
        1,
        grant("alice", &["SignBlock", "Publish"], "/alice", 4_000_000_000),
    ); // valid
    grants.insert(2, grant("bob", &["Publish"], "/bob", 1_000)); // expired (expiry < NOW)
    grants.insert(3, grant("carol", &["SignBlock"], "/carol", 4_000_000_000)); // valid but revoked
    VerifierContext {
        now: NOW,
        supported_schema: SCHEMA,
        grants,
        revoked: HashSet::from([3u32]),
    }
}

fn action(actor: &str, action_type: &str, scope: &str, grant: u32) -> Action {
    Action {
        schema_version: SCHEMA,
        actor: actor.into(),
        action_type: action_type.into(),
        scope: scope.into(),
        authorizing_grant: grant,
    }
}

// --------------------------------------------------------------------- accept

#[test]
fn accepts_a_fully_valid_action() {
    let a = action("alice", "SignBlock", "/alice/photos/2026", 1);
    assert_eq!(verify_(&a), VerifyResult::Accept);
}

fn verify_(a: &Action) -> VerifyResult {
    m15_verifier::verify(a, &ctx())
}

// -------------------------------------------------------- each denial in turn

#[test]
fn wrong_schema_is_denied_first() {
    let mut a = action("alice", "SignBlock", "/alice/x", 1);
    a.schema_version = 2;
    assert_eq!(
        verify_(&a),
        VerifyResult::Deny(DenyReason::UnsafeSchemaVersion)
    );
}

#[test]
fn an_unfetched_grant_is_unresolved_not_denied() {
    let a = action("alice", "SignBlock", "/alice/x", 99); // grant 99 not in the context
    assert_eq!(verify_(&a), VerifyResult::Unresolved { missing_grant: 99 });
}

#[test]
fn a_revoked_grant_is_denied() {
    let a = action("carol", "SignBlock", "/carol/x", 3); // grant 3 is revoked
    assert_eq!(verify_(&a), VerifyResult::Deny(DenyReason::GrantRevoked));
}

#[test]
fn an_expired_grant_is_denied() {
    let a = action("bob", "Publish", "/bob/x", 2); // grant 2 expired
    assert_eq!(verify_(&a), VerifyResult::Deny(DenyReason::GrantExpired));
}

#[test]
fn a_mismatched_actor_is_denied() {
    let a = action("eve", "SignBlock", "/alice/x", 1); // grant 1 is alice's
    assert_eq!(verify_(&a), VerifyResult::Deny(DenyReason::ActorMismatch));
}

#[test]
fn an_out_of_scope_action_is_denied() {
    let a = action("alice", "SignBlock", "/bob/x", 1); // grant 1 covers /alice, not /bob
    assert_eq!(verify_(&a), VerifyResult::Deny(DenyReason::ScopeViolation));
}

#[test]
fn an_unlisted_action_is_refused_by_default() {
    // "Delete" is not in grant 1's allowed set — default-refuse, not inferred.
    let a = action("alice", "Delete", "/alice/x", 1);
    assert_eq!(
        verify_(&a),
        VerifyResult::Deny(DenyReason::ActionNotPermitted)
    );
}

// ------------------------------------------------------------- keyring dispatch

#[test]
fn keyring_picks_the_longest_matching_prefix() {
    let tc = TrustContext::new(vec![
        ("/".into(), "root".into()),
        ("/alice".into(), "alice-key".into()),
        ("/alice/photos".into(), "alice-photos-key".into()),
    ]);
    assert_eq!(
        tc.anchor_for("/alice/photos/2026"),
        Some("alice-photos-key")
    );
    assert_eq!(tc.anchor_for("/alice/docs"), Some("alice-key"));
    assert_eq!(tc.anchor_for("/bob"), Some("root")); // only the root fallback covers it
}

#[test]
fn keyring_returns_none_when_nothing_covers() {
    let tc = TrustContext::new(vec![("/alice".into(), "alice-key".into())]);
    assert_eq!(tc.anchor_for("/alice/x"), Some("alice-key"));
    assert_eq!(tc.anchor_for("/bob"), None); // no prefix covers, and no root
    assert_eq!(tc.anchor_for("/alicia"), None); // component-wise: /alice does NOT cover /alicia
}
