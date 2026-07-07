# Capstone proposal — <your title>

**Capstone:** A (application) / B (contribution) · **Author:** <you> · **Date:** <date>

Copy this file into your capstone repo/branch, fill it in, and ask the tutor to review
it **before you write implementation code**. Keep it to about a page plus the decision
notes — a proposal that sprawls usually hasn't decided anything yet.

## 1. The one-page design doc

**What you're building** (one paragraph).
> …

**Why it's worth building / fixing** (the problem, in the codebase's terms).
> …

**Scope — what's in, and what's explicitly out.** An honest out-of-scope list is a sign
of judgment, not weakness.
> In: …
> Out: …

**The shape.** Crates/modules, the key types, and — for Capstone A — the *namespace* you
designed; for Capstone B — the files you'll touch and the feature flag (if any). A
diagram in words is fine.
> …

**How you'll test it.** The witness(es) that will prove it works — and, for a
contribution, that fail before your change and pass after.
> …

**Risks.** What could go wrong, and your fallback for each.
> …

## 2. Decision notes (at least two, house format)

One note per real fork in the design. Capture *the reasons, not just the artifact*.

### D-1 — <the decision, one line>
- **Principle (litmus):** <a one-line rule a reader could apply to a similar choice>
- **Why:** <the reasoning>
- **Replaces / rejects:** <the alternative you did NOT take, and why>
- **Authority:** ruling / recommendation / advisory
- **Status:** proposed <date>

### D-2 — <the decision, one line>
- **Principle (litmus):** …
- **Why:** …
- **Replaces / rejects:** …
- **Authority:** …
- **Status:** proposed <date>

## 3. Milestones

1. Proposal approved (this doc).
2. Walking skeleton — the thinnest end-to-end slice that proves the pipe works.
3. The real thing, with tests + `tracing`.
4. README / PR narrative, then the viva.
