//! m15-verifier — trust and the verifier: the last core module.
//!
//! Read SPEC.md. Run the witness with `./course check m15-verifier`.
//!
//! A verifier answers one question — "is this action allowed?" — but the shape of
//! the answer is the lesson. This is a short-circuiting pipeline of checks (the
//! AC.12 shape from ndf-core's `verify_action`): the FIRST failing step decides,
//! with a TYPED reason; a step it can't evaluate yet returns `Unresolved` (M14's
//! fourth verdict, in the verifier's domain); and the last gate is **default-refuse**
//! — an action not explicitly granted is denied, never inferred.
//!
//! You write `verify` (the pipeline) and `TrustContext::anchor_for` (longest-prefix
//! keyring dispatch). The stubs compile; the tests are red until you fill them in.

use std::collections::{HashMap, HashSet};

/// Identifies a capability grant.
pub type GrantId = u32;

/// An action a principal wants to take, naming the grant it claims authorizes it.
pub struct Action {
    pub schema_version: u32,
    pub actor: String,
    pub action_type: String, // e.g. "SignBlock"
    pub scope: String,       // the name the action targets, e.g. "/alice/photos/2026"
    pub authorizing_grant: GrantId,
}

/// A capability grant: who may do what, where, until when.
pub struct Grant {
    pub holder: String,
    pub allowed_actions: Vec<String>, // DEFAULT-REFUSE: an action not listed here is denied
    pub scope_prefix: String,         // the name prefix this grant covers
    pub expiry: u64,
}

/// Everything the verifier knows at decision time.
pub struct VerifierContext {
    pub now: u64,
    pub supported_schema: u32,
    pub grants: HashMap<GrantId, Grant>,
    pub revoked: HashSet<GrantId>,
}

/// Why an action was denied — a typed, step-tagged reason (never a bare bool).
#[derive(Debug, PartialEq, Eq)]
pub enum DenyReason {
    UnsafeSchemaVersion,
    GrantRevoked,
    GrantExpired,
    ActorMismatch,
    ScopeViolation,
    /// Default-refuse: the action isn't in the grant's allowed set.
    ActionNotPermitted,
}

/// The verdict — the four-verdict shape from M14, in the authorization domain.
#[derive(Debug, PartialEq, Eq)]
pub enum VerifyResult {
    Accept,
    Deny(DenyReason),
    /// Can't decide: the authorizing grant hasn't been fetched. This is NOT a
    /// denial — fetch the grant and re-verify. (Unfetched ≠ refuse.)
    Unresolved {
        missing_grant: GrantId,
    },
}

/// Component-wise name prefix: does `prefix` prefix `name`? `/a/b` covers `/a/b/c`
/// and `/a/b`, but NOT `/a/bc` or `/a`. (provided — the shared trust primitive)
#[allow(dead_code)]
fn covers(prefix: &str, name: &str) -> bool {
    let p: Vec<&str> = prefix.split('/').filter(|s| !s.is_empty()).collect();
    let n: Vec<&str> = name.split('/').filter(|s| !s.is_empty()).collect();
    p.len() <= n.len() && p.iter().zip(&n).all(|(a, b)| a == b)
}

/// Decide whether `action` is authorized, running the checks in order and letting
/// the first failure decide. See SPEC/HINTS for the exact step order + reasons.
pub fn verify(action: &Action, ctx: &VerifierContext) -> VerifyResult {
    if action.schema_version != ctx.supported_schema {
        return VerifyResult::Deny(DenyReason::UnsafeSchemaVersion);
    }
    let grant = match ctx.grants.get(&action.authorizing_grant) {
        Some(g) => g,
        None => {
            return VerifyResult::Unresolved {
                missing_grant: action.authorizing_grant,
            }
        }
    };
    if ctx.revoked.contains(&action.authorizing_grant) {
        return VerifyResult::Deny(DenyReason::GrantRevoked);
    }
    if grant.expiry <= ctx.now {
        return VerifyResult::Deny(DenyReason::GrantExpired);
    }
    if action.actor != grant.holder {
        return VerifyResult::Deny(DenyReason::ActorMismatch);
    }
    if !covers(&grant.scope_prefix, &action.scope) {
        return VerifyResult::Deny(DenyReason::ScopeViolation);
    }
    if !grant.allowed_actions.contains(&action.action_type) {
        return VerifyResult::Deny(DenyReason::ActionNotPermitted);
    }
    VerifyResult::Accept
}

/// A trust keyring: name prefixes mapped to the anchor (key/policy) that governs them.
pub struct TrustContext {
    #[allow(dead_code)] // read once you implement `anchor_for`
    anchors: Vec<(String, String)>, // (name_prefix, anchor_id)
}

impl TrustContext {
    /// Build a keyring from `(prefix, anchor)` pairs. (provided)
    pub fn new(anchors: Vec<(String, String)>) -> Self {
        Self { anchors }
    }

    /// The anchor that governs `name`: among all whose prefix covers `name`, the
    /// one with the LONGEST (most-specific) prefix wins. `None` if none match.
    pub fn anchor_for(&self, name: &str) -> Option<&str> {
        self.anchors
            .iter()
            .filter(|(prefix, _)| covers(prefix, name))
            .max_by_key(|(prefix, _)| prefix.split('/').filter(|s| !s.is_empty()).count())
            .map(|(_, anchor)| anchor.as_str())
    }
}
