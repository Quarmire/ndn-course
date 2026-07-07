---
title: Phase 3 — Threads, processes, async
description: Concurrency at the level the OS runs it, before any framework hides it.
---

**Checkpoint: _I can add a real component._** (The junior-contributor checkpoint.)

## M9 · Processes and threads

Concurrency where the kernel does the work: `thread::scope` and channels for a parallel
work distributor, one thread per job with panics contained by `join`, and
`std::process::Command` to drive a whole child process. Placed _before_ async on
purpose, so the OS stays visible.

## M10 · Sharing state

The other half: many threads reading and writing one structure. Build a **sharded PIT**
— the pattern the engine's `DashMap` uses inside — with `Arc`, per-shard `Mutex`es,
lock-free atomic counters, and a static `Send + Sync` assertion the compiler enforces.

## M11 · Async Rust for real

Dependency-free and hand-rolled: implement a `Future` by hand — `poll`, `Pin`, the
`Waker` — and run it on a tiny executor, so `.await` becomes a state machine you can
point at rather than magic you trust. The logging arc closes here, with `tracing`.

## M12 · Add a real component

The junior-contributor checkpoint: build a new face behind the real **Transport +
LinkService** split (an in-memory channel transport, the way the engine ships one),
and learn the _process_ — feature flags, the scope buckets and dependency rules, and a
PR narrative a reviewer can say yes to.

Next: [Phase 4 →](/modules/phase-4-architecture/).
