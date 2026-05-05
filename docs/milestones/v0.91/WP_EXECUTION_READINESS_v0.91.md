# WP Execution Readiness - v0.91

## Purpose

This document is the card-authoring source for the v0.91 moral governance,
cognitive-being, structured workflow, and secure local communication issue wave.
WP-01 should keep the opened issue bodies and local task bundles aligned with
the relevant section before implementation begins.

The reviewed candidate issue sequence is recorded in
[WP_ISSUE_WAVE_v0.91.yaml](WP_ISSUE_WAVE_v0.91.yaml). The official v0.91
issue wave is open as #2735 through #2759, with issue numbers assigned in the
YAML and the local v0.91 issue-card bundles.

## Global Execution Rules

- Implement assigned feature surfaces completely for v0.91; do not leave core
  moral/cognitive-being features as planning-only placeholders.
- Keep moral metrics as evidence and diagnostics, not verdicts or reputation
  scores.
- Preserve private wellbeing diagnostics; expose redacted operator, reviewer,
  and public views only where policy allows.
- Keep structured planning and structured review policy as durable issue-bundle
  artifacts, not chat-only behavior.
- Keep Agent Comms local, authenticated, traceable, and redacted. External or
  cross-polis communication remains gated on TLS or mutual-TLS-equivalent
  protection.
- Treat A2A as an adapter over the ADL communication substrate, not a parallel
  communication architecture.
- Keep v0.91.1 adjacent-system work out of the v0.91 core wave unless the
  v0.91 feature explicitly requires a boundary hook.
- Keep v0.92 birthday and v0.93 constitutional-governance work downstream.
- Preserve the standard closeout order: flagship demo, demo matrix, coverage
  and quality, docs and review, internal review, external review, remediation,
  next milestone planning, release ceremony.

## WP-01: Design Pass (Milestone Docs + Planning)

Required outputs:

- Reviewed v0.91 milestone package.
- Issue wave opened from WP_ISSUE_WAVE_v0.91.yaml.
- Issue numbers written back into WBS_v0.91.md and
  WP_ISSUE_WAVE_v0.91.yaml.
- Issue cards updated with relevant readiness sections from this document.

Required validation:

- Check the issue wave matches WP ordering, including the standard closeout
  sequence from WP-17 through WP-25.
- Check no issue body is generic or missing required outputs, validation,
  non-goals, source docs, and proof expectations.
- Check tracked milestone docs contain no host paths, unresolved scaffold
  markers, or aspirational shipped claims.

## WP-02: Moral Event Contract

Required outputs:

- Moral event contract for choices, alternatives, refusals, uncertainty,
  affected parties, evidence, policy context, and review visibility.
- Fixtures for allowed, denied, deferred, and challenged moral events.
- Non-claim language separating moral evidence from production moral agency.

Required validation:

- Valid fixtures bind to an accountable identity and trace context.
- Invalid fixtures fail when required evidence or actor context is missing.

## WP-03: Moral Event Validation

Required outputs:

- Validation rules for incomplete, evasive, contradictory, unreviewable, or
  policy-incoherent moral events.
- Negative fixtures for hidden delegation, missing affected-party information,
  missing alternative consideration, and unsupported certainty.

Required validation:

- Unsafe or incomplete moral-event records fail closed.
- Validation errors do not expose private diagnostic content.

## WP-04: Moral Trace Schema

Required outputs:

- Moral trace schema connecting event, choice, alternatives, refusal, outcome,
  attribution, visibility, and review references.
- Examples for ordinary action, refusal, delegation, and deferred decision.

Required validation:

- Same input ordering produces stable trace records.
- Trace records preserve reviewability without requiring public exposure of
  private state.

## WP-05: Outcome Linkage And Attribution

Required outputs:

- Outcome-linkage record connecting downstream consequences to prior choices
  while preserving uncertainty.
- Attribution model for actor, delegation, policy, tool, and environment
  contributions.
- Fixtures for known, unknown, partial, delayed, and contested outcomes.

Required validation:

- Outcome linkage must not collapse uncertainty into false certainty.
- Delegated outcomes retain visible attribution chains.

## WP-06: Moral Metrics

Required outputs:

- Trace-derived moral metric definitions for review and trend detection.
- Fixture report showing metric inputs, outputs, and limitations.
- Non-scoreboard language for metric interpretation.

Required validation:

- Metrics derive from explicit trace evidence.
- Metrics are not exposed as scalar karma, happiness, or reputation.

## WP-07: Moral Trajectory Review

Required outputs:

- Trajectory review packet over event sequences and longitudinal windows.
- Review criteria for drift, repetition, repair, refusal, escalation, and
  unresolved uncertainty.
- Synthetic trajectory fixtures.

Required validation:

- Review output cites trace evidence rather than hidden judgment.
- Ordering and tie-break behavior are deterministic.

## WP-08: Anti-Harm Trajectory Constraints

Required outputs:

- Anti-harm constraint model covering decomposed, delegated, delayed, and
  disguised harm.
- Synthetic delegated-harm proof packet.
- Denial and escalation records for unsafe trajectories.

Required validation:

- Harmful trajectories are detected across steps, not only as single forbidden
  actions.
- Safe synthetic examples remain bounded and non-operational.

## WP-09: Wellbeing Metrics v0

Required outputs:

- Decomposed wellbeing diagnostic covering coherence, agency, continuity,
  progress, moral integrity, and participation.
- Access policy for citizen self-view, operator view, reviewer view, and public
  redacted view.
- Fixtures for low, medium, high, unknown, and privacy-restricted diagnostics.

Required validation:

- Wellbeing output remains diagnostic, not a scalar happiness score.
- Private diagnostic details are not exposed to unauthorized views.

## WP-10: Moral Resources

Required outputs:

- Moral-resources contract for care, refusal, attention, dignity,
  anti-dehumanization, and repair.
- Fixtures showing resources used under conflict and uncertainty.
- Review surface showing resource claims and evidence.

Required validation:

- Moral-resource claims must link to trace evidence.
- Refusal and care are both represented without sentimentality or coercion.

## WP-11: Kindness Model

Required outputs:

- Kindness contract covering non-harm, dignity, autonomy, constructive benefit,
  and long-horizon support.
- Conflict fixtures where kindness requires refusal, delay, boundary-setting,
  or repair.

Required validation:

- Kindness is not treated as politeness or universal agreement.
- Unsafe accommodation cases fail or escalate.

## WP-12: Humor And Absurdity

Required outputs:

- Bounded frame-detection and reframing record.
- Fixtures for constructive reframing, failed reframing, manipulation risk, and
  inappropriate humor.
- Non-claim language avoiding entertainment or therapy claims.

Required validation:

- Reframing must preserve truth and dignity.
- Manipulative or minimizing reframes fail closed.

## WP-13: Affect Reasoning-Control Surface

Required outputs:

- Affect-like signal record as explicit reasoning-control evidence.
- Policy hooks for uncertainty, urgency, attention, friction, and deferral.
- Fixtures for bounded candidate shifts and high-risk review requirements.

Required validation:

- Affect signals do not claim hidden emotions or subjective experience.
- High-risk contexts preserve review policy.

## WP-14: Cultivating Intelligence

Required outputs:

- Cultivation contract for restraint, reasonableness, reality contact, moral
  participation, and learning posture.
- Review criteria and fixtures.
- Links to v0.91.1 capability and intelligence architecture boundaries.

Required validation:

- Cultivation evidence remains reviewable and trace-linked.
- Claims do not absorb v0.91.1 aptitude, intelligence, ToM, or memory work.

## WP-15: Structured Planning And SRP Workflow Surfaces

Required outputs:

- Structured Plan Prompt artifact definition and template.
- Plan-review workflow and freshness rules.
- Planning skill proposal or implementation slice.
- Structured Review Policy artifact definition and reviewer-policy binding.
- Validator and card-bundle integration plan or implementation slice.

Required validation:

- SPP and SRP artifacts are durable issue-bundle records.
- Plans are reviewable before execution and reconcile to SOR closeout truth.
- SRP links review scope, evidence requirements, refusal policy, and non-claims.

## WP-16: Secure Agent Comms Substrate And A2A Boundary

Required outputs:

- Secure local Agent Communication and Invocation Protocol substrate slice.
- Identity-bound message envelope, invocation, visibility, redaction, and trace
  surfaces.
- A2A adapter boundary doc and fixtures showing Agent Card ingestion,
  capability mapping, trust classification, and agent.invoke enforcement.
- Explicit deferral of external transport until TLS or mutual-TLS-equivalent
  protection is available.

Required validation:

- Local messages remain inside the polis/substrate boundary.
- External or cross-polis communication fails closed without required transport
  security.
- A2A maps into ADL governance rather than creating a parallel authority model.

## WP-17: Cognitive-Being Flagship Demo

Required outputs:

- Runnable flagship demo showing moral trace, anti-harm, wellbeing, kindness,
  affect/reframing, moral resources, structured planning/review, and secure
  local comms where applicable.
- Demo proof packet with artifacts, non-claims, and replay instructions.

Required validation:

- Demo produces reviewable artifacts, not only narrative docs.
- Demo avoids birthday, legal personhood, production moral agency, and
  cross-polis communication claims.

## WP-18: Demo Matrix And Feature Proof Coverage

Required outputs:

- Demo matrix rows for v0.91 features.
- Feature proof coverage record tying claims to demos, tests, fixtures, docs, or
  explicit deferrals.
- Non-proving demo classification where appropriate.

Required validation:

- Every milestone feature has a proof status.
- No release claim lacks a proof or explicit deferral.

## WP-19: Coverage / Quality Gate

Required outputs:

- Quality gate record for tests, coverage, docs validation, demos, review
  readiness, and known exceptions.
- Runtime/cost posture for any heavy proof lanes.

Required validation:

- Required checks are recorded with pass/fail/skip truth.
- Exceptions are explicit and assigned.

## WP-20: Docs + Review Pass

Required outputs:

- Review-ready milestone docs.
- README, changelog, feature list, release notes, Cargo metadata, and milestone
  package alignment where relevant.
- Claim-boundary review for moral, wellbeing, comms, SPP/SRP, A2A, and v0.91.1
  deferral language.

Required validation:

- Active-line docs do not contain stale prior-milestone truth.
- v0.91.1 and v0.92/v0.93 boundaries remain explicit.

## WP-21: Internal Review

Required outputs:

- Internal review packet.
- Findings register.
- External review handoff draft.
- Remediation queue draft.

Required validation:

- Findings are evidence-backed and severity-classified.
- Review packet names residual risks and non-claims.

## WP-22: External / 3rd-Party Review

Required outputs:

- Third-party review handoff using the established filename pattern.
- External review results archived in the review directory.
- Release readiness update with review outcome.

Required validation:

- Handoff uses final reviewed docs and no host-local paths.
- Findings are captured in machine-readable Markdown, not only PDF.

## WP-23: Review Findings Remediation

Required outputs:

- Accepted-finding remediation or explicit no-finding/no-remediation record.
- Follow-up issues for deferred or out-of-scope findings.
- Release readiness update.

Required validation:

- Every accepted finding is fixed or dispositioned.
- No hidden remediation work remains before next milestone planning.

## WP-24: Next Milestone Planning

Required outputs:

- v0.91.1 adjacent-systems handoff.
- v0.92 birthday and v0.93 constitutional-governance boundary handoff.
- TBD/backlog disposition update for any v0.91 findings.
- Next milestone WP candidate package where appropriate.

Required validation:

- v0.91.1 owns capability testing, intelligence architecture, ANRM/Gemma, ToM,
  memory/identity, runtime-v2/polis docs, ACIP hardening, and model comparison
  report work.
- v0.92 and v0.93 remain downstream consumers rather than absorbed scope.

## WP-25: Release Ceremony

Required outputs:

- Final release notes.
- Release evidence index.
- End-of-milestone report.
- Tag and release ceremony record.
- Next handoff confirmation.

Required validation:

- Ceremony script preflight passes.
- Tag and release publication are verified when the milestone is released.
- The next milestone package is ready enough for immediate start.
