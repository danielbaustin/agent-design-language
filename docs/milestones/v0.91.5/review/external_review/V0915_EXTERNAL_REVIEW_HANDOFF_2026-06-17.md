# v0.91.5 External Review Handoff

Date: `2026-06-17`
Issue: `#3580`
Work package: `WP-17`
Status: `ready_for_external_review_packet`

## Purpose

Provide a compact, source-grounded handoff for the v0.91.5 external / third-party review.

This packet is not the external review result. It is the reviewer entry surface for the external review pass after WP-16 internal review and second-pass remediation.

## Current Release-Tail Truth

- WP-16 internal review `#3576` is closed.
- Second-pass internal review execution `#3923` is closed.
- First-pass remediation baseline refresh `#3943` is closed.
- WP-17 external / third-party review `#3580` is open and should run next.
- WP-18 remediation / final v0.92 preflight `#3577` remains open and must not close until WP-17 is complete or explicitly blocked/deferred.
- WP-19 next milestone planning `#3581` and WP-20 release ceremony `#3578` remain open.
- Sprint 4 umbrella `#3574` remains open.

## Recently Merged Remediation Inputs

The external reviewer should treat the following merged PRs as part of the current review baseline:

| PR | Purpose | Review relevance |
| --- | --- | --- |
| `#3947` | Normalized Sprint 4 / release-tail docs and refreshed stale first-pass baseline truth. | Reduces stale release-tail-state drift before external review. |
| `#3948` | Preserved closed-issue card-truth blocker evidence in tracked review packets. | Makes prior card-truth evidence reviewable without relying only on ignored local `.adl` paths. |
| `#3949` | Fixed PR validation latest-check handling for superseded/cancelled runs. | Prevents merged green PRs from being misreported as cancelled due to stale check runs. |
| `#3950` | Fixed `pr finish` so finish-written SOR output truth is re-staged and committed. | Addresses a root cause of stale closeout-card truth in PR commits. |
| `#3951` | Redacted raw OpenRouter matrix prompt/output artifacts and hardened validators. | Repairs a review-facing evidence hygiene and redaction issue. |

All five PRs reached green checks before merge. `#3950` initially hit an intermittent GitHub runner/linker bus error in the slow-proof lane; rerun passed without source changes.

## Primary Source Packet

External review should start with these tracked sources:

- Milestone README: [README.md](../../README.md)
- Sprint plan: [SPRINT_v0.91.5.md](../../SPRINT_v0.91.5.md)
- WBS: [WBS_v0.91.5.md](../../WBS_v0.91.5.md)
- Quality gate: [QUALITY_GATE_v0.91.5.md](../../QUALITY_GATE_v0.91.5.md)
- Demo matrix: [DEMO_MATRIX_v0.91.5.md](../../DEMO_MATRIX_v0.91.5.md)
- v0.92 activation map: [V092_ACTIVATION_TEST_MAP_v0.91.5.md](../../V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- Next milestone handoff: [NEXT_MILESTONE_HANDOFF_v0.91.5.md](../../NEXT_MILESTONE_HANDOFF_v0.91.5.md)
- First internal review register: [V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md](../internal_review/V0915_FIRST_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-16.md)
- First internal review remediation queue: [V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md](../internal_review/V0915_FIRST_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-16.md)
- Second internal review register: [V0915_SECOND_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-17.md](../internal_review/V0915_SECOND_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-17.md)
- Second internal review remediation queue: [V0915_SECOND_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-17.md](../internal_review/V0915_SECOND_INTERNAL_REVIEW_REMEDIATION_QUEUE_2026-06-17.md)
- Live issue-state snapshot: [V0915_SECOND_INTERNAL_REVIEW_LIVE_ISSUE_STATE_2026-06-17.md](../internal_review/V0915_SECOND_INTERNAL_REVIEW_LIVE_ISSUE_STATE_2026-06-17.md)
- Closed-issue card-truth evidence: [V0915_CLOSED_ISSUE_CARD_TRUTH_EVIDENCE_2026-06-17.md](../internal_review/V0915_CLOSED_ISSUE_CARD_TRUTH_EVIDENCE_2026-06-17.md)

## Proof And Evidence Surfaces To Inspect

The review should include code and evidence, not just milestone prose.

### Workflow / tooling

- `adl/src/cli/pr_cmd/`
- `adl/src/cli/tooling_cmd/`
- `adl/tools/`
- [TOOLS_REMEDIATION_SPRINT_CLOSEOUT_3845.md](../tooling_adoption/TOOLS_REMEDIATION_SPRINT_CLOSEOUT_3845.md)
- [TOOLKIT_SIMPLIFICATION_SPRINT_CLOSEOUT_3740.md](../tooling_adoption/TOOLKIT_SIMPLIFICATION_SPRINT_CLOSEOUT_3740.md)
- [PR_LIFECYCLE_SMALL_BINARIES_PROOF_3838.md](../tooling_adoption/PR_LIFECYCLE_SMALL_BINARIES_PROOF_3838.md)
- [OCTOCRAB_TEMPLATES_AST_PRUNING_PROOF_3775.md](../tooling_adoption/OCTOCRAB_TEMPLATES_AST_PRUNING_PROOF_3775.md)

### Logging / observability

- [LOGGING_MINI_SPRINT_CLOSEOUT_3703.md](../logging_observability/LOGGING_MINI_SPRINT_CLOSEOUT_3703.md)
- [CONTROL_PLANE_LOGGING_PROOF_3706.md](../logging_observability/CONTROL_PLANE_LOGGING_PROOF_3706.md)
- [RUNTIME_PROVIDER_LOGGING_PROOF_3707.md](../logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3707.md)
- [HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3708.md](../logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3708.md)
- [OTEL_INTEGRATION_BOUNDARY_PROOF_3709.md](../logging_observability/OTEL_INTEGRATION_BOUNDARY_PROOF_3709.md)
- [OBSERVATORY_LOG_CONSUMPTION_PROOF_3710.md](../logging_observability/OBSERVATORY_LOG_CONSUMPTION_PROOF_3710.md)

### Provider / review-provider / OpenRouter

- `adl/src/provider/`
- `adl/src/provider_communication.rs`
- `adl/src/provider_substrate.rs`
- [DEEPSEEK_NATIVE_PROVIDER_PROOF_3549.md](../native_deepseek_provider/DEEPSEEK_NATIVE_PROVIDER_PROOF_3549.md)
- [REVIEW_PROVIDER_V1_CONTRACT_3562.md](../review_provider/REVIEW_PROVIDER_V1_CONTRACT_3562.md)
- [OPENROUTER_MATRIX_PROOF_2026-06-14.md](../openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md)
- [README.md](../openrouter_matrix/README.md)
- [openrouter_matrix_state_2026-06-14.json](../openrouter_matrix/openrouter_matrix_state_2026-06-14.json)
- [provider_invocations.json](../openrouter_matrix/provider_invocations.json)

### Multi-agent / reasoning / delegation

- [V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md](../multi_agent_workcell/V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md)
- [MULTI_AGENT_OVERHEAD_COMPARISON_2026-06-14.md](../multi_agent_overhead/MULTI_AGENT_OVERHEAD_COMPARISON_2026-06-14.md)
- [MULTI_AGENT_QUALITY_COMPARISON_2026-06-15.md](../multi_agent_quality_comparison/MULTI_AGENT_QUALITY_COMPARISON_2026-06-15.md)
- [REASONING_GRAPH_CURRENT_CONTRACT_3688.md](../reasoning_graph/REASONING_GRAPH_CURRENT_CONTRACT_3688.md)
- [UPSTREAM_DELEGATION_CONTRACT_3689.md](../upstream_delegation/UPSTREAM_DELEGATION_CONTRACT_3689.md)
- [REASONING_GRAPH_UPSTREAM_DELEGATION_PROOF_3691.md](../reasoning_graph_upstream_delegation/REASONING_GRAPH_UPSTREAM_DELEGATION_PROOF_3691.md)

### Demo / v0.92 readiness

- [DEMO_MATRIX_v0.91.5.md](../../DEMO_MATRIX_v0.91.5.md)
- [DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md](../../features/DEMO_AND_UNITY_OBSERVATORY_READINESS_v0.91.5.md)
- [V092_ACTIVATION_TEST_MAP_v0.91.5.md](../../V092_ACTIVATION_TEST_MAP_v0.91.5.md)
- [AEE_COMPLETION_TRANCHE_v0.91.5.md](../../features/AEE_COMPLETION_TRANCHE_v0.91.5.md)

## Suggested External Review Lanes

1. `Release truth / issue graph`
   - Verify WP-16 and `#3923` are closed, WP-17 is active, WP-18 remains open, and release ceremony has not started early.
2. `Code and tooling correctness`
   - Review GitHub/Octocrab transport, PR validation, finish/closeout truth, prompt-template tooling, AST editing, and small-binary command surfaces.
3. `Evidence and redaction hygiene`
   - Check proof packets for raw prompt/provider-output leakage, private host paths, secret markers, and stale local `.adl` references.
4. `Provider / review-provider contracts`
   - Check that model/provider/review-provider claims match code and proof evidence, especially DeepSeek, OpenRouter, hosted provider, and Ollama boundaries.
5. `Multi-agent / C-SDLC proof`
   - Check whether workcell, overhead, quality comparison, and delegation/reasoning graph evidence supports only the claims made.
6. `Demo and v0.92 readiness`
   - Check that demo readiness and first-birthday readiness are inputs to WP-18, not prematurely claimed complete.
7. `Validation / PVF adequacy`
   - Check that focused validation is proportionate, skipped lanes are explicit, and CI/slow-proof evidence is not overstated.

## Known Risk Areas For The Reviewer

These are not asserted findings; they are review prompts worth checking.

- Some milestone status docs may still contain stale release-tail wording because WP-16 and `#3923` closed after earlier docs were written.
- First-pass review packets are historical inputs; reviewers should not treat all historical queue language as current active state without checking newer WP-18 handoff comments and merged PRs.
- WP-18 must not close until WP-17 external review is complete or explicitly blocked/deferred.
- `#3377` first-birthday readiness remains a critical open consumer for v0.92 readiness.
- The OpenRouter proof packet was redacted late in the release tail; external review should spot-check generated machine-readable artifacts and lane logs.
- `pr finish` SOR truth was fixed late in the release tail; external review should verify no important final-review card truth remains stranded only in local ignored state.
- CI slow-proof had an intermittent linker bus error on `#3950` that passed on rerun without source changes; this should be treated as infrastructure flake unless repeated.
- Rust refactor evidence may still contain historical/manual GitHub provenance; that is a polish concern unless it contradicts active workflow claims.

## Explicit Non-Claims

This handoff does not claim:

- v0.91.5 is release-ready;
- WP-18 is complete;
- WP-19 or WP-20 may start without WP-17 / WP-18 disposition truth;
- v0.92 first-birthday readiness is complete;
- OpenRouter or hosted-provider evidence proves broad model correctness;
- multi-agent execution is universally faster or better;
- external review findings are already resolved.

## Review Output Expected

External review should produce one of:

- a findings register suitable for WP-18 remediation;
- a no-findings / residual-risk packet suitable for WP-18 final preflight;
- a blocked/deferred external-review record with explicit rationale.

Findings should be severity-ranked and evidence-backed with file/line references where possible.

## Minimal Preflight Before Handing To Reviewer

Before sending this packet out, confirm:

- `#3580` remains open;
- `#3577` remains open;
- `#3576`, `#3923`, and `#3943` remain closed;
- PRs `#3947`, `#3948`, `#3949`, `#3950`, and `#3951` remain merged;
- this handoff file is tracked through the WP-17 PR.
