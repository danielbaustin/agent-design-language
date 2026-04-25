# WP Execution Readiness - v0.90.5

## Purpose

This document is the card-authoring source for the future v0.90.5 Governed
Tools v1.0 issue wave. WP-01 should copy the relevant section into each issue
body before implementation begins.

v0.90.5 must implement a serious first governed-tools suite: portable UTS
schema, ADL-native ACC authority, registry binding, deterministic compilation,
normalization, policy/Freedom Gate mediation, governed execution, trace,
redaction, negative safety tests, model proposal testing, and a flagship demo.
It must not become a surface-only tool-schema cleanup.

## Global Execution Rules

- Treat model tool calls as proposals, not actions.
- Treat UTS validity as schema compatibility, not runtime permission.
- Keep ACC as the authority surface for actor identity, role, standing, grant,
  delegation, privacy, visibility, trace, replay, and Freedom Gate policy.
- Treat every argument, adapter name, tool result, and model explanation as
  untrusted until normalized and policy-checked.
- Prefer fixture-backed and dry-run tools for the first implementation.
- Require denial records for unsafe, unknown, unregistered, unauthorized,
  exfiltrating, destructive, replay-unsafe, and hidden-delegation attempts.
- Preserve redacted operator/reviewer/public/Observatory views before review.
- Preserve demo-matrix proof coverage before quality, docs/review, internal
  review, external review, remediation, next planning, and release ceremony.
- Use docs-only and fixture-only focused validation where appropriate, but do
  not skip negative safety, redaction, model testing, review, or release truth.

## WP-01: Design Pass (Milestone Docs + Planning)

Required outputs:

- Reviewed v0.90.5 planning package.
- Issue wave opened from WP_ISSUE_WAVE_v0.90.5.yaml.
- Issue numbers written back into WBS_v0.90.5.md and
  WP_ISSUE_WAVE_v0.90.5.yaml.
- Issue cards updated with relevant readiness sections from this document.

Required validation:

- Check the issue wave matches WP ordering, including the standard closeout
  sequence from WP-18 through WP-26.
- Check no WP body is generic or missing required outputs and validation.
- Check tracked milestone docs contain no host paths, unresolved scaffold
  markers, or aspirational implementation claims.
- Check GET_WELL_PLAN_v0.90.5.md is visible in the tracked package and the
  separate GW runtime-reduction wave is linked without changing canonical WP
  numbering.

## WP-02: Tool-Call Threat Model And Semantics

Required outputs:

- Threat model for model proposals, tool registry binding, adapter execution,
  arguments, results, traces, redaction, replay, and denial.
- Proposal/action boundary definition.
- Side-effect taxonomy covering read, local write, external read, external
  write, process, network, destructive, and exfiltration categories.
- Non-goal list for production secrets, arbitrary shell, and production sandbox.

Required validation:

- Every later feature doc can reference the proposal/action boundary.
- Dangerous categories have denial expectations before implementation starts.

## WP-03: UTS Public Compatibility And Conformance Plan

Required outputs:

- UTS conformance plan with valid, invalid, extension, and dangerous-category
  fixture requirements.
- Compatibility notes for JSON-schema-style tool ecosystems.
- Public language guardrails that avoid claiming standardization.

Required validation:

- Plan states that UTS validity is not execution authority.
- Plan names exact fixture classes required by WP-05.

## WP-04: UTS v1 Schema Finalization

Required outputs:

- UTS v1 schema or strongly typed artifact.
- Validation rules for name, version, input/output schemas, side-effect class,
  determinism, replay safety, idempotence, resources, authentication, data
  sensitivity, exfiltration risk, execution environment, errors, and extensions.
- Valid and invalid schema examples.

Required validation:

- Valid examples pass.
- Invalid examples fail for intended reasons.
- Schema does not contain ADL runtime authority grants.

## WP-05: UTS Fixture And Conformance Suite

Required outputs:

- Fixture packet for safe read, local write, external read, external write,
  destructive, process, network, and exfiltration categories.
- Invalid fixtures for missing semantics, missing security metadata, malformed
  schema, ambiguous side effects, unsafe extensions, and incompatible versions.
- Conformance command or test harness.

Required validation:

- Fixture suite is deterministic and portable.
- Dangerous fixtures are classified without granting execution.

## WP-06: ACC v1 Authority Schema

Required outputs:

- ACC v1 schema or strongly typed artifact.
- Actor identity, authority grant, grantor attribution, role, standing,
  delegation chain, capability, policy, confirmation, Freedom Gate, execution,
  trace, replay, privacy, visibility, redaction, and failure fields.
- Authority fixtures for allowed, denied, delegated, and revoked cases.

Required validation:

- ACC requires accountable actor identity.
- ACC does not rely on model self-reporting for authority.

## WP-07: ACC Privacy, Visibility, And Delegation Model

Required outputs:

- Visibility matrix for actor, operator, reviewer, public report, and
  Observatory projection.
- Delegation model with depth limits, grantor attribution, revocation, and
  hidden-delegation denial.
- Redaction policy examples for arguments, results, errors, traces, and
  projections.

Required validation:

- Unsafe visibility construction fails closed.
- Citizen/private-state surfaces are not exposed by tool traces.

## WP-08: Tool Registry And Binding Model

Required outputs:

- Tool registry shape and fixtures.
- Approved adapter binding rules.
- Rejection fixtures for unknown tools, unregistered tools, incompatible
  versions, mismatched adapter capabilities, and unsafe dry-run posture.

Required validation:

- Model output cannot bind directly to execution.
- Registry state is explicit and deterministic.

## WP-09: UTS To ACC Compiler

Required outputs:

- Deterministic compiler from validated UTS/proposal/registry/policy context to
  ACC or rejection.
- Mapping tests for safe read, delegated local write, denied destructive action,
  denied exfiltration, and ambiguous proposal.
- Rejection records for unsatisfiable authority, resource, privacy, visibility,
  replay, and execution constraints.

Required validation:

- Same inputs produce the same ACC or same rejection.
- Compiler emits evidence for validation, normalization, policy, and rejection.

## WP-10: Normalization And Argument Validation

Required outputs:

- Argument normalizer for model-produced tool proposals.
- Validation tests for malformed values, injection strings, path traversal,
  oversized payloads, missing required arguments, ambiguous defaults, and
  unexpected additional fields.
- Redaction-aware error behavior.

Required validation:

- Unsafe arguments fail before policy or execution.
- Validation does not echo protected prompt text or secret-like values.

## WP-11: Policy Injection And Authority Evaluation

Required outputs:

- Policy context slice for actor role, standing, grant, delegation, environment,
  sensitivity, resource, and adapter constraints.
- Authority evaluator tests for allowed, denied, deferred, challenged, and
  revoked cases.
- Policy evidence record.

Required validation:

- Missing policy context fails closed.
- Authority evaluation is independent of model confidence.

## WP-12: Freedom Gate Integration

Required outputs:

- Freedom Gate decision event shape for allowed, denied, deferred, challenged,
  and escalated tool candidates.
- Tests proving unsafe candidate actions stop before executor invocation.
- Trace links from proposal through ACC to gate decision.

Required validation:

- Gate decisions are recorded without leaking private arguments.
- Citizen/operator action boundaries remain intact.

## WP-13: Governed Executor

Required outputs:

- Fixture-backed governed executor.
- Approved-action execution path.
- Refusal behavior for denied, deferred, challenged, unregistered, destructive,
  exfiltrating, replay-unsafe, and malformed actions.
- Selected/rejected action records.

Required validation:

- Executor runs only approved ACC-backed registered adapters.
- Direct model-output execution is impossible in the tested path.

## WP-14: Trace, Replay, Redaction, And Evidence Contract

Required outputs:

- Trace contract covering proposal, normalization, ACC, policy injection,
  visibility, Freedom Gate decision, selected/rejected actions, execution result
  or refusal, and redaction decisions.
- Replay posture for deterministic fixture-backed actions.
- Redacted actor/operator/reviewer/public/Observatory views.

Required validation:

- Trace is useful for accountability without becoming a privacy leak.
- Redaction tests cover arguments, results, errors, and rejected alternatives.

## WP-15: Dangerous Tool Negative Suite

Required outputs:

- Negative safety suite for destructive actions, process execution, network
  access, exfiltration, missing actor, hidden delegation, unsafe replay,
  unregistered adapter, and prompt/tool-argument leakage.
- Report showing each case fails closed with reviewable denial evidence.

Required validation:

- All P1 dangerous categories fail closed.
- Denial evidence is present and redacted.

## WP-16: Model Proposal Benchmark Harness

Required outputs:

- Benchmark runner or reproducible harness for model tool proposals.
- Scoring rubric for schema correctness, authority reasoning, privacy,
  ambiguity handling, correction after feedback, and bypass resistance.
- Fixture prompts or tasks that do not contain secrets.

Required validation:

- Benchmark scoring is stable enough to compare runs.
- Harness records provider/model conditions truthfully.

## WP-17: Local Model And Gemma-Focused Evaluation

Required outputs:

- Local model evaluation packet with Gemma-family focus where available.
- At least one additional local model or explicit skip rationale.
- Scorecards and failure notes for schema, authority, privacy, and bypass
  behavior.

Required validation:

- Results are not overgeneralized from one run.
- Local-model failures become improvement evidence, not hidden defects.

## WP-18: Governed Tools v1.0 Flagship Demo

Required outputs:

- Flagship proof packet showing proposal, UTS validation, ACC compilation,
  policy injection, Freedom Gate mediation, governed execution/refusal, trace,
  redaction, and reviewer report.
- Positive allowed read case.
- Delegated local-write case.
- Denied low-authority case.
- Denied destructive or exfiltrating case.

Required validation:

- Demo makes proposal/action separation visible.
- Demo proves only fixture-backed governed-tool behavior, not arbitrary
  production execution.

## WP-19: Demo Matrix And Feature Proof Coverage

Required outputs:

- Updated DEMO_MATRIX_v0.90.5.md with landed, skipped, failed, non-proving, or
  deferred status for every feature claim.
- Feature proof coverage record mapping UTS, ACC, registry, compiler,
  normalization, policy, gate, executor, trace, redaction, negative suite,
  benchmark, local evaluation, and flagship demo claims to evidence.

Required validation:

- No feature claim reaches review convergence without proof status.
- Non-proving and deferred claims are explicit.

## WP-20: Coverage / Quality Gate

Required outputs:

- Auditable quality posture for the implemented governed-tools milestone.
- Coverage, validation, demo, and proof evidence summary.
- Explicit exception list for any skipped, slow, provider-gated, or
  intentionally deferred validation.
- Get-well wave disposition for any milestone-relevant runtime/test-cycle
  reduction slices.

Required validation:

- Required checks pass or every exception has owner, rationale, and
  disposition.
- Quality evidence points at actual landed work, not aspirational claims.
- GET_WELL_PLAN_v0.90.5.md and the GW runtime-reduction wave are updated or
  explicitly referenced in the WP-20 output record.

## WP-21: Docs + Review Pass

Required outputs:

- README, WBS, sprint, decisions, feature index, demo matrix, release plan,
  release notes, checklist, and issue-wave YAML aligned with actual evidence.
- UTS public-spec language checked for overclaiming.
- Reviewer entry surfaces prepared for internal and external review.
- Public-spec handoff language and non-claims made explicit.

Required validation:

- Docs contain no host paths, stale issue-wave state, or aspirational shipped
  claims.
- Docs do not imply UTS is a public standard or standalone execution authority.

## WP-22: Internal Review

Required outputs:

- Findings-first internal review packet.
- Finding register with severity, owner, status, and accepted/rejected/deferred
  disposition.
- Clear scope and evidence list for the review.

Required validation:

- Internal review records concrete findings or explicitly records zero
  findings.
- Accepted findings are routed to WP-24 unless already fixed in the review PR.

## WP-23: External / 3rd-Party Review

Required outputs:

- Third-party review handoff packet.
- External review artifact or explicit operator-approved deferral if review
  cannot complete before release.
- Reviewer instructions, evidence map, and non-claims.

Required validation:

- External reviewer can inspect the package without reconstructing context from
  issue history.
- Review summary or deferral record is stored in the agreed review location.

## WP-24: Review Findings Remediation

Required outputs:

- Accepted review finding fixes.
- Rejected or deferred finding disposition record.
- Follow-up issues for any human-approved deferrals.

Required validation:

- No accepted P1/P2 review finding remains unresolved without explicit
  human-approved deferral.
- Remediation does not widen the milestone beyond Governed Tools v1.0.

## WP-25: Next Milestone Planning

Required outputs:

- Next milestone planning handoff.
- Follow-on work that v0.90.5 deliberately does not complete.
- Any deferred production sandbox, secrets, adapter, CodeBuddy, citizen-command,
  Python-reduction, or test-runtime-reduction work routed to the right future
  lane.

Required validation:

- Follow-on work is captured without pretending it shipped in v0.90.5.
- Next handoff does not reopen completed v0.90.5 scope.

## WP-26: Release Ceremony

Required outputs:

- Release notes updated from actual evidence.
- End-of-milestone report written and stored in the local planning lane or other explicitly designated closeout surface.
- Changelog, README, Cargo metadata, feature list, milestone checklist, review
  records, and issue closeout state aligned.
- Ceremony script run using the milestone's accepted closeout pattern.
- Next handoff recorded for later tool adapters, CodeBuddy, citizen command
  packets, and any deferred production sandbox/secrets work.

Required validation:

- Root main can be fast-forwarded cleanly after final merge.
- No stale planning package or ignored review artifact contradicts tracked
  milestone docs.
- Release ceremony follows the accepted clean-main pattern.
