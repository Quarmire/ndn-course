//! m04-name-view — borrow, don't own: lifetimes and iterators beyond trivial.
//!
//! Read SPEC.md first. Run the witness with `./course check m04-name-view`.
//!
//! A name on the wire is a run of TLV components (each `TYPE ‖ LENGTH ‖ VALUE`).
//! The naive way to work with one is to copy every component into a
//! `Vec<Vec<u8>>` — a heap allocation per component, owned forever. You'll build
//! the other way: a `NameView<'a>` that owns NOTHING and hands out borrowed
//! `&'a [u8]` slices onto the original bytes. That `'a` is the whole lesson.
//!
//! The stubs compile (`todo!()`); the tests are red until you fill them in.

// ── provided: the VAR-NUMBER decoder, lenient (None on truncation/malformed) ──
#[allow(dead_code)]
fn decode_varu64(input: &[u8]) -> Option<(u64, usize)> {
    let first = *input.first()?;
    match first {
        0..=0xFC => Some((first as u64, 1)),
        0xFD => {
            let b: [u8; 2] = input.get(1..3)?.try_into().ok()?;
            let v = u16::from_be_bytes(b) as u64;
            (v > 0xFC).then_some((v, 3))
        }
        0xFE => {
            let b: [u8; 4] = input.get(1..5)?.try_into().ok()?;
            let v = u32::from_be_bytes(b) as u64;
            (v > 0xFFFF).then_some((v, 5))
        }
        0xFF => {
            let b: [u8; 8] = input.get(1..9)?.try_into().ok()?;
            let v = u64::from_be_bytes(b);
            (v > 0xFFFF_FFFF).then_some((v, 9))
        }
    }
}

// ── Part A — warmup: return a borrow (the signature forbids cloning) ──────────

/// Return a longest component (by byte length) of `components`, borrowed — not a
/// copy. `None` if empty; on ties, any longest is fine. The `&'a` in the return
/// type means you *cannot* build a new `Vec` and hand it back: you must return
/// one of the slices you were given.
pub fn longest_component<'a>(components: &[&'a [u8]]) -> Option<&'a [u8]> {
    let _ = components;
    todo!("return the longest slice, borrowed — see HINTS")
}

// ── Part B — the borrowed view ────────────────────────────────────────────────

/// A zero-copy view over a name: a run of TLV components living in `bytes`. It
/// owns nothing; every component it yields borrows from `bytes` for `'a`.
pub struct NameView<'a> {
    // Read-only after construction. (unused until you implement `iter`.)
    #[allow(dead_code)]
    bytes: &'a [u8],
}

impl<'a> NameView<'a> {
    /// Wrap the component bytes of a name. (provided)
    pub fn new(bytes: &'a [u8]) -> Self {
        NameView { bytes }
    }

    /// Iterate the component VALUES, each borrowed from the original bytes.
    pub fn iter(&self) -> NameComponents<'a> {
        todo!("hand the iterator the bytes to walk — see HINTS")
    }

    /// The i-th component's value, if it exists. (provided, in terms of `iter`)
    pub fn get(&self, i: usize) -> Option<&'a [u8]> {
        self.iter().nth(i)
    }

    /// How many components the name has. (provided)
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// (provided — a `len` deserves an `is_empty`)
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

/// Iterator over a name's component values. Holds the bytes not yet walked.
pub struct NameComponents<'a> {
    // (unused until you implement `next`.)
    #[allow(dead_code)]
    rest: &'a [u8],
}

impl<'a> Iterator for NameComponents<'a> {
    type Item = &'a [u8];

    /// Decode one component off the front, borrow its value, advance. Return
    /// `None` to stop — at the end of the bytes, or on a malformed element (a
    /// lenient view stops rather than panicking).
    fn next(&mut self) -> Option<&'a [u8]> {
        todo!("decode type, then length, then borrow `length` value bytes — like M3's read, but returning None to stop")
    }
}

// ── Part C — use the view ─────────────────────────────────────────────────────

/// The number of leading components `a` and `b` share, in order — the length of
/// their common prefix. (NDN routing runs on this operation.) Build it from the
/// two views; don't allocate.
pub fn common_prefix_len(a: &NameView, b: &NameView) -> usize {
    let _ = (a, b);
    todo!("zip the two iterators and count the matching leading components")
}
