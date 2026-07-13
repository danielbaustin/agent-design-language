# Runtime v3 Cutover Decision (#5254)

Issue: #5254
Target: v0.91.7
Decision date: 2026-07-12
cutover_authorized: false

## Decision

Runtime v2 remains the default runtime for v0.91.7. Runtime v3 remains available
only through explicit opt-in selection, with Runtime v2 retained as the rollback
target.

This is a no-go decision for a default Runtime v3 switch. The retained parity
packet now reports `cutover_eligible: true` after the final live black-box
blocker was resolved by #5285. Follow-on Runtime v3 cutover sprint proofs
#5277, #5278, #5279, #5280, #5281, #5282, #5283, #5284, and #5285 reduce the remaining
blocker-class capability groups to zero without authorizing a default switch.
#5252 and #5253 resolved the weather/observability and soak/rollback
prerequisites, but they did not by themselves convert the remaining
capability-specific blockers into passed proof.

## Evidence

| Surface | Evidence | Decision input |
|---|---|---|
| Live black-box parity | `docs/architecture/runtime_v3_live_black_box_parity_5248.v1.json` | `cutover_eligible: true`; zero blocker-class capabilities remain after #5277, #5278, #5279, #5280, #5281, #5282, #5283, #5284, and #5285. |
| Explicit selection and rollback | `docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md` | Runtime v3 is explicit opt-in; Runtime v2 remains the default and rollback target. |
| Weather and CloudWatch boundary | `docs/architecture/runtime_v3_weather_cloudwatch_5252.v1.json` | Local weather/resource proof exists; observed GPU telemetry remains a non-pass deferred surface. |
| Soak and rollback | `docs/architecture/runtime_v3_soak_rollback_5253.v1.json` | Bounded production-like soak and rollback proof exists; remote multi-day and GPU lanes remain non-claims. |
| Shadow parity | `docs/architecture/runtime_v3_shadow_parity_report.v1.json` | Runtime v2 remains default and Runtime v3 stays opt-in. |

## Blocking Surfaces

No live black-box parity capability groups remain blocker-class for default
switch authorization.

Default switch authorization still remains separate release-decision truth.
#5220 closes v0.91.7 with Runtime v2 as the default unless a later reviewed
packet authorizes a default switch.

## Rollback

Rollback remains simple because no default switch is authorized:

- Without explicit selection, Runtime v2 is selected.
- To use Runtime v3, select it explicitly with `--runtime v3` or
  `ADL_RUNTIME_SELECTION=v3`.
- To roll back, remove explicit Runtime v3 selection or select `--runtime v2`.
- Selection rollback does not rewrite retained runtime state.

## Non-Claims

- This decision does not authorize Runtime v3 as the default runtime.
- This decision does not delete or decommission Runtime v2.
- This decision does not count blocked or deferred lanes as passed proof.
- This decision does not claim observed GPU telemetry or remote multi-day soak
  proof.
- This decision does not claim full Runtime v2 behavioral equivalence.
