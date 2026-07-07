# m13-macros — generating code with macro_rules!

**Module:** M13 · Macros and codegen · **Species:** template-assisted
**You write:** the bodies of `name_of` / `value_of` inside `tlv_registry!`, and the body of `tlv!`.

A macro runs at *compile time* and emits *code*. That's its whole reason to exist:
it can do things a function can't — declare a batch of `const`s and functions from
one table, or give you a little builder syntax. You'll write a `macro_rules!` type
registry and a byte-builder, and — just as important — learn when *not* to reach for
a macro at all.

## The decision ladder

Before writing a macro, walk down this list and stop at the first tool that fits:

1. **A function** — for runtime behavior. Almost always the answer.
2. **Generics / traits** — for behavior that varies by *type*. (M8.)
3. **`build.rs`** — to generate code from *external data* at build time (a schema, a
   table file).
4. **`macro_rules!`** — a *declarative* macro: pattern-match on tokens, emit code.
   Reach for it when you must generate *items* (consts, fns, impls) or accept a
   syntax a function can't — like a variable-length `tlv!(t, [..])`.
5. **A procedural macro** — a compiler plugin in its own crate (this is what
   `#[derive(...)]` is). Most power, most cost; last resort.

Macros are harder to read, debug, and error-message than functions. Earn the macro.

## macro_rules! in one page

A macro is `matcher => transcriber` rules. In the matcher, `$x:ident`, `$x:expr`,
`$x:literal`, `$x:ty` capture *fragments*. `$( ... )*` matches a **repetition**;
`$( ... ),*` uses a comma separator; a trailing `$(,)?` allows an optional final
comma. In the transcriber, `$( ... )*` **replays** the repetition once per captured
item — that's how `tlv_registry!` turns N pairs into N constants. `stringify!($name)`
turns an identifier into its `&'static str` spelling.

Macros are **hygienic**: names a macro introduces don't collide with the caller's,
so a macro can define a temporary without stomping your variables.

## What you write

- **`name_of` / `value_of`** (inside `tlv_registry!`) — the constants are done for
  you as the worked example of `$( pub const $name: u64 = $value; )*`. Write the two
  lookups the same way: replay the pairs. For `name_of`, "if the value matches
  `$value`, return `stringify!($name)`"; for `value_of`, the mirror. Fall through to
  `None`.
- **`tlv!`** — `tlv!(0x08, [0x6E, 0x64, 0x6E])` → `vec![0x08, 0x03, 0x6E, 0x64, 0x6E]`.
  Collect the value bytes into a `Vec`, prefix `[type, length]`, append the value.
  The macro computes the length for you at expansion — that's the point of a builder.

## Read the real thing (after you pass)

- **`cargo expand`** shows what a macro *becomes*: `cargo install cargo-expand`, then
  `cargo expand -p m13-macros` to see your `tlv_registry!` unrolled into plain code.
  Reading the expansion is how you debug a macro.
- **A derive is a different mechanism.** `#[derive(Debug)]` is a *procedural* macro —
  Rust code that takes a `TokenStream` and returns one, compiled as its own crate.
  Find a `#[derive(...)]` used across ndn-rs and read what it generates via
  `cargo expand`; note how much boilerplate it erases.
- **Critique a workspace macro.** Grep ndn-rs for `macro_rules!` and pick one. In
  your journal: what does it generate, could it have been a function or a generic
  instead, and is the macro pulling its weight?

## Done means

`./course check m13-macros` green: the registry's constants and both lookups, and
`tlv!` building correct bytes (including the empty value and trailing comma) — plus
clippy `-D warnings` and fmt. Then `./course submit`.

## Rules of engagement

No dependencies. The registry must be driven *by the macro* — no hand-written match
arms listing each type twice. If you find yourself repeating the type table, that
repetition is exactly what the `$( ... )*` is supposed to erase.
