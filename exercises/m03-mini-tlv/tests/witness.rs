//! Witness suite for m03-mini-tlv. These tests ARE the contract — read them
//! like a spec. No external deps: the property test uses a fixed-seed LCG so
//! failures reproduce exactly.

use m03_mini_tlv::{TlvError, TlvReader, TlvWriter};

fn write_one(type_num: u64, value: &[u8]) -> Vec<u8> {
    let mut w = TlvWriter::new();
    w.write(type_num, value);
    w.into_bytes()
}

// ------------------------------------------------------------- exact bytes

#[test]
fn writes_type_length_value_exactly() {
    // /ndn name component "ndn": type 0x08, length 3, then the bytes.
    assert_eq!(write_one(0x08, b"ndn"), vec![0x08, 0x03, b'n', b'd', b'n']);
    // Empty value is a length of zero, not a missing field.
    assert_eq!(write_one(0x14, b""), vec![0x14, 0x00]);
}

#[test]
fn round_trips_a_single_element() {
    let bytes = write_one(0x08, b"course");
    let mut r = TlvReader::new(&bytes);
    assert_eq!(r.read(), Ok((0x08, &b"course"[..])));
    assert!(r.is_empty());
}

// ------------------------------------------------------------- append order

#[test]
fn write_appends_without_disturbing_prior_elements() {
    let mut w = TlvWriter::new();
    w.write(0x08, b"ndn");
    w.write(0x08, b"course");
    let bytes = w.into_bytes();

    let mut r = TlvReader::new(&bytes);
    assert_eq!(r.read(), Ok((0x08, &b"ndn"[..])));
    assert_eq!(r.read(), Ok((0x08, &b"course"[..])));
    assert!(r.is_empty());
}

// ------------------------------------------------------------------ nesting

#[test]
fn a_value_can_itself_be_tlv() {
    // Build a Name (type 0x07) whose value is two name components.
    let mut inner = TlvWriter::new();
    inner.write(0x08, b"ndn");
    inner.write(0x08, b"course");
    let name_value = inner.into_bytes();

    let bytes = write_one(0x07, &name_value);

    // Outer read hands back the inner bytes as a borrowed slice.
    let mut outer = TlvReader::new(&bytes);
    let (ty, value) = outer.read().unwrap();
    assert_eq!(ty, 0x07);
    assert!(outer.is_empty());

    // Feed that same slice to a second reader — no copy involved.
    let mut components = TlvReader::new(value);
    assert_eq!(components.read(), Ok((0x08, &b"ndn"[..])));
    assert_eq!(components.read(), Ok((0x08, &b"course"[..])));
    assert!(components.is_empty());
}

// ------------------------------------------- larger type / length shapes

#[test]
fn type_and_length_span_the_var_number_shapes() {
    // type 300 needs a 3-byte VAR-NUMBER; a 300-byte value needs a 3-byte length.
    let value = vec![0x5A; 300];
    let bytes = write_one(300, &value);
    // FD 01 2C (type 300) · FD 01 2C (length 300) · 300 value bytes = 306 total.
    assert_eq!(&bytes[..6], &[0xFD, 0x01, 0x2C, 0xFD, 0x01, 0x2C]);
    assert_eq!(bytes.len(), 306);

    let mut r = TlvReader::new(&bytes);
    let (ty, got) = r.read().unwrap();
    assert_eq!(ty, 300);
    assert_eq!(got, &value[..]);
    assert!(r.is_empty());
}

// ------------------------------------------------------------------- errors

#[test]
fn every_incomplete_prefix_is_unexpected_end() {
    let full = write_one(0x08, b"ndn"); // [08 03 6E 64 6E]
    for cut in 0..full.len() {
        // Every proper prefix ends before the element is whole.
        let mut r = TlvReader::new(&full[..cut]);
        assert_eq!(
            r.read(),
            Err(TlvError::UnexpectedEnd),
            "prefix of length {cut}"
        );
    }
    // The whole thing decodes.
    let mut r = TlvReader::new(&full);
    assert_eq!(r.read(), Ok((0x08, &b"ndn"[..])));
}

#[test]
fn declared_length_beyond_the_input_is_unexpected_end() {
    // type 0x08, length says 5, but only 2 value bytes are present.
    let mut r = TlvReader::new(&[0x08, 0x05, 0xAA, 0xBB]);
    assert_eq!(r.read(), Err(TlvError::UnexpectedEnd));
}

#[test]
fn non_minimal_type_or_length_is_rejected() {
    // type 8 written in 3 bytes (FD 00 08), then a zero length.
    let mut r = TlvReader::new(&[0xFD, 0x00, 0x08, 0x00]);
    assert_eq!(r.read(), Err(TlvError::NonMinimal));
    // type 8, then length 0 written in 3 bytes (FD 00 00).
    let mut r = TlvReader::new(&[0x08, 0xFD, 0x00, 0x00]);
    assert_eq!(r.read(), Err(TlvError::NonMinimal));
}

// ---------------------------------------------------------------- read_type

#[test]
fn read_type_returns_the_value_when_it_matches() {
    let bytes = write_one(0x08, b"ndn");
    let mut r = TlvReader::new(&bytes);
    assert_eq!(r.read_type(0x08), Ok(&b"ndn"[..]));
    assert!(r.is_empty());
}

#[test]
fn read_type_mismatch_reports_both_numbers_and_consumes_nothing() {
    let bytes = write_one(0x08, b"ndn");
    let mut r = TlvReader::new(&bytes);
    assert_eq!(
        r.read_type(0x07),
        Err(TlvError::UnexpectedType {
            expected: 0x07,
            found: 0x08
        })
    );
    // Nothing was consumed: a plain read still sees the element.
    assert_eq!(r.read(), Ok((0x08, &b"ndn"[..])));
}

// ---------------------------------------------------- seeded round-trip

#[test]
fn one_thousand_pseudo_random_elements_round_trip() {
    // Deterministic LCG: the same 1000 elements every run.
    let mut x: u64 = 0x9E3779B97F4A7C15;
    let mut next = || {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        x
    };

    let mut elems: Vec<(u64, Vec<u8>)> = Vec::new();
    let mut w = TlvWriter::new();
    for i in 0..1000u32 {
        let r = next();
        // Vary type across VAR-NUMBER shapes.
        let type_num = match i % 3 {
            0 => r % 253,
            1 => 253 + (r % 60_000),
            _ => r % 100_000,
        };
        // Values 0..=63 bytes so some lengths cross the 253 boundary too.
        let vlen = (next() % 260) as usize;
        let value: Vec<u8> = (0..vlen).map(|j| (r >> (8 * (j % 8))) as u8).collect();
        w.write(type_num, &value);
        elems.push((type_num, value));
    }

    let bytes = w.into_bytes();
    let mut reader = TlvReader::new(&bytes);
    for (type_num, value) in &elems {
        let (got_ty, got_val) = reader
            .read()
            .unwrap_or_else(|e| panic!("read {type_num}: {e:?}"));
        assert_eq!(got_ty, *type_num, "type round-trip");
        assert_eq!(got_val, &value[..], "value round-trip");
    }
    assert!(reader.is_empty(), "all bytes consumed");
}
