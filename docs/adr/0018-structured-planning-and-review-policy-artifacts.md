# ADR 0018: Structured Planning And Review Policy Artifacts

- Status: Accepted
- Date: 2026-05-07
- Related milestone: v0.91
- Related release line: v0.91.0
- Builds on: ADR 0015, ADR 0017
- Refined by: ADR 0024, ADR 0028

## Context

ADL already treats issue intent, selected task intent, and outcome truth as
durable issue-bundle artifacts through `SIP`, `STP`, and `SOR`. v0.91 adds two
adjacent workflow artifacts:

- `SPP`, the Structured Plan Prompt, for saved execution planning
- `SRP`, the Structured Review Prompt, for durable review policy

v0.91.2 refines the issue-card semantics into the canonical lifecycle:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

In that refined lifecycle, `SIP` is the Structured Issue Prompt, `STP` is the
selected task/solution prompt, and `SRP` remains Structured Review Prompt while
expanding from policy-only into both review instructions and review results.
This is a lifecycle clarification rather than a rejection of the v0.91 decision
to make `SPP` and `SRP` durable workflow artifacts.

This ADR is grounded in:

- `docs/milestones/v0.91/SPP_READINESS_v0.91.md`
- `docs/milestones/v0.91/features/STRUCTURED_PLANNING_AND_PLAN_REVIEW.md`
- `docs/milestones/v0.91/features/STRUCTURED_REVIEW_POLICY_AND_SRP.md`
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `adl/templates/cards/`
- `adl/src/cli/tooling_cmd/structured_prompt.rs`
- `adl/src/cli/tooling_cmd/tests/structured_prompt.rs`
- `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`

## Decision

ADL adopts durable `SPP` and `SRP` artifacts as first-class workflow surfaces in
v0.91.

This decision requires:

1. Plans must be saved, not only spoken in chat.

   A tracked execution plan should be recordable as `spp.md` inside the issue
   bundle. It should preserve the goal, sequence, assumptions, touched surfaces,
   validation, proof expectations, delegation boundaries, risks, stop
   conditions, and replan triggers.

2. Review policy must be saved, not only implied by reviewer prompts.

   A tracked review policy should be recordable as `srp.md` inside the issue
   bundle. It should preserve review mode, lifecycle timing, scope basis,
   evidence classes, validation context, allowed dispositions, constraints, and
   follow-up routing.

3. `SPP` and `SRP` complement rather than replace `SIP`, `STP`, and `SOR`.

   In the refined lifecycle, `SIP` records issue intent, `STP` records the
   selected task or solution, and `SOR` remains outcome truth. `SPP` records
   intended execution discipline. `SRP` records review instructions and the
   resulting findings, dispositions, residual risks, and recommended outcome.

4. The artifacts must be validator-visible and editor-safe.

   Malformed planning or review-prompt artifacts should be detectable by the
   structured-prompt validation path, and editor skills should be able to
   normalize them without inventing execution progress.

5. Full workflow automation can follow after the artifact contract.

   `pr-plan`, mandatory plan-review gates, stale-plan detection, and automatic
   reviewer-agent consumption are follow-on workflow hardening. The v0.91
   decision is to make the artifacts durable and reviewable first.

## Rationale

The project has repeatedly seen that chat-only plans are easy to lose and
review-only instructions are easy to misapply. ADL's own workflow works best
when intent, context, and output truth are stored as artifacts. Planning and
review policy should follow the same pattern.

This also makes ACIP reviewer invocation safer: a reviewer-agent can reference
a durable `srp_ref` rather than relying only on session text.

## Consequences

### Positive

- Makes planning quality inspectable before execution.
- Gives review policy a durable artifact rather than a loose prompt.
- Improves handoff, delegation, and reviewer invocation discipline.
- Preserves compatibility with existing `STP`, `SIP`, and `SOR` lifecycle
  records.

### Negative

- Issue bundles now have more artifacts to keep truthful.
- Future lifecycle tooling must distinguish missing, stale, reviewed, and
  unreviewed plans/policies instead of treating existence as enough.

## Alternatives Considered

### 1. Keep plans in chat only

This is fast, but it loses the plan when context rolls off and makes review
harder.

### 2. Fold planning into `STP`

This would reduce file count, but it would blur source intent with an
execution-agent's chosen implementation plan.

### 3. Fold review policy into `SOR`

This would place policy after execution, which is too late for pre-PR reviewer
discipline.

## Validation Evidence

The decision is supported by:

- the v0.91 SPP readiness record
- the structured planning and SRP feature docs
- structured-prompt validation support for `SPP` and `SRP`
- demo matrix and feature-proof coverage entries showing `SPP` and `SRP` in the
  cognitive-being flagship proof surface

## Non-Claims

This ADR does not claim:

- that planning eliminates judgment or replanning
- that `SRP` alone guarantees review quality
- that `pr-plan` or mandatory plan-review gates are complete
- that reviewer-agent automation is fully productionized
