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

/// Decode one VAR-NUMBER from the front of `input`.
/// Returns the value and how many bytes it consumed.
pub fn decode_varu64(input: &[u8]) -> Result<(u64, usize), VarintError> {
    let first = *input.first().ok_or(VarintError::UnexpectedEnd)?;
    match first {
        0..=0xFC => Ok((first as u64, 1)),
        0xFD => {
            let b: [u8; 2] = input
                .get(1..3)
                .ok_or(VarintError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u16::from_be_bytes(b) as u64;
            if v <= 0xFC {
                return Err(VarintError::NonMinimal);
            }
            Ok((v, 3))
        }
        0xFE => {
            let b: [u8; 4] = input
                .get(1..5)
                .ok_or(VarintError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u32::from_be_bytes(b) as u64;
            if v <= 0xFFFF {
                return Err(VarintError::NonMinimal);
            }
            Ok((v, 5))
        }
        0xFF => {
            let b: [u8; 8] = input
                .get(1..9)
                .ok_or(VarintError::UnexpectedEnd)?
                .try_into()
                .unwrap();
            let v = u64::from_be_bytes(b);
            if v <= 0xFFFF_FFFF {
                return Err(VarintError::NonMinimal);
            }
            Ok((v, 9))
        }
    }
}
