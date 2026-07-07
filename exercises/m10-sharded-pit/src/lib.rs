//! m10-sharded-pit — sharing state safely across threads.
//!
//! Read SPEC.md. Run the witness with `./course check m10-sharded-pit`.
//!
//! M9 passed values *between* threads. Sometimes you instead need many threads to
//! read and write one shared thing — a forwarder's Pending Interest Table, hit by
//! every face at once. You'll build a `ShardedPit`: a map split across independent
//! `Mutex` locks so threads working on different names rarely wait on each other,
//! shared as `Arc<ShardedPit>`, with lock-free atomic counters. (The real engine's
//! PIT is a `DashMap`, which does exactly this internally.)
//!
//! Every method takes `&self` — the mutation happens *through* a shared reference
//! (interior mutability). That's what lets an `Arc<ShardedPit>` be shared and still
//! be written. The stubs compile; the tests are red until you fill them in.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// A face identifier.
pub type FaceId = u32;

type Shard = Mutex<HashMap<Vec<u8>, Vec<FaceId>>>;

/// A pending-interest table sharded across independent locks. Share it as
/// `Arc<ShardedPit>` and call it from as many threads as you like.
pub struct ShardedPit {
    shards: Vec<Shard>,
    hits: AtomicU64,
    misses: AtomicU64,
}

/// A small deterministic hash (FNV-1a) for choosing a shard. (provided)
#[allow(dead_code)]
fn hash_name(name: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in name {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

impl ShardedPit {
    /// A PIT with `num_shards` independent locks (at least one). (provided)
    pub fn new(num_shards: usize) -> Self {
        let shards = (0..num_shards.max(1))
            .map(|_| Mutex::new(HashMap::new()))
            .collect();
        Self {
            shards,
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// How many shards this PIT has. (provided)
    pub fn num_shards(&self) -> usize {
        self.shards.len()
    }

    /// The shard responsible for `name` — the only lock this operation should take.
    #[allow(dead_code)] // until `insert`/`take` call it
    fn shard_for(&self, name: &[u8]) -> &Shard {
        let _ = name;
        todo!("index self.shards by hash_name(name) modulo the shard count — see HINTS")
    }

    /// Record that `face` is waiting for `name` (append it to that name's entry).
    pub fn insert(&self, name: &[u8], face: FaceId) {
        let _ = (name, face);
        todo!("lock the right shard, then add `face` to the entry for `name` — see HINTS")
    }

    /// Satisfy `name`: remove its entry and return the faces that were waiting
    /// (empty if there were none). A found entry is a hit; nothing found is a miss.
    pub fn take(&self, name: &[u8]) -> Vec<FaceId> {
        let _ = name;
        todo!("lock the shard, remove the entry, and bump hits or misses — see HINTS")
    }

    /// How many `take`s found a waiting entry. (provided)
    pub fn hits(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }

    /// How many `take`s found nothing. (provided)
    pub fn misses(&self) -> u64 {
        self.misses.load(Ordering::Relaxed)
    }
}
