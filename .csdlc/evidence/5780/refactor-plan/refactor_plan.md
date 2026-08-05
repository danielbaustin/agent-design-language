# Refactor Plan: Issue 5780 terminal reconciliation and receipt-authority deletion

## Refactor Plan Summary

- Status: ready
- Run id: 5780-terminal-authority-deletion
- Planned slices: 8

## Scope

- Scope: Issue 5780 terminal reconciliation and receipt-authority deletion
- Mode: slice_refactor
- Target paths:
- csdlc-v2/src/bin/csdlc-closeout.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/readiness.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/lib.rs and csdlc-v2/src/schema.rs
- csdlc-v2/operator and csdlc-v2/Cargo.toml
- csdlc-v2/tests/gate7_lifecycle.rs
- active C-SDLC v2 operator documentation

## Current Behavior

# Current Behavior

The normal v2 lifecycle can finish an exact reviewed green PR through `csdlc-finish`, which derives immutable terminal truth in the Git common directory without rewriting tracked lifecycle state. Cleanup is independently provided by `csdlc-clean`.

An older parallel authority remains: `csdlc-closeout` exposes post-merge readiness, `merge_ready`, `merged`, and `closed_out` mutations; full terminal receipts duplicating the record, six cards, design, diagram, and authored artifacts; repair and transport commands; historical reconciliation; and closeout-coupled prune operations. The public schema and operator installation inventory still advertise those mutation surfaces.

Legacy tracked records and retained receipts are consumed read-only by the compatibility index added in issue 5779. Lifecycle phase variants and terminal receipt deserialization must therefore remain readable even after their writers are removed.

## Refactor Intent

# Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.

## Invariants

- `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage.
- `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them.
- Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes.
- No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt.
- No second PR is required solely to record that an implementation PR merged.
- Historical tracked records and evidence are not rewritten or deleted.
- Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed.
- All tracked implementation changes remain in the issue 5780 worktree.

## Risk Inventory

- R-001: Removing receipt writers may accidentally remove the reader needed for legacy compatibility indexing. Mitigation: Address or validate before the affected slice lands.
- R-002: Store terminal-repair code is interleaved with current lifecycle persistence helpers, so broad deletion could break non-terminal operations. Mitigation: Address or validate before the affected slice lands.
- R-003: Public schema and library exports can silently preserve unsupported mutation authority even after the CLI binary is deleted. Mitigation: Address or validate before the affected slice lands.
- R-004: Existing tests may encode obsolete closeout behavior and hide a missing negative guard against its reintroduction. Mitigation: Address or validate before the affected slice lands.
- R-005: Active documentation or installer manifests may continue advertising the deleted binary. Mitigation: Address or validate before the affected slice lands.
- R-006: Historical architecture and evidence files must remain immutable even when they describe the retired design. Mitigation: Address or validate before the affected slice lands.

## Refactor Slices

### S-001: Refactor csdlc-v2/src/bin/csdlc-closeout.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/src/bin/csdlc-closeout.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/src/bin/csdlc-closeout.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-002: Refactor csdlc-v2/src/store.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/src/store.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/src/store.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-003: Refactor csdlc-v2/src/readiness.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/src/readiness.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/src/readiness.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-004: Refactor csdlc-v2/src/model.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/src/model.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/src/model.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-005: Refactor csdlc-v2/src/lib.rs and csdlc-v2/src/schema.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/src/lib.rs and csdlc-v2/src/schema.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/src/lib.rs and csdlc-v2/src/schema.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-006: Refactor csdlc-v2/operator and csdlc-v2/Cargo.toml

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/operator and csdlc-v2/Cargo.toml
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/operator and csdlc-v2/Cargo.toml fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-007: Refactor csdlc-v2/tests/gate7_lifecycle.rs

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: csdlc-v2/tests/gate7_lifecycle.rs
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for csdlc-v2/tests/gate7_lifecycle.rs fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: Continue with the next bounded slice after validation passes.

### S-008: Refactor active C-SDLC v2 operator documentation

- Intent: # Refactor Intent

This is an explicit supported-behavior change: remove every supported command and library export that creates, repairs, transports, reconciles, or treats tracked post-merge terminal projections and receipts as authority. Delete the `csdlc-closeout` binary and skill instead of retaining a compatibility wrapper. Preserve read-only deserialization and legacy terminal indexing, while making `csdlc-finish`, `csdlc-pr-state`, and `csdlc-clean` the only supported finish, status, and cleanup surfaces.

Sequence the work as characterization, operator-surface deletion, writer/API deletion, test contraction with new negative guards, then documentation and reduction evidence.
- Behavior change: true
- Target files: active C-SDLC v2 operator documentation
- Invariants: `csdlc-finish` remains the sole terminal mutation authority and still validates the exact published head, required checks, review evidence, live GitHub identity, and claim lineage., `csdlc-clean` remains independent of delivery truth and continues to read historical terminal projections and receipts without mutating them., Existing `merge_ready`, `merged`, and `closed_out` records and terminal receipts continue to deserialize and resolve to the same compatibility outcomes., No supported binary, skill, schema writer, or library export can create or reconcile a tracked post-merge terminal projection or full terminal receipt., No second PR is required solely to record that an implementation PR merged., Historical tracked records and evidence are not rewritten or deleted., Current issue initialization, binding, validation, review, publication, finish, status, and cleanup behavior remains deterministic and fail-closed., All tracked implementation changes remain in the issue 5780 worktree.
- Validation commands: `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`, `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`, `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`, `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`, `bash adl/tools/install_owner_binaries.sh --check`, `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling
- Rollback notes: Revert this slice independently if validation for active C-SDLC v2 operator documentation fails.
- Residual risk: Review call sites and tests for behavior assumptions not visible in the supplied bundle.
- Follow-up: None.


## Validation Plan

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_finish`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate_cleanup`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate10a`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`
- `bash adl/tools/install_owner_binaries.sh --check`
- `rg` negative guards for the deleted binary, skill, mutation requests, receipt writers, reconciliation commands, and prune coupling

## Rollback Notes

- Revert this slice independently if validation for csdlc-v2/src/bin/csdlc-closeout.rs fails.
- Revert this slice independently if validation for csdlc-v2/src/store.rs fails.
- Revert this slice independently if validation for csdlc-v2/src/readiness.rs fails.
- Revert this slice independently if validation for csdlc-v2/src/model.rs fails.
- Revert this slice independently if validation for csdlc-v2/src/lib.rs and csdlc-v2/src/schema.rs fails.
- Revert this slice independently if validation for csdlc-v2/operator and csdlc-v2/Cargo.toml fails.
- Revert this slice independently if validation for csdlc-v2/tests/gate7_lifecycle.rs fails.
- Revert this slice independently if validation for active C-SDLC v2 operator documentation fails.

## Residual Risk

- Behavior change is explicitly in scope; review this separately from structural cleanup.
- Human review and CI remain required before any slice is merged.

## Stop Boundary

- Performed refactor: false.
- Changed behavior: false.
- Created issues: false.
- Created PRs: false.
- Committed changes: false.
- Mutated repository: false.
