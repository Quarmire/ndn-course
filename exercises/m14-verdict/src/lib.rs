//! m14-verdict — NDF's fourth verdict: making "I don't know" a first-class answer.
//!
//! Read SPEC.md. Run the witness with `./course check m14-verdict`.
//!
//! This is the architecture module, and its code slice is small on purpose — the
//! weight is on the *idea*. Most systems answer a permission question two ways:
//! yes or no. NDF refuses to collapse "I can't tell" into "no." It answers four
//! ways — and the fourth, **Unresolved**, is first-class: not a disguised denial,
//! but "I don't know, and here's exactly what's missing," handed back to the caller
//! to decide.
//!
//! You implement the pure three-zone core (faithful to ndf-core's `spatial::verdict`)
//! and the authorizer that keeps `Unresolved` first-class. The stubs compile; the
//! tests are red until you fill them in.

/// A coverage estimate with its error margin: the true value is believed to lie in
/// `estimate ± margin`.
#[derive(Clone, Copy, Debug)]
pub struct Coverage {
    pub estimate: f64,
    pub margin: f64,
}

impl Coverage {
    /// A measured estimate with a known margin. (provided)
    pub fn new(estimate: f64, margin: f64) -> Self {
        Self { estimate, margin }
    }

    /// A backend that measured NOTHING: an estimate of 0.5 that could be anything
    /// (margin 1.0). Its band never clears a threshold decisively, so it is always
    /// `Unresolved` — the honest answer when you can't measure. (provided)
    pub fn unknown() -> Self {
        Self {
            estimate: 0.5,
            margin: 1.0,
        }
    }
}

/// The pure three-zone core — the atom of the fourth verdict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpatialVerdict {
    /// The band clears the threshold decisively.
    Covered,
    /// The band falls decisively short of the threshold.
    NotCovered,
    /// The threshold lies *inside* the error band — honestly undecidable here.
    Unresolved,
}

/// Decide the zone: is coverage decisively above the threshold, decisively below,
/// or does the error band `estimate ± margin` STRADDLE it (so we can't tell)?
///
/// - `estimate - margin >= threshold` → `Covered`
/// - `estimate + margin <  threshold` → `NotCovered`
/// - otherwise the threshold is inside the band → `Unresolved`
pub fn verdict(c: Coverage, threshold: f64) -> SpatialVerdict {
    let _ = (c, threshold);
    todo!("the three-zone check above — see HINTS")
}

/// An authorization answer. `Unresolved` is NOT a disguised `Refused`: it carries
/// exactly what could not be separated (the estimate, its margin, and the threshold),
/// so the caller can choose — accept lower assurance, escalate to a finer-grained
/// backend, or route to human ratification.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Verdict {
    Authorized,
    Refused,
    Unresolved {
        estimate: f64,
        margin: f64,
        threshold: f64,
    },
}

/// Elevate a coverage decision into an authorization verdict, keeping `Unresolved`
/// first-class — it must carry the three numbers that couldn't be separated, and it
/// must NOT be turned into `Refused`.
pub fn authorize(c: Coverage, threshold: f64) -> Verdict {
    let _ = (c, threshold);
    todo!("map the three zones to Authorized / Refused / Unresolved{{..}} — see HINTS")
}
