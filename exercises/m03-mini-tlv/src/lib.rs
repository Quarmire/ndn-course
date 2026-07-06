//! m03-mini-tlv — the reader/writer pattern over NDN's TLV wire format.
//!
//! Read SPEC.md first. The witness suite in `tests/witness.rs` is the
//! definition of done; run it with `./course check m03-mini-tlv`.
//!
//! Every element on an NDN wire is `TYPE ‖ LENGTH ‖ VALUE`: a type number, a
//! byte length, and exactly that many value bytes — where TYPE and LENGTH are
//! the VAR-NUMBERs you built in M2. You build:
//!
//!   * `TlvReader`, which walks a byte slice element by element and hands back
//!     each value as a **borrowed** `&[u8]` — never a copy, and
//!   * `TlvWriter`, which appends elements into a `Vec<u8>`.
//!
//! The stubs compile as-is (`todo!()` type-checks as anything); the tests are
//! red, which is where every exercise begins.

// ── provided: the VAR-NUMBER codec from M2 ──────────────────────────────────
// You earned these in m02-varint; they are the atom under TLV, so here they are
// cleaned up — build on them, don't rewrite them. `decode_varu64` reports its
// failures as the same `TlvError` your reader returns, so a `?` just works.

/// Append the shortest VAR-NUMBER encoding of `value` to `out`.
// `allow(dead_code)`: unused until you wire these into `write`/`read`. Once you
// do, the attribute is harmless — you can leave it or delete it.
#[allow(dead_code)]
fn encode_varu64(value: u64, out: &mut Vec<u8>) {
    match value {
        0..=0xFC => out.push(value as u8),
        0xFD..=0xFFFF => {
            out.push(0xFD);
            out.extend_from_slice(&(value as u16).to_be_bytes());
        }
        0x1_0000..=0xFFFF_FFFF => {
            out.push(0xFE);
            out.extend_from_slice(&(value as u32).to_be_bytes());
        }
        _ => {
            out.push(0xFF);
            out.extend_from_slice(&value.to_be_bytes());
        }
    }
}

/// Decode one VAR-NUMBER from the front of `input`: `(value, bytes_consumed)`.
#[allow(dead_code)]
fn decode_varu64(input: &[u8]) -> Result<(u64, usize), TlvError> {
    let first = *input.first().ok_or(TlvError::UnexpectedEnd)?;
    match first {
        0..=0xFC => Ok((first as u64, 1)),
        0xFD => {
            let b: [u8; 2] = input
                .get(1..3)
                .ok_or(TlvError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u16::from_be_bytes(b) as u64;
            if v <= 0xFC {
                return Err(TlvError::NonMinimal);
            }
            Ok((v, 3))
        }
        0xFE => {
            let b: [u8; 4] = input
                .get(1..5)
                .ok_or(TlvError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u32::from_be_bytes(b) as u64;
            if v <= 0xFFFF {
                return Err(TlvError::NonMinimal);
            }
            Ok((v, 5))
        }
        0xFF => {
            let b: [u8; 8] = input
                .get(1..9)
                .ok_or(TlvError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u64::from_be_bytes(b);
            if v <= 0xFFFF_FFFF {
                return Err(TlvError::NonMinimal);
            }
            Ok((v, 9))
        }
    }
}

// ── your work starts here ───────────────────────────────────────────────────

/// What can go wrong reading a TLV element. Note that one variant carries data:
/// designing the *shape* of an error type is half of good error handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TlvError {
    /// The input ran out before an element (type, length, or value) was whole.
    UnexpectedEnd,
    /// A type or length VAR-NUMBER used a longer shape than its value needs.
    NonMinimal,
    /// `read_type` asked for one type number but the next element had another.
    UnexpectedType { expected: u64, found: u64 },
}

/// A cursor that reads TLV elements from a byte slice, front to back.
///
/// The `'a` lifetime is the whole point: the value slices it returns borrow
/// straight from the input, so reading is allocation-free.
pub struct TlvReader<'a> {
    // The bytes not yet consumed. (You may restructure this if you prefer, as
    // long as the public methods keep their signatures.)
    rest: &'a [u8],
}

impl<'a> TlvReader<'a> {
    /// Start reading at the front of `input`. (provided)
    pub fn new(input: &'a [u8]) -> Self {
        TlvReader { rest: input }
    }

    /// True when no bytes remain — your read loop's stop condition. (provided)
    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    /// Read the next element: its type number and a **borrowed** value slice.
    /// Advances the cursor past the whole element.
    pub fn read(&mut self) -> Result<(u64, &'a [u8]), TlvError> {
        todo!("decode the type VAR-NUMBER, then the length, then borrow `length` value bytes — see HINTS")
    }

    /// Read the next element and require it to be `expected`, returning just its
    /// value. On a mismatch, nothing is consumed and you get `UnexpectedType`.
    pub fn read_type(&mut self, expected: u64) -> Result<&'a [u8], TlvError> {
        let _ = expected;
        todo!("read(), then compare the type — UnexpectedType carries both numbers")
    }
}

/// Appends TLV elements into an owned byte buffer.
pub struct TlvWriter {
    buf: Vec<u8>,
}

impl TlvWriter {
    /// A fresh, empty writer. (provided)
    pub fn new() -> Self {
        TlvWriter { buf: Vec::new() }
    }

    /// Append one element: `type_num`, then `value.len()`, then the value bytes.
    /// Existing bytes in the buffer are never disturbed.
    pub fn write(&mut self, type_num: u64, value: &[u8]) {
        let _ = (type_num, value);
        todo!("encode the type, then the value's length, then extend with the value — see HINTS")
    }

    /// Consume the writer and hand back the encoded bytes. (provided)
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

impl Default for TlvWriter {
    fn default() -> Self {
        Self::new()
    }
}
