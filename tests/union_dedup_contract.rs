//! Additive RED integration suite — UNION-STEP-2 symbol-merge contract.
//!
//! Convergence context: grit is meta's merge/lock substrate. The UNION-STEP-2
//! use case is reconciling two ~95%-identical crates: most symbols are
//! byte-identical (auto-mergeable), a small divergent subset must be locked and
//! resolved symbol-by-symbol. grit ALREADY has the two primitives this needs —
//! per-symbol source hashing (`Symbol.hash`, src/parser/mod.rs:15, computed at
//! src/parser/mod.rs:329 via `hash_str` src/parser/mod.rs:420) and symbol-level
//! locking (`LockStore::try_lock`, src/db/lock_store.rs:29) — but it exposes NO
//! capability that points at TWO sources, partitions their symbols into
//! identical-vs-conflicting by hash, and hands the conflicts to the locker.
//! The `Command` enum (src/cli/mod.rs:31-175) has no reconcile/diff/converge
//! variant, and the crate ships no library target (no src/lib.rs), so the only
//! public surface an integration test can drive is the compiled binary.
//!
//! These tests drive the REAL `grit` binary (CARGO_BIN_EXE_grit). They COMPILE
//! and RUN; they FAIL because the union-step-2 reconciliation capability is not
//! yet present (the `reconcile` subcommand does not exist), NOT because of a
//! compile error. This is RED for the right reason. GREEN is defined in the
//! findings file's "FF test-build spec".

use std::path::{Path, PathBuf};
use std::process::Command;

/// Absolute path to the grit binary built by cargo for this integration test.
fn grit_bin() -> &'static str {
    env!("CARGO_BIN_EXE_grit")
}

/// Run the grit binary with `args`, returning (success, stdout, stderr).
fn run_grit(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(grit_bin())
        .args(args)
        .output()
        .expect("failed to spawn the grit binary");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Materialize a crate fixture under `root` whose `src/core.rs` contains the
/// given `checksum_body`. Every fixture shares the same parse/validate/render/
/// normalize/encode/decode/merge/split/dedupe helpers (byte-identical => equal
/// `Symbol.hash`); only `checksum` differs between the two crates, modelling a
/// near-identical (~95%) pair with a single divergent symbol.
fn write_crate(root: &Path, checksum_body: &str) {
    let src = root.join("src");
    std::fs::create_dir_all(&src).expect("create src dir");
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"unionfix\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("write Cargo.toml");
    let identical = "\
fn parse(input: &str) -> usize { input.len() }
fn validate(x: usize) -> bool { x > 0 }
fn render(n: usize) -> String { format!(\"{}\", n) }
fn normalize(s: &str) -> String { s.trim().to_lowercase() }
fn encode(b: &[u8]) -> String { b.iter().map(|x| format!(\"{:02x}\", x)).collect() }
fn decode(s: &str) -> Vec<u8> { s.bytes().collect() }
fn merge(a: usize, b: usize) -> usize { a + b }
fn split(s: &str) -> Vec<&str> { s.split(',').collect() }
fn dedupe(mut v: Vec<u32>) -> Vec<u32> { v.sort_unstable(); v.dedup(); v }
";
    let content = format!("{identical}\nfn checksum(data: &[u8]) -> u64 {{ {checksum_body} }}\n");
    std::fs::write(src.join("core.rs"), content).expect("write core.rs");
}

/// Build the two near-identical crate fixtures (A and B). They agree on 9
/// symbols and disagree only on `checksum` — exactly the UNION-STEP-2 shape.
fn make_near_identical_pair(base: &Path) -> (PathBuf, PathBuf) {
    let crate_a = base.join("crate_a");
    let crate_b = base.join("crate_b");
    write_crate(&crate_a, "data.iter().map(|b| *b as u64).sum()");
    write_crate(
        &crate_b,
        "data.iter().fold(0u64, |acc, b| acc.wrapping_add(*b as u64))",
    );
    (crate_a, crate_b)
}

/// Contract 1 — the substrate must EXPOSE a symbol-level reconciliation entry
/// point for union-step-2. Today there is no `reconcile` subcommand, so the
/// binary rejects it and exits non-zero.
///
/// RED reason: union-step-2 reconciliation capability absent (no `reconcile`
/// command in src/cli/mod.rs:31-175).
#[test]
fn union_step2_reconcile_subcommand_is_supported() {
    let (ok, _stdout, stderr) = run_grit(&["reconcile", "--help"]);
    assert!(
        ok,
        "grit must expose a `reconcile` subcommand so two near-identical crates \
         can be symbol-merged (UNION-STEP-2). It does not yet exist. stderr:\n{stderr}"
    );
}

/// Contract 2 — given two near-identical crates, reconcile must PARTITION their
/// symbols: the 9 byte-identical symbols are auto-mergeable (equal `Symbol.hash`)
/// and the single divergent `checksum` is a conflict. The hashing primitive that
/// makes this decidable already exists (src/parser/mod.rs:329) but is not
/// surfaced across two sources.
///
/// RED reason: no command consumes two source roots and emits an
/// identical-vs-conflicting partition.
#[test]
fn union_step2_partitions_identical_and_conflicting_symbols() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (crate_a, crate_b) = make_near_identical_pair(tmp.path());

    let (ok, stdout, stderr) = run_grit(&[
        "reconcile",
        crate_a.to_str().unwrap(),
        crate_b.to_str().unwrap(),
    ]);

    assert!(
        ok,
        "grit reconcile must succeed on two near-identical crates. stderr:\n{stderr}"
    );
    // The identical symbols must be reported as auto-mergeable, and the one
    // divergent symbol must be reported as a conflict.
    let mentions_identical = stdout.contains("parse") && stdout.contains("dedupe");
    let mentions_conflict =
        stdout.contains("checksum") && stdout.to_lowercase().contains("conflict");
    assert!(
        mentions_identical && mentions_conflict,
        "reconcile output must mark the 9 byte-identical symbols as auto-mergeable \
         and `checksum` as a conflict requiring resolution. Got stdout:\n{stdout}"
    );
}

/// Contract 3 — the divergent symbol must be handed to grit's lock substrate:
/// reconcile should surface `core.rs::checksum` as a lockable conflict target so
/// an agent can claim it and resolve the merge under symbol-level mutual
/// exclusion (LockStore::try_lock, src/db/lock_store.rs:29).
///
/// RED reason: reconcile→lock handoff does not exist; the divergent symbol id is
/// never produced.
#[test]
fn union_step2_divergent_symbol_is_flagged_for_lock() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (crate_a, crate_b) = make_near_identical_pair(tmp.path());

    let (ok, stdout, stderr) = run_grit(&[
        "reconcile",
        "--lock-conflicts",
        crate_a.to_str().unwrap(),
        crate_b.to_str().unwrap(),
    ]);

    assert!(
        ok,
        "grit reconcile --lock-conflicts must succeed and lock divergent symbols. \
         stderr:\n{stderr}"
    );
    assert!(
        stdout.contains("core.rs::checksum"),
        "reconcile must emit the divergent symbol id `src/core.rs::checksum` as a \
         lockable conflict target for symbol-level resolution. Got stdout:\n{stdout}"
    );
}
