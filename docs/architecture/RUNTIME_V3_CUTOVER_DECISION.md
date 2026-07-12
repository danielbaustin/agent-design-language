# Runtime v3 Cutover Decision

Issue: #5218
Sprint: #5174
Decision date: 2026-07-12
cutover_authorized: false

> Supersession note: #5254 is the final v0.91.7 default-switch/decommission
> decision packet. It keeps Runtime v2 as the default runtime, retains Runtime
> v3 as explicit opt-in only, and routes release-gate closure through #5220.
> See `docs/architecture/RUNTIME_V3_CUTOVER_DECISION_5254.md`.

## Decision

Runtime v3 remains in incubation. Do not switch default runtime behavior yet.

The current evidence supports continued Runtime v3 development and scoped
explicit opt-in work, but it does not authorize cutover. The release blocker is
not philosophical: Horust 0.1.13 is the pinned guardian candidate, and the
retained #5211 qualification report proves that it does not enforce bounded
`on-failure` restart attempts for repeated post-start crashes.

The fixed-Horust release path is tracked by #5221. Runtime v3 weather and GPU
proof is tracked by #5222. GPU proof deferred to #5222 is not counted as a
guardian cutover blocker for #5211, and it is not counted as passed proof here.

## Evidence Table

| Surface | Issue | State | Decision input |
|---|---:|---|---|
| Runtime v3 sprint umbrella | #5174 | open | Cutover is not complete while the sprint umbrella remains open. |
| Guardian package bakeoff and soak | #5175 | closed | 100-cycle soak passed; final review says continue incubation, not cutover. |
| Parity inventory and component contracts | #5176 | closed | 18 capability groups and baseline routing exist; routing is not behavioral closure. |
| Control API health and observability | #5177 | closed | Implemented control/observability components; exporter delivery remains a non-claim where recorded. |
| Freedom Gate and governed execution | #5178 | closed | Governed execution components are implemented and tested in Runtime v3 scope. |
| Shadow parity and migration harness | #5179 | closed | Explicit opt-in and rollback facade exist; report decision is continue incubation. |
| Reasoning loops and adaptive learning DAGs | #5180 | closed | Runtime v3 reasoning components are implemented with bounded loop/adaptation proof. |
| Continuity replay and governed recovery | #5181 | closed | Signed checkpoint, replay, recovery, and quarantine proof exist in Runtime v3. |
| Configuration registry and topology | #5182 | closed | Configuration and topology construction proof exists. |
| Agent/provider/scheduler integrations | #5183 | closed | Operational components exist with bounded fixtures, not production cutover authority. |
| Horust adoption and qualification | #5211 | open | Evidence merged; adoption blocked by Horust 0.1.13 restart-budget defect. |
| Fixed Horust rollout | #5221 | open | Required successor before guardian cutover can be reconsidered. |
| Weather/GPU qualification | #5222 | open | GPU proof deferred to #5222; no GPU-runtime claim is made here. |
| Guardian fallback review | #5224 | open | No reviewed COTS candidate is a drop-in cross-platform external guardian replacement. |

## Go / No-Go

No-go for default Runtime v3 cutover.

Required before reconsidering:

1. #5221 pins a released Horust version containing the restart-budget fix from
   FedericoPonzi/Horust#319.
2. Native qualification proves deterministic restart exhaustion under the
   corrected Horust release.
3. #5220 runs the Runtime v3 release proof gate for the selected entrypoint
   scope.
4. #5219 provides an explicit, reversible Runtime v3 entrypoint switch without
   silently changing default behavior.
5. #5222 either proves weather/GPU monitoring on an actual approved GPU host or
   retains a reviewed deferral that the release decision explicitly accepts.

## Non-Claims

- This packet does not authorize production cutover.
- This packet does not switch the default runtime.
- This packet does not delete Runtime v2.
- This packet does not claim Horust 0.1.13 enforces bounded restart attempts.
- This packet does not claim GPU monitoring or GPU runtime qualification.
- This packet does not turn Runtime v3 capability routing into full behavioral
  equivalence with Runtime v2.

## Next Issues

- #5219: add explicit Runtime v3 entrypoint switch.
- #5220: run release proof gate.
- #5221: pin fixed Horust release and rollout guardian.
- #5222: qualify resource and GPU monitoring.
- #5224: review guardian fallback candidates while fixed Horust remains blocked.
