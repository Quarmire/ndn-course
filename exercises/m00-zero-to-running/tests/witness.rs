//! Witness for m00-zero-to-running. Unlike a code exercise, this checks the
//! FACTS you observed against the real pinned config — so it passes only if you
//! actually read ndn-fwd's shipped default configuration.

use std::path::{Path, PathBuf};

use m00_zero_to_running as obs;

/// The pinned ndn-fwd's shipped default config, found the same way `setup.sh`
/// places the sibling repos: `../ndn-fwd` next to this course repo.
fn default_config() -> String {
    let course_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("exercise crate is two levels below the course root");
    let cfg: PathBuf = course_root.join("../ndn-fwd/binaries/ndn-fwd/ndn-fwd.default.toml");
    let raw = std::fs::read_to_string(&cfg).unwrap_or_else(|_| {
        panic!(
            "could not read the pinned ndn-fwd default config at\n  {}\n→ run \
             `bash setup.sh` from the course root to fetch the pinned repos first.",
            cfg.display()
        )
    });
    // Collapse whitespace so our substring checks don't depend on the config's
    // column alignment (it writes `capacity_mb      = 64`, not `capacity_mb = 64`).
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn udp_port_matches_the_default_config() {
    let cfg = default_config();
    assert_ne!(
        obs::DEFAULT_UDP_PORT,
        0,
        "record the UDP port you found in the config's [[face]] blocks"
    );
    assert!(
        cfg.contains(&format!("0.0.0.0:{}", obs::DEFAULT_UDP_PORT)),
        "no face binds 0.0.0.0:{} in the default config — re-read the [[face]] blocks",
        obs::DEFAULT_UDP_PORT
    );
}

#[test]
fn websocket_port_matches_the_default_config() {
    let cfg = default_config();
    assert_ne!(obs::WEBSOCKET_PORT, 0, "record the WebSocket face's port");
    assert!(
        cfg.contains(&format!("0.0.0.0:{}", obs::WEBSOCKET_PORT)),
        "no face binds 0.0.0.0:{} — find the web-socket face",
        obs::WEBSOCKET_PORT
    );
}

#[test]
fn cs_capacity_matches_the_default_config() {
    let cfg = default_config();
    assert_ne!(
        obs::CS_CAPACITY_MB,
        0,
        "record the content store capacity, in MB, from the [cs] block"
    );
    assert!(
        cfg.contains(&format!("capacity_mb = {}", obs::CS_CAPACITY_MB)),
        "the config's content store isn't {} MB — check [cs] capacity_mb",
        obs::CS_CAPACITY_MB
    );
}

#[test]
fn mgmt_socket_matches_the_default_config() {
    let cfg = default_config();
    assert!(
        !obs::MGMT_SOCKET.is_empty(),
        "record the [management] face_socket path"
    );
    assert!(
        cfg.contains(obs::MGMT_SOCKET),
        "the config's [management] face_socket is not {:?} — read it again \
         (and note in your journal that it differs from the tools' default!)",
        obs::MGMT_SOCKET
    );
}
