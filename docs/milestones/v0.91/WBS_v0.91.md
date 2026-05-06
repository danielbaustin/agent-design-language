# v0.91 Work Breakdown Structure

## Status

Active issue-wave allocation with a tracked YAML, card-authoring readiness
document, and opened GitHub issues #2735-#2759.

The official WP sequence is recorded in
[WP_ISSUE_WAVE_v0.91.yaml](WP_ISSUE_WAVE_v0.91.yaml). WP-01 owns the first
execution pass over this package and should keep the issue cards aligned with
the concrete readiness sections from
[WP_EXECUTION_READINESS_v0.91.md](WP_EXECUTION_READINESS_v0.91.md).

## WBS Summary

v0.91 should develop moral governance, wellbeing, secure intra-polis
communication, and first-class cognitive-being foundations without stealing work
from Runtime v2, identity, birthday, constitutional citizenship, reputation, or
Theory of Mind milestones.

## Work Areas

| Area | Work Area | Description | Primary Deliverable | Key Dependencies |
| --- | --- | --- | --- | --- |
| A | Moral event contract | Define the Freedom Gate moral event shape and required evidence. | Moral event feature contract and fixtures. | v0.90.3 identity/standing, Freedom Gate context. |
| B | Moral event validation | Reject incomplete, evasive, or unreviewable moral events. | Validation rules and negative fixtures. | A. |
| C | Moral trace schema | Link choices, alternatives, refusals, outcomes, attribution, and review. | Trace schema and examples. | A, B. |
| D | Outcome linkage and attribution | Preserve uncertainty while connecting downstream consequences to choices. | Outcome-linkage record and tests. | C. |
| E | Moral metrics | Produce trace-derived signals for review and trend detection. | Metric definitions and fixture report. | C, D. |
| F | Moral trajectory review | Review event sequences over segments and longitudinal windows. | Trajectory review packet. | C, D, E. |
| G | Anti-harm trajectory constraints | Detect decomposed or delegated harm across steps. | Synthetic delegated-harm proof packet. | C through F. |
| H | Moral resources | Model care, refusal, attention, and anti-dehumanization as implemented design resources. | Design contract, fixtures, and reviewable implementation surface. | A through G. |
| I | Wellbeing metrics | Emit a decomposable diagnostic over coherence, agency, continuity, progress, moral integrity, and participation. | Diagnostic report and policy views. | C through F. |
| J | Kindness model | Make kindness inspectable under conflict as non-harm, dignity, autonomy, constructive benefit, and long-horizon support. | Kindness contract and fixture set. | C through I. |
| K | Humor and absurdity | Add bounded frame detection and reframing without entertainment or manipulation claims. | Reframing event and negative fixtures. | C through I. |
| L | Affect reasoning-control surface | Represent affect-like signals as explicit reasoning-control evidence, not hidden emotion claims. | Affect signal record and policy hooks. | C through I. |
| M | Cultivating intelligence | Define formation evidence for restraint, reasonableness, reality contact, and moral participation. | Cultivation contract and review criteria. | C through L. |
| N | Secure Agent Comms prerequisites | Implement secure local ACIP envelope, invocation, trace, visibility, redaction, structured-planning/SRP policy targets, and A2A-adapter planning boundaries. | ACIP v1 substrate feature with v0.91.1 hardening continuation if needed. | v0.90.5 governed tools, C through F. |
| O | Cognitive-being flagship demo | Show moral governance, wellbeing, kindness, affect/reframing, moral resources, and secure local comms as one reviewable proof story. | Runnable proof demo and artifacts. | A through N. |
| P | Demo matrix and proof coverage | Align demos with milestone claims, non-claims, and the v0.91.1 adjacent-systems completion lane. | Demo matrix rows and validation commands. | O. |
| Q | Review, docs, and release tail | Align docs, update feature list, run review, fix findings, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Official WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 (#2735) | Design pass (milestone docs + planning) | docs | tracked docs, reviewed YAML, and issue cards | v0.90.5 closeout |
| WP-02 (#2736) | Moral event contract | docs | moral event feature contract and fixtures | WP-01 (#2735) |
| WP-03 (#2737) | Moral event validation | tools | validation rules and negative fixtures | WP-02 (#2736) |
| WP-04 (#2738) | Moral trace schema | tools | trace schema and examples | WP-02, WP-03 |
| WP-05 (#2739) | Outcome linkage and attribution | runtime | outcome-linkage record and tests | WP-04 (#2738) |
| WP-06 (#2740) | Moral metrics | runtime | metric definitions and fixture report | WP-04, WP-05 |
| WP-07 (#2741) | Moral trajectory review | runtime | trajectory review packet | WP-04-WP-06 |
| WP-08 (#2742) | Anti-harm trajectory constraints | runtime | delegated-harm proof packet | WP-04-WP-07 |
| WP-09 (#2743) | Wellbeing metrics | runtime | decomposed diagnostic report and policy views | WP-04-WP-07 |
| WP-10 (#2744) | Moral resources | runtime | moral-resources contract, fixtures, and implementation surface | WP-05-WP-09 |
| WP-11 (#2745) | Kindness model | runtime | kindness contract and conflict fixtures | WP-05-WP-10 |
| WP-12 (#2746) | Humor and absurdity | runtime | reframing event and negative fixtures | WP-05-WP-10 |
| WP-13 (#2747) | Affect reasoning-control surface | runtime | affect signal record and policy hooks | WP-05-WP-10 |
| WP-14 (#2748) | Cultivating intelligence | runtime | cultivation contract and review criteria | WP-05-WP-13 |
| WP-15 (#2749) | Structured planning and SRP workflow surfaces | tools | SPP/SRP artifacts, planning skill, and review-readiness checks | WP-01 (#2735) |
| WP-16 (#2750) | Secure Agent Comms substrate and A2A boundary | runtime | local ACIP substrate slice plus explicit A2A adapter boundary | WP-04-WP-05, WP-15 |
| WP-17 (#2751) | Cognitive-being flagship demo | demo | runnable proof demo and artifacts | WP-08-WP-16 |
| WP-18 (#2752) | Demo matrix and feature proof coverage | demo | demo matrix rows and proof coverage record | WP-17 (#2751) |
| WP-19 (#2753) | Coverage / quality gate | quality | quality gate and validation posture record | WP-18 (#2752) |
| WP-20 (#2754) | Docs + review pass | docs | review-ready docs package | WP-19 (#2753) |
| WP-21 (#2755) | Internal review | review | internal review record | WP-20 (#2754) |
| WP-22 (#2756) | External / 3rd-party review | review | external review handoff and record | WP-21 (#2755) |
| WP-23 (#2757) | Review findings remediation | review | remediation record and follow-up issues | WP-22 (#2756) |
| WP-24 (#2758) | Next milestone planning | docs | v0.91.1/v0.92/v0.93 handoff record | WP-23 (#2757) |
| WP-25 (#2759) | Release ceremony | release | release evidence, end-of-milestone report, and next handoff | WP-24 (#2758) |

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
- Runtime-v2/polis docs, agent lifecycle states, ACIP state eligibility,
  capability/aptitude testing, intelligence metric architecture, ANRM/Gemma,
  ToM, memory/identity, ACIP/A2A hardening, and observatory-visible inhabitant
  proof should be routed to v0.91.1, not absorbed into v0.91.
- UTS + ACC multi-model benchmarking, runtime/test-cycle recovery, CodeBuddy,
  Google Workspace CMS, code modernization, publication packets, rustdoc/doc
  cleanup, and workflow guardrails should be routed to v0.91.2, not absorbed
  into v0.91 or v0.91.1.
- v0.92 birthday and v0.93 constitutional governance must consume v0.91 evidence
  rather than being pulled into v0.91.
