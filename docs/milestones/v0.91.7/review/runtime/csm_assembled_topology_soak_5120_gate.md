# CSM Assembled Topology Soak Gate for #5120

Issue: `#5120 [v0.91.7][WP-07][soak][runtime] Prove assembled CSM runtime topology soak`

## Gate Decision

Status: `HELD`

The assembled topology soak is prepared but must not be counted as executed until the remaining WP-07 prerequisite issues are merged or explicitly dispositioned. This packet records the current gate truth so the eventual soak can start from a reviewable dependency boundary instead of chat memory.

Last gate refresh: 2026-07-12.

## Current Runtime Baseline

- Runtime owner: `csm`
- Control plane: `csmctl` (architecture role; stable `.adl/bin/csmctl` was
  not present during this gate refresh)
- ADL language tooling: `adl`
- C-SDLC tooling: `csdlc`
- Main runtime API port: `127.0.0.1:19997`
- Service binary source of truth: `.adl/bin/csm`
- Current observed liveness at gate update: `bound_port`

The main CSM service manifest was repaired during #5120 preparation to point at `.adl/bin/csm` instead of a disposable issue-worktree `target/debug/csm`. That operational repair restored the embedded runtime API and must remain true before any soak starts.

## Prerequisite State

| Issue | Component / proof area | Current gate state | Action before soak |
| --- | --- | --- | --- |
| `#5110` | runtime crate boundary | `settled` | none |
| `#5111` | deterministic core / nondeterministic shell | `settled` | none |
| `#5112` | component supervision policy matrix | `settled` | none |
| `#5113` | typed channel backpressure matrix | `settled` | none |
| `#5114` | governed shutdown DAG | `settled` | none |
| `#5115` | cloud bridge fail-closed cursors | `settled` | none |
| `#5116` | runtime API auth | `settled` | none |
| `#5117` | CSM-managed Vector observability | `settled` | none |
| `#5118` | native reasoning runtime | `settled` | none |
| `#5119` | checkpoint continuity vs lifelog history | `settled` | none |
| `#5169` | recovered storage-pressure state without restart | `settled` | none |
| `#5122` | Freedom Gate runtime component | `settled` | none |
| `#5123` | CAV runtime component | `checks_pending_no_checks_attached` | wait for PR #5158 checks on `d04e51ace866b6fef47bae96c7bf83d1f19dce84`; if checks attach and pass, merge and close out |
| `#5125` | Constructability Gate runtime component | `draft_pr_checks_running_after_coverage_repair` | wait for PR #5255 checks on `00f99206d31ab6171e302821342f17e681d02548`; if green, publish/merge and close out |
| `#5126` | ACIP carrier / protobuf / WebSocket runtime paths | `merged_pending_closeout` | run normal closeout for merged PR #5241 or explicitly accept closeout handoff evidence |
| `#5164` | CodeBuild Rust validation determinism | `checks_pending_after_clippy_repair` | wait for PR #5210 checks on `1f04c8a026745f5f4b40f01ac2721d8a2a3592cc`; if green, merge and close out |

## Required Soak Proof When Gate Opens

The soak must not start while `#5123`, `#5125`, `#5126`, or `#5164/#5210`
remain open/pending unless the operator explicitly dispositiones them out of the
soak dependency set with retained evidence. When the gate opens, the soak must
use one continuous real CSM process identity and retain a correlated artifact
packet proving:

- startup and API readiness on the canonical `19997` port
- supervised component health for runtime API, Chronosense, scheduler, reasoning runtime, Freedom Gate, CAV, AEE, checkpoint, lifelog, cloud bridge, Shepherd, and Vector observability
- typed-channel flow and backpressure behavior
- authenticated API status, health, ready, metrics, events, chronosense, shepherd, and API-gateway-bridge truth
- checkpoint continuity and lifelog history as distinct primitives
- storage-pressure recovery without process replacement
- cloud bridge fail-closed behavior without false cursor advancement
- observability through the CSM-managed Vector path with retained local evidence
- governed shutdown-DAG disposition without hidden kill/request/cycle budgets
- negative cases for unavailable component, failed observability sink, failed cloud bridge, checkpoint failure, and shutdown timeout

Mocks, fake transports, detached prototypes, stale evidence packets, skipped negative cases, and prose-only records do not satisfy this gate.

## Non-Claims

- This packet does not claim the #5120 soak has run.
- This packet does not claim product readiness.
- This packet does not close or supersede any prerequisite issue.
- This packet does not rename Observability to Evidence.
