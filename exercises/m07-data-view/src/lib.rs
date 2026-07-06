//! m07-data-view — zero-copy parsing: lazy fields, `Cow`, and `Bytes`.
//!
//! Read SPEC.md. Run the witness with `./course check m07-data-view`.
//!
//! An NDN Data packet is a TLV whose value is a run of sub-elements (Name,
//! Content, Signature, …). The naive parser decodes and copies every field up
//! front. You'll build the other way: a `DataView<'a>` that decodes NOTHING at
//! parse time and, when asked for a field, hands back a slice BORROWED from the
//! original buffer. Then you'll meet the two escapes from the borrow — `Cow`
//! (own only when you must) and `Bytes` (own to outlive the buffer).

use std::borrow::Cow;

use bytes::Bytes;

/// TLV type numbers for the Data fields we care about.
pub const DATA: u64 = 0x06;
pub const NAME: u64 = 0x07;
pub const CONTENT: u64 = 0x15;

/// Lenient VAR-NUMBER decoder — `None` on truncation. (provided)
fn decode_varu64(input: &[u8]) -> Option<(u64, usize)> {
    let first = *input.first()?;
    match first {
        0..=0xFC => Some((first as u64, 1)),
        0xFD => {
            let b: [u8; 2] = input.get(1..3)?.try_into().ok()?;
            Some((u16::from_be_bytes(b) as u64, 3))
        }
        0xFE => {
            let b: [u8; 4] = input.get(1..5)?.try_into().ok()?;
            Some((u32::from_be_bytes(b) as u64, 5))
        }
        0xFF => {
            let b: [u8; 8] = input.get(1..9)?.try_into().ok()?;
            Some((u64::from_be_bytes(b), 9))
        }
    }
}

/// A zero-copy view over a Data packet. It owns nothing: every field it returns
/// borrows from the original buffer for `'a`, and nothing is decoded until asked.
pub struct DataView<'a> {
    // The Data's VALUE bytes (its sub-elements), outer header already stripped.
    // (unused until you implement `find`.)
    #[allow(dead_code)]
    inner: &'a [u8],
}

impl<'a> DataView<'a> {
    /// Parse the outer Data (0x06) TLV and remember its inner bytes. Lazy: no
    /// field is scanned yet. `None` if `bytes` isn't a Data element. (provided)
    pub fn parse(bytes: &'a [u8]) -> Option<DataView<'a>> {
        let (typ, n1) = decode_varu64(bytes)?;
        if typ != DATA {
            return None;
        }
        let (len, n2) = decode_varu64(&bytes[n1..])?;
        let start = n1 + n2;
        let end = start + len as usize;
        let inner = bytes.get(start..end)?;
        Some(DataView { inner })
    }

    /// The VALUE of the first sub-element whose type is `type_num`, borrowed from
    /// the buffer. This is the lazy heart: it scans on demand and copies nothing.
    pub fn find(&self, type_num: u64) -> Option<&'a [u8]> {
        let _ = type_num;
        todo!("walk the inner TLVs; return the value of the first whose type matches — see HINTS")
    }

    /// The Name field's bytes. (provided — an example of using `find`)
    pub fn name(&self) -> Option<&'a [u8]> {
        self.find(NAME)
    }

    /// The Content field's bytes, if present. (provided)
    pub fn content(&self) -> Option<&'a [u8]> {
        self.find(CONTENT)
    }

    /// The content as text — BORROWED when it is already valid UTF-8, OWNED only
    /// when lossy replacement was needed. `None` if there is no content.
    pub fn content_as_text(&self) -> Option<Cow<'a, str>> {
        todo!("std has a function that returns exactly this Cow — see HINTS")
    }

    /// Copy the content into an owned, cheaply-shareable handle that can outlive
    /// the buffer `'a`. This copy is the price of escaping the borrow. `None` if
    /// there is no content.
    pub fn to_owned_content(&self) -> Option<Bytes> {
        todo!("copy the content bytes into a Bytes — see HINTS")
    }
}
