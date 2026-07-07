# Hints — m11-async

Revealed one rung at a time by `./course hint m11-async`.

## Hint 1 — Countdown::poll is a tiny state machine

`Countdown` is `Unpin` (it holds only numbers, no self-references), so start with
`let this = self.get_mut();` to get an ordinary `&mut Self`. Then: if
`this.remaining == 0`, you're done — `Poll::Ready(this.value)`. Otherwise decrement
`this.remaining`, and return `Poll::Pending`. That's the whole future.

## Hint 2 — don't forget to wake

Before returning `Poll::Pending`, call `cx.waker().wake_by_ref()`. This is the
future telling the executor "I made progress, poll me again." Our `block_on` re-polls
regardless, so you won't hang without it — but a real executor (tokio) *parks* until
woken, so a `Pending` that doesn't wake would sleep forever. Writing it now is the
habit that makes your futures work on a real runtime.

## Hint 3 — accumulate: await in a loop

`accumulate` is an `async fn`, so inside it you can use `.await` directly. Keep a
running total and, for each value, await a fresh countdown:

```rust
let mut total = 0;
for v in values {
    total += Countdown::new(2, v).await;
}
total
```

Each `.await` suspends this function until that countdown is ready; the compiler
turns the loop into a state machine that remembers which iteration it was on.

## Hint 4 — both: join, then await the pair

`join` (provided) takes two futures and returns one future of the pair. Await it and
destructure:

```rust
let (x, y) = join(Countdown::new(3, a), Countdown::new(3, b)).await;
x + y
```

Unlike `x = fut_a.await; y = fut_b.await;` (which finishes `a` completely before
starting `b`), `join` polls both each time — they progress together on one thread.
