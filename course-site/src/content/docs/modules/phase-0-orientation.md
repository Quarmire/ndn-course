---
title: Phase 0 — Orientation
description: Run the forwarder, read a big codebase.
---

**Checkpoint: _I can run it and find things._**

Before you write a line, you run the real thing and learn to navigate it — because an
intern's hardest first day is a codebase they can't move around in.

## M0 · Zero to running

Get the toolchain and workspace running, start `ndn-fwd` from its default config, and
poke it with the CLI tools. You record what you observe — the ports, the content-store
size, the management socket — and catch your first _doc-vs-reality_ mismatch (the
no-config forwarder opens zero faces, whatever a README claims). The reflex it builds:
trust the code over the prose.

## M1 · Reading a big codebase

A scavenger hunt through `ndn-rs`: find where a PIT entry is created, where Data is
cached and why a `ctx.verified` flag gates it, and locate the real forwarding pipeline.
Then the Machine strand's first appearance — measure what real types cost in memory
with `size_of`. Grep discipline and workspace anatomy are themselves the skill.

Next: [Phase 1 →](../phase-1-bytes/).
