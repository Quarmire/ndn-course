# Tutor memory (optional) — PMB

The files under `student/` — `progress.json`, `journal.md`, `bug-journal.md` — are the
student's **authored** record, and they're graded. They are deliberately _not_ the
tutor's memory.

This is the tutor's memory: cross-session observations about _how a student learns_
that no one journals, but that make a tutor genuinely better over time — "confused
reborrowing with moving three times," "the cache-line analogy landed, the vtable one
didn't," "rubric scores run high on idiom, low on error design." It's powered by
**[PMB](https://pmbai.dev)** (Personal Memory Brain) — local-first persistent memory
for AI coding agents over MCP: one local SQLite store under `~/.pmb/`, no cloud, no API
keys, nothing committed.

It is **optional.** The course works without it — the tutor falls back to the `student/`
files. PMB just lets the tutor remember you across sessions instead of meeting you fresh
each time.

## Enable it (once)

```sh
pip install pmb-ai                                     # pure Python, Apache-2.0
pmb connect claude-code --workspace ndn-course-tutor   # wire it into Claude Code over MCP
pmb connect claude-code --probe                        # confirm the server starts
```

Run these from the course repo root. `pmb connect` writes Claude Code's MCP config and
appends its own generic usage rules to `CLAUDE.md` — that's expected; the course-specific
protocol already lives in `CLAUDE.md` under _Tutor memory_. `pmb connect --list` shows
where the config was written. The `--workspace ndn-course-tutor` keeps your course memory
separate from your other projects.

## What the tutor stores (and what it doesn't)

**Stores** — cross-session _teaching signal_ only, written after a session with PMB's
`record_batch_async`:

- recurring misconceptions ("keeps reaching for `.clone()` under a borrow-check error"),
- which explanations landed and which didn't (analogies, the right depth),
- rubric-score patterns across exercises (strong on idiom, weak on error design),
- pacing, and where hints were actually needed.

**Never stores** your graded artifacts. Your journal, bug journal, and progress are yours
to author and live in `student/`. PMB is the tutor's private notebook _about teaching
you_, not a second copy of your work.

## How the tutor uses it

- Before a review or a reflection, it `recall`s (or `prepare`s) what it knows about you,
  and lets that shape _how_ it explains — not _what_ it grades.
- After a session, it records the teaching signal above.

## Privacy

Everything is local: `~/.pmb/ndn-course-tutor/` on your machine. No network, no keys, no
telemetry. Wipe the tutor's memory of you any time with
`rm -rf ~/.pmb/ndn-course-tutor`.
