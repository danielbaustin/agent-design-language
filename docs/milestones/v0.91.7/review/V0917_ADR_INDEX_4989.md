# v0.91.7 ADR Index

Issue: #4989

WP-17 verification: 2026-07-18

## Summary

This index records the v0.91.7 ADR disposition for the architecture decisions
named by #4989. It writes the required v0.91.7 ADR set while preserving
existing ADR baselines and explicitly deferring decisions that do not yet have
implementation evidence.

WP-17 verified that every path in the written set exists in `docs/adr/`, ADR
0043 through ADR 0050 are listed by the accepted-record index, and ADR 0051 is
listed as deferred. Candidate ADRs 0030, 0031, 0034, and 0040 remain in
`docs/architecture/adr/` with candidate status; this index does not silently
promote them.

## Written In This Issue

| ADR | Decision | Source |
| --- | --- | --- |
| `docs/adr/0043-adl-platform-cli-binary-taxonomy.md` | ADL Platform CLI binary taxonomy and ownership boundaries | #4983, #4989, #4995, v0.91.7 WBS, WP-06/build-throughput and workflow-stabilization evidence |
| `docs/adr/0044-c-sdlc-operational-coordination-boundary.md` | Scheduler/watcher/shepherd/session-ledger coordination boundary | #4433, #4443, #4713, #4950 |
| `docs/adr/0045-validation-manager-and-fast-slow-proof-boundary.md` | Validation manager and fast/slow/remote proof boundary | #4676, #4678, #4679, #4806 |
| `docs/adr/0046-repo-native-workflow-transport-boundary.md` | Repo-native GitHub/workflow transport boundary | #4622, #4806, #4960 |
| `docs/adr/0047-repo-binaries-and-warm-cache-validation-boundary.md` | Repo binaries, warm cache, and remote validation acceleration boundary | #4726, #4806, #4837, #4838, #4879 |
| `docs/adr/0048-runtime-observability-and-otel-boundary.md` | Runtime observability and OTel-compatible mapping boundary | #4718, #4682 |
| `docs/adr/0049-runtime-soak2-pre-v092-readiness-boundary.md` | Runtime Soak #2 as pre-v0.92 integration gate | #4681, #4682, #4683, #4843 |
| `docs/adr/0050-scheduler-provider-local-agent-delegation-boundary.md` | Scheduler/provider/local-agent delegation boundary | #4671-#4675, #4849, #4932 |
| `docs/adr/0051-chronosense-and-memory-palace-adr-disposition.md` | Chronosense baseline and Memory Palace deferred ADR disposition | ADR 0010, v0.91.7 handoff docs |

## Existing Accepted ADRs Consumed

| Topic | Existing ADR |
| --- | --- |
| C-SDLC workflow guardrails and issue lifecycle | ADR 0024 |
| C-SDLC tracked workflow state and signed trace | ADR 0028 |
| Merge readiness and PR gate truth | ADR 0033 |
| Local polis SSM operations boundary | ADR 0035 |
| Validation lane selector and PVF test-cost policy | ADR 0036 |
| GitHub/C-SDLC projection ownership | ADR 0037 |
| Runtime integration soak boundary | ADR 0038 |
| Cognitive scheduler authority boundary | ADR 0039 |
| Provider/model suitability boundary v2 | ADR 0041 |
| Public prompt records publication boundary | ADR 0042 |

## Deferred Candidates

| Candidate | Disposition |
| --- | --- |
| Chronosense v0.91.7 refinement | Recorded in ADR 0051: existing ADR 0010 remains the accepted baseline unless current implementation evidence changes the time substrate boundary. |
| Memory Palace / continuity-context topology | Recorded in ADR 0051: deferred until implementation evidence is present; do not record an accepted decision from planning intent alone. |
| Full OpenTelemetry/runtime observability boundary | Existing issue/proof evidence should be consumed by runtime/observability closeout first; write a follow-on ADR only if the accepted boundary changes. |
| Guild/polis governance expansion | Existing deferred governance candidates remain outside this issue unless WP-13 produces accepted implementation evidence. |

## Validation Plan

- `git diff --check`
- spot-check ADR links and existing ADR references
- bounded docs review for unsupported claims and stale v0.91.7 status

## Non-Claims

- This packet does not claim every v0.91.7 feature is complete.
- This packet does not accept deferred candidates.
- This packet does not implement the CLI taxonomy; it records the architecture boundary.
- CLI taxonomy implementation is tracked by #4995.
