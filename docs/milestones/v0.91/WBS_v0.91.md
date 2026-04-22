# v0.91 Candidate Work Breakdown Structure

## Status

Candidate allocation only. v0.91 has no final issue wave yet.

The exact WP sequence should be produced by the v0.91 WP-01 planning pass after
v0.90.3 citizen-state and v0.90.5 governed-tool prerequisites are stable enough
to consume where relevant.

## WBS Summary

v0.91 should develop moral governance and wellbeing foundations without stealing
work from identity, birthday, constitutional citizenship, reputation, or Theory
of Mind milestones.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary Deliverable | Key Dependencies |
| --- | --- | --- | --- | --- |
| A | Moral event contract | Define the Freedom Gate moral event shape and required evidence. | Moral event feature contract and fixtures. | v0.90.3 identity/standing, Freedom Gate context. |
| B | Moral event validation | Reject incomplete, evasive, or unreviewable moral events. | Validation rules and negative fixtures. | A. |
| C | Moral trace schema | Link choices, alternatives, refusals, outcomes, attribution, and review. | Trace schema and examples. | A, B. |
| D | Outcome linkage and attribution | Preserve uncertainty while connecting downstream consequences to choices. | Outcome-linkage record and tests. | C. |
| E | Moral metrics | Produce trace-derived signals for review and trend detection. | Metric definitions and fixture report. | C, D. |
| F | Moral trajectory review | Review event sequences over segments and longitudinal windows. | Trajectory review packet. | C, D, E. |
| G | Anti-harm trajectory constraints | Detect decomposed or delegated harm across steps. | Synthetic delegated-harm proof packet. | C through F. |
| H | Moral resources | Model care, refusal, attention, and anti-dehumanization as design resources. | Design contract and implementation slice if stable. | A through G. |
| I | Wellbeing metrics v0 | Emit a decomposable diagnostic over coherence, agency, continuity, progress, moral integrity, and participation. | Diagnostic report and policy views. | C through F. |
| J | Moral governance demo | Show moral event, validation, trace, outcome linkage, trajectory review, anti-harm, and wellbeing evidence. | Runnable proof demo and artifacts. | A through I. |
| K | Demo matrix and proof coverage | Align demos with milestone claims and non-claims. | Demo matrix rows and validation commands. | J. |
| L | Review, docs, and release tail | Align docs, update feature list, run review, fix findings, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Define moral event, validation, and trace contracts first.
2. Add outcome linkage and attribution.
3. Add metrics and trajectory review.
4. Add anti-harm proof surfaces.
5. Add wellbeing metrics only after trace and review surfaces exist.
6. Add moral resources only after the evidence layer can carry them.
7. Build demos and review packets last.

## Acceptance Mapping

- Moral events must be attached to governed identities and trace.
- Validation must reject incomplete or evasive evidence.
- Outcome linkage must preserve uncertainty.
- Metrics must remain evidence, not judgment.
- Wellbeing diagnostics must remain decomposed, self-accessible to the citizen,
  and policy-mediated for others.
- Anti-harm proof must show a harmful trajectory, not just a forbidden action.
- v0.92 birthday and v0.93 constitutional governance must consume v0.91 evidence
  rather than being pulled into v0.91.
