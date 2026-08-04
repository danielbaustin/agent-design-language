# Exact-head validation: `5eb2fd0a801431285c7f84002722a6ffe4a17c70`

Validated after merging current `origin/main` into `codex/5748-terminal-recovery`
and correcting the synthetic terminal-repair fixture authority exposed by the
merged test surface.

## Passed proof

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --quiet`
  - all unit, integration, public-contract, lifecycle, transport, recovery,
    soak, and documentation test binaries passed with zero failures.
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- --deny warnings`
  - passed with zero warnings.
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`
  - passed.
- `git diff --check`
  - passed.
- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards`
  - `v0.91.8 inventory path-guard self-test PASS`.
- `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh`
  - `v0.91.8 terminal inventory PASS: 114 terminal (1 closed NOT_PLANNED), zero fail-closed exceptions`.
- `.adl/bin/csdlc-v2/csdlc-install verify --repo . --bin-dir .adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json`
  - passed after installation from exact source revision
    `git:5eb2fd0a801431285c7f84002722a6ffe4a17c70`.

`CARGO_TARGET_DIR=/Volumes/FastWork/adl-5748/csdlc-v2-install-target` was used
only as a same-host build cache/output location and is not validation evidence.
