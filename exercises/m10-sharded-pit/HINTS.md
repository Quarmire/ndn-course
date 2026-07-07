# Hints — m10-sharded-pit

Revealed one rung at a time by `./course hint m10-sharded-pit`.

## Hint 1 — shard_for: hash, then index

You have `hash_name(name) -> u64` and `self.shards: Vec<Shard>`. Pick the shard by
reducing the hash into range: `&self.shards[(hash_name(name) % self.shards.len() as
u64) as usize]`. `new` guarantees at least one shard, so the modulo is safe. The
point of returning `&Shard` (a `&Mutex<…>`) is that the caller locks *only* this one.

## Hint 2 — insert: lock, then push

`self.shard_for(name).lock().unwrap()` gives you a `MutexGuard` you can use like the
`HashMap` inside. `entry(name.to_vec()).or_default()` gets (or creates) the `Vec` for
that name — `or_default()` makes an empty `Vec` on first insert — then `.push(face)`.
The guard unlocks automatically when it goes out of scope at the end of the statement.

## Hint 3 — take: remove, then count

Lock the shard and `remove(name)` — it returns `Option<Vec<FaceId>>`. Do the removal
in one statement so the lock releases before you touch the counter:

```rust
let removed = self.shard_for(name).lock().unwrap().remove(name);
match removed {
    Some(faces) => { self.hits.fetch_add(1, Ordering::Relaxed); faces }
    None => { self.misses.fetch_add(1, Ordering::Relaxed); Vec::new() }
}
```

`fetch_add` is the atomic increment — no lock needed for a counter. `Relaxed` is
fine here: the counts don't order any other memory, they're just tallies. (You'll
need `use std::sync::atomic::Ordering;` — it's already imported.)

## Hint 4 — if the concurrent test hangs or loses updates

A hang usually means you locked *two* shards at once, or held a guard across a
`.lock()` on the same shard (a deadlock). Each operation should take exactly one
lock, briefly. Lost or duplicated faces mean two threads' pushes raced — which can't
happen if the `push` is inside the guarded section, so check that you're pushing
while the lock is held, not after cloning the map out.
