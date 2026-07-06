# Hints — m07-data-view

Revealed one rung at a time by `./course hint m07-data-view`.

## Hint 1 — find is a scan you've written before

`find` walks the inner sub-elements exactly like M4's iterator. Copy `self.inner`
into a local (`let mut rest = self.inner;` — a reference is `Copy`), then loop:
`decode_varu64(rest)` for the type, `decode_varu64(&rest[n1..])` for the length,
slice the value with `rest.get(start..end)`, and if the type matches, return
`Some(value)`. Otherwise advance `rest` past the whole element and continue. Return
`None` when the bytes run out. Use `?` on the decodes so a truncated element just
ends the scan.

## Hint 2 — the lifetime, again

The value you return must borrow from the buffer (`'a`), not from `&self`. Copying
`self.inner` into a local at the top is what lets the returned `&'a [u8]` outlive
the `&self` borrow — the same move as M4's `next`. If the compiler complains that
you're returning something borrowed from `self`, you sliced `self.inner` directly
instead of a local copy of it.

## Hint 3 — content_as_text is one std call

You want: valid UTF-8 → borrow, invalid → owned-with-replacements. That is exactly
what `String::from_utf8_lossy(bytes) -> Cow<'_, str>` does. So `content_as_text` is
`self.content().map(String::from_utf8_lossy)`. Notice there's no `if` — the `Cow`
encodes the branch, and the caller can still discover which arm it got.

## Hint 4 — to_owned_content is one Bytes call

`Bytes::copy_from_slice(&[u8]) -> Bytes` copies bytes into an owned handle, so it's
`self.content().map(Bytes::copy_from_slice)`. The `copy_` in the name is the lesson:
this is where you finally pay for the bytes, in exchange for a value that no longer
depends on the buffer's lifetime.
