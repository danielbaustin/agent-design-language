# v0.8 Third-Party Review Pass

This document records the formal third-party-style review pass for v0.8.

It is a review artifact only. It does not implement fixes.

## Reviewed Scope

Reviewed materials:

- milestone navigation and baseline docs (`README.md`, execution/ordering/boundary docs)
- schema/workflow surfaces (`ExperimentRecord`, `Evidence View`, `Mutation`, `EvaluationPlan`, workflow template)
- ObsMem indexing surfaces and ToolResult contract docs
- demo matrix and quality-gate docs (`DEMOS_V0.8.md`, `QUALITY_GATE_V0.8.md`)
- docs convergence handoff (`DOCS_CONVERGENCE_V0.8.md`)

## Review Criteria

1. Determinism and replay-surface clarity
2. Security/privacy hygiene in docs artifacts
3. Scope discipline (bounded v0.8 vs deferred work)
4. Cross-doc consistency and navigation coherence
5. Follow-up actionability for release-tail execution

## Findings (Deterministic Order)

Ordered by severity (`blocker`, `high`, `medium`, `low`) then finding ID.

| Finding ID | Severity | Area | Summary | Recommended Action | Target |
|---|---|---|---|---|---|
| R-01 | medium | Demo evidence closure | Demo matrix defines evidence surfaces but completion state must be explicitly confirmed against implemented artifacts before ceremony. | In `#708`, validate each required demo row (`D8-01`..`D8-05`) against concrete evidence and mark pass/fail. | pre-release |
| R-02 | medium | Quality gate execution proof | Quality-gate policy is documented; execution proof must be captured in release-tail output artifacts. | In `#708`, attach command/status evidence for required pre-release checks and unresolved exceptions. | pre-release |
| R-03 | low | Cross-doc reference resilience | Some planning docs depend on issue-number references that may drift if issue scope is retitled. | In `#708`, add a short note in release-tail docs mapping issue IDs to final merged artifacts. | pre-release |

## Blocker Classification

- Blockers: **none identified** at docs/review-artifact stage.
- High-risk non-blockers: none.
- Medium items requiring follow-through before ceremony: `R-01`, `R-02`.

## Handoff to #708 (Review Findings / Fixes)

Required `#708` inputs from this review:

1. Resolve or explicitly defer each finding ID with owner + rationale.
2. Produce concrete evidence for required demo/gate completion states.
3. Update release-tail docs to reflect final resolution state.

## Handoff to #709 (Release Ceremony)

`#709` should proceed only when:

1. `R-01` and `R-02` are resolved or explicitly accepted with risk sign-off.
2. No blocker-grade findings are open.
3. Release docs reflect shipped state and decision trail.

## Out of Scope

- implementing findings directly in this review pass,
- adding new v0.8 features,
- redefining accepted schema contracts.
