# Logging And Observability Gap Map

Issue: #3704  
Parent: #3703  
Captured: 2026-06-16
Status: inventory_refreshed_after_tooling_mini_sprint

## Summary

ADL now has several real logging surfaces, but they are still fragmented and
their operator ergonomics are uneven. The repo has shell and Rust `adl_event`
terminal events, runtime `logs/action_log.jsonl`, provider run JSONL logs,
long-lived-agent heartbeat/ledger state, multi-agent timing packets, and
Octocrab operation logs. Those are useful baselines, not a complete
observability system.

The remaining gap is no longer "add first logging surfaces." That tranche has
landed. The current gap is narrower: keep the logging baseline truthful after
the tooling mini-sprint, separate completed hardening from residual non-claims,
and use the refreshed packet as the starting point for toolkit simplification
instead of relying on stale pre-hardening assumptions.

## What Changed After The Tools Mini-Sprint

Since the first refreshed inventory:

- the tools workflow mini-sprint `#3796` landed and closed several practical
  workflow blockers around queue residue, prompt-card quality, and issue-body
  repair;
- the validator/editor hot path was split into direct small binaries by
  `#3832`, reducing recursive broad-binary routing for structured-prompt and
  prompt-template work;
- the logging-tail follow-ons that were still open in earlier review packets
  are now closed:
  - `#3789` machine-readable JSON surface cleanup
  - `#3807` fail-closed compatibility log behavior
  - `#3808` embedded absolute-path redaction
  - `#3809` bounded uniqueness for redacted provider artifact refs

That means the logging baseline is stronger than the original `#3704` packet
described, and the toolkit simplification sprint should start from that newer
truth.

## Existing Baselines

| Surface | Evidence | Current status | Notes |
| --- | --- | --- | --- |
| Shell control-plane events | `adl/tools/observability.sh` | implemented_baseline | Emits `adl_event schema=adl.observability.event.v1` lines, sanitizes secrets/paths, supports optional `ADL_OBSERVABILITY_LOG`. |
| Rust CLI events | `adl/src/cli/observability.rs` | implemented_baseline | Emits sanitized `adl_event` lines. Used by dispatcher and selected subcommands. |
| C-SDLC control-plane contract | `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md` and #3609 | implemented_baseline | Defines the `adl_event` vocabulary and an OTEL-ready mapping, but does not complete OTEL export or all command coverage. |
| Closeout silent-hang fix | #3698 | implemented_followup | Closed a reproduced post-merge closeout silence case. This is evidence that baseline logging needed follow-up hardening. |
| Octocrab GitHub operation logs | `adl/src/cli/pr_cmd/github.rs` and #3697 | implemented_baseline | Covered GitHub operations emit `github_octocrab` started/completed/failed events. Useful and recently dogfooded. |
| Integrated C-SDLC timing proof | `docs/milestones/v0.91.5/review/INTEGRATED_C_SDLC_TIMING_PROOF_3716.md` | implemented_evidence | Confirms Octocrab-backed operations, timing capture, and explicit merge fallback behavior across a real issue/PR path. |
| Runtime action log | `docs/milestones/v0.91.5/RUNTIME_ACTION_LOG_CONTRACT_3556.md`, `adl/src/instrumentation/action_log.rs`, #3556 | implemented_first_slice | Generates `logs/action_log.jsonl` from trace events and keeps it as a projection, not a second truth source. |
| Provider run log | `adl/src/provider_communication.rs`, `adl/src/provider_adapter_cli.rs`, #3480 | implemented_baseline | Provider JSONL logs exist with redacted event fields and duration/failure metadata. |
| Long-lived agent heartbeat and ledgers | `adl/src/long_lived_agent.rs`, `adl/src/long_lived_agent/`, `adl/src/long_lived_agent/storage.rs` | implemented_domain_specific | Has heartbeat intervals, stale-lease timing, cycle ledgers, provider binding history, and operator events, but not unified with C-SDLC/runtime observability. |
| Multi-agent timing packets | `docs/milestones/v0.91.5/review/multi_agent_*` | implemented_evidence | Sprint 2 packets record wall durations and lane timing; they are evidence artifacts, not a general logging framework. |

## Historical Gap Map

| Gap | Risk | Owner issue | Current truth |
| --- | --- | --- | --- |
| No single shared observability contract across shell, Rust CLI, runtime, providers, long-lived agents, and Observatory. | Different subsystems keep adding compatible-looking but divergent logs. | #3705 | Closed. Shared vocabulary and OTEL-ready mapping exist; remaining work is implementation breadth, not missing contract definition. |
| C-SDLC control-plane logging remained uneven beyond the fixed high-pain paths. | `doctor`, `finish`, `closeout`, validators, and watchers could be hard to diagnose when a new stage lacked progress output or lifecycle truth drifted after merge. | #3706 | Closed. Control-plane logging now exists on the main governed workflow paths; remaining defects are follow-on tooling/runtime hardening, not absence of a baseline. |
| Observability events shared stdout with machine-readable JSON in several tool paths. | JSON consumers and shell pipelines could fail or misparse because `adl_event` lines appeared before the JSON payload. | #3789 | Closed. This is no longer an active open gap from the logging packet, though future command surfaces still need to preserve the same channel policy. |
| Runtime/provider logging was split between trace-derived action logs and provider-specific JSONL. | Multi-agent/provider failures could be misread as model-quality failures if runtime/provider logs were not correlated. | #3707 | Closed as a first correlated slice. Provider-side request/artifact correlation now exists; full end-to-end unification remains a non-claim. |
| Heartbeat, timeout, and progress policy was not unified. | Operators still asked “is it hung?” for long commands or long-lived processes. | #3708 | Closed as a bounded policy/proof slice. Long-path diagnostics are materially better, though exhaustive coverage is still not claimed. |
| OpenTelemetry was only planned/OTEL-ready, not implemented. | Claims of standard observability could overstate reality; future exporters might be bolted on inconsistently. | #3709 | Closed as a boundary/contract issue. OTEL export remains intentionally unimplemented. |
| Observatory consumption was not defined against the current event model. | Unity/Observatory could invent a separate telemetry truth instead of consuming ADL runtime/C-SDLC events. | #3710 | Closed. Observatory consumption rules now exist as a contract/proof surface. |
| Docs, skills, AGENTS, and validation had to teach the completed logging model. | If guidance lagged implementation, future agents could regress into old or silent paths. | #3711 | Closed. Guidance and checklist surfaces now exist, and later tool hardening issues extended them. |

## Not Missing After This Inventory

- The repo does not need a brand-new logging concept from nothing. It already
  has `adl_event`, runtime action logs, provider run logs, and long-lived-agent
  ledgers.
- Sprint 2 multi-agent child work appears closed; the logging sprint should be
  treated as hardening before further reliance, not as a prerequisite to
  starting Sprint 2.
- `#3697` is closed and provides operational Octocrab logging evidence for
  GitHub transport operations.
- `#3698` is closed and provides a direct fix for one reproduced closeout
  silent-hang class.
- `#3716` provides real timing and Octocrab-backed workflow evidence for one
  integrated control-plane path.
- `#3609` is the implemented C-SDLC control-plane logging baseline and shared
  `adl_event` vocabulary, not a full OpenTelemetry implementation.
- `#3556` is the implemented first slice of runtime action-log projection,
  with remaining branch/span/dashboard coverage explicitly deferred.
- `#3480` is the implemented provider run-log baseline, not a unified
  runtime/provider/C-SDLC correlation model.
- `#3796` is closed and removed several workflow-adoption blockers that were
  making observability dogfooding harder than the code itself.
- `#3832` is closed and moved the validator/editor hot path onto direct small
  binaries, which reduces recursive control-plane noise for prompt-card proof.
- `#3789`, `#3807`, `#3808`, and `#3809` are closed and should no longer be
  carried as active open logging-tail defects in successor packets.

## Still Missing Or Partial

- OpenTelemetry export is not implemented.
- Runtime action logs are a first slice; direct emission from every validation
  branch, low-level artifact write, long-running span, and external dashboard
  remains deferred in `RUNTIME_ACTION_LOG_CONTRACT_3556.md`.
- Control-plane logging has a contract and important implementation, but not a
  complete proof that every long-running command path emits progress or that
  merged/ready/closed lifecycle states always reconcile cleanly.
- Provider logs exist, but provider/runtime/C-SDLC correlation is not unified.
- Long-lived-agent heartbeat and ledgers exist, but they are domain-specific and
  not mapped into a shared observability contract.
- Observatory/Unity consumption requirements are now documented, but live
  downstream adoption still depends on future Unity/Observatory implementation
  work rather than this sprint alone.
- Repo-native logging guidance now exists, but future workflow/tooling issues
  still need to apply it consistently and route follow-on defects instead of
  overclaiming maturity.
- The logging umbrella `#3703` is still open even though the original child wave
  is closed; that is now a closeout-truth/governance issue, not a missing
  implementation slice.

## Remaining Practical Baseline For Toolkit Simplification

Toolkit simplification should treat the following as the live starting truth:

1. the logging baseline is real and should not be re-routed through broad,
   silent, compatibility-heavy workflow paths unless a current issue proves that
   path safe;
2. direct small-binary validator/editor paths now exist and should be preferred
   when a simplification slice only touches prompt-card proof or prompt-template
   editing;
3. machine-readable JSON surfaces must remain channel-safe by construction;
4. future simplification cuts should capture any newly exposed observability
   regressions as follow-on issues instead of weakening the current boundary
   claims.

## Tool Problems To Capture For Future Remediation

- Prompt-template import/edit tooling is not robust on every current card
  surface; repeated template-token resolution and some rendered `stp` cards can
  fail round-trip import even when the Markdown is human-readable.
- Legacy bundles created before `#3837` can still preserve generic `STP`/`SPP`
  surfaces and may need normalization even though current bootstrap/init now
  generates issue-specific cards by default.
- Non-closing issue/PR lifecycle truth can drift: a PR may merge while the
  linked issue remains open and the local sprint state still treats the child as
  pending review.
- Full tools-workflow simplification is still incomplete beyond the validator
  and prompt-template slice; that remaining decomposition is now explicitly
  routed through toolkit simplification issue `#3838`.

## Validation Notes

This issue is an inventory and routing issue. It intentionally does not modify
runtime behavior. Focused validation should therefore check:

- the audit file exists;
- markdown has no obvious formatting/hygiene errors;
- the packet explicitly distinguishes closed historical gap issues from still
  partial non-claims;
- no raw secrets or absolute host-local paths were introduced.
