# ct_demo_001 Merge-Gate Profile Report

## Gate Identity

- transition id: `csdlc.v0_91_4.wp_07.ct_demo_001`
- gate kind: `governed_merge_readiness_gate.v2`
- decision mode: `policy_and_fixture_record`
- outcome: `gate_hardened`

## Validation Profile Truth

- docs-only finish mode: `DocsOnly`
- focused local-CI-gated finish mode: `FocusedLocalCiGated`
- focused Rust test filter: `cli::pr_cmd`
- feature doc:
  `docs/milestones/v0.91.4/features/MERGE_READINESS_AND_PR_GATE_HARDENING.md`
- tooling policy:
  `docs/tooling/merge_readiness_gate_policy_v0.91.4.md`
- broadened focused scope now covers the real `adl/src/cli/pr_cmd/` subtree,
  not only `finish_support.rs`

## Lifecycle Blockers

- stale lifecycle blocker fixture:
  `card_lifecycle_blocks_completed_sor_before_terminal_closeout`
- docs-only review exception fixture:
  `card_lifecycle_allows_explicit_srp_policy_exception`
- expected blocked truth:
  stale `SOR` / closeout truth must keep finish-readiness blocked
- expected allowed truth:
  explicit docs-only review exceptions may satisfy `SRP` review truth without
  claiming remote merge success

## Review / PR Truth Boundary

- local focused validation is allowed to prove gate logic only
- merge truth is not inferred from local validation
- remote CI, PR state, and merge state must be preserved as separate truth
- this packet records that boundary; it does not add new live GitHub-state
  reconciliation logic by itself
- human review and merge authority remain required

## Evidence / Dependency Link

- source issue: [#3355](https://github.com/danielbaustin/agent-design-language/issues/3355)
- upstream transition issue: [#3354](https://github.com/danielbaustin/agent-design-language/issues/3354)
- upstream proof PR: [#3388](https://github.com/danielbaustin/agent-design-language/pull/3388)
- upstream evidence bundle:
  `docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json`
- upstream review synthesis:
  `docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_review_synthesis.json`
- upstream signed trace:
  `docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_signed.adl.yaml`
- SOR-truth blocker coverage is carried by the tracked doctor fixture
  `card_lifecycle_blocks_completed_sor_before_terminal_closeout`

## Structured Snapshot

- snapshot:
  `docs/milestones/v0.91.4/review/merge_readiness/ct_demo_001_merge_gate_snapshot.json`
- validation mode:
  the packet validator reconciles this report against the structured snapshot
  and the referenced tracked artifacts
- live-state boundary:
  this packet records merge-readiness gate policy and fixture truth; it does not
  claim live GitHub reconciliation during validation

## Decision

- decision: `gate_hardened`
- decision basis:
  - focused validation now covers the actual PR-gate code surface and bounded
    `WP-07` policy/proof docs
  - stale lifecycle truth still fails closed
  - docs-only review exceptions stay explicit rather than implied
  - merge truth remains separate from local validation success

## Residual Risks

- this packet does not replace later sprint/release quality gates
- `WP-08` still owns the memory-handoff boundary
- release approval still depends on later review and closeout work
