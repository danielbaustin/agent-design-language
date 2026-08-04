# Structured Task Prompt

Template: 1.0.0

Issue: 5361

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Prepare acceptance cards now; execute acceptance only after every declared dependency is integrated and proven.

## Deliverables

- reviewed Runtime v3 acceptance contract and dependency graph
- exact-revision parity and consumer proof register including #5350 shadow parity
- guardian, access, Observatory, operations, and rollback evidence
- line-count, module-growth, dependency-audit, test-count, CI, and exact-revision review report
- truthful acceptance or blocker disposition for #5384 consumption

## Acceptance

1. AC-1: #5591 completes before #5592, #5589, and #5590, and all four parity packets are reviewed and integrated
2. AC-2: Runtime v3 executes canonical ingress, continuity, checkpoint/replay/resume, and graceful pressure shutdown at the accepted revision
3. AC-3: Reasoning graphs, loops, affect-control, adaptive learning, governed operations, and secure access have explicit parity proof or blocker disposition
4. AC-4: #5341, #5349, #5350, and #5501 prove real Runtime v3 consumers and exact-revision shadow parity without Runtime v2 implementation dependencies
5. AC-5: Guardian, HTTPS local/remote API, Observatory, telemetry, rollback, and recovery proof passes with no hard-coded address
6. AC-6: Focused tests, strict lint, CI, line-count, module-growth, dependency, and exact-revision review gates pass
7. AC-7: Unsupported GPU, remote-provider, or deployment claims remain explicit non-claims and cannot be promoted by this umbrella

## Dependencies

- merged WP-01 #5594
- Runtime v3 architecture authority #5336
- Parity-A #5591 before Parity-B #5592, Parity-C #5589, and Parity-D #5590
- ADL v2 Runtime adapter #5341
- provider and tool adapter owner #5349
- exact-revision shadow parity #5350 with every mismatch dispositioned
- live multi-agent workcell proof #5501

## Inputs

- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/features/RUNTIME_V3_ADAPTER_v0.91.8.md
- docs/milestones/v0.91.8/FEATURE_PROOF_COVERAGE_v0.91.8.md
- docs/milestones/v0.91.8/V092_ACTIVATION_TEST_MAP_v0.91.8.md
- adl-runtime
- adl-runtime-kernel
- infra/runtime-v3

## Non Goals

- acceptance execution during card preparation
- Runtime v2 implementation reuse or deletion
- v0.92 activation or release approval
- new feature scope beyond declared parity
- AWS use
- hard-coded network addresses
