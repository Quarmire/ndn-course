//! m13-macros — macro_rules!, repetition, and generating code.
//!
//! Read SPEC.md. Run the witness with `./course check m13-macros`.
//!
//! A macro isn't a function — it runs at compile time and produces *code*. That
//! lets it do what a function can't: generate a whole table of items, or a little
//! builder syntax, from one compact declaration. You'll write two:
//!
//!   * `tlv_registry!` — from `NAME = value` pairs, generate constants plus
//!     `name_of` / `value_of` lookups (the constants are the worked example; you
//!     write the two lookups), and
//!   * `tlv!` — a builder that turns `tlv!(0x08, [..])` into wire bytes.
//!
//! The stubs compile; the tests are red until you fill the macro bodies in.

/// Generate a TLV type registry from `NAME = value` pairs: a `pub const` for each,
/// plus reverse (`name_of`) and forward (`value_of`) lookups.
macro_rules! tlv_registry {
    ( $( $name:ident = $value:expr ),* $(,)? ) => {
        // Worked example — one `pub const` per pair, via `$( ... )*` repetition:
        $( pub const $name: u64 = $value; )*

        /// The type name for a value, if the value is a known type.
        pub fn name_of(value: u64) -> Option<&'static str> {
            let _ = value;
            todo!("repeat over the pairs: if value == $value, return Some(stringify!($name)) — see HINTS")
        }

        /// The value for a type name, if the name is a known type.
        pub fn value_of(name: &str) -> Option<u64> {
            let _ = name;
            todo!("repeat over the pairs: if name == stringify!($name), return Some($value) — see HINTS")
        }
    };
}

// Invoke the macro once to generate `DATA`, `NAME`, …, and the two lookup fns.
tlv_registry! {
    DATA = 0x06,
    NAME = 0x07,
    INTEREST = 0x05,
    CONTENT = 0x15,
    META_INFO = 0x14,
}

/// Build the wire bytes of one TLV element from a type and its value bytes:
/// `tlv!(0x08, [0x6E, 0x64, 0x6E])` produces `vec![0x08, 0x03, 0x6E, 0x64, 0x6E]`
/// (type, then length, then the value). A tiny test-corpus builder — you write it.
///
/// `#[macro_export]` puts it at the crate root so tests (and other crates) can
/// `use m13_macros::tlv;`.
#[macro_export]
macro_rules! tlv {
    ( $t:expr, [ $( $b:expr ),* $(,)? ] ) => {{
        // TODO: build the value bytes, then a Vec of [type, length, ...value].
        // This placeholder compiles (a typed empty Vec) but is wrong, so the
        // tests are red until you replace it. See HINTS.
        Vec::<u8>::new()
    }};
}
