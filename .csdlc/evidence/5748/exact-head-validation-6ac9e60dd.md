# Exact-head validation — `6ac9e60dd3f7071b0ae055e2dde3e929e59de543`

Observed in the bound `codex/5748-terminal-recovery` worktree on 2026-07-31.

| Lane | Command | Result |
|---|---|---|
| Current integration base | `git merge-base --is-ancestor origin/main HEAD` | PASS; current `origin/main` (`ccca46abceb117150efbc3b69248fba611d90fff`) is an ancestor of the candidate. |
| Owner-binary provenance | `.adl/bin/csdlc-v2/csdlc-install verify --repo . --bin-dir .adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json` | PASS; all 21 typed v2 binaries are installed from exact revision `6ac9e60dd3f7071b0ae055e2dde3e929e59de543`. |
| Path-guard self-test | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh --self-test-path-guards` | PASS; final-file, ancestor-component, and dangling symlinks are rejected. |
| Terminal inventory | `bash .csdlc/prepared/issues/5748/validate-final-inventory.sh` | PASS; 91 authenticated receipt-backed terminal projections, 11 exact fail-closed exceptions, and one noneligible exclusion form the retained 103-issue universe. |
| #5346 exception authentication | The terminal inventory validates `.csdlc/evidence/5748/exceptions/5346.json`, the exact committed projection/card hashes, the typed no-mutation failure, and the expected `review_publication_dead_end` doctor result | PASS. |
| Live-universe parity | Compare sorted retained issue number, closure time, state, and state-reason tuples with read-only `gh issue list --state closed --label version:v0.91.8 --limit 200` | PASS; the retained 103-issue snapshot exactly equals the live GitHub observation. |
| Full C-SDLC v2 suite | `cargo test --locked --all-targets` from `csdlc-v2/` | PASS at revision `a1defc4eb867c0b09fb7d94e301c1878d4314d84`; 78 library tests and every binary/integration target passed with 0 failures. |
| Strict lint | `cargo clippy --locked --all-targets -- -D warnings` from `csdlc-v2/` | PASS at revision `a1defc4eb867c0b09fb7d94e301c1878d4314d84`. |
| Tested-source identity | `git diff --quiet a1defc4eb867c0b09fb7d94e301c1878d4314d84..HEAD -- csdlc-v2` | PASS; the later commit changes only retained #5346 evidence and its validator, so tested C-SDLC v2 source is byte-identical. |
| Retention symlink regressions | `cargo test --locked first_terminal_receipt_retention_rejects -- --nocapture` | PASS; receipt and authored-artifact leaf and ancestor symlinks fail closed. |
| Durable recovery symlink regression | `cargo test --locked terminal_recovery_rejects_authored_parent_symlink_after_durable_boundaries -- --nocapture` | PASS; both post-journal and simulated post-projection recovery reject the injected ancestor symlink and write zero bytes outside the checkout. |
| Narrow implemented card repair | `cargo test --locked --test gate2 planning_replacements_are_phase_bounded_and_allow_narrow_implemented_corrections -- --exact --nocapture` | PASS; SIP constraint, STP criterion, and SRP prompt corrections are typed and phase-bounded while unrelated widening remains rejected. |
| #5347 terminal authority | `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5347` | PASS; the late merged issue remains claim-free, receipt-backed, and `closed_out` at generation 26. |
| #5748 readiness | `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5748` | PASS; implemented generation 22 has zero findings before final review. |
| Aggregate diff hygiene | `git diff --check origin/main...HEAD` | PASS; no output. |
| Publication-tree hygiene | `git status --porcelain` | PASS at the validating revision. |

An earlier pre-fix suite attempt observed the timing-sensitive Gate 4 test
`high_output_drains_and_failed_peer_cancels_process_group` exceed its one-second
test assertion. Its isolated rerun passed in 0.05 seconds. The final complete
suite passed all 17 Gate 4 tests in 1.07 seconds and every remaining target with
zero failures. The transient is retained here rather than omitted.

Rust compilation used repository-external build targets under
`/Volumes/FastWork`; those caches accelerated builds and are not validation
evidence. No AWS or Spot resources were used.
