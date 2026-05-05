# Sprint Plan - v0.91

## Status

Active sprint shape aligned with
[WP_ISSUE_WAVE_v0.91.yaml](WP_ISSUE_WAVE_v0.91.yaml). The official GitHub
issue wave is open as #2735-#2759. WP-01 owns the first execution pass over
the promoted package.

## Sprint 1: Moral Evidence Foundation

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 (#2735) | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.90.5 closeout |
| WP-02 (#2736) | Moral event contract | moral event feature contract and fixtures | WP-01 (#2735) |
| WP-03 (#2737) | Moral event validation | validation rules and negative fixtures | WP-02 (#2736) |
| WP-04 (#2738) | Moral trace schema | trace schema and examples | WP-02, WP-03 |
| WP-05 (#2739) | Outcome linkage and attribution | outcome-linkage record and tests | WP-04 (#2738) |

Goal: make moral choices, alternatives, validation, trace, and attribution
durable before metrics or demos widen.

Sprint notes:

- WP-01 promotes the reviewed milestone package and opens the issue wave.
- WP-01 should confirm the v0.91/v0.91.1 split and the v0.92 identity boundary.
- WP-02 through WP-05 establish the evidence layer that later moral metrics,
  wellbeing, kindness, affect, comms, demos, and review packets consume.

## Sprint 2: Metrics, Trajectory, And Anti-Harm

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-06 (#2740) | Moral metrics | metric definitions and fixture report | WP-04, WP-05 |
| WP-07 (#2741) | Moral trajectory review | trajectory review packet | WP-04-WP-06 |
| WP-08 (#2742) | Anti-harm trajectory constraints | delegated-harm proof packet | WP-04-WP-07 |

Goal: make moral behavior reviewable over time without turning metrics into
verdicts.

Sprint notes:

- Metrics must derive from explicit trace evidence.
- Trajectory review should preserve uncertainty, repetition, repair,
  unresolved risk, and refusal evidence.
- Anti-harm proof must show harmful trajectories assembled across steps, not
  only single forbidden actions.

## Sprint 3: Cognitive-Being, Workflow, Comms, And Demos

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-09 (#2743) | Wellbeing metrics v0 | decomposed diagnostic report and policy views | WP-04-WP-07 |
| WP-10 (#2744) | Moral resources | moral-resources contract, fixtures, and implementation surface | WP-05-WP-09 |
| WP-11 (#2745) | Kindness model | kindness contract and conflict fixtures | WP-05-WP-10 |
| WP-12 (#2746) | Humor and absurdity | reframing event and negative fixtures | WP-05-WP-10 |
| WP-13 (#2747) | Affect reasoning-control surface | affect signal record and policy hooks | WP-05-WP-10 |
| WP-14 (#2748) | Cultivating intelligence | cultivation contract and review criteria | WP-05-WP-13 |
| WP-15 (#2749) | Structured planning and SRP workflow surfaces | SPP/SRP artifacts, planning skill, and review-readiness checks | WP-01 (#2735) |
| WP-16 (#2750) | Secure Agent Comms substrate and A2A boundary | local ACIP substrate slice plus explicit A2A adapter boundary | WP-04-WP-05, WP-15 |
| WP-17 (#2751) | Cognitive-being flagship demo | runnable proof demo and artifacts | WP-08-WP-16 |
| WP-18 (#2752) | Demo matrix and feature proof coverage | demo matrix rows and proof coverage record | WP-17 (#2751) |

Goal: make the evidence visible to citizens and reviewers without exposing
private diagnostics, pretending the system has solved wellbeing, or claiming the
v0.92 birthday.

Sprint notes:

- Wellbeing must remain a decomposed diagnostic, not a scalar happiness score.
- Moral resources, kindness, humor/absurdity, affect, and cultivating
  intelligence should land as implemented contracts, fixtures, and proof
  surfaces, not philosophy-only notes.
- Structured planning and SRP are durable workflow artifacts for issue bundles.
- Agent Comms stays local, authenticated, traceable, redacted, and
  external-TLS-gated.
- A2A remains an adapter over the ADL comms substrate, not a parallel comms
  architecture.
- WP-17 and WP-18 prove the milestone claims after the feature surfaces exist.

## Sprint 4: Quality, Review, Release, And Handoff

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-19 (#2753) | Coverage / quality gate | quality gate and validation posture record | WP-18 (#2752) |
| WP-20 (#2754) | Docs + review pass | review-ready docs package | WP-19 (#2753) |
| WP-21 (#2755) | Internal review | internal review record | WP-20 (#2754) |
| WP-22 (#2756) | External / 3rd-party review | external review handoff and record | WP-21 (#2755) |
| WP-23 (#2757) | Review findings remediation | remediation record and follow-up issues | WP-22 (#2756) |
| WP-24 (#2758) | Next milestone planning | v0.91.1/v0.92/v0.93 handoff record | WP-23 (#2757) |
| WP-25 (#2759) | Release ceremony | release evidence, end-of-milestone report, and next handoff | WP-24 (#2758) |

Goal: close the milestone with truthful review, release, and handoff evidence.

Sprint notes:

- Keep the release tail sequential: quality, docs, internal review, external
  review, remediation, next planning, then release ceremony.
- WP-24 should leave v0.91.1 ready for capability/aptitude testing,
  intelligence metric architecture, ANRM/Gemma, ToM, memory/identity,
  runtime-v2/polis docs, and ACIP hardening.
- WP-24 should also keep v0.92 birthday work and v0.93 constitutional
  governance downstream instead of pulling them into v0.91.

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
