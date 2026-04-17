# WP Execution Readiness - v0.90

## Metadata

- Milestone: v0.90
- Version: v0.90
- Date: 2026-04-17
- Owner: Daniel Austin
- Status: execution readiness gate
- Corrective issue: #2046

## Purpose

This document turns the v0.90 feature contracts into execution gates for the
open work-package issues.

The review finding was not that v0.90 lacks a serious plan. The feature docs
are strong. The problem was that several WP execution cards were still generic
enough that a session could complete "docs/tests/code as needed" without proving
the intended feature contract.

This gate makes the work harder to wiggle out of. Each implementation session
should use the matching source docs, produce the named artifact or proof
surface, run focused validation, and preserve the stated non-goals.

## Work Package Classes

Not every v0.90 WP should produce runtime code. That is intentional.

- Code and runtime WPs: WP-02 through WP-06, plus WP-14 when evidence justifies
  a refactor.
- Demo and proof WPs: WP-07 through WP-09.
- Quality and process sidecars: WP-10 through WP-12.
- Docs, review, remediation, planning, and release discipline: WP-13 through
  WP-20.

The quality bar is not "every WP writes code." The quality bar is "every WP
ships a concrete, reviewable output appropriate to its class."

## Global Execution Rules

- Every WP must name its source docs before implementation starts.
- Every runtime WP must name the artifact files, CLI/API surface, or module
  boundary it expects to create or change.
- Every demo WP must name the proof claim, proof command or proof packet, and
  non-goals before it begins.
- Every quality/process WP must measure first, change second, and record the
  evidence that justified the change.
- Every release-discipline WP must record review, readiness, or release truth
  without pretending to be runtime implementation.
- Public artifacts must avoid private host paths, secrets, broker credentials,
  and financial-advice language.
- Full v0.92 identity and capability governance remain out of scope for v0.90.

## WP-02: Long-Lived Supervisor And Heartbeat

Source docs:

- features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md
- features/LONG_LIVED_AGENT_RUNTIME_FEATURE_SET.md

Required outputs:

- Supervisor spec parsing or a locked supervisor contract.
- Agent initialization that creates the expected state root.
- Bounded tick, run, status, and stop behavior.
- Lease behavior that prevents overlapping cycles.
- Status artifact showing idle, running, stopped, failed, and stale-lease states
  where applicable.

Required validation:

- Focused tests or proof commands for lease refusal, stale-lease recovery,
  status reporting, stop handling, and max-cycle limits.
- A reviewer-readable artifact or command output proving the heartbeat path is
  bounded.

Non-goals:

- No daemon service requirement.
- No distributed scheduler.
- No full v0.92 identity tuple.
- No unbounded autonomous run loop.

## WP-03: Cycle Contract And Artifact Root

Source docs:

- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md
- features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md

Required outputs:

- Cycle artifact root layout.
- Cycle manifest.
- Observations artifact.
- Decision request and decision result artifacts.
- Run reference artifact.
- Memory-write candidate artifact.
- Guardrail report artifact.
- Cycle summary artifact.
- Deterministic cycle identifiers and stable ordering rules.

Required validation:

- Focused schema or shape tests for the cycle artifacts.
- A command or fixture showing one complete cycle artifact bundle can be
  generated and inspected.
- Failure-path proof that failed or blocked cycles still leave reviewable
  artifacts.

Non-goals:

- No unbounded multi-cycle scheduler.
- No live ObsMem dependency.
- No claim that the cycle contract is already the full identity substrate.

## WP-04: State And Continuity Handles

Source docs:

- features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md
- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md

Required outputs:

- Continuity handle.
- Locked agent spec.
- Append-only cycle ledger.
- Provider binding history where model/provider binding is available.
- Memory index or durable memory-write references that do not require a live
  ObsMem backend.
- Migration boundary that explicitly marks the handle as pre-v0.92.

Required validation:

- Focused tests or proof commands showing initialization creates the required
  files.
- Multiple-cycle proof that the ledger appends without deleting prior entries.
- Restart or resume proof that the latest cycle can be recovered from the
  ledger.
- Explicit check that artifacts do not claim full v0.92 identity.

Non-goals:

- No capability governance.
- No autonomous legal or social identity claim.
- No destructive history rewrite.

## WP-05: Operator Control And Safety

Source docs:

- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md
- features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md

Required outputs:

- Operator status surface.
- Stop control that prevents the next cycle.
- Guardrail report for each cycle.
- Artifact sanitization check for public demo mirrors.
- Explicit policy defaults for no broker, no real trading, no personalized
  financial advice, no real-world side effects, and no writes outside allowed
  roots.

Required validation:

- Focused tests or proof commands for stop authority, stale lease behavior,
  consecutive failure stop behavior, guardrail failures, and sanitization.
- Stock-league illegal action proof that broker or order execution remains
  rejected.

Non-goals:

- No real broker integration.
- No live financial advice.
- No silent guardrail bypass.
- No demo cycle without max-cycle or stop policy.

## WP-06: Minimal Inspection And Trace Boundary

Source docs:

- features/FEATURE_LONG_LIVED_SUPERVISOR_HEARTBEAT.md
- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md
- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md
- features/SIGNED_TRACE_ARCHITECTURE.md, only if the WP explicitly accepts a
  narrow trace slice.
- features/TRACE_QUERY_LANGUAGE.md, only if the WP explicitly accepts a narrow
  query slice.

Required outputs:

- Smallest reviewer-useful inspection surface for status and cycle artifacts.
- Clear decision on whether minimal trace/query behavior belongs in v0.90 or is
  deferred.
- Reviewer proof packet that shows how to inspect a supervised agent without
  reading the whole artifact tree manually.

Required validation:

- Proof command for the inspection path.
- Fixture or demo output showing the command points at real status, cycle,
  guardrail, and summary artifacts.

Non-goals:

- No full TQL platform unless explicitly accepted by issue scope.
- No full signed-trace architecture implementation unless explicitly accepted by
  issue scope.
- No broad repo-indexing behavior.

## WP-07: Stock League Demo Scaffold

Source docs:

- features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md
- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md

Required outputs:

- Fixture-backed stock league scaffold.
- Paper-only league rules.
- Agent identity/style cards.
- Demo artifact root.
- No-financial-advice and no-broker guardrails in every public-facing surface.
- Fixture data path that is cheap, deterministic, and reviewable.

Required validation:

- Demo scaffold command or proof packet.
- Artifact scan showing no private host paths, secrets, broker credentials, or
  financial-advice claims.
- Proof that the scaffold can run or render without live trading systems.

Non-goals:

- No live trading.
- No personalized advice.
- No market-beating claims.
- No expensive or paid data dependency for the core proof path.

## WP-08: Long-Lived Demo Integration

Source docs:

- features/LONG_LIVED_STOCK_PICKING_AGENTS_DEMO_PLAN.md
- features/FEATURE_LONG_LIVED_AGENT_CYCLE_CONTRACT.md
- features/FEATURE_LONG_LIVED_STATE_AND_CONTINUITY.md
- features/FEATURE_LONG_LIVED_OPERATOR_CONTROL_AND_SAFETY.md

Required outputs:

- Integrated demo that exercises recurring bounded cycles.
- Continuity and ledger evidence across more than one cycle.
- Status and guardrail artifacts connected to the demo run.
- Reviewer-readable proof that an agent preserves history without rewriting
  prior commitments.

Required validation:

- Runnable or replayable integration command.
- Proof packet with state root, cycle ledger, cycle summaries, guardrail
  reports, and sanitized public artifacts.
- Deterministic fixture mode remains the canonical reviewer path.

Non-goals:

- No live market loop as the required proof path.
- No hidden long-running process required for review.
- No investment recommendation framing.

## WP-09: Demo Extensions And Proof Expansion

Source docs:

- DEMO_MATRIX_v0.90.md
- WBS_v0.90.md
- Any selected demo-specific feature or plan doc.

Required outputs:

- Named demo choices before execution begins.
- For each selected demo: proof claim, proof command or proof packet, non-goals,
  and expected artifact location.
- Explicit deferral note if no demo extension is selected.

Required validation:

- Each selected demo must be runnable, replayable, or explicitly classified as a
  non-proving surface.
- The demo matrix must distinguish landed, skipped, deferred, and non-proving
  demos.

Non-goals:

- No grab-bag demo cleanup.
- No demo expansion that weakens the stock-league proof path.
- No unreviewed new product claim.

## WP-10: Coverage Ratchet To 93 Percent

Source docs:

- WBS_v0.90.md
- Existing coverage tracker and quality-gate docs.

Required outputs:

- Baseline measurement before any threshold change.
- Focused tests for real uncovered behavior.
- Coverage threshold raised to 93 percent only after the evidence is green.
- Short report explaining what was covered and what remains below the next
  ratchet.

Required validation:

- Coverage command output.
- Focused test command output.
- Gate proof after the threshold update, if the threshold is changed.

Non-goals:

- No threshold theater.
- No broad fixture inflation.
- No unrelated refactor hidden under coverage work.

## WP-11: Milestone Compression Pilot

Source docs:

- WBS_v0.90.md
- SPRINT_v0.90.md
- WP_ISSUE_WAVE_v0.90.yaml
- Existing ADL workflow and release-tail docs.

Required outputs:

- Canonical milestone-state model for issue, PR, docs, demo, review, and release
  truth.
- Drift checks that find mismatch between docs, issue state, and release-tail
  readiness.
- Bounded pilot report.

Required validation:

- Run the drift check against current v0.90 state.
- Show at least one expected pass or known mismatch classification.

Non-goals:

- No autonomous merge or release approval.
- No root-checkout mutation workflow.
- No replacement for human ceremony decisions.

## WP-12: Repo Visibility Prototype

Source docs:

- WBS_v0.90.md
- FEATURE_DOCS_v0.90.md
- Existing repo visibility notes and backlog decisions.

Required outputs:

- Bounded manifest for one milestone or one feature slice.
- Linkage report connecting canonical docs, code, tests, demos, review
  artifacts, and issue records.
- Clear distinction between tracked milestone docs, planning docs, ideas, and
  retired material.

Required validation:

- Manifest generation or inspection command.
- Linkage report review showing missing, present, and deferred surfaces.

Non-goals:

- No full repo semantic indexing platform.
- No claim that local planning state is public release truth.
- No broad cleanup outside the selected slice.

## WP-13: Docs And Review Pass

Required outputs:

- Review-ready docs package.
- Feature index and demo matrix aligned with landed or deferred work.
- No stale milestone-status claims.
- No private host paths in public docs.

Required validation:

- Docs scan for stale version references, private host paths, and unresolved
  planning placeholders.
- YAML and Markdown surface checks where applicable.

Non-goals:

- No new runtime feature scope.
- No release ceremony before review gates are complete.

## WP-14: Rust Refactoring Pass

Source docs:

- WBS_v0.90.md
- Internal review findings, test hotspots, coverage findings, or maintainability
  evidence produced earlier in the milestone.

Required outputs:

- Explicit target list justified by evidence.
- Bounded Rust refactor PRs or a truthful no-op/defer note if the evidence does
  not justify a change.
- Validation record proving behavior did not change unintentionally.

Required validation:

- Focused Rust tests for touched surfaces.
- Formatting/lint checks where available.
- Before/after explanation tied to maintainability, testability, or review
  evidence.

Non-goals:

- No aesthetic refactor.
- No broad architecture rewrite.
- No opportunistic feature work.

## WP-15 Through WP-20: Release Discipline

These WPs are not runtime-code WPs. Their value is milestone truth.

- WP-15 internal review must produce findings-first review output or an explicit
  no-finding record with residual risks.
- WP-16 third-party review must record scope, files reviewed, requested review
  questions, severity rubric, and findings.
- WP-17 findings remediation must fix or explicitly defer every finding.
- WP-18 final readiness must rerun quality, demo, docs, compression, visibility,
  and refactor checks.
- WP-19 next milestone planning must prepare the following milestone before the
  release ceremony.
- WP-20 release ceremony must verify release notes, version surfaces, tag, and
  cleanup truth.

## Card Update Rule

When a session executes a v0.90 WP, its local STP/SIP/SOR bundle should inherit
the relevant section of this document. If a card is more generic than this gate,
the session should treat that as a readiness problem and tighten the card before
implementation.

This document does not replace the issue cards. It makes the intended execution
contract explicit and reviewable in tracked milestone docs.

## Issue Body Backfill

As part of corrective issue #2046, the public GitHub issue bodies for WP-02
through WP-20 were backfilled with an Execution Readiness Gate section. The
ignored local root STP cards under the v0.90 task bundle were also backfilled so
future pr-run sessions see the same contract before implementation begins.

Those backfilled issue/card sections are operational mirrors of this document.
If they drift, this tracked readiness document remains the source of truth.
