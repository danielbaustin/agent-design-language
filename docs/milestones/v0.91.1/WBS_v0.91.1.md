# v0.91.1 Work Breakdown Structure

## Status

Candidate WBS for review. Issue numbers are intentionally not assigned yet.
This document should become the card-authoring basis only after v0.91 closes
and the candidate issue wave is accepted.

## WBS Summary

v0.91.1 is the inhabited-runtime readiness milestone. It converts the adjacent
systems left out of the v0.91 moral-governance core into a coherent runtime
and cognition implementation wave.

## Work Areas

| Area | Work Area | Description | Primary Deliverable | Key Dependencies |
| --- | --- | --- | --- | --- |
| A | Design pass | Promote the accepted v0.91.1 plan into opened issues and cards. | tracked docs, issue wave, and validated cards | v0.91 closeout |
| B | Runtime/polis architecture alignment | Reconcile Runtime v2, polis, kernel, manifold, and lifecycle docs with current code truth. | runtime/polis architecture package | A |
| C | CSM observatory active surface | Turn observatory planning into active packets, projections, and operator-visible runtime state. | observatory active-surface implementation | B |
| D | Citizen standing | Implement and document standing classes, transitions, and naked-actor rejection boundaries. | citizen-standing contract and fixtures | B |
| E | Citizen state | Harden citizen-state format, security, projection, and review boundaries. | citizen-state substrate update | B, D |
| F | Memory and identity architecture | Prepare memory/identity evidence surfaces without claiming full v0.92 identity continuity. | memory/identity architecture and fixtures | E |
| G | Theory of Mind foundation | Model bounded agent-state hypotheses and update events tied to evidence. | ToM contract, schemas, and fixtures | E, F |
| H | Capability and aptitude testing | Create the first executable capability/aptitude harness and report model. | capability test harness slice | B, E |
| I | Intelligence metric architecture | Define intelligence metrics as evidence-bound architecture, not reputation or mystique. | metric architecture and fixture report | H |
| J | Governed learning substrate | Bound learning updates, feedback, and adaptation evidence under policy. | governed-learning contract | F, H, I |
| K | ANRM/Gemma placement | Place ANRM/Gemma as a bounded local-model evidence and trace-dataset lane. | ANRM placement package and extractor spec | H, I, J |
| L | ACIP/A2A hardening | Harden local comms envelopes, redaction, invocation, conformance, and A2A adapter boundaries. | secure comms hardening slice | B, E |
| M | Runtime inhabitant integration | Integrate standing, state, memory, comms, capability, learning, and observatory into an agent-shaped run. | inhabitant runtime integration | C through L |
| N | Observatory-visible agent demo | Prove an agent-shaped CSM run with operator projection and reviewable traces. | flagship runtime demo artifacts | M |
| O | Review, quality, and release | Validate docs, demos, issue evidence, review findings, and release ceremony. | review-ready release package | N |

## Candidate WP Sequence

| WP | Title | Queue | Primary Deliverable | Dependencies |
| --- | --- | --- | --- | --- |
| WP-01 | Design pass (milestone docs + planning) | docs | tracked docs, reviewed YAML, and issue cards | v0.91 closeout |
| WP-02 | Runtime and polis architecture alignment | docs | runtime/polis architecture package | WP-01 |
| WP-03 | CSM observatory active surface | runtime | active packet and projection surface | WP-02 |
| WP-04 | Citizen standing model | runtime | standing contract and fixtures | WP-02 |
| WP-05 | Citizen state substrate | runtime | state format, security, and projection slice | WP-04 |
| WP-06 | Memory and identity architecture | runtime | memory/identity architecture and fixtures | WP-05 |
| WP-07 | Theory of Mind foundation | runtime | ToM schemas, update events, and tests | WP-05, WP-06 |
| WP-08 | Capability and aptitude testing foundation | tools | first executable capability harness and report | WP-02, WP-05 |
| WP-09 | Intelligence metric architecture | runtime | evidence-bound metric architecture | WP-08 |
| WP-10 | Governed learning substrate | runtime | learning update and feedback contract | WP-06, WP-08, WP-09 |
| WP-11 | ANRM/Gemma placement and trace dataset | tools | ANRM placement, trace extractor, dataset mapping | WP-08-WP-10 |
| WP-12 | ACIP conformance and local encryption hardening | runtime | secure local comms envelope and conformance fixtures | WP-02, WP-05 |
| WP-13 | A2A adapter boundary and compatibility plan | runtime | A2A-over-ACIP adapter slice and non-claims | WP-12 |
| WP-14 | Runtime inhabitant integration | runtime | integrated agent-shaped run surface | WP-03-WP-13 |
| WP-15 | Observatory-visible agent flagship demo | demo | runnable CSM inhabitant proof demo | WP-14 |
| WP-16 | Demo matrix and proof coverage | demo | demo matrix and proof coverage record | WP-15 |
| WP-17 | Coverage / quality gate | quality | validation posture and test/coverage record | WP-16 |
| WP-18 | Docs + review pass | docs | review-ready docs package | WP-17 |
| WP-19 | Internal review | review | internal review record | WP-18 |
| WP-20 | External / 3rd-party review | review | external review handoff and record | WP-19 |
| WP-21 | Review findings remediation | review | remediation record and follow-up issues | WP-20 |
| WP-22 | v0.92 birthday readiness handoff | docs | identity/birthday handoff record | WP-21 |
| WP-23 | Release ceremony | release | release evidence and end-of-milestone report | WP-22 |

## Sequencing Pressure

Runtime/polis architecture must land before the feature slices widen. Citizen
standing and state must land before memory, ToM, and inhabitant integration can
claim meaningful runtime grounding. ACIP/A2A hardening must land before the
flagship demo depends on agent-to-agent communication. The demo and review tail
must stay after implementation, not substitute for it.
