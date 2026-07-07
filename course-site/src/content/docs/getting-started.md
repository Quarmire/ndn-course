---
title: Overview & quick start
description: Clone, run three commands, and Module 0 begins.
---

Learn Rust the way you'll actually use it: inside `ndn-workspace`, a working named-data
networking stack. You start already knowing basic Rust syntax — roughly Chapter 6 of
_The Book_ — and leave able to contribute production-quality code to a large codebase,
and to defend the architecture decisions you make along the way.

## The three-command quick start

```sh
git clone https://github.com/Quarmire/ndn-course && cd ndn-course
bash setup.sh
./course start
```

`setup.sh` checks your toolchain, fetches the pinned workspace snapshots, builds the
course CLI, and runs `course doctor`. If doctor is green, Module 0 begins. This quick
start is a _tested invariant_ — CI runs it in a fresh container on a time budget.

## The loop

Every exercise has the same rhythm:

1. `./course next` — see what's up.
2. Read its `SPEC.md`; write code in `src/lib.rs`.
3. `./course check` — gate 1: your tests, `clippy -D warnings`, and `fmt`.
4. `./course submit` — on green, hand off to the tutor.
5. Open Claude Code in the repo and ask for a review; answer its reflection questions.
6. `./course next` again.

Stuck? `./course hint` reveals the ladder one rung at a time — climbing costs nothing,
but it's recorded honestly.

## What you need

- **Rust** — the course pins an exact toolchain in `rust-toolchain.toml`, so everyone
  compiles with the compiler the tests were written against.
- **~10 hours a week.** Each module is one to two weeks; the capstones are three to
  five weeks each; about seven months end to end. It's self-paced — gates, not the
  calendar, are the authority.
- **Basic Rust syntax.** Everything else, you build.

Next: [how the course works →](/the-course/).
