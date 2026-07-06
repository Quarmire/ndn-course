//! Witness suite for m07-data-view. Proves the view BORROWS (by pointer
//! identity), that `content_as_text` returns a `Cow` that only owns when it must,
//! and that `to_owned_content` copies into an owned handle.

use std::borrow::Cow;

use m07_data_view::{DataView, CONTENT, DATA, NAME};

fn push_varu64(v: u64, out: &mut Vec<u8>) {
    match v {
        0..=0xFC => out.push(v as u8),
        0xFD..=0xFFFF => {
            out.push(0xFD);
            out.extend_from_slice(&(v as u16).to_be_bytes());
        }
        0x1_0000..=0xFFFF_FFFF => {
            out.push(0xFE);
            out.extend_from_slice(&(v as u32).to_be_bytes());
        }
        _ => {
            out.push(0xFF);
            out.extend_from_slice(&v.to_be_bytes());
        }
    }
}

/// Encode a TLV element: type, length, value.
fn tlv(typ: u64, value: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    push_varu64(typ, &mut out);
    push_varu64(value.len() as u64, &mut out);
    out.extend_from_slice(value);
    out
}

/// A Data packet with the given name-value and content bytes.
fn data_packet(name_value: &[u8], content: &[u8]) -> Vec<u8> {
    let mut inner = tlv(NAME, name_value);
    inner.extend_from_slice(&tlv(CONTENT, content));
    tlv(DATA, &inner)
}

// ---------------------------------------------------------------- lazy fields

#[test]
fn finds_name_and_content_values() {
    let bytes = data_packet(b"the-name-bytes", b"hello world");
    let view = DataView::parse(&bytes).expect("valid Data");
    assert_eq!(view.name(), Some(&b"the-name-bytes"[..]));
    assert_eq!(view.content(), Some(&b"hello world"[..]));
    // find of an absent type is None, not a panic.
    assert_eq!(view.find(0x63), None);
}

#[test]
fn fields_borrow_from_the_buffer_no_copy() {
    let bytes = data_packet(b"n", b"payload");
    let view = DataView::parse(&bytes).unwrap();
    let content = view.content().unwrap();
    let base = bytes.as_ptr() as usize;
    let addr = content.as_ptr() as usize;
    assert!(
        addr >= base && addr < base + bytes.len(),
        "content() must borrow from the buffer, not copy it"
    );
}

#[test]
fn content_is_none_when_absent() {
    // A Data with only a Name, no Content sub-element.
    let bytes = tlv(DATA, &tlv(NAME, b"just-a-name"));
    let view = DataView::parse(&bytes).unwrap();
    assert_eq!(view.name(), Some(&b"just-a-name"[..]));
    assert_eq!(view.content(), None);
    assert_eq!(view.content_as_text(), None);
    assert_eq!(view.to_owned_content(), None);
}

#[test]
fn non_data_bytes_do_not_parse() {
    // Type 0x07 (a Name), not a Data.
    let bytes = tlv(NAME, b"whatever");
    assert!(DataView::parse(&bytes).is_none());
    assert!(DataView::parse(&[]).is_none());
}

// ----------------------------------------------------------------------- Cow

#[test]
fn text_borrows_when_valid_utf8() {
    let bytes = data_packet(b"n", b"hello");
    let view = DataView::parse(&bytes).unwrap();
    let text = view.content_as_text().unwrap();
    assert_eq!(text, "hello");
    // Valid UTF-8 costs no allocation — the Cow is Borrowed.
    assert!(
        matches!(&text, Cow::Borrowed(_)),
        "valid UTF-8 content should borrow, not allocate"
    );
}

#[test]
fn text_owns_only_when_it_must() {
    // 0xFF 0xFE are not valid UTF-8; lossy conversion must allocate.
    let bytes = data_packet(b"n", &[0xFF, 0xFE]);
    let view = DataView::parse(&bytes).unwrap();
    let text = view.content_as_text().unwrap();
    assert!(
        matches!(&text, Cow::Owned(_)),
        "invalid UTF-8 must produce an owned (replaced) string"
    );
    assert!(
        text.contains('\u{FFFD}'),
        "should contain the replacement char"
    );
}

// --------------------------------------------------------------------- Bytes

#[test]
fn to_owned_content_copies_into_an_owned_handle() {
    // The owned handle is built inside the block and moved out; the source buffer
    // and the borrowing view are dropped at the block's end. That it's still valid
    // afterward proves it owns its bytes rather than borrowing them.
    let owned = {
        let bytes = data_packet(b"n", b"payload");
        let view = DataView::parse(&bytes).unwrap();
        view.to_owned_content().unwrap()
    };
    assert_eq!(&owned[..], b"payload");
}
