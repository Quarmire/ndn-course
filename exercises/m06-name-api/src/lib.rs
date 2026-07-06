//! m06-name-api — designing a type: newtypes, canonical ordering, and the traits
//! that make a type feel native to the language.
//!
//! Read SPEC.md. Run the witness with `./course check m06-name-api`. Your ordering
//! is graded against the REAL `ndn-foundation-types` as an oracle: match the
//! spec's canonical order, or the oracle will tell you exactly where you diverge.
//!
//! The structs, derives, and simple accessors are given — you write the five
//! pieces of behavior that define what a `Name` *means*.

use std::cmp::Ordering;
use std::fmt;

/// The TLV type of a generic name component (the common case).
pub const GENERIC: u64 = 8;

/// One component of a name: a TLV type number and its raw bytes.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameComponent {
    pub typ: u64,
    pub value: Vec<u8>,
}

impl NameComponent {
    /// A component of an explicit type. (provided)
    pub fn new(typ: u64, value: Vec<u8>) -> Self {
        Self { typ, value }
    }

    /// A generic (type-8) component from raw bytes. (provided)
    pub fn generic(value: Vec<u8>) -> Self {
        Self::new(GENERIC, value)
    }
}

impl PartialOrd for NameComponent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NameComponent {
    fn cmp(&self, other: &Self) -> Ordering {
        // The canonical NDN component order — NOT what a struct `derive` gives you.
        // See SPEC and HINTS: type first, then LENGTH, then content.
        let _ = other;
        todo!("order by type, then by value LENGTH, then by content bytes")
    }
}

/// A name: an ordered sequence of components.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    components: Vec<NameComponent>,
}

impl Name {
    /// The empty (root) name. (provided)
    pub fn root() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Build a name from components. (provided)
    pub fn from_components(components: impl IntoIterator<Item = NameComponent>) -> Self {
        Self {
            components: components.into_iter().collect(),
        }
    }

    /// The components, in order. (provided)
    pub fn components(&self) -> &[NameComponent] {
        &self.components
    }

    /// Number of components. (provided)
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// (provided — a `len` earns an `is_empty`)
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// True if `prefix` is a name-prefix of `self`: it has no more components
    /// than `self`, and each of its components equals the matching leading one.
    pub fn has_prefix(&self, prefix: &Name) -> bool {
        let _ = prefix;
        todo!("reject a longer prefix, then compare leading components")
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lexicographic over components, using NameComponent's canonical order.
        let _ = other;
        todo!("compare component-by-component — the standard library does this in one call")
    }
}

impl fmt::Display for Name {
    /// Render as `/comp/comp/...`, each component's bytes as UTF-8 (lossily); the
    /// root name is just `/`. (A real NDN URI percent-encodes and marks component
    /// types — this is the readable subset.)
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = f;
        todo!("write a '/' before each component's value — see HINTS")
    }
}

impl From<&str> for Name {
    /// Parse `/a/b/c` into generic components. Empty segments (a leading slash,
    /// doubled slashes, a trailing slash) are skipped.
    fn from(uri: &str) -> Self {
        let _ = uri;
        todo!("split on '/', skip empty segments, each segment → a generic component")
    }
}
