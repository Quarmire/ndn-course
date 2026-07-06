# Rubric — m04-name-view

Gate 2 review. Score each criterion 0–2 with one line-anchored observation.
Pass = no zeros and total ≥ 10/14. This module is where lifetimes stop being
syntax and start being a tool — grade whether the student is *using* the borrow,
not fighting it.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | `next` borrows, not copies | Returns the value slice borrowed from the original bytes — no `to_vec`/`clone`; the borrowing witness passes because the data is genuinely shared |
| 2 | The lifetime move | `next` copies `self.rest` into a local before slicing so the returned `&'a` is tied to the input, not to `&mut self` — not fought with owned buffers or `unsafe` |
| 3 | Lenient stop | Malformed/truncated tail returns `None` (via `?` on the decoder's `Option`), stopping iteration cleanly rather than panicking or looping |
| 4 | `longest_component` | Returns an input slice by borrow; uses `iter/copied/max_by_key` or equivalent — no manual index bookkeeping, no clone |
| 5 | `common_prefix_len` | Built by *using* the iterators (`zip` + `take_while` + `count` or equivalent); allocation-free; correct when lengths differ and on the "later match doesn't count" case |
| 6 | Comments + the size story | The *why* of borrowing is noted where it matters; the journal has both `size_of` numbers (NameView 16 vs OwnedName + per-component heap) with a sentence on what the fat pointer buys |
| 7 | Simplicity | No speculative generality (no generic `View<T>`, no owned fallback field); `iter` is one line; the iterator carries only what it needs |

## Reflection prompts (gate 3, pick 2–3)

- `NameView` is 16 bytes regardless of the name's length. Where do the actual
  component bytes live, and what does the view's `'a` promise about them?
- You didn't write a `Drop` for `NameView`. Why is there nothing to free — and
  what would change if it held a `Vec<Vec<u8>>` instead?
- The view is *lenient*: it stops on a malformed component instead of erroring.
  When is that the right call, and when would you want a strict `Result` instead?
