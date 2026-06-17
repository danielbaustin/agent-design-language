# v0.91.6 Logging Completion Ledger

## Metadata

- Milestone: `v0.91.6`
- Wave: `WP-03`
- Umbrella: `#3968`
- Covered child issues: `#3995`-`#4000`
- Explicitly routed, not executed by this packet: `#4001`
- Captured: `2026-06-17`

## Purpose

This ledger is the authoritative burn-down surface for the `v0.91.6` logging
mini-sprint. WP-03 closeout must not claim completion unless every row below is
either:

- `complete`
- `blocked`
- `routed`
- `non_goal`

No row may remain as vague future work.

## Completion Matrix

| Lane | Surface | Status | Evidence | Notes |
| --- | --- | --- | --- | --- |
| `#3995` | Ledger, ownership map, and `v0.92` consumption gate | `complete` | this ledger; [`TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md`](features/TOOLING_PROOF_LOOP_RELIABILITY_v0.91.6.md) | WP-03 closeout now has one bounded checklist surface. |
| `#3996` | Control-plane logging, JSON channel policy, compatibility-log truth | `complete` | [`CONTROL_PLANE_LOGGING_PROOF_3996.md`](review/logging_observability/CONTROL_PLANE_LOGGING_PROOF_3996.md); `adl/src/cli/pr_cmd_cards/validation.rs`; `adl/tools/test_pr_json_observability.sh`; `adl/tools/test_control_plane_observability.sh` | Includes the `doctor --json` pollution fix for bootstrap validator success text. |
| `#3997` | Runtime/provider action logging, provider/model identity, failure classification | `complete` | [`RUNTIME_PROVIDER_LOGGING_PROOF_3997.md`](review/logging_observability/RUNTIME_PROVIDER_LOGGING_PROOF_3997.md); `adl/src/instrumentation/action_log.rs`; `adl/src/provider_communication.rs`; `docs/milestones/v0.91.5/RUNTIME_ACTION_LOG_CONTRACT_3556.md` | Treated as a bounded correlated slice, not full repo-wide telemetry unification. |
| `#3998` | Heartbeat, timeout, and progress diagnostics | `complete` | [`HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md`](review/logging_observability/HEARTBEAT_TIMEOUT_PROGRESS_PROOF_3998.md); `adl/src/cli/observability.rs`; `adl/src/cli/agent_cmd.rs`; `adl/src/cli/pr_cmd/finish_support.rs`; `adl/src/execute/support.rs` | Covers the currently claimed long-path surfaces and preserves explicit non-claims for exhaustive command-wide coverage. |
| `#3999` | OTel boundary and Observatory/Unity consumption example | `complete` | [`OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md`](review/logging_observability/OTEL_OBSERVATORY_CONSUMPTION_PROOF_3999.md); [`observatory_event_stream_example_3999.jsonl`](review/logging_observability/observatory_event_stream_example_3999.jsonl); `docs/milestones/v0.91.5/OPEN_TELEMETRY_INTEGRATION_BOUNDARY_3709.md`; `docs/milestones/v0.91.5/OBSERVATORY_LOG_CONSUMPTION_CONTRACT_3710.md` | Confirms the boundary and consumer packet without overclaiming a production exporter. |
| `#4000` | Validation, redaction, path hygiene, proof-loop checks | `complete` | [`LOGGING_VALIDATION_REDACTION_PROOF_4000.md`](review/logging_observability/LOGGING_VALIDATION_REDACTION_PROOF_4000.md); `docs/milestones/v0.91.5/LOGGING_VALIDATION_CHECKLIST_3711.md`; `docs/milestones/v0.91.5/DOCS_ONLY_VALIDATION_BUNDLE_3736.md` | Keeps focused proof selection explicit and captures remaining tooling defects as remediation rather than hidden scope. |
| `#4001` | GitHub, token, release, and projection observability | `routed` | row preserved here; umbrella issue `#3968` | Outside the user-approved execution scope for this run. It remains a tracked separate route and must not be claimed as done by `#3995`-`#4000`. |

## Historical Input Mapping

| Prior surface | Disposition in this wave | Notes |
| --- | --- | --- |
| `#3922` runtime logging/observability scheduling | consumed | The logging mini-sprint now leaves a bounded completion ledger and proof packet. |
| `#3925` repo-quality/staleness work | consumed_by_checklist | Logging validation proof consumes the quality/staleness posture where it affects durable artifacts. |
| `#3935` card-to-GitHub projection convergence | routed | Explicitly preserved as adjacent tooling reliability work, not claimed by the logging-only child wave. |
| `#3963` and `#3965` logging/tooling prep routes | consumed | Their planning and issue-wave readiness work is reflected in the WP-03 child issue set and proof packet. |
| `v0.91.5` proof packets `#3705`-`#3711` | consumed | Used as baseline contracts and non-claim boundaries rather than treated as stale pre-hardening docs. |

## v0.92 Consumption Gate

`v0.92` may consume the WP-03 logging baseline only if all of the following are
true:

1. Control-plane JSON surfaces remain parse-safe on stdout.
2. Human-oriented `adl_event` lines remain stderr-by-default or explicit
   compatibility-log output.
3. Runtime/provider action evidence is redacted and carries enough correlation
   for bounded debugging.
4. Heartbeat/progress/timeout claims are tied to named surfaces, not implied as
   repo-wide.
5. Observatory/Unity consumers use the bounded example packet and boundary
   mapping rather than assuming a production OTel exporter already exists.
6. Any remaining gaps are either explicitly routed or preserved as non-claims.

## Problems Captured For Remediation

- Shared runtime step progress still uses plain `STEP start/done` stderr lines
  rather than the shared `adl_event` vocabulary.
- Heartbeat coverage is real but not exhaustive for every control-plane
  subcommand; future lanes should widen only with focused proof.
- GitHub/token/release/projection observability remains a separate routed lane
  under `#4001` and is intentionally not claimed complete here.

## Closeout Rule

WP-03 may close only when this ledger remains truthful, the linked proof packet
exists, and no row above is silently treated as “future work.”
