# Sprint Plan - v0.91.1

## Status

Candidate sprint plan for review. This file follows the v0.91 pattern while
keeping issue numbers unassigned until the wave is opened.

## Sprint 1: Runtime, Polis, Standing, And State

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-01 | Design pass (milestone docs + planning) | tracked docs, reviewed YAML, and issue cards | v0.91 closeout |
| WP-02 | Runtime and polis architecture alignment | runtime/polis architecture package | WP-01 |
| WP-03 | CSM observatory active surface | active packet and projection surface | WP-02 |
| WP-04 | Citizen standing model | standing contract and fixtures | WP-02 |
| WP-05 | Citizen state substrate | state format, security, and projection slice | WP-04 |

Goal: make the CSM/polis runtime surfaces real enough that later cognition work
has an inhabited substrate instead of a decorative transcript layer.

## Sprint 2: Memory, ToM, Capability, Intelligence, And Learning

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-06 | Memory and identity architecture | memory/identity architecture and fixtures | WP-05 |
| WP-07 | Theory of Mind foundation | ToM schemas, update events, and tests | WP-05, WP-06 |
| WP-08 | Capability and aptitude testing foundation | first executable capability harness and report | WP-02, WP-05 |
| WP-09 | Intelligence metric architecture | evidence-bound metric architecture | WP-08 |
| WP-10 | Governed learning substrate | learning update and feedback contract | WP-06, WP-08, WP-09 |
| WP-11 | ANRM/Gemma placement and trace dataset | ANRM placement, trace extractor, dataset mapping | WP-08-WP-10 |

Goal: implement the bounded cognitive and evaluation surfaces that v0.92 needs
without claiming completed identity, intelligence, or learning theory.

## Sprint 3: Secure Comms And Inhabitant Runtime Proof

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-12 | ACIP conformance and local encryption hardening | secure local comms envelope and conformance fixtures | WP-02, WP-05 |
| WP-13 | A2A adapter boundary and compatibility plan | A2A-over-ACIP adapter slice and non-claims | WP-12 |
| WP-14 | Runtime inhabitant integration | integrated agent-shaped run surface | WP-03-WP-13 |
| WP-15 | Observatory-visible agent flagship demo | runnable CSM inhabitant proof demo | WP-14 |
| WP-16 | Demo matrix and proof coverage | demo matrix and proof coverage record | WP-15 |

Goal: prove a real agent-shaped runtime path inside the CSM boundary with
authenticated local communication and observatory-visible evidence.

## Sprint 4: Quality, Review, Handoff, And Release

| WP | Title | Primary Deliverable | Dependencies |
| --- | --- | --- | --- |
| WP-17 | Coverage / quality gate | validation posture and test/coverage record | WP-16 |
| WP-18 | Docs + review pass | review-ready docs package | WP-17 |
| WP-19 | Internal review | internal review record | WP-18 |
| WP-20 | External / 3rd-party review | external review handoff and record | WP-19 |
| WP-21 | Review findings remediation | remediation record and follow-up issues | WP-20 |
| WP-22 | v0.92 birthday readiness handoff | identity/birthday handoff record | WP-21 |
| WP-23 | Release ceremony | release evidence and end-of-milestone report | WP-22 |

Goal: leave v0.92 with a clean, evidence-backed path to identity and birthday
work rather than another loose planning backlog.

## Parallelization Notes

Capability testing can proceed beside citizen-state work once the runtime
architecture pass lands, but ToM and memory should wait for citizen state.
ACIP hardening can proceed beside capability and learning work if it does not
invent a parallel transport architecture. The inhabitant integration and demo
must wait for standing, state, observatory, and comms evidence to exist.
