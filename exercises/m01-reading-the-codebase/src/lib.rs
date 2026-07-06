//! m01-reading-the-codebase — learn to navigate a big Rust workspace by finding
//! real things in ndn-rs, then measuring what real types cost.
//!
//! An OBSERVATION lab, not a coding exercise. Read SPEC.md, hunt through
//! `../ndn-rs`, and record what you find. The witness checks your file locations
//! against the real repo and your measured sizes against the real types — so it
//! passes only if you actually navigated and measured.
//!
//! Sentinels (`""` / `0`) are wrong on purpose: the tests are red until you
//! replace them.

// ── the scavenger hunt: where does the forwarder do X? ──────────────────────
// Record each as a path RELATIVE TO ../ndn-rs, e.g. "crates/core/ndn-tlv/src/lib.rs".

/// The file where a PIT (Pending Interest Table) entry is created when an
/// Interest arrives and misses the table.
pub const PIT_CREATE_FILE: &str = "";

/// The file where a Data packet is inserted into the content store — and where
/// the `ctx.verified` flag decides whether it is allowed to be cached at all.
pub const CS_INSERT_FILE: &str = "";

/// The file that drives the forwarding pipeline: the ordered stage calls an
/// Interest (or Data packet) passes through — the real code, not the ASCII diagram.
pub const PIPELINE_FILE: &str = "";

// ── the machine strand: what does a real type cost? ─────────────────────────
// MEASURE these with std::mem::size_of — don't guess. HINTS shows you how.

/// `std::mem::size_of::<ndn_foundation_types::Hash>()`.
pub const SIZE_OF_HASH: usize = 0;

/// `std::mem::size_of::<ndn_foundation_types::NameComponent>()`.
pub const SIZE_OF_NAME_COMPONENT: usize = 0;
