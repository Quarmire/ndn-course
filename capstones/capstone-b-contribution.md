# Capstone B — The contribution

**Identity:** systems contributor · **Time:** ~3–5 weeks · **Deliverable:** a PR-quality change + design note + viva.

You land a real change in a real, moving codebase, through the real flow. This capstone
most resembles the job: you don't own the repo, you must fit its conventions, and a
reviewer has to be able to say yes.

## The contributor flow (this IS the deliverable — not just the diff)

1. **Issue** — state the problem and why it's worth fixing, in the repo's own terms.
2. **Design note** — *before code*, a decision note (house format, M14) on your
   approach, naming the alternative you rejected and why. Reviewed first.
3. **Implement** — behind a feature flag if it's a new surface (M12), respecting the
   scope buckets and the `draft → tooling → extension → spec` dependency rule (M12's
   doctrine). Read code the way M1 taught you before you touch it.
4. **Witness tests** — write the audit-witness that **fails before your change and
   passes after** (the workspace's own convention: exit 1 broken, 0 fixed — M5/M12).
5. **PR** — a narrative a reviewer can say yes to: what, why, what you tested, and one
   bug you hit and how you found it (from your bug journal).
6. **Review rounds** — respond to feedback like a colleague. The *rounds* are graded,
   not just the final state.

## Curated tickets

> ⚠️ These repos move. Confirm a ticket is still open and correctly scoped **with the
> maintainer** before you commit weeks to it — the "read the current state, don't trust
> a stale map" reflex you built in M0/M1 and that this course applied to NDF itself.

Sized for supervised student work. Pick one, or propose one you found:

- **ndn-workspace**
  - **Witness-coverage gap** — find a spec-scope crate behavior with no witness and
    write one, in the audit-witness style.
  - **A real face** — grow your M12 in-memory transport into a real backend (a second
    serial / websocket transport, feature-gated) following `implementing-a-face.md`.
  - **docs-contract** — a place where a doc and the code disagree (you found these in
    M0/M1); fix whichever is wrong, with a test that keeps them honest afterward.
  - **An `ndn-tools` improvement** — a missing flag or output mode on a CLI binary.
  - **The Dioxus course-site port** — port this course's landing page to a Dioxus app.
- **NDF (`ndf-rs`)**
  - **Chain/fork golden vectors** — ndf-core's corpus notes chain/fork "golden outcomes"
    as a follow-up. Mine a handful of vectors and wire them into the harness.
  - **A conformance-corpus case** — add a fixture exercising an untested verifier path
    (an unhandled `DenyReason`, an `Unresolved` boundary — you built the verifier's
    shape in M14/M15).
  - **A reserved `Kind`** — implement one against its criterion, with vectors.

## Requirements (the rubric enforces these)

- The design note **precedes** the code and names a rejected alternative.
- The change respects scope + dependency direction; new surfaces are feature-gated.
- Your witness tests are **fail-before / pass-after**.
- The PR narrative is reviewable, and you handled **at least one** review round.
- `clippy -D warnings` + `fmt` green; the repo's CI conventions honored.

## Done means

A merged-or-mergeable change, its design note and witnesses in place, its PR narrative
written, at least one review round handled — and in the viva you can defend *why this
approach and not the one you rejected*, and say what would break first under change.
