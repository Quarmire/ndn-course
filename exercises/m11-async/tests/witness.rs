//! Witness suite for m11-async. We poll `Countdown` directly to prove it yields,
//! then run everything through the provided executor.

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use m11_async::{accumulate, block_on, both, Countdown};

// -------------------------------------------------------------- Countdown::poll

#[test]
fn countdown_is_ready_immediately_when_zero() {
    let fut = Countdown::new(0, 42);
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Ready(42)));
}

#[test]
fn countdown_yields_the_right_number_of_times() {
    let fut = Countdown::new(2, 99);
    let mut fut = pin!(fut);
    let mut cx = Context::from_waker(Waker::noop());
    // Two Pendings (the cooperative yields), then Ready.
    assert!(
        matches!(fut.as_mut().poll(&mut cx), Poll::Pending),
        "poll 1 should be Pending"
    );
    assert!(
        matches!(fut.as_mut().poll(&mut cx), Poll::Pending),
        "poll 2 should be Pending"
    );
    assert!(
        matches!(fut.as_mut().poll(&mut cx), Poll::Ready(99)),
        "poll 3 should be Ready(99)"
    );
}

// ------------------------------------------------------------------- block_on

#[test]
fn block_on_drives_a_countdown_to_completion() {
    assert_eq!(block_on(Countdown::new(0, 7)), 7);
    assert_eq!(block_on(Countdown::new(5, 7)), 7);
    assert_eq!(block_on(Countdown::new(1000, 123)), 123);
}

// ------------------------------------------------------------------ async fns

#[test]
fn accumulate_sums_the_awaited_values() {
    assert_eq!(block_on(accumulate(vec![1, 2, 3, 4])), 10);
    assert_eq!(block_on(accumulate(vec![100])), 100);
    assert_eq!(block_on(accumulate(vec![])), 0);
}

#[test]
fn both_awaits_two_futures_concurrently() {
    assert_eq!(block_on(both(10, 20)), 30);
    assert_eq!(block_on(both(0, 0)), 0);
    assert_eq!(block_on(both(7, 0)), 7);
}
