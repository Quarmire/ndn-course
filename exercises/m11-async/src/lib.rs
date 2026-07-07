//! m11-async — async Rust for real: how a `Future` actually works.
//!
//! Read SPEC.md. Run the witness with `./course check m11-async`.
//!
//! This module is dependency-free on purpose. Instead of reaching for tokio, you
//! implement a `Future` by hand and run it on a tiny executor, so `async`/`.await`
//! becomes a state machine you can see rather than magic you trust. A `Future` is
//! just a value with a `poll` method: each call either returns `Ready(value)` or
//! `Pending` ("not yet — poll me again"). An executor calls `poll` in a loop; the
//! `async`/`.await` sugar builds the `poll` state machine for you.
//!
//! `block_on` and `join` are provided. You write `Countdown` (a future by hand),
//! and two `async fn`s that use `.await`. Stubs compile; tests are red until filled.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// A minimal executor: poll `fut` to completion on this thread and return its
/// output. A real executor parks the thread and wakes on the `Waker`; this one
/// simply re-polls (with a safety cap) — enough to run cooperative futures. (provided)
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let mut cx = Context::from_waker(Waker::noop());
    let mut fut = std::pin::pin!(fut);
    for _ in 0..5_000_000u64 {
        if let Poll::Ready(value) = fut.as_mut().poll(&mut cx) {
            return value;
        }
    }
    panic!("block_on: the future never became Ready — did a poll forget to make progress?");
}

/// Drive two futures CONCURRENTLY on one thread, finishing when both are done.
/// This is the thing threads aren't needed for: cooperative concurrency. (provided)
pub fn join<A, B>(a: A, b: B) -> Join<A, B>
where
    A: Future,
    B: Future,
{
    Join {
        a: Box::pin(a),
        out_a: None,
        b: Box::pin(b),
        out_b: None,
    }
}

/// The future returned by [`join`]. (provided)
pub struct Join<A: Future, B: Future> {
    a: Pin<Box<A>>,
    out_a: Option<A::Output>,
    b: Pin<Box<B>>,
    out_b: Option<B::Output>,
}

impl<A, B> Future for Join<A, B>
where
    A: Future,
    B: Future,
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.out_a.is_none() {
            if let Poll::Ready(v) = this.a.as_mut().poll(cx) {
                this.out_a = Some(v);
            }
        }
        if this.out_b.is_none() {
            if let Poll::Ready(v) = this.b.as_mut().poll(cx) {
                this.out_b = Some(v);
            }
        }
        match (this.out_a.take(), this.out_b.take()) {
            (Some(a), Some(b)) => Poll::Ready((a, b)),
            (a, b) => {
                this.out_a = a;
                this.out_b = b;
                Poll::Pending
            }
        }
    }
}

// ── your work starts here ────────────────────────────────────────────────────

/// A future that isn't ready right away: it returns `Poll::Pending` `polls` times
/// (a cooperative yield each time), then `Poll::Ready(value)`.
#[allow(dead_code)] // fields go unread until you implement `poll`
pub struct Countdown {
    remaining: u32,
    value: u64,
}

impl Countdown {
    /// A future that yields `polls` times, then completes with `value`. (provided)
    pub fn new(polls: u32, value: u64) -> Self {
        Self {
            remaining: polls,
            value,
        }
    }
}

impl Future for Countdown {
    type Output = u64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u64> {
        let this = self.get_mut();
        if this.remaining == 0 {
            Poll::Ready(this.value)
        } else {
            this.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Await one `Countdown` per value and return the sum of the values.
pub async fn accumulate(values: Vec<u64>) -> u64 {
    let mut total = 0;
    for v in values {
        total += Countdown::new(2, v).await;
    }
    total
}

/// Await two `Countdown`s CONCURRENTLY with `join`, and return the sum of their
/// values. Both futures make progress on the same thread — no extra threads.
pub async fn both(a: u64, b: u64) -> u64 {
    let (x, y) = join(Countdown::new(3, a), Countdown::new(3, b)).await;
    x + y
}
