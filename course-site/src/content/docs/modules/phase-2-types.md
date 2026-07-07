---
title: Phase 2 — Types that carry meaning
description: Design an API, parse without copying, and choose your dispatch.
---

**Checkpoint: _I can design an API and prove a parser._**

## M6 · Names and API design

Design a type, not just a function: a `NameComponent`/`Name` with the `Eq`/`Ord`/`Hash`
contract, `From`/`Display`, and NDN's **canonical ordering** — type, then _length_,
then content. It's graded against the real `ndn-foundation-types` as an oracle, which
catches the classic trap of a plain `#[derive(Ord)]`.

## M7 · Zero-copy parsing

A lazy `DataView<'a>` over a packet that decodes nothing until asked, and the two
escapes from a borrow: `Cow` (own only when you must) and `bytes::Bytes` (own to
outlive the buffer). The three ways to hold bytes, and when to reach for each.

## M8 · Traits, generics, closures

One `Strategy` trait and every dispatch tool Rust offers, each with a real reason:
generics (monomorphized) vs `dyn` (a vtable, for heterogeneity), closures behind an
`Fn` bound, `OnceLock`, and object-safety taught through the workspace's deliberately
_non_-object-safe `Face` trait. The course's first property/fuzz lab lands here.

Next: [Phase 3 →](/modules/phase-3-concurrency/).
