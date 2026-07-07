# m12-face — your first real component

**Module:** M12 · Growing the system · **Species:** template-assisted
**You write:** `impl Transport for ChannelTransport`, `LinkService::send_packet`, `LinkService::recv_packet`.

This is the junior-contributor checkpoint: not "learn a feature" but "add a piece to
a real system, the way a real system wants it added." You'll build the classic first
face — an in-memory channel transport — behind the same **Transport + LinkService**
split the engine uses, and then learn the *process* that turns working code into a
merged contribution.

## The split (and a correction worth having)

A face is two layers:

- **`Transport`** moves opaque **byte frames** over a medium. It knows *nothing*
  about NDN packets — it's a pipe. UDP, TCP, a Unix socket, a pair of channels: each
  is a different `Transport`. Yours is `ChannelTransport`.
- **`LinkService`** sits above it: it **frames** a network-layer packet — tags it
  with link metadata (which face it's from, congestion marks, fragmentation) — hands
  the bytes down to the transport, and un-frames on the way up.

A common myth: "Transport moves bytes, LinkService moves Interests/Data." Not quite —
**both move bytes.** The real boundary is *link-frame bytes* (Transport) vs
*network-packet bytes + extracted metadata* (LinkService yields a `LinkServiceFrame`).
Typed `Interest`/`Data` are decoded *above* the face, in the engine. Your analog
honors this: `send_packet` takes `&[u8]`, and `recv_packet` yields a `LinkFrame`
(payload + the source face id it un-tagged).

## What you write

- **`Transport for ChannelTransport`** — `id` returns the field; `send_frame` pushes
  onto the outbox and turns a dead channel into `Err(Closed)`; `recv_frame` takes the
  next frame off the inbox without blocking.
- **`send_packet`** — prepend `self.transport.id()` as 4 big-endian bytes (the whole
  "link header" here), then `transport.send_frame`.
- **`recv_packet`** — `transport.recv_frame()`; split the first 4 bytes back into the
  source id and the rest into the payload.

The witness is an **integration test**: it builds a connected pair, wraps each end in
a `LinkService`, and sends packets across — two faces talking, end to end. That's the
right shape of test for a component: exercise it the way the engine will.

## How the real ones differ (read after you pass)

Read `../ndn-rs/docs/wiki/src/guides/implementing-a-face.md` and the traits in
`../ndn-rs/crates/core/ndn-transport/`. Note:
- The real `Transport` methods are **`async`** (`send_bytes`/`recv_bytes` return
  `impl Future`), which makes `Transport` **not object-safe** — so the engine holds
  faces as `Arc<dyn ErasedTransport>`, an object-safe wrapper auto-implemented for
  every transport. That's the RPITIT-vs-`dyn` tension from M8, in production.
- `LinkService` *is* object-safe (it boxes its futures deliberately) so it can be
  `Arc<dyn LinkService>`; the two ship impls are `PassthroughLinkService` and
  `LpLinkService` (NDNLPv2 fragmentation).
- `Face` is a struct — `{ transport: Arc<dyn ErasedTransport>, link_service:
  Arc<dyn LinkService> }` — and `Face::from_transport` picks the LinkService for you.

## The contributor's craft (the real point of M12)

Working code isn't a contribution yet. The workspace has doctrine, and a reviewer
will hold you to it:

- **Feature flags.** New transports are opt-in. In `ndn-face/Cargo.toml`, features
  like `net`, `local`, `l2` gate whole modules; in `ndn-fwd/Cargo.toml`,
  `serial = ["dep:ndn-face-serial"]` gates an *optional dependency* — the face crate
  is only compiled when asked. In your journal: which feature would your new
  transport live behind, and why is opt-in the default for a face?
- **Scope + dependency direction.** Every crate declares `[package.metadata.scope]
  classification = "…"` — one of **`spec`** (implements an NDN spec; strictest),
  **`extension`** (engineering without a spec), **`tooling`** (CLIs), **`draft`** /
  **`research`** (exploratory). The rule: `draft → tooling → extension → spec` — a
  `spec` crate may not depend on anything to its right. Which bucket is your face, and
  what may it depend on?
- **Honesty ledger.** `testbed/EXPECTED_FAILURES.md` records known gaps (status tags
  `EXPECTED-FAIL`, `RESOLVED`, `BLOCKED-BY-INTEROP`, `NOT-WITNESSABLE`). If your face
  has a known limitation, it goes there — not hidden.
- **The PR narrative.** Write your submission as a PR would read: what you added,
  why, what you tested, and one bug you hit and how you found it (from your bug
  journal). A merge is a conversation; make yours easy to say yes to.

## Done means

`./course check m12-face` green: two faces exchange tagged packets both ways, order
preserved, a dropped peer yields `Closed` — plus clippy `-D warnings` and fmt. Then
`./course submit`, and bring the journal: your feature-flag choice, your scope
bucket, and your PR narrative.

## Rules of engagement

No `unsafe`, no dependencies. `send_frame` must report a gone peer as `Closed`, not
panic. Keep the layers separate — the transport must not know about the id header;
that's the LinkService's job.
