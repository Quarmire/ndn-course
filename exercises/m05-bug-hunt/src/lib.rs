//! m05-bug-hunt — four bugs are hiding in this file.
//!
//! This is not `todo!()` — it's worse. The code compiles, reads reasonably, and
//! is WRONG: some functions give bad answers, some panic. Read SPEC.md, run the
//! witness, and hunt. Each `pub` function's doc comment states what it *should*
//! do — a bug is code that disagrees with its own contract. `decode_varu64` is
//! correct: build your debugging on a foundation you've verified.
//!
//! Four bugs, one per `pub fn`. Fix each minimally, and log each in
//! `student/bug-journal.md`: symptom → hypothesis → fix → lesson.

/// Decode one VAR-NUMBER from the front of `input`, returning `(value, consumed)`.
/// `None` on truncation. (Correct — trusted.)
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

/// The first component of a name, or `None` if the name has no components.
pub fn first_component<'a>(name: &[&'a [u8]]) -> Option<&'a [u8]> {
    name.first().copied()
}

/// True if `prefix` is a name-prefix of `name`: `prefix` has no more components
/// than `name`, and each of its components equals the matching leading component
/// of `name`.
pub fn is_prefix(prefix: &[&[u8]], name: &[&[u8]]) -> bool {
    if prefix.len() > name.len() {
        return false;
    }
    prefix.iter().zip(name.iter()).all(|(p, n)| p == n)
}

/// The number of leading components `a` and `b` share, in order.
pub fn common_prefix_len(a: &[&[u8]], b: &[&[u8]]) -> usize {
    let mut n = 0;
    while n < a.len() && n < b.len() {
        if a[n] != b[n] {
            break;
        }
        n += 1;
    }
    n
}

/// Split a name's component bytes into the VALUE slice of each TLV component,
/// stopping cleanly at the first malformed or truncated element.
pub fn split_components(name_bytes: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut rest = name_bytes;
    while !rest.is_empty() {
        let Some((_ty, n1)) = decode_varu64(rest) else {
            break;
        };
        let Some((len, n2)) = decode_varu64(&rest[n1..]) else {
            break;
        };
        let start = n1 + n2;
        let end = start + len as usize;
        if end > rest.len() {
            break;
        }
        out.push(&rest[start..end]);
        rest = &rest[end..];
    }
    out
}
