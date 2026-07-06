# Hints — m04-name-view

Revealed one rung at a time by `./course hint m04-name-view`.

## Hint 1 — longest_component: the borrow is already forced

The return type `Option<&'a [u8]>` is doing the work: you literally cannot build a
new `Vec` and return `&` to it (it would be dropped at the end of the function).
So you must return one of the input slices. `components.iter()` gives you
`&&[u8]`; `.copied()` turns that into `&[u8]`; then pick the longest by length.
`Iterator::max_by_key` and `slice::len` are all you need.

## Hint 2 — iter: hand over the bytes

`iter` builds a `NameComponents` that starts at the front of the whole name:
`NameComponents { rest: self.bytes }`. The field is private, but you're in the
same module, so you can name it. That's the entire method.

## Hint 3 — next: it's M3's `read`, returning None to stop

Same shape as your M3 reader, but the iterator protocol says "return `None` when
there's nothing more." Copy `self.rest` into a local (`let bytes = self.rest;` — a
reference is `Copy`), decode the type with `decode_varu64`, then the length off
`&bytes[n1..]`, slice out the value with `bytes.get(off..end)`, advance
`self.rest = &bytes[end..]`, and return `Some(value)`. Use `?` on the `Option`s:
any `None` (truncated or malformed) turns into "stop iterating," which is exactly
the lenient behavior you want.

## Hint 4 — common_prefix_len: let the iterators do it

Two views, two iterators. `a.iter().zip(b.iter())` walks them in lockstep and
stops when either ends. `take_while(|(x, y)| x == y)` keeps the leading matches,
`.count()` counts them. One line, no allocation, no indexing. (`zip` naturally
handles the different-length case; `take_while` handles the first mismatch.)
