//! m00-zero-to-running — the first module. Get the stack running, poke it, and
//! start the habit that carries the whole course: observe, then record.
//!
//! This is an OBSERVATION lab, not a coding exercise. Read SPEC.md, follow it on
//! a real running forwarder, then fill in the four facts below with what you find
//! in ndn-fwd's shipped default config. The witness in `tests/witness.rs` checks
//! your answers against that real file — it passes only if you actually looked.
//!
//! The sentinels (`0` / `""`) are wrong on purpose: the tests are red until you
//! replace them with what you observed.

/// The UDP port a face binds to in ndn-fwd's default config.
pub const DEFAULT_UDP_PORT: u16 = 0;

/// The port the WebSocket face binds to in the default config.
pub const WEBSOCKET_PORT: u16 = 0;

/// The content store's capacity, in megabytes, per the default config.
pub const CS_CAPACITY_MB: u32 = 0;

/// The management (control) Unix-socket path the default config uses.
/// (Worth noting for your journal: it is NOT the path the CLI tools default to.)
pub const MGMT_SOCKET: &str = "";
