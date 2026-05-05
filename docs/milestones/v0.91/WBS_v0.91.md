# v0.91 Candidate Work Breakdown Structure

## Status

Reviewed candidate allocation with a tracked candidate issue-wave YAML. v0.91
still has no opened issue wave yet.

The exact WP sequence should be produced by the v0.91 WP-01 planning pass after
v0.90.3 citizen-state and v0.90.5 governed-tool prerequisites are stable enough
to consume where relevant.

## WBS Summary

v0.91 should develop moral governance, wellbeing, secure intra-polis
communication, and first-class cognitive-being foundations without stealing work
from Runtime v2, identity, birthday, constitutional citizenship, reputation, or
Theory of Mind milestones.

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
| H | Moral resources | Model care, refusal, attention, and anti-dehumanization as implemented design resources. | Design contract, fixtures, and reviewable implementation surface. | A through G. |
| I | Wellbeing metrics v0 | Emit a decomposable diagnostic over coherence, agency, continuity, progress, moral integrity, and participation. | Diagnostic report and policy views. | C through F. |
| J | Kindness model | Make kindness inspectable under conflict as non-harm, dignity, autonomy, constructive benefit, and long-horizon support. | Kindness contract and fixture set. | C through I. |
| K | Humor and absurdity | Add bounded frame detection and reframing without entertainment or manipulation claims. | Reframing event and negative fixtures. | C through I. |
| L | Affect reasoning-control surface | Represent affect-like signals as explicit reasoning-control evidence, not hidden emotion claims. | Affect signal record and policy hooks. | C through I. |
| M | Cultivating intelligence | Define formation evidence for restraint, reasonableness, reality contact, and moral participation. | Cultivation contract and review criteria. | C through L. |
| N | Secure Agent Comms prerequisites | Implement secure local ACIP envelope, invocation, trace, visibility, redaction, structured-planning/SRP policy targets, and A2A-adapter planning boundaries. | ACIP v1 substrate feature with v0.91.1 hardening continuation if needed. | v0.90.5 governed tools, C through F. |
| O | Cognitive-being flagship demo | Show moral governance, wellbeing, kindness, affect/reframing, moral resources, and secure local comms as one reviewable proof story. | Runnable proof demo and artifacts. | A through N. |
| P | Demo matrix and proof coverage | Align demos with milestone claims, non-claims, and the v0.91.1 adjacent-systems completion lane. | Demo matrix rows and validation commands. | O. |
| Q | Review, docs, and release tail | Align docs, update feature list, run review, fix findings, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Define moral event, validation, and trace contracts first.
2. Add outcome linkage and attribution.
3. Add metrics and trajectory review.
4. Add anti-harm proof surfaces.
5. Add wellbeing metrics only after trace and review surfaces exist.
6. Add moral resources, kindness, humor/absurdity, affect, and cultivating
   intelligence after the evidence layer can carry them.
7. Add secure Agent Comms where it is needed for review, handoff, invocation,
   and demo proof; defer conformance expansion and adjacent-system alignment to
   v0.91.1 if necessary.
8. Build demos and review packets last.

## Acceptance Mapping

- Moral events must be attached to governed identities and trace.
- Validation must reject incomplete or evasive evidence.
- Outcome linkage must preserve uncertainty.
- Metrics must remain evidence, not judgment.
- Wellbeing diagnostics must remain decomposed, self-accessible to the citizen,
  and policy-mediated for others.
- Anti-harm proof must show a harmful trajectory, not just a forbidden action.
- Kindness, affect, humor/absurdity, cultivated-intelligence, and moral-resource
  features must have implemented contracts, fixtures, and proof surfaces.
- Agent communication must remain local, authenticated, traceable, redacted, and
  external-TLS-gated.
- Structured planning and `SRP` must exist as durable issue-bundle workflow
  artifacts rather than only chat behavior or TBD notes.
- A2A must remain an adapter over the comms substrate, not a second comms
  architecture.
- Capability/aptitude testing, intelligence metric architecture, ANRM/Gemma,
  ToM, memory/identity, and runtime-v2/polis docs should be routed to v0.91.1,
  not absorbed into v0.91.
- v0.92 birthday and v0.93 constitutional governance must consume v0.91 evidence
  rather than being pulled into v0.91.
