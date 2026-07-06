# WP-07 Pre-v0.92 Runtime-Coherence Disposition (#4845)

Generated: 2026-07-05T03:11:00Z

This packet is the current pre-v0.92 runtime-coherence disposition for WP-07
after the final `#4880` Soak 2 rerun. It consumes current v0.91.7 evidence only
and intentionally does not claim runtime readiness from historical planning
packets, component tests, mocks, or stale pre-merge PR state.

Machine-readable companion:

- `docs/milestones/v0.91.7/review/runtime/wp07_pre_v092_runtime_coherence_disposition_4845.json`

## #4890 Runtime Ownership Addendum

Follow-on issue `#4890` separates daemon ownership from the ADL
compiler/control-plane CLI. Current runtime daemon ownership is `csm daemon`;
`adl agent daemon` is not retained as a public command. The CSM proof packet is
`docs/milestones/v0.91.7/review/runtime/csm_4890/` and validates
`command=csm`, `process_class=csm_runtime_daemon`, local `ADL_OTEL_LOG`, and
`ADL_OTEL_STATUS` monitor status for the daemon path. Historical WP-07 packets
that record the former `adl agent daemon` proof are preserved as historical
evidence, not as the current runtime entrypoint.

## Decision

Disposition: **blocked pending operator disposition**

v0.92 activation decision: **not ready for full runtime-coherence claims**

WP-07 closeout basis: keep umbrella `#4634` open until `#4880` is merged and
the operator either approves non-claims/defer for the remaining blocked rows or
routes them to Soak 3.

Reason: `#4880` proves the local integrated runtime path, inherited lifecycle
runner, AEE/memory slice, resilience failure injection, provider fixture,
scheduler packet, logging/OTel proof consumption, ObsMem handoff, and
identity/continuity evidence. The `#4634` umbrella repair adds a retained
local `ADL_OTEL_LOG` JSONL export and `ADL_OTEL_STATUS` monitor-status proof
for the daemon path; this is still not a network OTLP collector or hosted
telemetry backend claim. It emits runtime-v2 security-boundary and
operator-control static contract artifacts, but those packets record
`generated_at_utc: not_started` and are not fresh integrated proof. The run also
does not prove live AWS/signal bridge, Unity editor live consumption, or WP-12
ACIP/A2A activation.

## Evidence Inputs

| Source | Current state | Evidence consumed |
| --- | --- | --- |
| #4842 Runtime v2 reconciliation | merged/closed | PR #4851 on `main`; consumed by #4880 status packet. |
| #4718 logging/OTel proof | merged/closed and rerun | `docs/milestones/v0.91.7/review/observability_4718/generated/proof_summary.json`. |
| #4681 canonical runtime path | merged/closed | PR #4868 on `main`; `docs/milestones/v0.91.7/review/runtime/MINIMAL_INTEGRATED_RUNTIME_PATH_4681.md`. |
| #4783 scheduler watcher/AEE resilience middleware | merged/closed | PR #4869 on `main`; consumed by #4880 resilience rows. |
| #4784 resilience failure injection | merged/closed | `docs/milestones/v0.91.7/review/runtime/soak2_4682/resilience/runtime_failure_injection_proof.json`. |
| #4843 Soak 2 matrix | merged/closed | `docs/milestones/v0.91.7/review/runtime/soak2_feature_list_matrix_4843.json`. |
| #4682 Soak 2 execution | superseded blocked-status packet | Historical `blocked_before_full_soak` attempt superseded by #4880. |
| #4880 final Soak 2 rerun | merged/closed | `docs/milestones/v0.91.7/review/runtime/soak2_4682/soak2_execution_status_4682.json`. |
| #4634 umbrella OTel monitor repair | in progress | `docs/milestones/v0.91.7/review/runtime/soak2_4682/otel_monitor/otel_status.json`. |

## Activation Decision Table

| Surface | Decision | Owner | v0.92 consequence |
| --- | --- | --- | --- |
| Canonical runtime path | integrated_proven | #4681/#4880 | Can be cited as local integrated runtime-path evidence. |
| Runtime v2 reconciliation | integrated prerequisite | #4842/#4880 | Can be cited as substrate reconciliation. |
| Logging/OTel | integrated_proven with boundary | #4718/#4880/#4885/#4634 | stdout/stderr, redaction, event samples, daemon trace/span/service fields, local `ADL_OTEL_LOG` JSONL export, and `ADL_OTEL_STATUS` monitor status are proven; no network OTLP collector or hosted backend claim. |
| Scheduler watcher/AEE resilience middleware | integrated_proven | #4783/#4784/#4880 | Local scheduler/resilience rows are proven with retained evidence. |
| Soak 2 execution | completed_with_blockers | #4880 | Remaining blockers require operator disposition before #4634 closes. |
| Runtime AWS/signal bridge | blocked_with_evidence | WP-08 owners/#4880 | Must stay non-claimed, operator-deferred, or routed to Soak 3. |
| Unity live consumption | blocked_with_evidence | Unity owners/#4880 | Runtime contract exists; Unity editor live run remains non-claimed/deferred or Soak 3. |
| WP-12 ACIP/A2A activation | blocked_with_evidence | WP-12 owners/#4880 | Local ACIP cases exist; activation claims remain blocked/deferred. |
| Runtime-v2 capability envelope | blocked_with_evidence | #4696/#4880 | Static operator-control packet exists; fresh integrated capability proof remains blocked/deferred. |
| Runtime-v2 security/CAV boundary | blocked_with_evidence | #4657/#4880 | Static fail-closed packet exists; fresh integrated security/CAV proof remains blocked/deferred. |
| Curiosity/constructability optional row | explicitly deferred | #4692/#4693/#4880 | Does not block v0.92 unless promoted into activation scope. |

## Proposed Operator Disposition

This packet does not approve non-claims and does not start Soak 3. The minimum
operator decision needed before closing `#4634` is one of:

1. Approve v0.92 non-claims/defer for AWS signal bridge, Unity live consumption,
   WP-12 ACIP/A2A activation, runtime-v2 capability freshness, and runtime-v2
   security/CAV freshness; or
2. Route those rows to Soak 3 with owners and acceptance criteria.

## Non-Claims

- No network OTLP collector, hosted telemetry service, or external exporter
  backend is claimed. WP-07 now claims the local durable `ADL_OTEL_LOG` JSONL
  sink and `ADL_OTEL_STATUS` monitor file proven by `#4634`.
- No live AWS signal bridge/SNS/SSM heartbeat proof is claimed.
- No Unity editor live-consumption run is claimed.
- No WP-12 ACIP/A2A activation closure is claimed.
- No fresh runtime-v2 capability-envelope or security/CAV proof is claimed from
  static packets whose generated timestamp is `not_started`.
- This packet does not close umbrella `#4634` by itself.

## WP-07 Closeout Instruction

Do not close umbrella `#4634` from this disposition alone. Closeout requires
the `#4880` PR to merge plus explicit operator disposition for the remaining
blocked rows.
