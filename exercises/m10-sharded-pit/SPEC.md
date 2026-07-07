# m10-sharded-pit — sharing state without races

**Module:** M10 · Sharing state: smart pointers and concurrency · **Species:** template-assisted
**You write:** `shard_for`, `insert`, `take`.

M9 gave you two ways for threads to cooperate; this is the other one. When many
threads must read and write *one* structure — a forwarder's Pending Interest Table,
touched by every arriving Interest and Data — you share the state and synchronize
access to it. You'll build the pattern the real engine uses: a map sharded across
independent locks, wrapped in an `Arc`, with atomic counters.

## The tools, and when

- **`Rc<RefCell<T>>`** — share + mutate within *one* thread. `RefCell` moves the
  borrow check to run time; `Rc` is a non-atomic refcount. Neither is `Send`/`Sync`,
  so the compiler won't let either cross a thread boundary. (Not used here — but
  know why it *can't* be.)
- **`Arc<Mutex<T>>` / `Arc<RwLock<T>>`** — share + mutate *across* threads. `Arc` is
  an atomic refcount (safe to clone from any thread); `Mutex` gives one writer at a
  time; `RwLock` gives many readers *or* one writer (reach for it when reads
  dominate). This is what makes `Arc<ShardedPit>` work.
- **Atomics (`AtomicU64`, …)** — a single value updated lock-free. A counter doesn't
  need a whole `Mutex`; `fetch_add` is one instruction's worth of synchronization.
  Your `hits`/`misses` are atomics for exactly this reason.

## Interior mutability

Notice every method takes `&self`, yet `insert` and `take` change the table. That's
*interior mutability*: the `Mutex` (and the atomics) let you mutate through a shared
`&`. It's the whole reason an `Arc<ShardedPit>` — which only ever hands out shared
references — can still be written by every thread holding a clone.

## Sharding (why not one big lock)

One `Mutex<HashMap>` is correct but serializes *every* operation — all threads queue
on one lock. Split the map into N shards, each its own lock, and pick a shard by
hashing the name. Now two threads on different names take different locks and run in
parallel; only same-shard collisions wait. That's all a `DashMap` is, and it's how
the engine's PIT (and its FIB, a trie of per-node `RwLock`s) scale.

- `shard_for(name)` → `&self.shards[hash_name(name) % num_shards]` (both provided).
- `insert` → lock that shard, `entry(name).or_default().push(face)`.
- `take` → lock that shard, `remove(name)`; `Some(faces)` is a hit, `None` a miss —
  bump the right atomic and return the faces (or an empty `Vec`).

Take the lock for as *little* as possible: don't hold a shard's guard while you
touch an atomic — let the guard drop first.

## Send + Sync

The witness shares your `ShardedPit` across eight threads via `Arc` and asserts
`ShardedPit: Send + Sync`. You get both for free *because* you used `Mutex` and
atomics (which are `Sync`). Had you reached for `RefCell` or `Rc`, the type would
not be `Sync`, and neither the `Arc` sharing nor that assertion would compile — the
compiler enforcing "no data races" at the type level. Read a real `ndn-store` type
and note what makes it thread-shareable.

## Done means

`./course check m10-sharded-pit` green: single-threaded correctness, eight threads
piling inserts onto shared entries with **nothing lost or duplicated**, exact
atomic counts, and the `Send + Sync` assertion — plus clippy `-D warnings` and fmt.
Then `./course submit`.

## Machine strand + rules

No `unsafe`, no dependencies. Contention is real: read Mara Bos's *Rust Atomics and
Locks* (ch. 1–3) and note *false sharing* — two atomics on the same cache line
fighting even when logically independent. Decision-tree homework: write your own
rule for message-passing (M9) vs shared-state (M10), and defend it in review.
