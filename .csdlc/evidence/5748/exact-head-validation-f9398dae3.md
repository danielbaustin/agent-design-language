# Exact-head validation — `f9398dae385906b74b45d9ce08e12f72b9876419`

Observed in the bound `codex/5748-terminal-recovery` worktree on 2026-07-31.

| Lane | Command | Result |
|---|---|---|
| Owner-binary provenance | `.adl/bin/csdlc-v2/csdlc-install verify --repo . --bin-dir .adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json` | PASS; all 21 typed v2 binaries are present and the install receipt names exact head `f9398dae385906b74b45d9ce08e12f72b9876419`. |
| Path-guard self-test | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards` | PASS; final-file, ancestor-component, and dangling symlinks are rejected. |
| Terminal inventory | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh` | PASS; 91 authenticated receipt-backed terminal projections, 11 exact fail-closed exceptions, and one noneligible exclusion form the retained 103-issue universe. |
| Live-universe parity | Compare the sorted retained issue number, closure time, state, and state-reason tuples with `gh issue list --state closed --label version:v0.91.8 --limit 200` | PASS; the retained 103-issue snapshot exactly equals the live GitHub observation. |
| Gate 7 lifecycle | `cargo test --locked --test gate7_lifecycle -- --nocapture` from `csdlc-v2/` | PASS at source revision `78f6299fd99e1af914307c4d5596f9c380a6f114`; 39 passed, 0 failed. |
| Full C-SDLC v2 suite | `cargo test --locked --all-targets` from `csdlc-v2/` | PASS at source revision `78f6299fd99e1af914307c4d5596f9c380a6f114`; every library, binary, and integration target passed with 0 failures. |
| Strict lint | `cargo clippy --locked --all-targets -- -D warnings` from `csdlc-v2/` | PASS at source revision `78f6299fd99e1af914307c4d5596f9c380a6f114`. |
| Source identity | `git diff --quiet 78f6299fd99e1af914307c4d5596f9c380a6f114..HEAD -- csdlc-v2` | PASS; subsequent commits contain only lifecycle projection and audit evidence, so the tested and linted C-SDLC v2 source is byte-identical to exact head. |
| #5347 terminal authority | `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5347` | PASS; the late merged issue is now claim-free, receipt-backed, and `closed_out` at generation 26 on the dedicated closeout branch. |
| #5748 readiness | `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5748` | PASS; implemented generation 17 has zero findings before final review. |
| Aggregate diff hygiene | `git diff --check origin/main...HEAD` | PASS; no output. |
| Publication-tree hygiene | `git status --porcelain` | PASS; no tracked or untracked worktree state remained after the validating commit. |

One complete-suite attempt observed the timing-sensitive Gate 4 test
`high_output_drains_and_failed_peer_cancels_process_group` exceed its one-second
test assertion. Its isolated rerun passed in 0.05 seconds, and the subsequent
clean complete-suite run passed all 17 Gate 4 tests in 1.08 seconds and every
remaining target with zero failures. This transient is recorded rather than
silently omitted; it did not recur in the proving run.

Rust compilation used repository-external build targets under
`/Volumes/FastWork`; those caches accelerated builds and are not validation
evidence. No AWS or Spot resources were used.
