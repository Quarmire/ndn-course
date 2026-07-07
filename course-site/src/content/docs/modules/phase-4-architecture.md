---
title: Phase 4 — Architecture & NDF
description: Macros, an architecture with a fourth verdict, and a real verifier.
---

**Checkpoint: _I can decide and defend._**

## M13 · Macros and codegen

A macro runs at compile time and emits code, so it does what a function can't. Write a
`macro_rules!` type registry that generates constants and lookups from one table, and a
builder macro — plus the decision ladder for _when not_ to reach for a macro at all.

## M14 · NDF: architecture as a first-class skill

No new syntax — judgment. Implement NDF's signature idea in a dozen lines: the **fourth
verdict**. Most systems answer a permission question two ways; NDF answers four, and
refuses to collapse "I can't tell" into "no." `Unresolved` is first-class — "I don't
know, and here's exactly what's missing." Then you read a live design corpus and write a
decision note taking a position, in the house format.

## M15 · Trust and the verifier

The last core module, where every habit has to be exactly right because a mistake is a
security bug. A short-circuiting authorization pipeline with a typed reason taxonomy and
**default-refuse**, where an unfetched grant returns `Unresolved` (not a denial) — plus
a longest-prefix trust keyring, and crypto hygiene (why you never compare a signature
with `==`).

Next: [the capstones →](/capstones/).
