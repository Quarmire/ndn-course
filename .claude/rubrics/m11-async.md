# Rubric — m11-async

Gate 2 review. The tests prove it runs; grade whether the student understands the
model underneath `.await`. Score each 0–2; pass = no zeros and ≥ 10/14. Comment
quality and observability are standing criteria (and the logging arc closes here).

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Countdown is a real future | `poll` yields `Pending` the right number of times then `Ready`; uses `self.get_mut()` (Countdown is `Unpin`), no busy state, no unsafe |
| 2 | Wakes on Pending | Calls `cx.waker().wake_by_ref()` before `Pending`; the student can say why it matters on tokio even though `block_on` re-polls anyway |
| 3 | async/await understood | Can explain that `accumulate` compiles to a `poll` state machine and that each `.await` is a suspension point, not a blocking call |
| 4 | Concurrency vs sequence | Can state how `both`/`join` differ from `a.await; b.await;` — both progress per poll vs one-then-the-other — and why that lets one thread serve many |
| 5 | Pin at the right depth | Correctly reasons that `poll` takes `Pin<&mut Self>` because futures can self-reference, and that `Unpin` is why `get_mut` is safe here |
| 6 | Cancellation + comments | Understands that dropping a future cancels it (Drop runs); comments/journal note where that's powerful and where it's a footgun |
| 7 | tracing + the runtime | Engaged with why async wants `tracing` spans over `log`, and can point at where a span would wrap `both` to see the interleave |

## Reflection prompts (gate 3, pick 2–3)

- Your `Countdown` returns `Pending` and wakes itself. On tokio, what happens to a
  `Pending` future that *forgets* to arrange a wake — and how is that different here?
- `both(a, b)` and `a.await; b.await;` give the same answer. When would the
  difference actually matter — sketch a case where sequential is a bug.
- Cancelling an async task is just dropping its future. What could go wrong if a
  future is dropped while it's halfway through writing a packet — and how do real
  systems guard against it?
