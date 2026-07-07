---
title: Capstones
description: Build something real and defend it — two identities, one three-gate arc.
---

You've finished the sixteen modules. The capstones are where you _prove_ it — not with
a fill-in-the-blank, but by building something real and standing behind it. There are
two, and they're two **identities**, not two difficulties.

## Capstone A — The application

You are an _app developer_. Build a real named-data application on `ndn-app`'s Node API,
running against a live `ndn-fwd` — a tap-to-share tool, a LAN chat over sync, a sensor
telemetry dashboard, or your own idea. It must run in minutes from its README, be tested
and instrumented with `tracing`, and sign and verify its data. You designed the
namespace; you'll defend it.

## Capstone B — The contribution

You are a _systems contributor_. Land a PR-quality change in `ndn-workspace` or the NDF
refounding, through the real flow: issue → design note → implement → a witness that fails
before your change and passes after → a PR narrative → review rounds. The flow _is_ the
deliverable, not just the diff.

## The shape of a capstone

Both follow the three-gate arc, scaled up:

1. **Proposal gate** — a one-page design doc plus at least two decision notes, reviewed
   _before_ you write code. The "plan to back it" the course promised on day one.
2. **Build** — your own crate or fork, your own tests, your own minutes-to-running
   README, your own `tracing`.
3. **Viva** — a spoken defense: _why this and not that_, what breaks first, what you'd
   change.

The rules also flip here: using an AI agent to build was off-limits through the modules;
in the capstone it's encouraged, and what's graded is your **judgment** — the choices
you can defend, not whether you typed every line.

The full briefs and a proposal template live in the repo under
[`capstones/`](https://github.com/Quarmire/ndn-course/tree/main/capstones).
