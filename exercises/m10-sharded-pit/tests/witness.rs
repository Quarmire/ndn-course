//! Witness suite for m10-sharded-pit. Correctness alone, then correctness under
//! eight threads hammering shared entries — the sharding must lose nothing.

use std::sync::Arc;
use std::thread;

use m10_sharded_pit::{FaceId, ShardedPit};

// ------------------------------------------------------------ single-threaded

#[test]
fn insert_take_and_counts() {
    let pit = ShardedPit::new(4);
    pit.insert(b"/ndn/a", 1);
    pit.insert(b"/ndn/a", 2);
    pit.insert(b"/ndn/b", 3);

    assert_eq!(pit.take(b"/ndn/a"), vec![1, 2]); // faces in insertion order
    assert_eq!(pit.take(b"/ndn/b"), vec![3]);
    assert_eq!(pit.take(b"/ndn/a"), vec![]); // already satisfied → miss
    assert_eq!(pit.take(b"/ndn/none"), vec![]); // never inserted → miss

    assert_eq!(pit.hits(), 2);
    assert_eq!(pit.misses(), 2);
}

#[test]
fn correct_regardless_of_shard_count() {
    for shards in [1usize, 2, 7, 64] {
        let pit = ShardedPit::new(shards);
        for i in 0..200u32 {
            pit.insert(format!("k{i}").as_bytes(), i);
        }
        for i in 0..200u32 {
            assert_eq!(
                pit.take(format!("k{i}").as_bytes()),
                vec![i],
                "shards={shards}, key k{i}"
            );
        }
        assert_eq!(pit.hits(), 200);
        assert_eq!(pit.misses(), 0);
    }
}

// -------------------------------------------------------------- under threads

#[test]
fn concurrent_inserts_to_shared_entries_lose_nothing() {
    let pit = Arc::new(ShardedPit::new(8));
    let threads = 8u32;
    let keys = 50usize;

    let handles: Vec<_> = (0..threads)
        .map(|t| {
            let pit = Arc::clone(&pit);
            thread::spawn(move || {
                for j in 0..keys {
                    pit.insert(format!("shared-{j}").as_bytes(), t);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }

    // Every key must now hold exactly one face from each thread — no lost or
    // duplicated pushes, which is what the per-shard Mutex guarantees.
    for j in 0..keys {
        let mut faces = pit.take(format!("shared-{j}").as_bytes());
        faces.sort_unstable();
        let expected: Vec<FaceId> = (0..threads).collect();
        assert_eq!(
            faces, expected,
            "key shared-{j} lost or duplicated an insert"
        );
    }

    assert_eq!(pit.hits() as usize, keys);
    assert_eq!(pit.misses(), 0);
}

// ----------------------------------------------------------------- Send + Sync

#[test]
fn the_pit_is_send_and_sync() {
    // Sharing across threads above only compiles because ShardedPit is Send + Sync
    // — which you get from Mutex + atomics, and would LOSE with Rc/RefCell. This
    // makes the requirement explicit and permanent.
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ShardedPit>();
}
