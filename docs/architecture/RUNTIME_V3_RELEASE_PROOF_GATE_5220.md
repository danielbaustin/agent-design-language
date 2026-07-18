# Runtime v3 Release Proof Gate (#5220)

Issue: #5220
Sprint: #5227
Target: v0.91.7
Decision date: 2026-07-12
release_gate_result: complete_no_default_cutover
default_cutover_authorized: false

## Summary

The v0.91.7 release-decision review is complete, but live cutover proof is not.
The gate closes as a no-go for default Runtime v3 cutover.

Runtime v3 has retained proof for explicit opt-in selection, selected control
API and observability contracts, weather/resource monitoring, bounded
soak/rollback, private-state security, identity/memory continuity, and governed
cognition adapters. The live black-box parity packet now reports
`cutover_eligible: true` with zero remaining blocker-class capability groups
after #5277, #5278, #5279, #5280, #5281, #5282, #5283, #5284, and #5285.
#5286 records Observatory consumption truth for Runtime v3 explicit opt-in.
Runtime v2 remains the default runtime and rollback target.

## Selected Scope

This release gate covers the selected v0.91.7 Runtime v3 cutover scope:

- the explicit Runtime v3 selection report and independent kernel command;
- Runtime v2 default and rollback preservation;
- guardian fallback behavior for the selected small Tokio child-process scope;
- resource weather and graceful-stop policy;
- runtime component topology and API contracts;
- continuity, replay, recovery, private-state, identity, memory, and governed
  cognition evidence;
- observability/stdout-stderr boundary evidence;
- parity routing and residual non-equivalence.

## Evidence Inputs

| Surface | Evidence | Result |
|---|---|---|
| Entrypoint report | `docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md` | Contract only: reports the requested runtime but invokes neither runtime; Runtime v2 remains default. |
| Cutover decision | `docs/architecture/runtime_v3_cutover_decision_5254.v1.json` | #5254 records no-go for default switch and no Runtime v2 decommission. |
| Live black-box parity | `docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json` | `cutover_eligible: true`; zero blockers remain for live black-box parity after #5277, #5278, #5279, #5280, #5281, #5282, #5283, #5284, and #5285. |
| Weather/resource monitoring | `docs/architecture/runtime_v3_weather_cloudwatch_5252.v1.json` | CPU, memory, continuity-filesystem disk, CloudWatch-shape event, and graceful-stop policy are retained; observed GPU telemetry is deferred. Pressure stop atomically pauses new control admission before signing current runtime state. Checkpoint failure reopens admission and retries without stopping the kernel or API. |
| Observatory consumption | `docs/architecture/runtime_v3_observatory_consumption_5286.v1.json` | Runtime v3 owns a loopback read feed at `GET /v1/observatory` on port `20997`; HTML Observatory consumes it only by explicit opt-in, retained Runtime v2/CSM evidence remains the default mirror, and Unity remains proven-limited while #4739/#4741 are open. |
| Soak and rollback | `docs/architecture/runtime_v3_soak_rollback_5253.v1.json` | Retained packet; the 100-cycle test is ignored by default and is not live completion evidence. |
| Shadow parity | `docs/architecture/runtime_v3_shadow_parity_report.v1.json` | Runtime v2 remains default and Runtime v3 stays opt-in. |
| Cutover checklist | `docs/architecture/runtime_v3_cutover_checklist.v1.json` | Current gate state remains no default cutover. |

## Gate Results

| Gate | Result | Notes |
|---|---|---|
| Explicit entrypoint report | contract only | Reports an explicit Runtime v3 request without invoking either runtime. |
| Default runtime switch | not authorized | Runtime v2 remains default for v0.91.7. |
| Runtime v2 deletion/decommission | not authorized | Runtime v2 remains intact and is the rollback target. |
| Guardian fallback | passed for selected fallback scope | Tiny Tokio child-process fallback is retained; Horust/native lanes remain non-pass unless separately qualified. |
| Weather/resource monitoring | passed with GPU deferral | CPU, memory, continuity-filesystem disk, CloudWatch-shape event, and graceful-stop policy are retained. Pressure checkpoint failure keeps service alive and re-arms monitoring; observed GPU telemetry is deferred. |
| API health and port policy | passed by contract | Runtime v3 control endpoint remains `127.0.0.1:20997`. |
| Observatory live consumption | passed explicit opt-in | Runtime v3 exposes a runtime-owned read feed at `GET /v1/observatory`; browser mutation authority remains false and signed commands remain required for control mutation. |
| Observability contract | passed by contract | Machine-readable output and human `adl_event` streams remain separated by existing policy. |
| Soak and rollback | ignored | #5253 is retained, but its 100-cycle test is not part of the default executed proof. |
| Live black-box parity | passed without default cutover | #5248 now reports `cutover_eligible: true` with zero blocker-class capability groups; #5220 still closes as no-go for default cutover until a later reviewed decision authorizes a default switch. |

Every machine-readable gate now declares one evidence class: `executed`,
`contract_only`, `ignored`, or `deferred`. A gate may satisfy a live requirement
only when its evidence class is `executed`.

The deterministic size inventory measures the independent Runtime v3 kernel
against the 12,000-line challenge and reports the selected shared guardian as a
separate auxiliary surface. The combined figure is retained rather than hidden;
the guardian variance is explicit because that file includes the reusable
cross-process contract and its co-located unit tests, not another kernel module.

## Child Issue Results

| Issue | Purpose | State |
|---:|---|---|
| #5247 | v0.91.7 Runtime v3 cutover-readiness umbrella. | closed |
| #5248 | Live black-box parity fixtures for cutover-critical groups. | closed |
| #5249 | Private-state and security equivalence. | closed |
| #5250 | Citizen identity and memory-continuity adapters. | closed |
| #5251 | Governed cognition adapters or reviewed non-cutover dispositions. | closed |
| #5252 | Retained GPU/weather and CloudWatch proof. | closed |
| #5253 | Production-like soak and rollback proof. | closed |
| #5254 | Default switch and Runtime v2 decommission decision path. | closed |

## Residual Risks

- Runtime v3 live black-box parity blockers and Observatory opt-in consumption
  truth are recorded, but default cutover still requires a separate reviewed
  decision.
- Observed GPU telemetry remains deferred until an approved GPU host run is
  retained.
- Remote multi-day soak and Horust/native guardian qualification are not counted
  as passed proof in this gate.
- Unix process-group containment is executed proof for the selected guardian;
  Windows Job Object containment is not claimed.
- Runtime v2 decommission remains a separate reviewed decision.

## Focused Proof Commands

The retained local proof surface for this release gate is:

```text
python3 -m json.tool docs/architecture/runtime_v3_release_proof_gate_5220.v1.json
python3 -m json.tool docs/architecture/runtime_v3_cutover_checklist.v1.json
python3 -m json.tool docs/architecture/runtime_v3_observatory_consumption_5286.v1.json
cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test control -- observatory_feed_serves_runtime_owned_read_projection_without_mutation_authority --nocapture
bash adl/tools/test_v0917_html_observatory_integrated_proof.sh
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test parity -- release_proof_gate_closes_without_authorizing_default_cutover final_cutover_decision_keeps_v2_default_after_live_parity_clear kernel_lifecycle_proof_resolves_only_kernel_lifecycle_blocker topology_backpressure_proof_resolves_only_topology_backpressure_blocker service_contracts_configuration_proof_resolves_only_service_contracts_blocker continuity_replay_recovery_proof_resolves_only_continuity_blocker adaptive_learning_dag_proof_resolves_only_learning_blocker governance_freedom_gate_aee_proof_resolves_only_governance_blocker delegation_resources_proof_resolves_only_delegation_blocker acip_a2a_cloud_network_proof_resolves_final_live_black_box_blocker --nocapture
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test guardian_soak -- production_like_soak_rollback_packet_retains_cutover_boundaries packaging_preserves_one_guardian_neutral_child_contract --nocapture
cargo test --manifest-path adl-runtime/Cargo.toml guardian::tests -- --nocapture
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test guardian_soak -- pressure_checkpoint_failure_keeps_signal_shutdown_responsive --nocapture
cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path adl-runtime-kernel/Cargo.toml
```

GitHub required PR checks remain integration proof for the published branch:

```text
adl-path-policy
adl-coverage
adl-tooling-contracts
adl-rust-fmt-clippy
adl-rust-tests
adl-demo-proof
adl-ci
```

## Non-Claims

- This packet does not authorize production cutover.
- This packet does not switch Runtime v3 to the default.
- This packet does not delete, rewrite, or decommission Runtime v2.
- This packet does not claim full Runtime v2 behavioral equivalence.
- This packet does not count GPU, remote multi-day, or Horust/native deferred
  lanes as passed proof.
- This packet does not claim Windows process-tree containment.
