# Sprint Plan - v0.91.1

## Status

Active sprint plan. The complete v0.91.1 issue wave is open as `#2823`
through `#2846`, and every WP has a prepared local STP, SIP, SPP, SRP, and SOR
bundle before execution binding.

## Sprint 1: Runtime, Polis, Lifecycle, Standing, And State

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 (#2823) | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91 closeout |
| WP-02 (#2824) | Runtime and polis architecture alignment | runtime/polis architecture package | WP-01 |
| WP-03 (#2825) | Agent lifecycle state model | lifecycle state contract, transition matrix, ACIP eligibility, and fixtures | WP-02 |
| WP-04 (#2826) | CSM observatory active surface | active packet and projection surface | WP-02, WP-03 |
| WP-05 (#2827) | Citizen standing model | standing contract and fixtures | WP-02, WP-03 |
| WP-06 (#2828) | Citizen state substrate | state format, security, and projection slice | WP-05 |

Goal: make the CSM/polis runtime surfaces real enough that later cognition work
has an inhabited substrate instead of a decorative transcript layer. The
lifecycle state model should answer which states can receive ACIP messages,
which can invoke actions, and which must queue, reject, or quarantine requests.

## Sprint 2: Memory, ToM, Capability, Intelligence, And Learning

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-07 (#2829) | Memory and identity architecture | memory/identity architecture and fixtures | WP-06 |
| WP-08 (#2830) | Theory of Mind foundation | ToM schemas, update events, and tests | WP-06, WP-07 |
| WP-09 (#2831) | Capability and aptitude testing foundation | first executable capability harness and report | WP-02, WP-03, WP-06 |
| WP-10 (#2832) | Intelligence metric architecture | evidence-bound metric architecture | WP-09 |
| WP-11 (#2833) | Governed learning substrate | learning update and feedback contract | WP-07, WP-09, WP-10 |
| WP-12 (#2834) | ANRM/Gemma placement and trace dataset | ANRM placement, trace extractor, dataset mapping | WP-09, WP-10, WP-11 |

Goal: implement the bounded cognitive and evaluation surfaces that v0.92 needs
without claiming completed identity, intelligence, or learning theory.

Closeout status: complete. The Sprint 2 child lane merged through:
- `#2829` via `#2908`
- `#2830` via `#2920`
- `#2831` via `#2929`
- `#2832` via `#2935`
- `#2833` via `#2938`
- `#2834` via `#2941`

The tracked closeout note lives in
`docs/milestones/v0.91.1/SPRINT_2_CLOSEOUT_v0.91.1.md`.

## Sprint 3: Secure Comms And Inhabitant Runtime Proof

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-13 (#2835) | ACIP conformance and local encryption hardening | secure local comms envelope and conformance fixtures | WP-03, WP-06 |
| WP-14 (#2836) | A2A adapter boundary and compatibility plan | A2A-over-ACIP adapter slice and non-claims | WP-13 |
| WP-15 (#2837) | Runtime inhabitant integration | integrated agent-shaped run surface | WP-04, WP-05, WP-06, WP-07, WP-08, WP-09, WP-10, WP-11, WP-12, WP-13, WP-14 |
| WP-16 (#2838) | Observatory-visible agent flagship demo | runnable CSM inhabitant proof demo | WP-15 |
| WP-17 (#2839) | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-16 |

Goal: prove a real agent-shaped runtime path inside the CSM boundary with
authenticated local communication and observatory-visible evidence.

## Sprint 4: Quality, Review, Handoff, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-18 (#2840) | Coverage / quality gate | validation posture and test/coverage record | WP-17 |
| WP-19 (#2841) | Docs + review pass | review-ready docs package | WP-18 |
| WP-20 (#2842) | Internal review | internal review record | WP-19 |
| WP-21 (#2843) | External / 3rd-party review | external review handoff and record | WP-20 |
| WP-22 (#2844) | Review findings remediation | remediation record and follow-up issues | WP-21 |
| WP-23 (#2845) | Next milestone planning | downstream milestone handoff record | WP-22 |
| WP-24 (#2846) | Release ceremony | release evidence and end-of-milestone report | WP-23 |

Goal: leave the downstream milestone package with a clean, evidence-backed path
for immediate `v0.91.2` execution and later `v0.92` birthday/identity
consumption rather than another loose planning backlog.

Supplemental pre-ceremony review step:
- `WP-23A` / `#2985` performs the final next-milestone review pass after
  `WP-23` planning and before `WP-24` ceremony without renumbering the core
  release-tail sequence.

## Supplemental Mini-Sprint: Test-Cycle Recovery

| Issue | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| #2897 | Sprint umbrella: test-cycle recovery mini-sprint | ordered sprint-management umbrella with review/closeout tracking | none |
| #2867 | Test-cycle recovery mini-sprint plan | bounded execution plan and sequencing truth for the mini-sprint | #2897 |
| #2864 | Tighten coverage path-policy escalation | narrower fail-closed escalation policy with tests | #2867 |
| #2865 | Trim authoritative coverage reporting tail | reduced authoritative coverage post-pass overhead | #2864 |
| #2866 | Expand PR-fast and coverage-impact filter mappings | broader truthful focused-lane routing coverage | #2865 |
| #2868 | Collapse remaining heavyweight proof-test families | sprint-level residual disposition confirming the named hotspot wave already landed via GW-01..GW-05 | #2866 |

Goal: cut avoidable CI and authoritative-coverage wall time in v0.91.1 without
weakening proof posture, coverage policy, or review integrity. This
mini-sprint is supplemental to the main WP wave and should run through the
sprint-conductor slow path with one active child issue at a time.

Disposition note: `#2868` remained in the mini-sprint as the implementation-heavy
tail check, but the original heavyweight proof-family collapses were already
delivered in the merged GW-01 through GW-05 runtime-reduction wave recorded in
`docs/milestones/v0.90.5/GET_WELL_TRACKING_v0.90.5.md`. The truthful v0.91.1
action is therefore closeout-by-evidence rather than reopening those runtime
family rewrites under a new issue number.

Closeout status: complete. The mini-sprint child lane merged through:
- `#2867` via `#2902`
- `#2864` via `#2903`
- `#2865` via `#2904`
- `#2866` via `#2905`
- `#2868` via `#2906`

Sprint outcome:
- the planned lane-shape and policy fixes landed
- the final residual runtime-heavy family issue closed by evidence because the
  named GW hotspot collapses were already complete
- the sprint-conductor trial exposed follow-on skill and lifecycle gaps now
  tracked in `#2901` and `#2907`
- the tracked closeout note lives in
  `docs/milestones/v0.91.1/TEST_CYCLE_MINI_SPRINT_CLOSEOUT_v0.91.1.md`

## Parallelization Notes

Capability testing can proceed beside citizen-state work once the runtime
architecture and lifecycle-state passes land, but ToM and memory should wait
for citizen state. ACIP hardening can proceed beside capability and learning
work after lifecycle-state rules define which agent states may receive,
queue, reject, or invoke messages. The inhabitant integration and demo must
wait for standing, state, observatory, lifecycle, and comms evidence to exist.
