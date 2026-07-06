//! Witness suite for m02-varint. These tests ARE the contract — read them
//! like a spec. No external deps: the "property" tests use a fixed-seed LCG
//! so failures reproduce exactly.

use m02_varint::{decode_varu64, encode_varu64, VarintError};

fn enc(value: u64) -> Vec<u8> {
    let mut out = Vec::new();
    encode_varu64(value, &mut out);
    out
}

// ---------------------------------------------------------------- boundaries

#[test]
fn boundary_vectors_encode_exactly() {
    let cases: &[(u64, &[u8])] = &[
        (0, &[0x00]),
        (1, &[0x01]),
        (252, &[0xFC]),
        (253, &[0xFD, 0x00, 0xFD]),
        (254, &[0xFD, 0x00, 0xFE]),
        (65_535, &[0xFD, 0xFF, 0xFF]),
        (65_536, &[0xFE, 0x00, 0x01, 0x00, 0x00]),
        (4_294_967_295, &[0xFE, 0xFF, 0xFF, 0xFF, 0xFF]),
        (
            4_294_967_296,
            &[0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00],
        ),
        (
            u64::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        ),
    ];
    for (value, expected) in cases {
        assert_eq!(&enc(*value), expected, "encoding of {value}");
    }
}

#[test]
fn boundary_vectors_decode_exactly() {
    let cases: &[(&[u8], u64, usize)] = &[
        (&[0x00], 0, 1),
        (&[0xFC], 252, 1),
        (&[0xFD, 0x00, 0xFD], 253, 3),
        (&[0xFD, 0xFF, 0xFF], 65_535, 3),
        (&[0xFE, 0x00, 0x01, 0x00, 0x00], 65_536, 5),
        (
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
            u64::MAX,
            9,
        ),
    ];
    for (bytes, value, len) in cases {
        assert_eq!(
            decode_varu64(bytes),
            Ok((*value, *len)),
            "decoding {bytes:02X?}"
        );
    }
}

// ------------------------------------------------------------------ appends

#[test]
fn encode_appends_without_disturbing_existing_bytes() {
    let mut out = vec![0xAA, 0xBB];
    encode_varu64(253, &mut out);
    assert_eq!(out, vec![0xAA, 0xBB, 0xFD, 0x00, 0xFD]);
}

// ----------------------------------------------------- round-trip "property"

#[test]
fn ten_thousand_pseudo_random_round_trips() {
    // Deterministic LCG: same 10 000 values every run, so a failure is a
    // reproducible bug, not a flake.
    let mut x: u64 = 0x9E3779B97F4A7C15;
    for i in 0..10_000u32 {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        // Vary magnitude so all four shapes get exercised.
        let value = match i % 4 {
            0 => x % 253,
            1 => 253 + (x % (65_536 - 253)),
            2 => 65_536 + (x % (4_294_967_296 - 65_536)),
            _ => x,
        };
        let bytes = enc(value);
        let (decoded, consumed) =
            decode_varu64(&bytes).unwrap_or_else(|e| panic!("decode {value}: {e:?}"));
        assert_eq!(decoded, value, "round-trip value");
        assert_eq!(consumed, bytes.len(), "round-trip length");
    }
}

#[test]
fn decode_reports_consumed_and_ignores_trailing_bytes() {
    let mut bytes = enc(65_536);
    let len = bytes.len();
    bytes.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(decode_varu64(&bytes), Ok((65_536, len)));
}

// ------------------------------------------------------------------- errors

#[test]
fn truncated_inputs_are_unexpected_end() {
    let cases: &[&[u8]] = &[
        &[],
        &[0xFD],
        &[0xFD, 0x01],
        &[0xFE, 0x00, 0x01],
        &[0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    ];
    for bytes in cases {
        assert_eq!(
            decode_varu64(bytes),
            Err(VarintError::UnexpectedEnd),
            "input {bytes:02X?}"
        );
    }
}

#[test]
fn non_minimal_encodings_are_rejected() {
    let cases: &[&[u8]] = &[
        &[0xFD, 0x00, 0x00],                                     // 0 in 3 bytes
        &[0xFD, 0x00, 0xFC],                                     // 252 in 3 bytes
        &[0xFE, 0x00, 0x00, 0xFF, 0xFF],                         // 65535 in 5 bytes
        &[0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF], // u32 range in 9 bytes
    ];
    for bytes in cases {
        assert_eq!(
            decode_varu64(bytes),
            Err(VarintError::NonMinimal),
            "input {bytes:02X?}"
        );
    }
}

#[test]
fn boundary_of_each_shape_is_minimal_not_rejected() {
    // The smallest value that legitimately NEEDS each wider shape.
    assert_eq!(decode_varu64(&[0xFD, 0x00, 0xFD]), Ok((253, 3)));
    assert_eq!(decode_varu64(&[0xFE, 0x00, 0x01, 0x00, 0x00]), Ok((65_536, 5)));
    assert_eq!(
        decode_varu64(&[0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]),
        Ok((4_294_967_296, 9))
    );
}
