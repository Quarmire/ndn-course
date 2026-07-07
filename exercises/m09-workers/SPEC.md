# m09-workers — threads, channels, and whole processes

**Module:** M9 · Processes and threads · **Species:** template-assisted
**You write:** `parallel_sum`, `run_jobs`, `run_command`.

Async (M11) will hide the machinery; this module makes you see it first, at the
level the OS actually works — real threads scheduled by the kernel, channels
carrying values between them, panics that stay contained, and child processes with
their own memory and exit codes. Get concrete about this now, and async later will
be sugar you understand rather than magic you trust.

## 1. Scoped threads + a channel — `parallel_sum`

`std::thread::spawn` needs its closure to be `'static` (the thread might outlive the
caller). That's a pain when you just want to fan out over a slice you already own.
`std::thread::scope` fixes it: threads spawned inside a `scope` are **guaranteed to
finish before it returns**, so they may *borrow* from the parent's stack — like the
slice you're summing.

Split `data` into a chunk per worker, spawn a scoped thread for each, and have each
send its partial sum down an `std::sync::mpsc` channel. `Sender` is `Clone`, so give
each worker its own clone; the `Receiver` collects them. After the scope (all
threads joined), drop the original sender and sum what the receiver hands you.

## 2. One thread per job, panics contained — `run_jobs`

A panic on a spawned thread does **not** abort the process — it unwinds *that*
thread, and `JoinHandle::join` returns `Err(payload)` instead of `Ok(value)`. That's
the isolation that lets a server survive one bad request. Spawn each boxed job, then
`join` each handle and translate the result: `Ok(v)` stays `Ok(v)`; an `Err` becomes
`Err(message)` (the provided `panic_message` digs the string out of the payload).
Keep the results in the jobs' original order. You'll see the panic printed to
stderr as it happens — that's expected; the point is the *process kept going*.

## 3. A child process — `run_command`

`std::process::Command` launches a whole separate program. Build it with
`Command::new(program).args(args)`, run it to completion with `.output()`, and pull
out `status.code()` (the exit code) and `stdout` (raw bytes → lossy `String`).
Launch failure (no such program) is an `Err` from `.output()` — propagate it with
`?`. A child process is heavier than a thread (its own address space) but isolated
by the same wall: it can crash without touching you.

## The orchestration lab (journal — not gated)

Use what you built to drive the real thing. Spawn `ndn-fwd` as a child (M0's
command), let it start, and confirm from the parent that it's alive — capture a
startup log line, or `ndn-ctl status` it, then shut it down cleanly. In your journal:
what did you have to get right about *timing* (the child isn't ready the instant
`spawn` returns), and about *cleanup* (a child you don't wait on becomes a zombie)?
This is exactly how this course's own CLI and CI drive a forwarder in tests.

## Machine strand

A thread context switch costs the kernel real work (save/restore registers, TLB
effects). Spawning 100 threads to sum 100 numbers is *slower* than one thread — the
coordination dwarfs the work. Time `parallel_sum` on a large slice at 1, 4, and 64
workers and note where more threads stop helping (and start hurting). For CPU-bound
data parallelism, reach for `rayon` (`data.par_iter().sum()`) — read its docs and
note what it does that your hand-rolled version doesn't (work-stealing).

## Done means

`./course check m09-workers` green: the parallel sum matches sequential, panics are
isolated and ordered, the child's stdout and exit code are captured — plus clippy
`-D warnings` and fmt. Then `./course submit`.

## Rules of engagement

No `unsafe`, no dependencies. `parallel_sum` must actually use threads (not just
`data.iter().sum()`), and its result must be correct for any `workers >= 1`.
