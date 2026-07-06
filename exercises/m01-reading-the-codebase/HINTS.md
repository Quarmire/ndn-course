# Hints — m01-reading-the-codebase

Revealed one rung at a time by `./course hint m01-reading-the-codebase`.

## Hint 1 — grep is how you read a big codebase

Don't read top-to-bottom; search. `rg PitEntry ../ndn-rs` finds every mention of
the PIT entry type in seconds; `rg "fn.*insert" ../ndn-rs/crates/forwarding` narrows
to the forwarding crates. The pipeline stages all live under one directory —
`../ndn-rs/crates/forwarding/ndn-engine/src/stages/` — and that directory alone
answers most of the hunt.

## Hint 2 — the PIT and the content store

An Interest that misses the table creates a PIT entry inside a *stage*. Grep for
`with_entry_or_insert` and for `PitEntry::new` — they're in the same file. For
caching, grep for `insert_erased`: the stage that calls it is where Data enters the
content store, and the `if !ctx.verified` a few lines above it is the gate you're
also asked about.

## Hint 3 — the pipeline driver

The ASCII diagram in ARCHITECTURE.md is not code. The real ordered stage calls live
in a file under `dispatcher/`, in methods literally named `interest_pipeline` and
`data_pipeline`. Grep for `interest_pipeline`.

## Hint 4 — measuring a type's size

Add a throwaway test to `src/lib.rs` and run it with output shown:

```rust
#[test]
fn measure() {
    println!("Hash = {}", std::mem::size_of::<ndn_foundation_types::Hash>());
    println!("NameComponent = {}", std::mem::size_of::<ndn_foundation_types::NameComponent>());
}
```

Run `cargo test -p m01-reading-the-codebase measure -- --nocapture`, read the two
numbers off stdout, and record them. Then delete the throwaway — or keep it; a
self-authored test is a point in the rubric.
