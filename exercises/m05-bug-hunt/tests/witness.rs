//! Witness suite for m05-bug-hunt. These tests define what the four functions
//! SHOULD do. On the shipped (buggy) code, several fail — some by a wrong answer,
//! some by a panic. Green means you found and fixed all four bugs.

use m05_bug_hunt::{common_prefix_len, first_component, is_prefix, split_components};

/// Encode one name component as a TLV element (type 0x08, VAR-NUMBER length).
fn component(value: &[u8]) -> Vec<u8> {
    let mut out = vec![0x08];
    let n = value.len() as u64;
    if n <= 0xFC {
        out.push(n as u8);
    } else {
        out.push(0xFD);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    }
    out.extend_from_slice(value);
    out
}

fn name(components: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    for c in components {
        out.extend_from_slice(&component(c));
    }
    out
}

// --------------------------------------------------------- first_component

#[test]
fn first_component_returns_the_first() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    assert_eq!(first_component(&[a, b]), Some(&b"ndn"[..]));
}

#[test]
fn first_component_of_empty_name_is_none() {
    // Must return None, not panic, on a name with no components.
    assert_eq!(first_component(&[]), None);
}

// ---------------------------------------------------------------- is_prefix

#[test]
fn is_prefix_accepts_a_shorter_prefix() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    let c: &[u8] = b"v1";
    // [ndn] and [ndn, course] are prefixes of [ndn, course, v1].
    assert!(is_prefix(&[a], &[a, b, c]));
    assert!(is_prefix(&[a, b], &[a, b, c]));
}

#[test]
fn is_prefix_rejects_a_longer_or_divergent_name() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    let x: &[u8] = b"other";
    // A longer "prefix" is not a prefix.
    assert!(!is_prefix(&[a, b], &[a]));
    // A divergent component is not a prefix.
    assert!(!is_prefix(&[x], &[a, b]));
}

#[test]
fn is_prefix_equal_names_and_empty_prefix() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    assert!(is_prefix(&[a, b], &[a, b])); // equal is a (improper) prefix
    assert!(is_prefix(&[], &[a, b])); // empty prefixes everything
}

// -------------------------------------------------------- common_prefix_len

#[test]
fn common_prefix_len_when_one_is_a_full_prefix() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    // [ndn] fully-prefixes [ndn, course]: they share 1 component (must not panic).
    assert_eq!(common_prefix_len(&[a], &[a, b]), 1);
    assert_eq!(common_prefix_len(&[a, b], &[a, b]), 2);
}

#[test]
fn common_prefix_len_partial_and_disjoint() {
    let a: &[u8] = b"ndn";
    let b: &[u8] = b"course";
    let c: &[u8] = b"v1";
    let d: &[u8] = b"v2";
    assert_eq!(common_prefix_len(&[a, b, c], &[a, b, d]), 2);
    assert_eq!(common_prefix_len(&[a], &[b]), 0);
    assert_eq!(common_prefix_len(&[], &[a]), 0);
}

// ------------------------------------------------------- split_components

#[test]
fn split_components_recovers_each_value() {
    let bytes = name(&[b"ndn", b"course", b"v1"]);
    let got = split_components(&bytes);
    assert_eq!(got, vec![&b"ndn"[..], &b"course"[..], &b"v1"[..]]);
}

#[test]
fn split_components_handles_multi_byte_length_and_truncation() {
    let big = vec![0x5A; 300];
    let mut bytes = name(&[b"a", &big]);
    let full = split_components(&bytes);
    assert_eq!(full, vec![&b"a"[..], &big[..]]);

    // A trailing, truncated element is dropped, earlier ones survive.
    bytes.extend_from_slice(&[0x08, 0x05, 0xAA]); // claims 5 value bytes, has 1
    let partial = split_components(&bytes);
    assert_eq!(partial, vec![&b"a"[..], &big[..]]);
}
