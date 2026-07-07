//! Witness suite for m13-macros. It exercises the items your macros generate:
//! the registry constants + lookups, and the `tlv!` byte builder.

use m13_macros::{name_of, tlv, value_of, CONTENT, DATA, INTEREST, META_INFO, NAME};

// ------------------------------------------------------- generated constants

#[test]
fn registry_generates_the_constants() {
    // These `pub const`s exist only because the macro generated them.
    assert_eq!(DATA, 0x06);
    assert_eq!(NAME, 0x07);
    assert_eq!(INTEREST, 0x05);
    assert_eq!(CONTENT, 0x15);
    assert_eq!(META_INFO, 0x14);
}

// ----------------------------------------------------------------- name_of

#[test]
fn name_of_is_the_reverse_lookup() {
    assert_eq!(name_of(0x06), Some("DATA"));
    assert_eq!(name_of(0x07), Some("NAME"));
    assert_eq!(name_of(0x05), Some("INTEREST"));
    assert_eq!(name_of(0x14), Some("META_INFO"));
    assert_eq!(name_of(0x99), None); // unknown type
}

// ----------------------------------------------------------------- value_of

#[test]
fn value_of_is_the_forward_lookup() {
    assert_eq!(value_of("DATA"), Some(0x06));
    assert_eq!(value_of("CONTENT"), Some(0x15));
    assert_eq!(value_of("META_INFO"), Some(0x14));
    assert_eq!(value_of("nonsense"), None); // unknown name
}

// -------------------------------------------------------------- tlv! builder

#[test]
fn tlv_builds_wire_bytes() {
    // type 0x08, three value bytes → type, length=3, value.
    assert_eq!(
        tlv!(0x08, [0x6E, 0x64, 0x6E]),
        vec![0x08, 0x03, 0x6E, 0x64, 0x6E]
    );
    // an empty value is a length of zero.
    assert_eq!(tlv!(0x14, []), vec![0x14, 0x00]);
    // a single byte, and a trailing comma is accepted.
    assert_eq!(tlv!(0x07, [0x41,]), vec![0x07, 0x01, 0x41]);
}

#[test]
fn tlv_composes_into_a_name() {
    // Build two components and concatenate them into a name's bytes — the sort
    // of corpus construction the macro is for.
    let mut name = tlv!(0x08, [b'n', b'd', b'n']);
    name.extend(tlv!(0x08, [b'c', b'o', b'u', b'r', b's', b'e']));
    assert_eq!(
        name,
        vec![0x08, 0x03, b'n', b'd', b'n', 0x08, 0x06, b'c', b'o', b'u', b'r', b's', b'e']
    );
}
