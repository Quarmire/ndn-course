# m14-verdict — NDF: architecture as a first-class skill

**Module:** M14 · NDF: architecture as a first-class skill · **Species:** template-assisted + decision note
**You write:** `verdict` and `authorize` (code) + a **decision note** in your journal (the real weight).

This module isn't about new syntax — it's about *judgment*. You'll read a live
architecture, implement its signature idea in a dozen lines, and then do the thing a
senior engineer actually does: read a design corpus, take a position, and write it
down so others can argue with it.

## NDF in one breath

**NDF (Named Data Fabric)** is the *meaning* layer built on NDN's *mechanism* layer.
Its thesis: information should be **named and owned directly** — addressed by *what
it is*, not *where it lives* — so it can move between apps and devices without being
trapped in a silo. Its stance is **"interoperation over consolidation"**: instead of
everyone moving onto one platform, many systems speak the same named data.

The atom is a **Block** — but a Block is realized as *Block-as-Data*: an ordinary
NDN Data packet carrying an in-Content NDF header, with **one unified NDN
signature**, so a *stock* NDN verifier accepts it. (You built the NDN half of this
in M7/M12.)

## The signature idea: four verdicts

Most systems answer a permission question two ways. NDF answers **four** — and the
fourth is the whole point:

- **Accept** — yes.
- **Accept with caveats** — yes, but note this.
- **Deny** — a genuine, crisp no.
- **Unresolved** — *"I can't tell at this precision, and here's exactly what's
  missing."*

`Unresolved` is **first-class — never a disguised `Deny`.** Collapsing "I don't
know" into "no" is the quiet lie most systems tell; NDF refuses it. The caller of an
`Unresolved` chooses: accept lower assurance, escalate to a finer backend, or route
to human ratification. This is *totality over partial knowledge* — the system is
allowed to say "I don't know," out loud, with the gap attached.

The **pure core** of that idea is a three-zone decision over a coverage estimate
`estimate ± margin` against a `threshold` (faithful to
`ndf-rs/refounding/ndf-core/src/spatial.rs`):

- band clears the threshold decisively → **Covered**
- band falls short decisively → **NotCovered**
- threshold sits *inside* the band → **Unresolved** (you can't honestly pick a side)

## The code (small on purpose)

- **`verdict(c, threshold)`** — the three-zone check above, returning `SpatialVerdict`.
- **`authorize(c, threshold)`** — elevate it: `Covered → Authorized`,
  `NotCovered → Refused`, `Unresolved → Unresolved { estimate, margin, threshold }`.
  The one assertion that matters: a near-threshold estimate must come back
  `Unresolved` **carrying the three numbers**, *not* `Refused`. That single boundary
  is the architecture.

## The architecture reading + your decision note (the real work)

Read the current NDF corpus — the thesis (`ndf-rs/site/guide.html`), the code you're
modeling (`spatial.rs`, `verify.rs`), and the decision doctrine. NDF records
architecture as a living argument:

- **Decision notes (`D-N`)** each carry *what was decided, why, and what it replaced*,
  with **supersession edges** so churn stays visible.
- **Rulings** are tagged by authority: **ruling** (binding), **recommendation**
  (structural precedent, overturnable), **advisory** (opinion only) — and capture
  *the reasons, not just the artifacts*.
- Designs are proven by surviving **adversarial campaigns** (the "gauntlet" /
  red-team), which produce numbered **findings** and obey a **starvation rule**:
  additions require a *failing obligation*, not convenience.
- The **boundary litmus** still governs, now as the **waterline**: *moving/storing
  bytes → ndn-workspace; the meaning of bytes → NDF.* Refined into three strata —
  mechanism (ndn-rs) below, **policy-free calculi** (spec crates that may sink below
  the waterline) in the middle, and **policy/governance/authority** (NDF) above. Any
  NDF crate that binds an engine or forks a crypto primitive gets an *eviction date
  the day it's written.*

**Your decision note.** Pick a real tension and write a decision note in the house
form — *the principle (a one-line litmus), why, what it replaces or rejects, and its
authority tag (ruling / recommendation / advisory)*. A good tension to take a
position on:

> A caller receives `Unresolved`. Should the *system* pick a safe default (treat it
> as `Deny`), or must the caller **always** choose? Argue it, with the four-verdict
> thesis as your evidence.

Put it in `student/journal.md`. This is graded as heavily as the code.

## Done means

`./course check m14-verdict` green: the three zones, the `unknown()` floor (always
`Unresolved`), and `Unresolved` proven distinct from `Refused` — plus clippy
`-D warnings` and fmt. And a decision note in your journal. Then `./course submit`.

## Rules of engagement

No dependencies. Do not let `Unresolved` collapse into `Refused` "to be safe" — that
collapse is exactly the design error this module exists to inoculate you against.
