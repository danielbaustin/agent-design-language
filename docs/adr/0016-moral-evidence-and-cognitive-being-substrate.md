# ADR 0016: Moral Evidence And Cognitive-Being Substrate

- Status: Accepted
- Date: 2026-05-07
- Related milestone: v0.91
- Related release line: v0.91.0
- Builds on: ADR 0012, ADR 0013, ADR 0015

## Context

v0.91 turns ADL's moral-governance and cognitive-being language into bounded
runtime evidence. Before this milestone, ADL had Runtime v2, citizen-state
continuity, governed tools, and reviewable traces, but it did not have one
accepted architecture record for moral events, moral traces, outcome linkage,
trajectory review, wellbeing diagnostics, kindness, affect-like control, moral
resources, and cultivation posture.

This ADR is grounded in:

- `docs/milestones/v0.91/README.md`
- `docs/milestones/v0.91/COGNITIVE_BEING_FEATURES_v0.91.md`
- `docs/milestones/v0.91/MORAL_GOVERNANCE_ALLOCATION_v0.91.md`
- `docs/milestones/v0.91/DEMO_MATRIX_v0.91.md`
- `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- `docs/milestones/v0.91/QUALITY_GATE_v0.91.md`
- `docs/milestones/v0.91/features/MORAL_EVENT_CONTRACT.md`
- `docs/milestones/v0.91/features/MORAL_TRACE_SCHEMA.md`
- `docs/milestones/v0.91/features/OUTCOME_LINKAGE_AND_ATTRIBUTION.md`
- `docs/milestones/v0.91/features/MORAL_METRICS.md`
- `docs/milestones/v0.91/features/MORAL_TRAJECTORY_REVIEW.md`
- `docs/milestones/v0.91/features/ANTI_HARM_TRAJECTORY_CONSTRAINTS.md`
- `docs/milestones/v0.91/features/WELLBEING_AND_HAPPINESS.md`
- `docs/milestones/v0.91/features/KINDNESS.md`
- `docs/milestones/v0.91/features/HUMOR_AND_ABSURDITY.md`
- `docs/milestones/v0.91/features/AFFECT_REASONING_CONTROL.md`
- `docs/milestones/v0.91/features/MORAL_RESOURCES.md`
- `docs/milestones/v0.91/features/CULTIVATING_INTELLIGENCE.md`
- `adl/src/runtime_v2/moral_event_validation.rs`
- `adl/src/runtime_v2/moral_trace_schema.rs`
- `adl/src/runtime_v2/outcome_linkage_attribution.rs`
- `adl/src/runtime_v2/moral_metrics.rs`
- `adl/src/runtime_v2/moral_trajectory_review.rs`
- `adl/src/runtime_v2/anti_harm_trajectory_constraints.rs`
- `adl/src/runtime_v2/wellbeing_metrics.rs`
- `adl/src/runtime_v2/kindness_model.rs`
- `adl/src/runtime_v2/humor_and_absurdity.rs`
- `adl/src/runtime_v2/affect_reasoning_control.rs`
- `adl/src/runtime_v2/moral_resources.rs`
- `adl/src/runtime_v2/cultivating_intelligence.rs`
- `adl/src/runtime_v2/cognitive_being_flagship_demo.rs`

This ADR records the v0.91 architecture. It does not introduce new runtime
behavior.

## Decision

ADL adopts a moral evidence and cognitive-being substrate for v0.91.

The substrate is evidence-first. It records morally significant decisions,
alternatives, reasons, constraints, affected parties, authority, uncertainty,
and downstream outcomes without pretending that the runtime has final moral
judgment.

This decision requires:

1. Moral events are reviewable evidence records, not moral verdicts.

   Moral events must preserve selected and rejected alternatives, reasons,
   constraints, actor context, affected parties, authority posture, and trace
   references. Validity means the event is reviewable, not that the action was
   morally perfect.

2. Moral traces connect events across time.

   Moral review must be able to inspect ordinary, refusal, delegation, and
   deferred paths without relying on chat reconstruction. Traces preserve
   causality, ordering, provenance, and redaction boundaries.

3. Outcome linkage preserves uncertainty.

   Downstream consequences can be linked to decisions, but attribution must
   keep uncertainty, delegation lineage, and contested-outcome posture visible.

4. Metrics are evidence summaries, not scores.

   Moral metrics must stay decomposed and trace-derived. They must not become
   scalar karma, scalar happiness, hidden reward channels, or final moral
   judgment.

5. Trajectory review and anti-harm constraints are first-class.

   ADL must be able to review patterns over time, including harmful trajectories
   assembled from individually benign-looking steps or delegated actors.

6. Wellbeing diagnostics are private and decomposed.

   Wellbeing evidence belongs to the citizen first. Operator, reviewer, and
   public views must be governed and redacted. The diagnostic is not a public
   reputation score or proof that the system is happy.

7. Kindness, humor/absurdity, affect-like control, moral resources, and
   cultivation posture are engineering surfaces.

   These concepts must appear as reviewable artifacts, validation surfaces, and
   proof packets. They are not prompt tone, theatrical personality, or
   unsupported claims of consciousness.

8. Runtime v2 is inherited as the substrate.

   v0.91 consumes Runtime v2, citizen-state, governed-tool, and trace evidence.
   It does not reopen those foundations or claim the v0.92 birthday.

## Rationale

ADL needs a disciplined way to represent moral and affect-adjacent behavior
without either reducing it to sentiment or overclaiming personhood. v0.91
therefore treats moral cognition as a set of bounded records, validators,
review packets, and demos that future identity, birthday, and constitutional
governance milestones can consume.

This architecture keeps the project ambitious without letting claims outrun
evidence.

## Consequences

### Positive

- Gives v0.91 one durable architecture record for moral evidence and
  cognitive-being foundations.
- Makes moral events, traces, attribution, metrics, wellbeing, and
  cognitive-being surfaces reviewable as engineering artifacts.
- Preserves v0.92 birthday and v0.93 constitutional governance as downstream
  consumers rather than accidental v0.91 claims.
- Gives reviewers a clear boundary between implemented evidence and later
  identity, citizenship, or production-readiness work.

### Negative

- Future changes to moral event shape, trace semantics, wellbeing visibility,
  metric interpretation, or cognitive-being proof posture now carry
  architectural weight.
- Public-facing language must remain careful: v0.91 can claim evidence
  substrate and proof surfaces, not consciousness, legal personhood, or
  production moral agency.

## Alternatives Considered

### 1. Leave the v0.91 substrate documented only by feature docs

This would avoid one ADR, but it would scatter the core architecture across
many files and make review harder.

### 2. Treat moral metrics as the central abstraction

This would be simpler to present, but it would distort the design into a
scoreboard. ADL instead treats metrics as decomposed evidence summaries over
events and traces.

### 3. Defer all cognitive-being surfaces to v0.92

That would keep v0.91 smaller, but it would leave the birthday milestone
without enough moral, wellbeing, and cultivation evidence to consume.

## Validation Evidence

The decision is supported by:

- the v0.91 feature docs and allocation records
- the v0.91 demo matrix and feature-proof coverage map
- Runtime v2 moral, wellbeing, affect, kindness, moral-resource, cultivation,
  and flagship demo modules
- focused Rust validation surfaces named by
  `docs/milestones/v0.91/FEATURE_PROOF_COVERAGE_v0.91.md`
- the v0.91 quality gate recording current CI and coverage evidence

## Non-Claims

This ADR does not claim:

- production moral agency
- legal personhood
- consciousness or subjective feeling
- scalar karma or scalar happiness
- public wellbeing surveillance
- the first true birthday
- constitutional citizenship completion
- durable identity architecture completion
