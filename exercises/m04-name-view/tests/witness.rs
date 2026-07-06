//! Witness suite for m04-name-view. These tests ARE the contract. No external
//! deps. The borrowing test proves — by pointer identity — that you handed back
//! a window onto the input, not a copy of it.

use m04_name_view::{common_prefix_len, longest_component, NameView};

/// Encode one name component as a TLV element (type 0x08, VAR-NUMBER length).
fn component(value: &[u8]) -> Vec<u8> {
    let mut out = vec![0x08];
    let n = value.len() as u64;
    if n <= 0xFC {
        out.push(n as u8);
    } else if n <= 0xFFFF {
        out.push(0xFD);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        out.push(0xFE);
        out.extend_from_slice(&(n as u32).to_be_bytes());
    }
    out.extend_from_slice(value);
    out
}

/// Concatenate components into a name's component bytes.
fn name(components: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    for c in components {
        out.extend_from_slice(&component(c));
    }
    out
}

// ------------------------------------------------------ Part A: longest_component

#[test]
fn longest_component_returns_the_longest_borrowed() {
    let a: &[u8] = b"a";
    let bbb: &[u8] = b"bbb";
    let cc: &[u8] = b"cc";
    let comps = [a, bbb, cc];
    assert_eq!(longest_component(&comps), Some(&b"bbb"[..]));
    assert_eq!(longest_component(&[]), None);
    assert_eq!(longest_component(&[a]), Some(&b"a"[..]));
}

// ------------------------------------------------------------- Part B: the view

#[test]
fn view_yields_each_component_value() {
    let bytes = name(&[b"ndn", b"course", b"v1"]);
    let view = NameView::new(&bytes);
    assert_eq!(view.len(), 3);
    assert!(!view.is_empty());
    assert_eq!(view.get(0), Some(&b"ndn"[..]));
    assert_eq!(view.get(1), Some(&b"course"[..]));
    assert_eq!(view.get(2), Some(&b"v1"[..]));
    assert_eq!(view.get(3), None);

    let collected: Vec<&[u8]> = view.iter().collect();
    assert_eq!(collected, vec![&b"ndn"[..], &b"course"[..], &b"v1"[..]]);
}

#[test]
fn empty_name_has_no_components() {
    let bytes: Vec<u8> = Vec::new();
    let view = NameView::new(&bytes);
    assert_eq!(view.len(), 0);
    assert!(view.is_empty());
    assert_eq!(view.get(0), None);
    assert_eq!(view.iter().count(), 0);
}

#[test]
fn components_borrow_from_the_input_no_copy() {
    let bytes = name(&[b"ndn", b"course"]);
    let view = NameView::new(&bytes);
    let c0 = view.get(0).unwrap();
    // A borrowed slice points INTO `bytes`; a copy would live elsewhere.
    let base = bytes.as_ptr() as usize;
    let c0_addr = c0.as_ptr() as usize;
    assert!(
        c0_addr >= base && c0_addr < base + bytes.len(),
        "get(0) must borrow from the input, not copy it"
    );
    assert_eq!(c0, b"ndn");
}

#[test]
fn multi_byte_length_component_is_handled() {
    // A 300-byte component forces a 3-byte VAR-NUMBER length (FD 01 2C).
    let big = vec![0x5A; 300];
    let bytes = name(&[b"a", &big, b"z"]);
    let view = NameView::new(&bytes);
    assert_eq!(view.len(), 3);
    assert_eq!(view.get(1), Some(&big[..]));
    assert_eq!(view.get(2), Some(&b"z"[..]));
}

#[test]
fn truncated_tail_stops_the_view_leniently() {
    // A whole component, then a second one whose value is cut short.
    let mut bytes = name(&[b"ndn"]);
    bytes.extend_from_slice(&[0x08, 0x05, 0xAA, 0xBB]); // says 5 value bytes, has 2
    let view = NameView::new(&bytes);
    // The view yields the well-formed prefix and stops.
    assert_eq!(view.len(), 1);
    assert_eq!(view.get(0), Some(&b"ndn"[..]));
}

// ------------------------------------------------ Part C: common_prefix_len

#[test]
fn common_prefix_len_counts_matching_leading_components() {
    let a = name(&[b"ndn", b"course", b"a"]);
    let b = name(&[b"ndn", b"course", b"b"]);
    assert_eq!(common_prefix_len(&NameView::new(&a), &NameView::new(&b)), 2);

    let c = name(&[b"ndn", b"course"]);
    let d = name(&[b"ndn", b"course"]);
    assert_eq!(common_prefix_len(&NameView::new(&c), &NameView::new(&d)), 2);

    let e = name(&[b"x"]);
    let f = name(&[b"y"]);
    assert_eq!(common_prefix_len(&NameView::new(&e), &NameView::new(&f)), 0);

    let empty: Vec<u8> = Vec::new();
    assert_eq!(
        common_prefix_len(&NameView::new(&a), &NameView::new(&empty)),
        0
    );
}

#[test]
fn common_prefix_stops_at_first_difference_even_if_more_matches_follow() {
    let a = name(&[b"ndn", b"X", b"same"]);
    let b = name(&[b"ndn", b"Y", b"same"]);
    // Shares only "ndn"; the later matching "same" does not count.
    assert_eq!(common_prefix_len(&NameView::new(&a), &NameView::new(&b)), 1);
}
