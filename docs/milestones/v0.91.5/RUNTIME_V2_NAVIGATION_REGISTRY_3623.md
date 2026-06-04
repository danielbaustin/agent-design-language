# Runtime v2 Navigation Registry Before v0.92 Activation (#3623)

Issue: #3623
Status: implemented as registry and routing

## Purpose

This registry makes `adl/src/runtime_v2/` navigable before v0.92 activation
work relies on runtime-v2 feature ownership. It is a source-grounded ownership
index, not a code movement PR.

The runtime-v2 cluster is currently large enough to need an explicit map:

- `adl/src/runtime_v2`: 163 Rust files
- approximate source size from module navigability review: 69,098 lines
- public surface pattern: `mod.rs` re-exports nearly all runtime-v2 modules
- crate-private surface: `validators`

No runtime behavior, artifact schema, public command behavior, or module export
behavior changes in #3623.

## Source Inputs

- `adl/src/runtime_v2/`
- `adl/src/runtime_v2/mod.rs`
- `adl/src/runtime_v2/tests/`
- `docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `docs/milestones/v0.91.5/MODULE_NAVIGABILITY_REVIEW_3612.md`
- `#3377`
- `#3612`

## Public Surface Snapshot

`adl/src/runtime_v2/mod.rs` declares and re-exports the runtime-v2 feature
modules. Most modules are exposed with `pub use`; `validators` is intentionally
`pub(crate)`.

This means future refactors must treat runtime-v2 module movement as public
surface work unless a characterization issue proves the moved item is private,
test-only, or safely hidden behind an unchanged re-export.

## Feature Ownership Registry

| Feature family | Primary modules | Test families | v0.92 activation surface | Proof posture | Movement posture |
| --- | --- | --- | --- | --- | --- |
| Core manifold and kernel state | `foundation`, `manifold`, `kernel_loop`, `snapshot`, `types`, `validators` | `foundation`-adjacent module tests through `tests/kernel_loop.rs`, `tests/manifold.rs`, `tests/snapshot_rehydration.rs` | Identity/continuity and birthday substrate | Contract/schema and snapshot proof | Do not move before characterization tests prove re-export and snapshot behavior. |
| Citizen, lifecycle, standing, and continuity | `citizen`, `agent_lifecycle_state`, `contract_lifecycle_state`, `standing`, `wake_continuity` test family, `transition_authority` | `tests/citizen_lifecycle.rs`, `tests/agent_lifecycle_state.rs`, `tests/contract_lifecycle_state.rs`, `tests/standing.rs`, `tests/transition_authority.rs`, `tests/wake_continuity.rs` | Identity and continuity; AEE wake/handoff semantics | Lifecycle matrix and negative-case proof | Standing already has a submodule cluster; keep layout stable until wake/handoff characterization exists. |
| Access, hardening, security, and admission | `access_control`, `acip_hardening`, `boot_admission`, `hardening`, `security`, `quarantine`, `recovery` | `tests/access_control.rs`, `tests/acip_hardening.rs`, `tests/boot_admission.rs`, `tests/hardening.rs`, `tests/security_boundary.rs`, `tests/quarantine.rs`, `tests/recovery_eligibility.rs` | Provider/model matrix safety; birthday admission and recovery policy | Fail-closed validation and policy-boundary proof | Do not split across binaries until policy/error code characterization exists. |
| A2A, ACIP, delegation, evaluation, and provider-facing coordination | `a2a_adapter_boundary`, `delegation_subcontract`, `evaluation_selection`, `external_counterparty`, `bid_schema`, `contract_schema`, `contracts` | `tests/a2a_adapter_boundary.rs`, `tests/delegation_subcontract.rs`, `tests/evaluation_selection.rs`, `tests/external_counterparty.rs`, `tests/bid_schema.rs`, `tests/contract_schema.rs`, `tests/contract_registry_accessors.rs` | ACIP/provider communications and model-role aptitude selection | Contract/golden fixture and rejection proof | Keep under `adl-runtime`; no `adl-provider` split until provider matrix proof stabilizes. |
| Private state, privacy, and witness surfaces | `private_state`, `private_state_envelope`, `private_state_equivocation`, `private_state_lineage`, `private_state_observatory`, `private_state_sanctuary`, `private_state_sealing`, `private_state_witness` | `tests/private_state*.rs` families | Memory v2 / ObsMem handoff; privacy-bound birthday evidence | Redaction, lineage, witness, and fail-closed proof | Do not move before privacy/redaction regression fixtures are explicit. |
| Moral governance and anti-harm proof | `anti_harm_trajectory_constraints`, `moral_event_validation`, `moral_metrics`, `moral_resources`, `moral_trace_schema`, `moral_trajectory_review`, `invariant`, `invariant_contract`, `outcome_linkage_attribution` | `tests/anti_harm_trajectory_constraints.rs`, `tests/moral_*.rs`, `tests/invariant_*.rs`, `tests/outcome_linkage_attribution.rs` | Birthday safety envelope; AEE policy stops | Contract, negative-case, trajectory, and trace proof | Keep together until moral trace schema and invariant contracts have explicit public API characterization. |
| Affect, wellbeing, humor, kindness, and theory-of-mind surfaces | `affect_reasoning_control`, `wellbeing_metrics`, `wellbeing_metrics_parts`, `humor_and_absurdity`, `kindness_model`, `theory_of_mind_foundation` | `tests/affect_reasoning_control.rs`, `tests/wellbeing_metrics.rs`, `tests/humor_and_absurdity.rs`, `tests/kindness_model.rs`, `tests/theory_of_mind_foundation.rs` | Affect/happiness activation and non-claims | Safe-test and non-claim proof | Do not present as birthday-ready until #3377 consumes test posture and non-claims. |
| Intelligence, learning, cultivation, and Gödel-adjacent mechanics | `intelligence_metric_architecture`, `cultivating_intelligence`, `cultivating_intelligence_parts`, `governed_learning_substrate`, `resource_stewardship_bridge` | `tests/intelligence_metric_architecture.rs`, `tests/cultivating_intelligence.rs`, `tests/governed_learning_substrate.rs`, `tests/resource_stewardship_bridge.rs` | Gödel mechanics, economics context, learning activation | Golden fixtures, contract validation, and activation-map proof | Keep code movement deferred until v0.92 activation names proof lanes. |
| Observatory, demos, and public review surfaces | `observatory`, `observatory_flagship`, `cognitive_being_flagship_demo`, `governed_tools_flagship_demo`, `governed_tools_flagship_demo_parts`, `contract_market_demo`, `csm_run`, `integrated_csm_run`, `governed_episode` | `tests/observatory*.rs`, `tests/cognitive_being_flagship_demo.rs`, `tests/governed_tools_flagship_demo.rs`, `tests/contract_market_demo.rs`, `tests/csm_run_packet.rs`, `tests/integrated_csm_run.rs`, `tests/governed_episode.rs` | Unity Observatory and demo readiness | Demo packet, artifact, path-hygiene, and non-proving proof | Do not mix Unity app work with runtime-v2 module movement. |
| Market, economic, operator, and contract surfaces | `contract_market_demo`, `operator`, `external_counterparty`, `bid_schema`, `resource_stewardship_bridge` | `tests/contract_market_demo.rs`, `tests/operator_control.rs`, `tests/external_counterparty.rs`, `tests/bid_schema.rs` | Economics context and provider/model matrix | Context-only or explicit-test decision proof | Keep economics as context unless v0.92 activation map promotes explicit tests. |
| Feature proof coverage and registry access | `feature_proof_coverage`, `contracts`, `contract_schema`, `types` | `tests/feature_proof_coverage.rs`, `tests/contract_registry_accessors.rs` | v0.92 readiness and release evidence | Coverage contract and accessor smoke proof | Treat as registry/coverage substrate; do not move without preserving discovery behavior. |

## Large-File Watch Points

The module navigability report identified these runtime-v2 files among the
largest source surfaces:

| File | Approximate LoC | Registry note |
| --- | ---: | --- |
| `contract_market_demo.rs` | 1382 | Demo/economics surface; split only with demo fixture proof. |
| `humor_and_absurdity.rs` | 1370 | Affect/non-claim surface; defer movement until safe-test posture is explicit. |
| `kindness_model.rs` | 1315 | Affect/kindness surface; defer movement until activation/non-claim proof exists. |
| `moral_resources.rs` | 1314 | Moral-governance surface; requires invariant and trace-schema characterization. |
| `delegation_subcontract.rs` | 1268 | ACIP/delegation contract surface; requires contract/golden fixture proof. |
| `private_state_observatory.rs` | 1268 | Privacy/observatory surface; requires redaction and path-hygiene proof. |
| `challenge.rs` | 1217 | Continuity challenge surface; requires slow-proof/golden fixture posture. |
| `transition_authority.rs` | 1203 | Identity/continuity authority surface; requires transition negative-case proof. |

These are candidates for later characterization-backed splits, not immediate
v0.91.5 movement.

## v0.92 Activation Crosswalk

| v0.92 activation-test-map surface | Runtime-v2 family to cite | Notes for #3377 |
| --- | --- | --- |
| AEE completion tranche | Citizen/lifecycle/standing, core manifold/kernel, access/security | Explicitly test wake/handoff, policy stops, queue/state transitions, and replayability before claiming completion. |
| Memory v2 / ObsMem handoff | Private state/privacy/witness; identity/continuity | Require redaction and witness proof before birthday evidence. |
| ACP / cognitive profiles | Core manifold/kernel; affect/wellbeing; theory of mind | Treat cognitive labels as bounded fixtures, not free authority. |
| Aptitude and capability selector | A2A/ACIP/evaluation/provider coordination | Connect provider/model role matrix to evaluation-selection proof. |
| Identity and continuity | Citizen/lifecycle/standing; transition authority; snapshot | Negative cases and transition authority proof required. |
| Affect and happiness surfaces | Affect/wellbeing/humor/kindness/theory-of-mind | Activation must carry non-claims and safe-test posture. |
| Gödel mechanics | Intelligence/learning/cultivation/resource stewardship | Birthday proof should map experiment and promotion mechanics without moving modules first. |
| Economics context | Market/economic/operator/contract surfaces | Decide context-only versus explicit tests before use. |
| Observatory and Unity demo readiness | Observatory/demo/public review surfaces | Unity app work must stay separate from runtime-v2 proof registry changes. |
| ACIP / provider communications | A2A/ACIP/delegation/evaluation/provider coordination | Verify schema and mock-carrier fallback readiness. |
| Provider/model matrix | A2A/ACIP/evaluation/provider coordination; access/security | Pair model identity with safety boundary proof. |
| Public prompt records | Feature proof coverage and registry access | Prompt packets should cite proof coverage, not local chat memory. |

## Re-Export And API Notes

- `mod.rs` re-exports nearly all feature modules with `pub use`.
- `validators` is crate-private and should remain an internal validation
  substrate unless a future public API issue approves exposure.
- Moving a module into a subdirectory is not automatically behavior-preserving
  if public re-export order, public type names, golden fixtures, or generated
  artifact paths change.
- Future splits should prove public API stability with targeted compile/test
  selectors before and after movement.

## Deferred Movement Routes

| Candidate movement | Route | Required proof before code movement |
| --- | --- | --- |
| Runtime-v2 large-file splits | Future runtime-v2 characterization issue after #3377 consumes this registry. | Pre/post focused tests for the file family, public re-export check, artifact fixture comparison. |
| Observatory/demo module split | Demo sprint or Unity Observatory planning issue, not #3623. | Demo packet/path-hygiene proof and non-proving demo boundary. |
| Private-state privacy split | Memory/ObsMem v2 or privacy review issue. | Redaction, witness, lineage, and negative-case tests. |
| Affect/wellbeing split | v0.92 activation readiness issue. | Safe-test posture and non-claim review. |
| ACIP/provider split | Provider/model matrix issue after OpenRouter/DeepSeek work. | Provider identity, transport fallback, and schema contract tests. |
| Validator exposure | Dedicated API issue only if external users need it. | Public API decision, compile proof, and backward compatibility review. |

## Validation

Focused validation for #3623 should prove registry consistency without running
runtime behavior.

Recommended checks:

- `bash adl/tools/report_module_navigability.sh --top 20 --format tsv`
- `find adl/src/runtime_v2 -maxdepth 2 -type f -name '*.rs' | sort`
- `rg -n "runtime_v2|v0.92|#3377|activation" docs/milestones/v0.91.5/RUNTIME_V2_NAVIGATION_REGISTRY_3623.md docs/milestones/v0.91.5/V092_ACTIVATION_TEST_MAP_v0.91.5.md`
- `git diff --check`

## Non-Claims

- This issue does not move runtime-v2 code.
- This issue does not claim v0.92 activation readiness.
- This issue does not introduce runtime logging or OpenTelemetry.
- This issue does not change artifact schemas, public command behavior, or
  module re-export behavior.
