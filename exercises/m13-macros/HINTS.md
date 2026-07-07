# Hints — m13-macros

Revealed one rung at a time by `./course hint m13-macros`.

## Hint 1 — replay the repetition, like the constants do

The constants line, `$( pub const $name: u64 = $value; )*`, generates one item per
pair. Your lookups do the same, but generate *statements*. Inside `name_of`, replay
the pairs as a chain of checks and fall through to `None`:

```rust
$( if value == $value { return Some(stringify!($name)); } )*
None
```

Each `$( ... )*` in the transcriber expands once per captured pair, substituting that
pair's `$name` and `$value`.

## Hint 2 — value_of is the mirror

Same shape, comparing the *name* instead. `stringify!($name)` is the identifier as a
string, so:

```rust
$( if name == stringify!($name) { return Some($value); } )*
None
```

## Hint 3 — tlv! builds a Vec at expansion

Collect the value bytes with a `vec!`, then build the element around them:

```rust
let value: Vec<u8> = vec![ $( $b ),* ];
let mut out = vec![$t, value.len() as u8];
out.extend(value);
out
```

`vec![ $($b),* ]` replays the byte list into a `Vec`; `value.len()` is the length the
macro computes for you; `$t` is the type. Keep it a `Vec<u8>` throughout (the type
annotation on `value` pins it, which also makes the empty `[]` case work).

## Hint 4 — debugging a macro

If it won't compile, the error often points at the *expansion*, not your source.
`cargo expand -p m13-macros` (after `cargo install cargo-expand`) prints the code your
macro generates — read that. A common trip-up: a fragment used where its kind isn't
allowed. Here everything stays in expression position (`$value` in an `if`, `$b` in a
`vec!`), which is the safe place for `:expr` fragments.
