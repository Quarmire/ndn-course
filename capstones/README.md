# Capstones

You've finished the sixteen modules. You can read this codebase, add to it, debug it,
and reason about its architecture. The capstones are where you *prove* it — not with a
fill-in-the-blank, but by building something real and defending it.

There are two, and they're two **identities**, not two difficulties. Do the one that
matches who you want to be; do both if you want the whole arc.

- **Capstone A — The application.** You are an *app developer*: build a real named-data
  application on ndn-app's Node API, running against a live `ndn-fwd`.
  → [`capstone-a-application.md`](capstone-a-application.md)
- **Capstone B — The contribution.** You are a *systems contributor*: land a PR-quality
  change in ndn-workspace or the NDF refounding, through the real contributor flow.
  → [`capstone-b-contribution.md`](capstone-b-contribution.md)

## The shape of a capstone

Both follow the same three-gate arc — the same gates as every module, scaled up:

1. **Proposal gate (before you write code).** A one-page design doc plus **at least
   two decision notes** in the house format (M14). You do not start building until the
   tutor has reviewed and approved it. This is the "plan to back it" the course
   promised on day one — a capstone that begins with code and no plan fails the
   proposal gate on principle. Use [`PROPOSAL-TEMPLATE.md`](PROPOSAL-TEMPLATE.md).
2. **Build.** Your own crate or fork, your own tests, your own `tracing`, your own
   minutes-to-running README. The mechanical gate is yours to run: `cargo test`,
   `cargo clippy -- -D warnings`, and `cargo fmt --check` all green, on your code.
3. **Viva.** A spoken defense: the tutor asks *why*, probes the seams, and asks what
   you'd do differently. From-scratch assessments and capstones both end in a viva
   because the point was never the code — it's whether you can stand behind it.

## The rules changed (on purpose)

Through the modules, using an agent to write an assessment before your first submission
was off-limits. **Not here.** Working effectively *with* an AI agent on a large codebase
is now a core professional skill, and the capstone is where you practice it — use the
tutor to scaffold, review, rubber-duck, and find the API. What's graded is the
**judgment**: the choices you made and can defend, not whether you typed every line. An
agent that builds the wrong thing fast is still building the wrong thing; the proposal
and the viva are where your judgment shows.

## Starting

Read both briefs, pick your identity, and copy [`PROPOSAL-TEMPLATE.md`](PROPOSAL-TEMPLATE.md)
into your capstone repo/branch. Draft your proposal, then open Claude Code here and say:

> review my capstone proposal

The tutor runs the proposal gate. **Do not write implementation code until it's approved** —
that gate is the whole point of the module that taught you architecture.
