# Sprint Plan - v0.91.1

## Status

Candidate sprint plan for review. This file follows the v0.91 pattern while
keeping issue numbers unassigned until the wave is opened.

## Sprint 1: Runtime, Polis, Lifecycle, Standing, And State

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91 closeout |
| WP-02 | Runtime and polis architecture alignment | runtime/polis architecture package | WP-01 |
| WP-03 | Agent lifecycle state model | lifecycle state contract, transition matrix, ACIP eligibility, and fixtures | WP-02 |
| WP-04 | CSM observatory active surface | active packet and projection surface | WP-02, WP-03 |
| WP-05 | Citizen standing model | standing contract and fixtures | WP-02, WP-03 |
| WP-06 | Citizen state substrate | state format, security, and projection slice | WP-05 |

Goal: make the CSM/polis runtime surfaces real enough that later cognition work
has an inhabited substrate instead of a decorative transcript layer. The
lifecycle state model should answer which states can receive ACIP messages,
which can invoke actions, and which must queue, reject, or quarantine requests.

## Sprint 2: Memory, ToM, Capability, Intelligence, And Learning

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-07 | Memory and identity architecture | memory/identity architecture and fixtures | WP-06 |
| WP-08 | Theory of Mind foundation | ToM schemas, update events, and tests | WP-06, WP-07 |
| WP-09 | Capability and aptitude testing foundation | first executable capability harness and report | WP-02, WP-03, WP-06 |
| WP-10 | Intelligence metric architecture | evidence-bound metric architecture | WP-09 |
| WP-11 | Governed learning substrate | learning update and feedback contract | WP-07, WP-09, WP-10 |
| WP-12 | ANRM/Gemma placement and trace dataset | ANRM placement, trace extractor, dataset mapping | WP-09-WP-11 |

Goal: implement the bounded cognitive and evaluation surfaces that v0.92 needs
without claiming completed identity, intelligence, or learning theory.

## Sprint 3: Secure Comms And Inhabitant Runtime Proof

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-13 | ACIP conformance and local encryption hardening | secure local comms envelope and conformance fixtures | WP-03, WP-06 |
| WP-14 | A2A adapter boundary and compatibility plan | A2A-over-ACIP adapter slice and non-claims | WP-13 |
| WP-15 | Runtime inhabitant integration | integrated agent-shaped run surface | WP-04-WP-14 |
| WP-16 | Observatory-visible agent flagship demo | runnable CSM inhabitant proof demo | WP-15 |
| WP-17 | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-16 |

Goal: prove a real agent-shaped runtime path inside the CSM boundary with
authenticated local communication and observatory-visible evidence.

## Sprint 4: Quality, Review, Handoff, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-18 | Coverage / quality gate | validation posture and test/coverage record | WP-17 |
| WP-19 | Docs + review pass | review-ready docs package | WP-18 |
| WP-20 | Internal review | internal review record | WP-19 |
| WP-21 | External / 3rd-party review | external review handoff and record | WP-20 |
| WP-22 | Review findings remediation | remediation record and follow-up issues | WP-21 |
| WP-23 | v0.92 birthday readiness handoff | identity/birthday handoff record | WP-22 |
| WP-24 | Release ceremony | release evidence and end-of-milestone report | WP-23 |

Goal: leave v0.92 with a clean, evidence-backed path to identity and birthday
work rather than another loose planning backlog.

## Parallelization Notes

Capability testing can proceed beside citizen-state work once the runtime
architecture and lifecycle-state passes land, but ToM and memory should wait
for citizen state. ACIP hardening can proceed beside capability and learning
work after lifecycle-state rules define which agent states may receive,
queue, reject, or invoke messages. The inhabitant integration and demo must
wait for standing, state, observatory, lifecycle, and comms evidence to exist.
