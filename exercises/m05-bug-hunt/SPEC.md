# m05-bug-hunt — find them, fix them, write it down

**Module:** M5 · Debugging, and code that debugs itself · **Species:** planted-bug hunt
**You fix:** four bugs in `src/lib.rs` + open `student/bug-journal.md` with one entry per bug.

Every module so far, you wrote code from nothing. This one hands you code that is
already written, compiles, reads fine — and is wrong. That's the real job: most of
a career is spent reading and fixing code that *runs* but misbehaves. This module
makes the covenant explicit — **write the best code you can, and assume you'll
debug it anyway** — and it opens your bug journal, which every checkpoint from here
will read.

## The hunt

`src/lib.rs` has **four bugs, one per `pub fn`** (`decode_varu64` is correct — trust
it). Each function's doc comment says what it *should* do; a bug is code that
disagrees with its own contract. Run `./course check m05-bug-hunt` and you'll see
red — several tests, but only **four root causes**. One bug can redden several
tests; tracing many symptoms back to one cause is part of the skill.

The four are four different *kinds* — the ones that get everyone:
- a **panic-hiding** unhandled case (fine on the happy path, panics on an edge),
- a **logic slip** (a comparison that says the opposite of what was meant),
- an **off-by-one** (a boundary that reaches one step too far — and panics),
- a **slice/advance misuse** (a cursor that moves by the wrong amount).

## The method — search, don't guess

- **Read the failing test names.** They point at the function and the case.
- **Backtrace the panics:** `RUST_BACKTRACE=1 cargo test -p m05-bug-hunt`. The top
  `src/lib.rs` frame is your crime scene.
- **Minimize:** what's the smallest input that reproduces — an empty name? a
  two-component one? The witness already does some of this; read it.
- **Compare to known-good:** you wrote correct versions of two of these in M4. A
  `git diff` in your head against what you know is right is a real technique.
- **Hypothesis, then test:** change *one* thing, re-run, confirm. No shotgun edits.

## Then prevention — code that debugs itself

Fixing is half the covenant. For each bug ask *what would have made it
impossible?* — and apply one such defense in the file:
- handle the edge with `slice::first`/`get` instead of `[i]` that can panic;
- a `debug_assert!` stating an invariant, so the next wrong assumption trips loudly
  in a test instead of silently in production;
- an exhaustive `match` (no `_ =>` hiding a case);
- run `cargo clippy` before you call anything done — it's a free reviewer.

## Comments and logging start counting here

From this module on, **comment quality and observability are graded on every
exercise.** Good comments say *why*, not *what* — capture the invariant a reader
would otherwise reverse-engineer. And picture the log line future-you would want at
a decision point; the level ladder is `error` (it broke) → `warn` (suspicious) →
`info` (a milestone) → `debug`/`trace` (the play-by-play). You'll wire real
`tracing` in M11; here, a `debug_assert!` and a precise comment are your instruments.

## Done means

`./course check m05-bug-hunt` green (all four fixed) **and** `student/bug-journal.md`
with four entries — one per bug — each: **symptom** (what you saw) → **hypothesis**
(what you suspected, and how you confirmed) → **fix** (the one-line change) →
**lesson** (the class of bug, and the defense you added). Then `./course submit`
and ask the tutor — expect to be asked how you *found* each one, not just what it was.

## Rules of engagement

Fix each bug **minimally** — change what's wrong, not what offends you (save the
refactor for a kata). No dependencies, no `unsafe`. The bug journal is not optional
and not busywork: it's what turns a fixed bug into a lesson you keep.
