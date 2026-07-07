# Rubric — m10-sharded-pit

Gate 2 review. The concurrent test proves correctness under contention; grade
whether the student understands *why* it's safe and *why* it's sharded. Score each
0–2; pass = no zeros and ≥ 10/14. Comment quality and observability are standing.

| # | Criterion | What "2" looks like here |
|---|---|---|
| 1 | Correct sharding | `shard_for` hashes into range and returns one `&Shard`; the student can say why the modulo is safe and what would break with a bad hash |
| 2 | Interior mutability grasped | Can explain how `&self` methods still mutate — the `Mutex`/atomics — and why that's what lets an `Arc` be written by every thread |
| 3 | Minimal locking | Exactly one shard locked per op, held briefly; the atomic bump is *outside* the guard (no lock held across the counter), no two-lock paths |
| 4 | Atomics vs Mutex | `hits`/`misses` use `fetch_add`, not a `Mutex<u64>`; the student can justify the atomic (a counter needs no exclusion) and the `Relaxed` ordering |
| 5 | Send/Sync understood | Can state that `Mutex` + atomics make the type `Sync`, that `Rc`/`RefCell` would not, and that the compiler is enforcing race-freedom at the type level |
| 6 | Sharding rationale + comments | The journal/comments explain sharding as contention reduction (one big lock serializes everyone), naming `DashMap` as the production version |
| 7 | Decision tree (M9 vs M10) | The student wrote their own rule for message-passing vs shared-state and can defend a concrete case each way |

## Reflection prompts (gate 3, pick 2–3)

- With one shard, this is just `Mutex<HashMap>` and correct. What exactly does
  adding shards buy, and what workload makes the difference show up?
- `hits`/`misses` are `Relaxed` atomics. When would `Relaxed` be *wrong* — what kind
  of value can't use it, and why can a plain tally?
- The engine's PIT is a `DashMap`, its FIB a trie of per-node `RwLock`s. Why might
  the FIB (mostly-read routes) prefer `RwLock` where the PIT (constant churn) uses
  shard mutexes?
