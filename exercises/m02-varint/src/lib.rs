//! m02-varint — your first real code in this course.
//!
//! Read SPEC.md first. The witness suite in `tests/witness.rs` is the
//! definition of done; run it with `./course check m02-varint`.
//!
//! The stubs below compile as-is (`todo!()` type-checks as anything), so the
//! whole workspace builds before you've written a line — the tests are red,
//! which is where every exercise begins.

/// Errors a strict VAR-NUMBER decoder can hit. See SPEC.md for when each fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarintError {
    /// The input ended before the promised shape was complete.
    UnexpectedEnd,
    /// The value was encoded in a longer shape than it needs.
    NonMinimal,
}

/// Append the shortest VAR-NUMBER encoding of `value` to `out`.
pub fn encode_varu64(value: u64, out: &mut Vec<u8>) {
    let _ = (value, out);
    todo!("see SPEC.md — four shapes, chosen by magnitude")
}

/// Decode one VAR-NUMBER from the front of `input`.
/// Returns the value and how many bytes it consumed.
pub fn decode_varu64(input: &[u8]) -> Result<(u64, usize), VarintError> {
    let _ = input;
    todo!("first byte tells you the shape; the shape tells you how many bytes to read")
}
