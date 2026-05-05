# Sprint Plan - v0.91

## Status

Reviewed candidate sprint shape aligned with
[WP_ISSUE_WAVE_v0.91.yaml](WP_ISSUE_WAVE_v0.91.yaml). v0.91 has no opened
GitHub issue wave yet; WP-01 owns that promotion step.

## Candidate WP Sprint Map

| Sprint | WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| Sprint 1 | WP-01 | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.90.5 closeout |
| Sprint 1 | WP-02 | Moral event contract | moral event feature contract and fixtures | WP-01 |
| Sprint 1 | WP-03 | Moral event validation | validation rules and negative fixtures | WP-02 |
| Sprint 1 | WP-04 | Moral trace schema | trace schema and examples | WP-02, WP-03 |
| Sprint 1 | WP-05 | Outcome linkage and attribution | outcome-linkage record and tests | WP-04 |
| Sprint 2 | WP-06 | Moral metrics | metric definitions and fixture report | WP-04, WP-05 |
| Sprint 2 | WP-07 | Moral trajectory review | trajectory review packet | WP-04-WP-06 |
| Sprint 2 | WP-08 | Anti-harm trajectory constraints | delegated-harm proof packet | WP-04-WP-07 |
| Sprint 3 | WP-09 | Wellbeing metrics v0 | decomposed diagnostic report and policy views | WP-04-WP-07 |
| Sprint 3 | WP-10 | Moral resources | moral-resources contract, fixtures, and implementation surface | WP-05-WP-09 |
| Sprint 3 | WP-11 | Kindness model | kindness contract and conflict fixtures | WP-05-WP-10 |
| Sprint 3 | WP-12 | Humor and absurdity | reframing event and negative fixtures | WP-05-WP-10 |
| Sprint 3 | WP-13 | Affect reasoning-control surface | affect signal record and policy hooks | WP-05-WP-10 |
| Sprint 3 | WP-14 | Cultivating intelligence | cultivation contract and review criteria | WP-05-WP-13 |
| Sprint 3 | WP-15 | Structured planning and SRP workflow surfaces | SPP/SRP artifacts, planning skill, and review-readiness checks | WP-01 |
| Sprint 3 | WP-16 | Secure Agent Comms substrate and A2A boundary | local ACIP substrate slice plus explicit A2A adapter boundary | WP-04-WP-05, WP-15 |
| Sprint 3 | WP-17 | Cognitive-being flagship demo | runnable proof demo and artifacts | WP-08-WP-16 |
| Sprint 3 | WP-18 | Demo matrix and feature proof coverage | demo matrix rows and proof coverage record | WP-17 |
| Sprint 4 | WP-19 | Coverage / quality gate | quality gate and validation posture record | WP-18 |
| Sprint 4 | WP-20 | Docs + review pass | review-ready docs package | WP-19 |
| Sprint 4 | WP-21 | Internal review | internal review record | WP-20 |
| Sprint 4 | WP-22 | External / 3rd-party review | external review handoff and record | WP-21 |
| Sprint 4 | WP-23 | Review findings remediation | remediation record and follow-up issues | WP-22 |
| Sprint 4 | WP-24 | Next milestone planning | v0.91.1/v0.92/v0.93 handoff record | WP-23 |
| Sprint 4 | WP-25 | Release ceremony | release evidence, end-of-milestone report, and next handoff | WP-24 |

## Sprint 1: Moral Evidence Foundation

- WP-01: Promote reviewed v0.91 milestone package.
- Confirm v0.91/v0.91.1 split and the v0.92 identity boundary.
- Promote the cognitive-being and Agent Comms split plans into tracked milestone
  planning.
- Promote structured planning, `SRP`, and A2A adapter planning into tracked
  milestone docs.
- Moral event contract.
- Moral event validation.
- Moral trace schema.
- Outcome linkage and attribution.

Goal: make moral choices and alternatives durable before metrics or demos widen.

## Sprint 2: Metrics, Trajectory, And Anti-Harm

- Moral metrics over trace evidence.
- Moral trajectory review.
- Anti-harm trajectory constraints.
- Delegated-harm proof fixtures.

Goal: make moral behavior reviewable over time without turning metrics into
verdicts.

## Sprint 3: Wellbeing, Moral Resources, And Demos

- Wellbeing metrics v0 diagnostic.
- Citizen self-access and redacted operator/reviewer/public views.
- Moral resources implementation slice.
- Kindness model and conflict fixtures.
- Humor/absurdity reframing slice.
- Affect reasoning-control surface.
- Cultivating-intelligence review criteria.
- Secure intra-polis Agent Comms substrate slice if prerequisite scope is stable.
- Structured planning and plan-review artifact slice.
- Structured Review Policy (`SRP`) slice.
- Cognitive-being flagship demo.
- Demo matrix and feature proof coverage.

Goal: make the evidence visible to citizens and reviewers without exposing
private diagnostics, pretending the system has solved wellbeing, or claiming the
v0.92 birthday.

## Sprint 4: Review And Release

- Quality/docs convergence.
- Internal review.
- Third-party review handoff.
- Accepted-finding remediation.
- v0.91.1 adjacent-systems plan for capability/aptitude testing, intelligence
  metric architecture, ANRM/Gemma, ToM, memory/identity, runtime-v2/polis docs,
  and any ACIP hardening that does not fit safely in v0.91.
- Next-milestone planning handoff to v0.92 and v0.93.
- Release ceremony.

Goal: close the milestone with truthful review, release, and handoff evidence.

## Parallelization Notes

Moral event validation can proceed beside trace-schema work once the event
contract is stable. Metrics should wait for trace and outcome linkage. Wellbeing
metrics should wait for trajectory review because they need evidence rather than
affect theater. Kindness, humor/absurdity, affect, and cultivating-intelligence
work can proceed in parallel only after the shared evidence and non-claim
language is stable. ACIP work can proceed beside those slices if it stays local,
secure, and external-TLS-gated. A2A planning may proceed only as an adapter
over that same substrate and should not define a parallel communication model.
Release-tail work should remain sequential.
