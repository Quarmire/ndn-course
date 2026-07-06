//! Witness for m01-reading-the-codebase. It checks your file locations against
//! the real ../ndn-rs tree and your measured sizes against the real types, so it
//! passes only if you actually navigated the codebase and measured.

use std::path::Path;

use m01_reading_the_codebase as found;
use ndn_foundation_types::{Hash, NameComponent};

/// Read a file by its path relative to the pinned `../ndn-rs` sibling repo.
fn ndn_rs_file(rel: &str) -> String {
    let course_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("exercise crate is two levels below the course root");
    let p = course_root.join("../ndn-rs").join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|_| {
        panic!(
            "could not read ../ndn-rs/{rel}\n→ record a real path relative to \
             ../ndn-rs (and run `bash setup.sh` if ../ndn-rs is missing)."
        )
    })
}

#[test]
fn located_where_a_pit_entry_is_created() {
    assert!(
        !found::PIT_CREATE_FILE.is_empty(),
        "record the file where a PIT entry is created"
    );
    let src = ndn_rs_file(found::PIT_CREATE_FILE);
    assert!(
        src.contains("PitEntry::new") || src.contains("with_entry_or_insert"),
        "{} is not where a PIT entry is created — search ../ndn-rs for where \
         `PitEntry::new` is called from a pipeline stage",
        found::PIT_CREATE_FILE
    );
}

#[test]
fn located_where_data_is_cached_and_gated() {
    assert!(
        !found::CS_INSERT_FILE.is_empty(),
        "record the file where Data is inserted into the content store"
    );
    let src = ndn_rs_file(found::CS_INSERT_FILE);
    assert!(
        src.contains("insert_erased") || src.contains("CsInsertStage"),
        "{} does not insert Data into the content store — grep for `insert_erased`",
        found::CS_INSERT_FILE
    );
    assert!(
        src.contains("verified"),
        "the caching file is also where `ctx.verified` gates admission, but {} \
         has no `verified` check — is this really the CS-insert stage?",
        found::CS_INSERT_FILE
    );
}

#[test]
fn located_the_pipeline_driver() {
    assert!(
        !found::PIPELINE_FILE.is_empty(),
        "record the file that drives the ordered pipeline stages"
    );
    let src = ndn_rs_file(found::PIPELINE_FILE);
    assert!(
        src.contains("interest_pipeline") || src.contains("data_pipeline"),
        "{} is not the pipeline driver — find the file with `interest_pipeline` / \
         `data_pipeline` (not the ASCII diagram in ARCHITECTURE.md)",
        found::PIPELINE_FILE
    );
}

#[test]
fn measured_hash_size() {
    assert_eq!(
        found::SIZE_OF_HASH,
        std::mem::size_of::<Hash>(),
        "measure size_of::<Hash>() and record it — don't guess (HINTS shows how)"
    );
}

#[test]
fn measured_name_component_size() {
    assert_eq!(
        found::SIZE_OF_NAME_COMPONENT,
        std::mem::size_of::<NameComponent>(),
        "measure size_of::<NameComponent>() — note it exceeds the bytes it holds a handle to"
    );
}
