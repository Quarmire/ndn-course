# Capstone A — The application

**Identity:** app developer · **Time:** ~3–5 weeks · **Deliverable:** a real named-data app + design doc + viva.

You build something a person would actually use, on named data, and run it against a
real forwarder. The point is to *feel* what NDN gives an application: data addressed by
name, signed at rest, fetched from wherever it lives — not from a particular server.

## Build on

- **`ndn-app`'s Node API** — `use ndn::Node;` (the `ndn-rs-prelude`, which you met in
  M1). A `Node` is your whole application surface: produce Data under a name, express
  Interests, consume by name. You don't touch faces or the engine directly — that's the
  point of the app layer.
- **A live `ndn-fwd`** — deploy against the forwarder. M0 taught you to run it, M9 to
  drive it as a child process; the workspace ships a docker-compose testbed for a
  multi-node setup. Your app connects to the forwarder like any real application would.

## Pick one (or propose your own)

- **Tap-to-share** — one device publishes a named blob; another fetches it by name over
  a shared link. The "hello world" of named data, done *properly*: signed, versioned,
  verified.
- **LAN chat over SVS sync** — a small group chat where messages are named Data and
  members converge via State Vector Sync. You'll meet eventual consistency for real.
- **Sensor telemetry + a dashboard** — a producer publishes named readings; a consumer
  plots them (optionally a panel in ndn-dashboard). Naming a time series is a real
  design problem.
- **A named-data toy for your own hardware** — an embedded / mobile / radio idea via
  ndn-embedded, ndn-mobile, or ndn-radio-drivers. Highest risk, highest reward.
- **Your own** — anything that genuinely exercises *produce → name → sign → fetch →
  verify*. Propose it in your design doc.

## Requirements (the rubric enforces these)

- **It runs in minutes.** A README with a three-command quick start that actually works
  on a fresh checkout — the same invariant this course holds itself to (`time-to-first-green`).
- **Tests.** Not "it compiled" — tests that pin behavior: a produce→consume→verify
  round-trip at minimum. The covenant ("assume you'll debug it") does not retire.
- **`tracing`.** Instrument the interesting path with spans (M11). You should be able to
  *watch* an Interest go out and Data come back.
- **Signed and verified.** Data is signed; the consumer verifies it. Your namespace is
  deliberate — you designed it, and you'll defend it (M6 taught you names are a design
  problem).
- **A bug journal.** Every real bug you hit gets an entry (M5). The capstone is where
  the covenant earns its keep.

## Milestones (put these in your proposal)

1. **Proposal approved** — design doc + at least two decision notes (e.g. your namespace
   design; your sync-or-trust choice).
2. **Walking skeleton** — produce one named Data, consume it, verify it, end to end.
   Nothing fancy; prove the pipe works.
3. **The real feature** — with tests and `tracing`.
4. **Ship** — the minutes-to-running README, a short demo, then the viva.

## Done means

The app runs from its README in minutes; `cargo test` / `clippy -D warnings` / `fmt`
are green; a trace shows the named-data path; the design doc and decision notes live in
the repo; and in the viva you can defend your namespace and trust choices — and name the
one thing you'd design differently next time.
