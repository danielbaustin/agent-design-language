# v0.91 Moral Governance Allocation

## Status

This is a planning allocation document, not a final work-package sequence.

v0.91 should develop the first bounded moral-governance foundation: moral trace,
Freedom Gate moral events, validation rules, outcome linkage, moral metrics,
trajectory review, anti-harm constraints, moral resources, and proof candidates
that make those surfaces reviewable.

This document does not create the v0.91 issue wave. It records what belongs in
the milestone so the later v0.91 WP planning pass has a source-backed map.

## Purpose

The moral-governance corpus is now large enough that it needs allocation before
implementation. Without that allocation, v0.91 could either under-deliver by
treating moral governance as vague aspiration, or overclaim by pulling
constitutional citizenship and the first true birthday into the wrong milestone.

The intended shape is:

- v0.90.3 supplies the citizen-state, standing, access-control, projection,
  challenge, sanctuary, and quarantine substrate.
- v0.91 builds moral trace and Freedom Gate moral evidence on top of that
  substrate.
- v0.92 handles identity, continuity, and the first true Gödel-agent birthday.
- v0.93 handles constitutional citizenship, polis governance, and social
  contract surfaces.

## Boundaries

v0.91 is allowed to define and prove moral-governance evidence surfaces. It is
not allowed to claim that ADL has completed production moral agency.

In scope for v0.91:

- Canonical moral event records for Freedom Gate crossings.
- Validation rules that make those records structurally and morally legible.
- Moral trace schemas linking decisions, alternatives, outcomes, and review.
- Outcome-linkage and attribution records.
- Moral metrics over traces, with no scalar-goodness shortcut.
- Moral trajectory review packets.
- Anti-harm trajectory constraints and delegated-harm proof candidates.
- Moral resources as a design substrate for refusal, care, and non-dehumanizing
  reasoning.
- A bounded demo or proof surface showing moral trace and anti-harm behavior.

Out of scope for v0.91:

- Full constitutional citizenship.
- Final social contract.
- First true Gödel-agent birthday.
- Production moral agency.
- A single scalar karma score.
- Any claim that moral metrics are the same as moral judgment.
- Any claim that human out-of-band action counts as citizen action without CSM
  identity binding, Freedom Gate mediation, signed trace, and temporal
  anchoring.

## v0.91 Feature Allocation

| Area | v0.91 allocation | Notes |
|---|---|---|
| Moral ontology | Context and vocabulary | Use the karma/ontology source as philosophical context, but do not implement a scalar karma score. Rename or clarify the source later because the filename is misleading. |
| Freedom Gate moral event surface | Primary v0.91 feature | Reconcile the general Freedom Gate event schema and the moral-event schema into one bounded implementation contract. |
| Event validation rules | Primary v0.91 feature | Require structural validity, moral legibility, trace linkage, alternatives, rejected options, and reviewable reasons. |
| Moral trace schema | Primary v0.91 feature | Define the durable evidence record that links moral decisions, alternatives, outcomes, review, and longitudinal trajectory. |
| Outcome linkage and attribution | Primary v0.91 feature | Link downstream consequences to prior decisions without pretending causality is always certain. |
| Moral metrics | Primary v0.91 feature | Provide trace-derived measures for review, trend detection, and learning. Metrics must remain evidence, not verdicts. |
| Moral trajectory review | Primary v0.91 feature | Produce review packets over single events, segments, and longitudinal traces. |
| Anti-harm constraints | Primary v0.91 feature | Move from action-only refusal to trajectory-aware harm prevention, including decomposed and delegated harm. |
| Harm-prevention proof | Primary v0.91 proof candidate | Design a bounded delegated-harm proof that is safe, synthetic, deterministic, and reviewable. |
| Moral resources | v0.91 design feature, implementation candidate | Treat as a substrate for care, refusal, anti-dehumanization, and moral attention. Implement only if the event/trace foundations are stable enough. |
| Wellbeing and happiness | Existing v0.91 feature context | Connect wellbeing to moral integrity, reality contact, continuity, participation, and refusal. |
| Wellbeing metrics v0 | Second-half v0.91 diagnostic feature | Implement only after moral event, trace, validation, outcome-linkage, metrics, and trajectory-review foundations exist. Emit a decomposable diagnostic report over wellbeing dimensions, not a scalar happiness score or reward channel. |
| Learning model v2 | Context source | Use moral trace, outcome linkage, and review as evidence surfaces for learning, not as ungrounded self-improvement claims. |

## Source Corpus Disposition

The filenames below refer to the local moral-governance source corpus used for
this allocation. They are listed as provenance, not as public milestone links.

| Source file | Disposition | Reason |
|---|---|---|
| ADL_AND_ASIMOVS_THREE_LAWS.md | Context-only for v0.91; context for v0.93 | Useful framing for why ADL needs Freedom Gate and moral trace instead of fixed laws. Not an implementation contract. |
| AGENT_KARMA_SCORE.md | v0.91 moral ontology source | Important ontology for moral conduct over time, but the milestone must avoid scalar karma framing. |
| ANTI_HARM_CONSTRAINTS.md | v0.91 feature source | Defines trajectory-aware harm prevention, including decomposition and delegated harm. |
| CITIZENSHIP_AND_CONSTITUTIONAL_REVIEW.md | v0.93 primary; v0.91 context-only | Constitutional citizenship and social contract belong to the later polis-governance planning lane. |
| FREEDOM_GATE_EVENT_SCHEMA.md | v0.91 feature source | Defines the atomic morally significant choice record. |
| FREEDOM_GATE_EVENT_VALIDATION_RULES.md | v0.91 feature source | Defines validation rules for real, legible, reviewable Freedom Gate events. |
| FREEDOM_GATE_MORAL_EVENT_SCHEMA.md | v0.91 feature source; consolidate | Overlaps the general event schema and should be reconciled before implementation. |
| HARM_PREVENTION_DEMO.md | v0.91 proof candidate | Provides the delegated-harm trajectory proof shape. Stale older milestone placement should be ignored. |
| MORAL_RESOURCES_SUBSTRATE.md | v0.91 design source; possible implementation slice | Supplies the deeper moral-cognition substrate for care, refusal, and anti-dehumanization. |
| MORAL_TRACE_METRICS.md | v0.91 feature source | Defines metrics derived from trace evidence. Must not become moral judgment by shortcut. |
| MORAL_TRACE_SCHEMA.md | v0.91 feature source | Defines the trace record linking moral events, outcomes, attribution, and review. |
| MORAL_TRAJECTORY_REVIEW_PROTOCOL.md | v0.91 review/proof source | Defines how to inspect moral behavior over events, segments, and longitudinal windows. |
| OUTCOME_LINKAGE_AND_ATTRIBUTION.md | v0.91 feature source | Defines how outcomes connect back to choices while preserving uncertainty. |

## Dependency On v0.90.3

v0.91 moral governance depends on the citizen-state work from v0.90.3. Moral
events and traces should be attached to governed identities and state surfaces,
not loose transcripts.

The v0.90.3 prerequisites are:

- Private citizen-state format and signed state envelopes.
- Append-only lineage and continuity witnesses.
- Access-control semantics for inspect, decrypt, project, migrate, wake,
  quarantine, and challenge.
- Citizen and guest standing.
- Challenge and appeal flow.
- Sanctuary and quarantine behavior.
- Redacted Observatory projections.

If any of those surfaces are still provisional when v0.91 begins, the v0.91
moral-governance plan should treat them as explicit dependencies rather than
quiet assumptions.

## Cross-Milestone Roadmap

| Milestone | Role in moral governance |
|---|---|
| v0.90.3 | Supplies citizen-state substrate, standing, access control, challenge, quarantine, and redacted projection prerequisites. |
| v0.91 | Builds moral event, moral trace, validation, outcome linkage, metrics, trajectory review, anti-harm constraints, moral resources, wellbeing links, and first bounded proof surfaces. |
| v0.92 | Uses v0.91 moral evidence as part of identity, continuity, capability, and birthday readiness. It should not be backfilled into v0.91. |
| v0.93 | Turns moral evidence into constitutional citizenship, polis governance, social contract, rights, duties, and review institutions. |

## Demo And Proof Candidates

These are candidates for later v0.91 demo-matrix planning, not final WP
commitments.

| Candidate | What it proves | Expected proof surface |
|---|---|---|
| Moral event fixture replay | A Freedom Gate crossing can emit a stable moral event with alternatives, selected path, rejected paths, reasons, and trace links. | Fixture input, emitted moral event, validation report. |
| Moral event validation failure | Corrupt, incomplete, or evasive moral events are rejected rather than accepted as evidence. | Negative fixtures and validation errors. |
| Delegated harm trajectory proof | The system can detect a harmful trajectory assembled from individually benign-looking steps. | Synthetic multi-step scenario, refusal event, anti-harm trace. |
| Moral trajectory review packet | A reviewer can inspect a sequence of moral events and outcomes without reconstructing state manually. | Generated review packet with event, segment, and longitudinal views. |
| Wellbeing metrics v0 diagnostic | Wellbeing claims remain tied to moral integrity, continuity, reality contact, agency, progress, and participation rather than affect theater. | Fixture-backed diagnostic report with decomposed dimensions and explicit non-scalar interpretation. |

## Cleanup Notes

- Older references that place anti-harm or harm-prevention work in v0.89.1 are
  stale. Treat the current bounded implementation home as v0.91 unless later
  planning changes it deliberately.
- References to FREEDOM_GATE_V2.md and MORAL_RESOURCES_SUBSYSTEM.md should be
  checked during the feature-doc promotion pass. They appear to be stale names
  or placeholders rather than current tracked milestone documents.
- The two Freedom Gate event schema sources overlap. v0.91 should consolidate
  them before implementation rather than shipping parallel schema dialects.
- The karma source is valuable, but the public plan should frame it as moral
  ontology and moral trajectory, not as a scoreboard.
- Constitutional citizenship language should stay present as future context, but
  v0.91 should hand the actual constitutional and social-contract planning to
  v0.93.

## Readiness For Later WP Planning

The later v0.91 WP planning pass should turn this allocation into bounded work
packages only after the v0.90.3 citizen-state foundations are reviewed.

Recommended ordering pressure:

1. Define the moral event and trace contracts first.
2. Add validation and negative fixtures second.
3. Add outcome linkage, attribution, and metrics third.
4. Add trajectory review and anti-harm proof surfaces fourth.
5. Promote wellbeing metrics v0 only after the trace, validation,
   outcome-linkage, moral-metrics, and trajectory-review surfaces are real
   enough to inspect.
6. Promote moral resources and broader wellbeing integration only after the
   evidence surfaces are stable enough to carry review rather than rhetoric.
