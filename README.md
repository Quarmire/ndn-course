# ndn-course

Learn Rust where it's real — an applied course taught inside **ndn-workspace + NDF**,
a working named-data networking stack. Sixteen modules, two capstones, and you leave
as a contributor.

```
git clone https://github.com/Quarmire/ndn-course && cd ndn-course
bash setup.sh
./course start
```

That's the whole quick start. `setup.sh` checks your toolchain, fetches the pinned
workspace snapshots, builds the course CLI, and runs `course doctor`. If doctor is
green, Module 0 begins. The quick start is a tested invariant — CI runs it in a
fresh container on a time budget (see `.github/workflows/time-to-first-green.yml`).

## What's here

| Path | What it is |
|---|---|
| `course` / `xtask/` | The course CLI: `doctor`, `start`, `next`, `check`, `hint`, `submit`, `progress` |
| `exercises/` | One crate per exercise, pinned against the workspace snapshots |
| `pins.toml` | The single source of truth for which repo tags this course release targets |
| `CLAUDE.md` | The tutor: persona, Socratic hint policy, grading procedure (gate 2 + 3) |
| `.claude/rubrics/` | Per-exercise review rubrics used by the tutor |
| `student/` | Your progress manifest, journal, and bug journal — commit these in your fork |
| `course-site/` | The Astro + Starlight site: multi-page course text + the landing page |
| `capstones/` | The two capstone briefs + proposal template — after the sixteen modules |
| `.github/workflows/` | `time-to-first-green` (onboarding budget) and `content-rot` (pin drift) |

## How grading works

Every graded artifact passes three gates:

1. **Mechanical** — `./course check <exercise>` runs the provided tests, clippy
   (`-D warnings`), and fmt. Written in the workspace's own audit-witness spirit:
   red before, green after.
2. **Rubric review** — after `./course submit <exercise>` goes green, open Claude
   Code in this repo and ask for a review; the tutor grades against
   `.claude/rubrics/<exercise>.md`, including comment quality and log quality.
3. **Reflection** — the tutor asks follow-ups; you defend your choices.

Hints are Socratic and escalate: `./course hint <exercise>` reveals one rung of
the ladder at a time and records how many you used (no shame — the ladder exists
to be climbed).

## The covenant

Write the best code you can, **and** assume you'll debug it anyway. Both halves
are graded. Every real bug you meet gets an entry in `student/bug-journal.md`:
symptom, hypothesis trail, fix, lesson.

## Status

**Early access.** The pins are cut — `pins.toml` targets `v0.1.0-alpha.2` of
ndn-rs / ndn-fwd / ndn-ext — and the toolchain is pinned to 1.96.0. Two exercises
are live: `m02-varint` and `m03-mini-tlv` (both self-contained; they need no
pinned repos yet). Content modules land phase by phase — see `docs/syllabus.md`
for the full 16-module map, and `course-site/course-landing.html` for the tour.

## Layout note

This repo lives as a sibling inside `ndn-workspace/`, alongside `ndn-rs`,
`ndn-fwd`, and the rest — exercise path-deps resolve as `../ndn-rs/...`, the same
convention the workspace itself uses. If you relocate it, keep it beside the
*split* repos (not beside the legacy monorepo, whose directory is also named
`ndn-rs` one level up).
