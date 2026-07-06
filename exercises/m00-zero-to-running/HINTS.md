# Hints — m00-zero-to-running

Revealed one rung at a time by `./course hint m00-zero-to-running`.

## Hint 1 — where the config is

The shipped default config is `../ndn-fwd/binaries/ndn-fwd/ndn-fwd.default.toml`
(relative to this course repo). Open it in your editor — every fact you need is in
there as plain TOML: `[[face]]` blocks, a `[cs]` block, a `[management]` block.

## Hint 2 — reading the faces

Each `[[face]]` has a `kind` and a `bind = "0.0.0.0:PORT"`. Two of them share one
port for different transports (udp and tcp); a third, the `web-socket` face, uses
a different port. The number after the colon is what `DEFAULT_UDP_PORT` and
`WEBSOCKET_PORT` want. `[cs] capacity_mb` and `[management] face_socket` are the
other two, read verbatim.

## Hint 3 — running it

`cargo run --release -p ndn-fwd -- --config binaries/ndn-fwd/ndn-fwd.default.toml`
from inside `../ndn-fwd`. If the socket won't bind, copy the config, point
`[management] face_socket` at a writable path (a file in your home dir works), and
pass that same path as `--socket` to the tools.

## Hint 4 — the mismatch to catch

Compare two things: what `../ndn-fwd/binaries/ndn-fwd/README.md` claims the
no-config default does, versus what actually happens. Run the forwarder with no
`--config` and check `ndn-ctl face list`, or read `ForwarderConfig::default()` in
`../ndn-rs/crates/platform/ndn-config/src/config.rs`. One says 6363; the other
opens nothing. That gap is your journal note — and your first lesson in trusting
the code over the prose.
