# Hints — m03-mini-tlv

Revealed one rung at a time by `./course hint m03-mini-tlv`. Each rung assumes
you've genuinely tried since the last one.

## Hint 1 — the reader is a shrinking view

The whole reader is one idea: `rest` is the bytes you haven't consumed yet.
`read` decodes an element off the *front* of `rest`, then sets `rest` to point
just past it. `is_empty` (given) is simply `rest` being empty — so a read loop
is `while !r.is_empty() { r.read()?; }`. Nothing else to track; no index counter.

## Hint 2 — decode type, then length, then borrow

You already have the hard part: `decode_varu64` is provided and returns
`(value, bytes_consumed)`. Call it once for the TYPE and once (on what's left)
for the LENGTH. Because it returns the same `TlvError` your `read` returns, a
plain `?` handles `UnexpectedEnd`/`NonMinimal` for free — you never match on its
error. After the length you know exactly how many value bytes to take.

## Hint 3 — returning `&'a` from `&mut self`

This is the exercise's real lesson. The value slice must borrow from the
*original input* (`'a`), not from the `&mut self` borrow — otherwise the caller
couldn't hold it after the next `read`. The move: `rest` is a `&'a [u8]`, and a
reference is `Copy`. Copy it into a local first (`let bytes = self.rest;`), do
all your slicing against that local, and only assign the leftover back to
`self.rest` at the end. `bytes.get(off .. off + len)` returns `Option<&'a [u8]>`
— a `None` there is exactly your remaining `UnexpectedEnd` (the length promised
more bytes than exist).

## Hint 4 — read_type backs off; write mirrors read

`read_type`: save the cursor first (`let saved = self.rest;` — it's `Copy`),
call `read()`, and if the type isn't what was asked, restore (`self.rest =
saved;`) and return `UnexpectedType { expected, found }`. That "try, else put it
back" shape is transactional parsing; you'll see it everywhere.

`write` is `read` run backwards: `encode_varu64(type_num, &mut self.buf)`, then
`encode_varu64(value.len() as u64, &mut self.buf)`, then
`self.buf.extend_from_slice(value)`. Appending never touches earlier bytes, so
two `write`s just accumulate.
