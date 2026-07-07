# m11-async — how a Future actually works

**Module:** M11 · Async Rust for real · **Species:** template-assisted
**You write:** `Countdown::poll`, `accumulate`, `both`.

Async isn't threads and it isn't magic — it's a value with a `poll` method and a
loop that calls it. Before you use tokio (which hides all of this), you'll build a
future by hand and run it on a tiny executor, so `.await` becomes a state machine
you can point at. This is the module M9 and M10 were leading to: cooperative
concurrency, where many tasks share one thread by *yielding* instead of blocking.

## The model

- A **`Future`** has one method: `poll(self: Pin<&mut Self>, cx) -> Poll<T>`. Each
  call returns `Poll::Ready(value)` (done) or `Poll::Pending` (not yet — but I'll
  arrange to be polled again).
- An **executor** (`block_on`, provided) owns the future and calls `poll` in a loop
  until it's `Ready`. A real one *parks* the thread between polls and is woken by
  the future's `Waker`; ours simply re-polls, which is enough here.
- The **`Waker`** (`cx.waker()`) is how a `Pending` future says "poll me again."
  When your `Countdown` returns `Pending`, it calls `cx.waker().wake_by_ref()` — the
  contract that keeps a real executor from parking forever. (Ours doesn't need it,
  but writing it is the habit that makes your futures work on tokio.)
- **`Pin`** appears in `poll`'s receiver because a future may hold references into
  itself once it's running; pinning promises it won't move. Your `Countdown` holds
  no self-references, so it's `Unpin`, and `self.get_mut()` gives you `&mut Self` —
  that's all the `Pin` you need at this depth.

## What you write

- **`Countdown::poll`** — if `remaining == 0`, `Poll::Ready(self.value)`. Otherwise
  decrement `remaining`, call `cx.waker().wake_by_ref()`, and return `Poll::Pending`.
  That's a future by hand: a little state machine that yields a few times, then
  finishes.

- **`accumulate(values)`** — an `async fn`. Await a `Countdown` for each value and
  sum the results: `total += Countdown::new(2, v).await;` in a loop. Each `.await`
  is a *suspension point* — the compiler turns this whole function into a `poll`
  state machine that remembers where it left off.

- **`both(a, b)`** — await two `Countdown`s *concurrently* with the provided `join`:
  `let (x, y) = join(Countdown::new(3, a), Countdown::new(3, b)).await;`. Both run on
  one thread, interleaved poll by poll — the essence of async tasks, and why one
  runtime thread can serve thousands of connections.

## Tasks, cancellation, and the real runtime

- **Tasks vs threads:** `join` (and tokio's `select!`, `spawn`) run many futures on
  a few OS threads. A thread that blocks stops; a task that `.await`s *yields*, so
  the thread runs someone else. That's how a forwarder handles every face at once.
- **Cancellation is just Drop:** stop awaiting a future — drop it — and it's
  cancelled; its destructors run, nothing half-finishes silently. There's no
  `kill()`; you drop the future (or the task handle). Powerful, and a footgun if a
  future was mid-way through something that needed finishing.
- **The real thing:** `ndn-runtime` abstracts `Spawn`/`Sleep`/`Now` so the engine's
  `face_task → pipeline_runner → expiry_task` topology runs on tokio natively and in
  the browser on wasm. Read the Tokio tutorial and the Async Book, and one "how does
  async actually work" deep-dive — you now have the vocabulary for all three.

## The logging arc completes here

Comments and logs have been graded since M5; this is where the logging story ends:
async needs `tracing`, not `log`, because a value flows across `.await` points and
threads, and a `tracing` **span** follows it. Read the `tracing` docs; note how a
span differs from a log line (it has a duration and nests), and where you'd wrap a
span around `both` to see the two Countdowns interleave.

## Done means

`./course check m11-async` green: `Countdown` yields then readies (checked by
polling it directly), `block_on` runs it, `accumulate` sums, `both` resolves —
plus clippy `-D warnings` and fmt. Then `./course submit`.

## Rules of engagement

No `unsafe`, no dependencies. `Countdown` must actually yield (return `Pending`
before `Ready`) — a future that's instantly `Ready` defeats the point.
