# m00-zero-to-running — get it running, then observe

**Module:** M0 · Zero to running · **Species:** observation lab
**You produce:** four observed facts in `src/lib.rs` + journal entry #1 in `student/journal.md`.

Before you write a line of NDN code, you run the real thing and watch it work.
This module builds the habit the whole course rests on — *observe, then record* —
and the first professional reflex: **trust the code over the docs.**

## 0. You're already set up

`bash setup.sh` cloned the pinned repos next to this one and `./course doctor` is
green. If it isn't, fix that first — everything below needs `../ndn-fwd` present.

## 1. Read the forwarder's default config

Open `../ndn-fwd/binaries/ndn-fwd/ndn-fwd.default.toml` — the shipped reference
configuration. Read the whole thing, then find and record in `src/lib.rs`:

- the `[[face]]` blocks — what transports does it listen on, and on what ports?
  (`DEFAULT_UDP_PORT`, `WEBSOCKET_PORT`)
- the `[cs]` block — how big is the content store? (`CS_CAPACITY_MB`)
- the `[management]` block — what Unix socket does the control interface use?
  (`MGMT_SOCKET`)

`./course check m00-zero-to-running` verifies these against that exact file.

## 2. Run the forwarder

From inside `../ndn-fwd`:

```
cargo run --release -p ndn-fwd -- --config binaries/ndn-fwd/ndn-fwd.default.toml
```

Watch the first lines it prints. The very first is a NOTICE that the software is
*primarily AI-authored and not yet proven spec-compliant.* Sit with that: this is
a codebase that tells you the truth about its own maturity. You'll practice that
same honesty (the covenant, the bug journal).

Heads-up: the default config's management socket lives under `/run/`, which must
exist and be writable (fine on Linux as root; on macOS you'll likely copy the
config and point `[management] face_socket` at a path in your home directory).
Getting it to bind is a good side-quest, not a requirement to pass this module.

## 3. Poke it with the tools

Build them: `cargo build --release -p ndn-tools` (in `../ndn-fwd`). Then, against a
running forwarder — matching `--socket` / `--face-socket` to the config's socket:

- `ndn-ctl --socket <sock> status` — ask the forwarder how it's doing.
- `ndn-ctl --socket <sock> face list` — list its faces.
- `ndn-peek --face-socket <sock> /ndn/example/data` — try to fetch a name.

You will hit friction — nothing answers `/ndn/example/data` yet, sockets must
match. **That friction is the point.** Write down what actually happened.

## 4. Find the gotcha

The tools default their socket to `/run/nfd/nfd.sock`, but the config above uses a
*different* path. And if you run the forwarder with **no** `--config` at all, it
opens **zero** faces — even though a README claims it listens on 6363. Confirm one
of these mismatches yourself (running `ndn-ctl face list`, or reading
`ForwarderConfig::default()` in `../ndn-rs/crates/platform/ndn-config/src/config.rs`).

## Done means

`./course check m00-zero-to-running` green (your four facts match the real default
config) **and** journal entry #1 in `student/journal.md`: what you ran, what
surprised you, and one doc-vs-reality mismatch you caught and how you confirmed it.
Then `./course submit m00-zero-to-running` and ask the tutor to review — the tutor
grades the journal and will ask what you saw.

## Rules of engagement

The four facts are open-book: the point isn't secrecy, it's that you *open the
file*. Understanding gets checked in the tutor's review — so actually run it.
"I read about it" reads very differently from "I ran it, and here's what broke."
