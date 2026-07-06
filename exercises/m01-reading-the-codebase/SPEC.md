# m01-reading-the-codebase — find your way around ndn-rs

**Module:** M1 · Reading a big codebase · **Species:** observation lab (scavenger hunt)
**You produce:** five findings in `src/lib.rs` + a pipeline trace in journal entry #2.

Real work starts with reading, not writing. ndn-rs is 31 crates; nobody holds it
all in their head. The skill is *finding the part you need* — with grep, with
`cargo doc`, by following `use` statements — and reading it precisely. This is a
guided hunt, plus your first measurement of what real types cost in memory.

## 1. The map

Read the top of `../ndn-rs/ARCHITECTURE.md`. It draws the forwarding pipeline as:

```
Interest: FaceCheck → TlvDecode → CsLookup → PitCheck → Strategy → Dispatch
Data:     FaceCheck → TlvDecode → PitMatch → Validation → CsInsert → Dispatch
```

That's the mental model. But the doc writes crate paths *flat*
(`crates/ndn-engine/...`) while the real tree is *grouped*
(`crates/forwarding/ndn-engine/...`). Your first lesson: **the doc orients you,
the code is the truth.** When a path from the doc 404s, that's expected — go find
the real one.

## 2. The hunt — record repo-relative paths in `src/lib.rs`

Search `../ndn-rs` (ripgrep is your friend: `rg PitEntry ../ndn-rs`) and record,
as paths relative to `../ndn-rs`:

- `PIT_CREATE_FILE` — where a PIT entry is created when an Interest misses the
  table. (Hunt: where is `PitEntry::new` called from a pipeline stage?)
- `CS_INSERT_FILE` — where a Data packet is inserted into the content store, and
  where the `ctx.verified` flag decides whether it may be cached.
- `PIPELINE_FILE` — the file that drives the ordered stages (the real
  `interest_pipeline` / `data_pipeline`, not the ASCII diagram).

The witness checks each named file exists and contains what it should — it
verifies you *found the right place*; the tutor checks that you understand what
happens there.

## 3. The machine strand — measure, don't guess

This exercise depends on the real `ndn-foundation-types` crate. Measure and record:

- `SIZE_OF_HASH` = `std::mem::size_of::<ndn_foundation_types::Hash>()`
- `SIZE_OF_NAME_COMPONENT` = `std::mem::size_of::<ndn_foundation_types::NameComponent>()`

`Hash` wraps a `[u8; 32]`. `NameComponent` holds a `u64` tag and a `bytes::Bytes`
value — and it is *larger* than the bytes it points at, because `Bytes` is a fat
handle (pointer + length + capacity/refcount), not the data itself. Why does that
distinction matter for a zero-copy parser? (Hold that thought for M7.)

## 4. Trace it — journal entry #2

Trace the **Data path** in prose: a Data packet arrives on a face — walk it through
`PitMatch → Validation → CsInsert`, naming the real stage structs and what each
decides. Then explain the `ctx.verified` gate: what happens to a Data packet whose
signature was *not* verified — dropped, forwarded, cached? (Read `CsInsertStage`.)
Put this in `student/journal.md` as entry #2.

## Done means

`./course check m01-reading-the-codebase` green (files located, sizes measured)
**and** journal entry #2 (the Data-path trace + the verified-gate explanation).
Then `./course submit` and ask the tutor — expect to be asked *why* the verified
gate exists and what it protects.

## Rules of engagement

Reading the code is the whole point — read as much of ndn-rs as you like, run
`cargo doc --open` inside `../ndn-rs`, grep freely. The findings are open-book; the
understanding is what the tutor probes in review.
