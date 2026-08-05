# Exact-head validation — `25fdcdafa131c76a8f80627961c01d2c5e5980f3`

Observed in the bound `codex/5748-terminal-recovery` worktree on 2026-07-31.

| Lane | Command | Result |
|---|---|---|
| Owner-binary provenance | `.adl/bin/csdlc-v2/csdlc-install verify --repo . --bin-dir .adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json` | PASS; v2 is the default generation, all 21 owner binaries are present, no forbidden v1 paths are present, and the install receipt names this exact revision. |
| Path guard | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards` | PASS; final-file, parent-component, and dangling symlinks are rejected. |
| Terminal inventory | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh` | PASS; 90 receipt-backed terminal projections, 10 exact fail-closed exceptions, and one noneligible exclusion form the retained 101-issue universe. |
| Focused terminal authority | `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle no_pr_closeout_produces_doctor_valid_terminal_state -- --exact` | PASS; 1 passed, 0 failed. |
| Full C-SDLC v2 suite | `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --quiet` | PASS; every library, binary, integration, and doc-test target completed with 0 failures. |
| Strict lint | `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings` | PASS. |
| Aggregate diff hygiene | `git diff --check origin/main..HEAD` | PASS; no output. |

Rust validation used the repository-external build cache at
`/Volumes/FastWork/adl-5748-csdlc-target`; that cache is acceleration only and
is not part of the proof surface.
