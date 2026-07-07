//! Witness suite for m08-strategy. Behavior tests for each dispatch form, plus a
//! property ("fuzz") test that throws random inputs at the strategies and checks
//! their invariants hold.

use m08_strategy::{
    compare_strategies, default_strategy, forward_static, BestRoute, FaceId, FnStrategy, Multicast,
    Strategy,
};

// ------------------------------------------------------------- named strategies

#[test]
fn multicast_sends_to_all_but_the_incoming() {
    assert_eq!(Multicast.choose(2, &[1, 2, 3]), vec![1, 3]);
    // Incoming not among the candidates: everyone gets it.
    assert_eq!(Multicast.choose(9, &[1, 2, 3]), vec![1, 2, 3]);
    // No candidates: nobody.
    assert_eq!(Multicast.choose(1, &[]), vec![]);
    assert_eq!(Multicast.name(), "multicast");
}

#[test]
fn best_route_picks_the_first_eligible() {
    assert_eq!(BestRoute.choose(3, &[3, 1, 2]), vec![1]); // skip the incoming 3
    assert_eq!(BestRoute.choose(1, &[1, 2]), vec![2]);
    assert_eq!(BestRoute.choose(5, &[5]), vec![]); // only candidate is the incoming
    assert_eq!(BestRoute.choose(0, &[]), vec![]);
    assert_eq!(BestRoute.name(), "best-route");
}

// --------------------------------------------------------------- closure-backed

#[test]
fn fn_strategy_runs_its_closure_and_keeps_its_name() {
    // A closure capturing nothing: send only to the incoming face (a silly policy,
    // but it proves the closure is actually called).
    let echo = FnStrategy::new("echo", |incoming, _candidates| vec![incoming]);
    assert_eq!(echo.choose(7, &[1, 2, 3]), vec![7]);
    assert_eq!(echo.name(), "echo");

    // A closure that captures its environment (a cap on how many to return).
    let cap = 2usize;
    let capped = FnStrategy::new("capped", move |_incoming, candidates: &[FaceId]| {
        candidates.iter().copied().take(cap).collect()
    });
    assert_eq!(capped.choose(0, &[10, 20, 30, 40]), vec![10, 20]);
}

// -------------------------------------------------------- static vs dynamic

#[test]
fn forward_static_dispatches_generically() {
    assert_eq!(forward_static(&Multicast, 2, &[1, 2, 3]), vec![1, 3]);
    assert_eq!(forward_static(&BestRoute, 2, &[1, 2, 3]), vec![1]);
}

#[test]
fn compare_strategies_holds_a_heterogeneous_set() {
    // Three DIFFERENT concrete types in one Vec — only possible behind `dyn`.
    let strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(Multicast),
        Box::new(BestRoute),
        Box::new(FnStrategy::new("drop", |_i, _c| vec![])),
    ];
    let report = compare_strategies(&strategies, 2, &[1, 2, 3]);
    assert_eq!(
        report,
        vec![
            ("multicast".to_string(), vec![1, 3]),
            ("best-route".to_string(), vec![1]),
            ("drop".to_string(), vec![]),
        ]
    );
}

// --------------------------------------------------------------- OnceLock

#[test]
fn default_strategy_is_usable_and_coerces_to_dyn() {
    let d = default_strategy();
    assert_eq!(d.choose(2, &[1, 2, 3]), vec![1, 3]);
    assert_eq!(d.name(), "multicast");
    // It's a real strategy: it can be used through a trait object too.
    let as_dyn: &dyn Strategy = default_strategy();
    assert_eq!(as_dyn.name(), "multicast");
}

// ------------------------------------------------------ property / fuzz test

#[test]
fn strategies_hold_their_invariants_under_random_inputs() {
    // A seeded LCG: the same 2000 cases every run (a reproducible fuzz).
    let mut x: u64 = 0xD1B5_4A32_D192_ED03;
    let mut next = || {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        x
    };

    for _ in 0..2000 {
        let incoming = (next() % 8) as FaceId;
        let n = (next() % 6) as usize;
        let candidates: Vec<FaceId> = (0..n).map(|_| (next() % 8) as FaceId).collect();

        // Multicast: exactly the candidates that aren't the incoming face, in order.
        let m = Multicast.choose(incoming, &candidates);
        assert!(
            m.iter().all(|f| candidates.contains(f)),
            "multicast returned a non-candidate: {m:?} from {candidates:?}"
        );
        assert!(
            !m.contains(&incoming),
            "multicast returned the incoming face {incoming}: {m:?}"
        );
        let expected: Vec<FaceId> = candidates
            .iter()
            .copied()
            .filter(|&f| f != incoming)
            .collect();
        assert_eq!(
            m, expected,
            "multicast (incoming={incoming}, {candidates:?})"
        );

        // BestRoute: at most one, a candidate, not the incoming, and the FIRST such.
        let b = BestRoute.choose(incoming, &candidates);
        assert!(b.len() <= 1, "best-route returned more than one: {b:?}");
        match b.first() {
            Some(&f) => {
                assert!(candidates.contains(&f) && f != incoming);
                let first_eligible = candidates.iter().copied().find(|&c| c != incoming);
                assert_eq!(
                    Some(f),
                    first_eligible,
                    "best-route wasn't the first eligible"
                );
            }
            None => assert!(
                candidates.iter().all(|&c| c == incoming),
                "best-route returned nothing but an eligible candidate existed: {candidates:?}"
            ),
        }
    }
}
