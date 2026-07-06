//! Witness suite for m06-name-api. Ordering is graded against the real
//! `ndn-foundation-types` as an oracle: your `cmp` must return the same
//! `Ordering` the production type does, for every pair.

use std::cmp::Ordering;
use std::collections::HashSet;

use bytes::Bytes;
use m06_name_api as student;
use ndn_foundation_types as oracle;

fn s_comp(typ: u64, v: &[u8]) -> student::NameComponent {
    student::NameComponent::new(typ, v.to_vec())
}
fn o_comp(typ: u64, v: &[u8]) -> oracle::NameComponent {
    oracle::NameComponent::new(typ, Bytes::copy_from_slice(v))
}

/// A student `Name` from generic string labels.
fn n(labels: &[&str]) -> student::Name {
    student::Name::from_components(
        labels
            .iter()
            .map(|l| student::NameComponent::generic(l.as_bytes().to_vec())),
    )
}

// -------------------------------------------------- component ordering (oracle)

/// Components chosen to exercise the type, length, and content tiers — including
/// the pairs where length-first and content-first orderings disagree.
fn component_corpus() -> Vec<(u64, Vec<u8>)> {
    vec![
        (8, vec![]),
        (8, vec![0x00]),
        (8, vec![0xFF]),
        (8, vec![0x00, 0x00]),
        (8, vec![0x00, 0x01]),
        (8, b"ndn".to_vec()),
        (8, b"course".to_vec()),
        (7, vec![0xFF, 0xFF]),
        (1, vec![0x05]),
        (0xFFFF, vec![0x00]),
    ]
}

#[test]
fn component_ordering_matches_the_canonical_oracle() {
    let corpus = component_corpus();
    for a in &corpus {
        for b in &corpus {
            let mine = s_comp(a.0, &a.1).cmp(&s_comp(b.0, &b.1));
            let canonical = o_comp(a.0, &a.1).cmp(&o_comp(b.0, &b.1));
            assert_eq!(
                mine, canonical,
                "component ({}, {:02X?}) vs ({}, {:02X?}): you said {:?}, canonical is {:?}",
                a.0, a.1, b.0, b.1, mine, canonical
            );
        }
    }
}

#[test]
fn length_orders_before_content() {
    // The classic derive trap: content-first would put [0xFF] AFTER [0x00,0x00]
    // (0xFF > 0x00). Canonical order is length-first, so [0xFF] (len 1) is LESS.
    let short = s_comp(8, &[0xFF]);
    let long = s_comp(8, &[0x00, 0x00]);
    assert_eq!(short.cmp(&long), Ordering::Less);
    assert_eq!(long.cmp(&short), Ordering::Greater);
}

#[test]
fn type_orders_before_everything() {
    // A long type-7 value still sorts before a short type-8 one.
    let t7 = s_comp(7, &[0xFF, 0xFF, 0xFF]);
    let t8 = s_comp(8, &[0x00]);
    assert_eq!(t7.cmp(&t8), Ordering::Less);
}

// ------------------------------------------------------- name ordering (oracle)

#[test]
fn name_ordering_matches_the_oracle() {
    let names: Vec<Vec<&str>> = vec![
        vec![],
        vec!["ndn"],
        vec!["ndn", "course"],
        vec!["ndn", "course", "v1"],
        vec!["ndn", "cou"],
        vec!["ndn", "about"],
        vec!["other"],
    ];
    let o_name = |labels: &[&str]| {
        oracle::Name::from_components(labels.iter().map(|l| o_comp(8, l.as_bytes())))
    };
    for a in &names {
        for b in &names {
            let mine = n(a).cmp(&n(b));
            let canonical = o_name(a).cmp(&o_name(b));
            assert_eq!(mine, canonical, "name {:?} vs {:?}", a, b);
        }
    }
}

// -------------------------------------------------------------- behavior tests

#[test]
fn has_prefix_behaves() {
    let full = n(&["ndn", "course", "v1"]);
    assert!(full.has_prefix(&n(&["ndn"])));
    assert!(full.has_prefix(&n(&["ndn", "course"])));
    assert!(full.has_prefix(&full)); // equal is an (improper) prefix
    assert!(full.has_prefix(&student::Name::root())); // root prefixes everything
    assert!(!full.has_prefix(&n(&["ndn", "course", "v1", "x"]))); // longer is not a prefix
    assert!(!full.has_prefix(&n(&["ndn", "other"]))); // divergent
}

#[test]
fn display_renders_a_slash_path() {
    assert_eq!(n(&["ndn", "course"]).to_string(), "/ndn/course");
    assert_eq!(n(&["a"]).to_string(), "/a");
    assert_eq!(student::Name::root().to_string(), "/");
}

#[test]
fn from_str_parses_a_uri_skipping_empties() {
    let name = student::Name::from("/ndn/course");
    assert_eq!(name.len(), 2);
    assert_eq!(name.components()[0].typ, student::GENERIC);
    assert_eq!(name.components()[0].value, b"ndn");
    assert_eq!(name.components()[1].value, b"course");

    assert_eq!(student::Name::from("/a//b/").len(), 2); // doubled + trailing slashes
    assert_eq!(student::Name::from("/").len(), 0); // just root
    assert_eq!(student::Name::from("").len(), 0);

    // Round-trips with Display for printable ASCII.
    assert_eq!(
        student::Name::from("/ndn/course").to_string(),
        "/ndn/course"
    );
}

#[test]
fn eq_and_hash_agree() {
    // Eq, Ord, and Hash must be consistent: equal values hash equal and compare
    // Equal. (Derived here — but the contract is yours to preserve.)
    let a = student::NameComponent::new(8, vec![1, 2, 3]);
    let b = student::NameComponent::new(8, vec![1, 2, 3]);
    assert_eq!(a.cmp(&b), Ordering::Equal);
    let mut set = HashSet::new();
    set.insert(a);
    assert!(set.contains(&b), "equal components must hash equal");
}
