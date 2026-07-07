---
title: How the course works
description: Three gates, four strands, a covenant, and an LLM tutor.
---

## Three gates on every exercise

1. **Mechanical** — `./course check` runs the provided tests, `clippy -D warnings`,
   and `fmt`, in the workspace's own audit-witness style: red before, green after.
2. **Rubric review** — after you submit, the LLM tutor grades your code against a
   per-exercise rubric: idiom, design, error handling, and — from module 5 on —
   comment quality and observability.
3. **Reflection** — the tutor asks what the rubric can't check: _why this design and
   not the nearest alternative?_ From-scratch assessments and the capstones add a viva.

## Four strands, braided through every module

- **Rust** — the language topic of the week.
- **Domain** — named data: Interests and Data, names instead of hosts, trust.
- **Machine** — what's under the hood: struct layout, the vtable in the assembly, the
  cost of a thread context switch. _Derive it; don't memorize it._
- **Craft** — testing, debugging, comments, logging, honest known-issues files.

## The covenant

Write the best code you can, **and** assume you'll debug it anyway — both halves are
graded. From module 5 on, every real bug you meet earns an entry in your bug journal:
symptom → hypothesis → fix → lesson.

## The tutor

The tutor is an LLM — Claude Code, opened in the repo. It is Socratic by policy: it
won't write a solution before your first submission, and hints go through the ladder.
But it will explain any concept, any workspace code, any compiler error, at any depth —
because explaining is teaching, and solving is not. In the capstones that rule flips:
working _with_ an agent on a large codebase becomes a graded skill.

Optionally, the tutor can gain a **cross-session memory** of how you learn — recurring
misconceptions, which explanations landed, your rubric patterns — via
[PMB](https://pmbai.dev), a local-first memory that plugs in over MCP. It's the tutor's
private notebook about teaching you; your graded work stays yours, in `student/`.

## It won't rot

The course is pinned to tagged snapshots of the real repos, and a scheduled CI job
rebuilds every reference solution against those pins — so drift becomes a red build,
not a silent surprise.

Next: [the journey begins →](../modules/phase-0-orientation/).
