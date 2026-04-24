# Architecture Document Generation Plan

## Goal

Generate first-class ADL architecture documentation from bounded repository
evidence, specialist review artifacts, and explicit validation gates. The output
must be reviewable by humans and safe to track publicly.

## Inputs

- Runtime source: `adl/src`
- Tooling source: `adl/tools`
- Demo source: `demos`
- Milestone source: `docs/milestones/v0.90`
- Existing architecture source: `docs/architecture`
- Workflow source: `docs/default_workflow.md`
- Closeout and package sources: `docs/milestones/v0.90.4`, `docs/reviews`,
  `CHANGELOG.md`
- Operational skills source:
  `docs/milestones/v0.89.1/features/OPERATIONAL_SKILLS_SUBSTRATE.md`
- Skill composition source:
  `docs/milestones/v0.89.1/features/SKILL_COMPOSITION_MODEL.md`

## Skill Order

1. Use the conductor to identify lifecycle state and route to `pr-run`.
2. Use source inspection to build a bounded evidence map.
3. Use `repo-architecture-review` to identify architecture boundaries and
   risks.
4. Use `security-threat-model` or a bounded threat-boundary review for trust
   boundary notes.
5. Use `diagram-author` to create diagram sources from the evidence map.
6. Use `architecture-fitness-function-author` to define machine-checkable
   invariants and human judgment gates.
7. Use `pr-finish` to publish the result for review.
8. Use `pr-janitor` and `pr-closeout` after publication and merge.

## Output Template

Every generated architecture packet should include:

- Canonical architecture narrative.
- Diagram packet with evidence, assumptions, and validation commands.
- Review automation plan.
- Document-generation plan.
- Candidate ADR list.
- Proof/demo artifact.
- Validation script or command.
- Residual-risk and missing-skill notes.

## Milestone Refresh Integration

This document should be reviewed at each milestone boundary.

- If an architecture claim, `ADL_ARCHITECTURE.md`, or architecture-support doc
  changed with accepted work in that milestone, update this document and its
  generated packet in the same cycle.
- If architecture did not change materially, explicitly record
  `architecture-reviewed-unchanged` in the milestone planning/closeout artifacts
  rather than silently skipping the review surface.
- The unchanged path is valid only when the reviewing owner, reviewed file set, and
  rationale are recorded.
- If unchanged is claimed while supporting docs remain untouched, explicitly
  confirm the canonical diagram packet is still complete for this milestone and
  note any non-urgent omissions as follow-up items.
- Update ADR-facing support docs whenever architecture decision logic is adjusted
  and capture that change in the milestone handoff artifacts.

## Diagram Template

Every diagram should have:

- Diagram id and source path.
- Purpose.
- Evidence paths.
- Assumptions.
- Render or validation command.
- Unsupported claims explicitly excluded.

## Review Template

Architecture reviews should use a findings-first format:

- Finding title with severity.
- Affected path or architecture surface.
- Trigger scenario.
- Evidence.
- User or operator impact.
- Recommended action.
- Validation or proof gap.
- Residual risk.

## Machine Validation

The first validation surface is:

```bash
python3 adl/tools/validate_architecture_docs.py
```

The first deterministic proof packet is:

```bash
bash adl/tools/demo_v090_architecture_document_generation.sh
```

## Deferred Dependencies

- Documentation specialist issue `#2042` should improve wording, navigation,
  onboarding, and public doc quality.
- Gap analysis issue `#2044` should compare the packet against implementation,
  demos, release state, and closeout records.
- Permanent closeout fixes should ensure generated architecture packets do not
  become stale after PR merge.

## Non-Goals

- No direct publication to external sites.
- No acceptance of ADRs without human review.
- No claim that all diagrams are complete system specifications.
- No live model calls required for the default proof packet.
- No use of private local traces or host-specific worktree paths as public
  evidence.
