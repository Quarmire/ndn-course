//! Witness suite for m14-verdict. The rows are real: they mirror the vectors in
//! ndf-core's `step_9b_spatial_is_the_fourth_verdict` and `three_zone_verdict`
//! tests. The load-bearing assertion is that Unresolved is NOT Refused.

use m14_verdict::{authorize, verdict, Coverage, SpatialVerdict, Verdict};

// -------------------------------------------------------------- the three zones

#[test]
fn covered_when_the_band_clears_the_threshold() {
    assert_eq!(
        verdict(Coverage::new(0.85, 0.02), 0.70),
        SpatialVerdict::Covered
    );
    assert_eq!(
        verdict(Coverage::new(0.95, 0.10), 0.70),
        SpatialVerdict::Covered
    );
}

#[test]
fn not_covered_when_the_band_falls_short() {
    assert_eq!(
        verdict(Coverage::new(0.50, 0.02), 0.70),
        SpatialVerdict::NotCovered
    );
    assert_eq!(
        verdict(Coverage::new(0.30, 0.05), 0.70),
        SpatialVerdict::NotCovered
    );
}

#[test]
fn unresolved_when_the_threshold_is_inside_the_band() {
    // 0.705 ± 0.02 = [0.685, 0.725] straddles 0.70 — honestly undecidable.
    assert_eq!(
        verdict(Coverage::new(0.705, 0.02), 0.70),
        SpatialVerdict::Unresolved
    );
    // 0.68 ± 0.05 = [0.63, 0.73] straddles 0.70 too.
    assert_eq!(
        verdict(Coverage::new(0.68, 0.05), 0.70),
        SpatialVerdict::Unresolved
    );
}

#[test]
fn unknown_coverage_is_always_unresolved() {
    // "Measured nothing" (0.5 ± 1.0) can never clear a threshold in [0, 1].
    assert_eq!(
        verdict(Coverage::unknown(), 0.70),
        SpatialVerdict::Unresolved
    );
    assert_eq!(
        verdict(Coverage::unknown(), 0.01),
        SpatialVerdict::Unresolved
    );
    assert_eq!(
        verdict(Coverage::unknown(), 0.99),
        SpatialVerdict::Unresolved
    );
}

// ------------------------------------------------------------ authorize elevates

#[test]
fn authorize_maps_the_decisive_zones() {
    assert_eq!(
        authorize(Coverage::new(0.85, 0.02), 0.70),
        Verdict::Authorized
    );
    assert_eq!(authorize(Coverage::new(0.50, 0.02), 0.70), Verdict::Refused);
}

#[test]
fn unresolved_is_first_class_not_a_disguised_refusal() {
    let v = authorize(Coverage::new(0.705, 0.02), 0.70);
    // It carries exactly what could not be separated...
    assert_eq!(
        v,
        Verdict::Unresolved {
            estimate: 0.705,
            margin: 0.02,
            threshold: 0.70,
        }
    );
    // ...and it is NOT a refusal. This is the whole architectural point.
    assert_ne!(v, Verdict::Refused);
    // A genuine shortfall, by contrast, IS a refusal.
    assert_eq!(authorize(Coverage::new(0.50, 0.02), 0.70), Verdict::Refused);
    // And "measured nothing" is Unresolved, never a silent Refused.
    assert!(matches!(
        authorize(Coverage::unknown(), 0.70),
        Verdict::Unresolved { .. }
    ));
}
