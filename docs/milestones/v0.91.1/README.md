# v0.91.1 Milestone README

## Status

Active milestone execution package. v0.91.1 is open for execution through
the complete issue wave: `WP-01` / `#2823` through `WP-24` / `#2846`.
Every WP has a prepared STP, SIP, SPP, SRP, and SOR bundle before execution
binding.

## Purpose

v0.91.1 is the adjacent-systems implementation milestone that makes the CSM
ready for inhabited runtime work before the v0.92 identity and birthday band.
It turns the remaining Runtime v2, polis, lifecycle-state, observatory,
communication, capability, intelligence, memory, learning, Theory of Mind, and
ANRM/Gemma source packets into concrete implementation work.

The milestone should end with an observatory-visible agent runtime proof
surface: agents should be able to run inside the CSM/polis boundary with
authenticated local communication, policy-bound invocation, traceable state,
and a reviewer-visible operator projection. This is not the birthday milestone,
but it must give v0.92 enough real runtime evidence to build on.

## Milestone Role

v0.91.1 should establish:

- CSM/polis runtime documentation aligned with the current Runtime v2 substrate.
- Agent lifecycle states that distinguish active, quiescent, suspended,
  dormant, simulation, in-transit, bootstrap, shutdown, and forced-suspension
  regimes.
- Citizen standing and citizen state as concrete runtime-facing contracts.
  The citizen-state band may still consume inherited `v0.90.3` canonical
  private-state baseline artifacts where current `v0.91.1` work has not
  renumbered the historical proof identity; that inheritance should be
  explicit, not silent.
- Memory and identity architecture strong enough to feed v0.92 without claiming
  full identity continuity yet.
- Theory of Mind foundations tied to citizen state, memory, and communication
  evidence.
- Capability and aptitude testing foundations for model, skill, and agent
  evaluation.
- Intelligence metric architecture that remains evidence-bound and
  non-scoreboard.
- Governed learning boundaries that distinguish adaptation evidence from
  unreviewable self-modification.
- ANRM/Gemma placement and trace-dataset architecture as bounded local-model
  evidence work.
- ACIP/A2A hardening needed for secure intra-polis communication before
  external transport work expands.
- ACIP reception and invocation rules for each lifecycle state so messages
  cannot wake, invoke, or commit an agent outside policy.
- Observatory-visible agent runtime proof that is concrete enough for human
  review.

"Foundation" in this milestone means the first implemented, validated,
downstream-consumable slice. A foundation WP is not complete if it only leaves
planning prose behind; it must produce schemas, fixtures, tests, executable
tooling, demo evidence, review records, or other durable work product that a
later WP can use directly.

## Boundaries

v0.91.1 should not claim:

- the first true Gödel-agent birthday
- full constitutional citizenship
- legal personhood
- production external federation
- external or cross-polis transport readiness without TLS or mutual-TLS
  equivalent protection
- broad provider-native tool-call conformance across all models
- final intelligence, wellbeing, or Theory of Mind answers

## Source Map

This package is grounded in the local TBD planning corpus:

- `.adl/docs/TBD/v0.91_1_runtime_observatory_dependency_note.md`
- `.adl/docs/TBD/ADL_AND_SLEEP.md`
- `.adl/docs/TBD/runtime_v2/`
- `.adl/docs/TBD/csm_observatory/`
- `.adl/docs/TBD/citizen_state/`
- `.adl/docs/TBD/citizen_standing/`
- `.adl/docs/TBD/memory_identity/`
- `.adl/docs/TBD/ToM/`
- `.adl/docs/TBD/capability_testing/`
- `.adl/docs/TBD/intelligence/`
- `.adl/docs/TBD/learning_model/`
- `.adl/docs/TBD/anrm/`
- `.adl/docs/TBD/acip/`
- `.adl/docs/TBD/a2a/`

The UTS + ACC multi-model benchmark plan is intentionally excluded from this
source map because it belongs to the `v0.91.2` tooling/evaluation milestone.
v0.91.1 may depend on governed tools and ACIP substrate evidence, but it should
not import the broad provider-native benchmark planning surface.

## Document Map

- WBS: [WBS_v0.91.1.md](WBS_v0.91.1.md)
- Vision: [VISION_v0.91.1.md](VISION_v0.91.1.md)
- Design: [DESIGN_v0.91.1.md](DESIGN_v0.91.1.md)
- Decisions: [DECISIONS_v0.91.1.md](DECISIONS_v0.91.1.md)
- Sprint plan: [SPRINT_v0.91.1.md](SPRINT_v0.91.1.md)
- Sprint 2 closeout:
  [SPRINT_2_CLOSEOUT_v0.91.1.md](SPRINT_2_CLOSEOUT_v0.91.1.md)
- Sprint 3 closeout:
  [SPRINT_3_CLOSEOUT_v0.91.1.md](SPRINT_3_CLOSEOUT_v0.91.1.md)
- Active issue wave: [WP_ISSUE_WAVE_v0.91.1.yaml](WP_ISSUE_WAVE_v0.91.1.yaml)
- Execution readiness:
  [WP_EXECUTION_READINESS_v0.91.1.md](WP_EXECUTION_READINESS_v0.91.1.md)
- Runtime/polis architecture package:
  [RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md](RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md)
- Demo matrix: [DEMO_MATRIX_v0.91.1.md](DEMO_MATRIX_v0.91.1.md)
- Feature proof coverage:
  [FEATURE_PROOF_COVERAGE_v0.91.1.md](FEATURE_PROOF_COVERAGE_v0.91.1.md)
- Quality gate: [QUALITY_GATE_v0.91.1.md](QUALITY_GATE_v0.91.1.md)
- Feature index: [features/README.md](features/README.md)
- Card bundle readiness:
  [CARD_BUNDLE_READINESS_v0.91.1.md](CARD_BUNDLE_READINESS_v0.91.1.md)
- SPP readiness: [SPP_READINESS_v0.91.1.md](SPP_READINESS_v0.91.1.md)
- Milestone checklist:
  [MILESTONE_CHECKLIST_v0.91.1.md](MILESTONE_CHECKLIST_v0.91.1.md)
- Release plan: [RELEASE_PLAN_v0.91.1.md](RELEASE_PLAN_v0.91.1.md)
- Release readiness: [RELEASE_READINESS_v0.91.1.md](RELEASE_READINESS_v0.91.1.md)
- Release evidence: [RELEASE_EVIDENCE_v0.91.1.md](RELEASE_EVIDENCE_v0.91.1.md)
- Release notes: [RELEASE_NOTES_v0.91.1.md](RELEASE_NOTES_v0.91.1.md)
- Third-party review handoff:
  [ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md](ADL_v0.91.1_THIRD_PARTY_REVIEW_HANDOFF.md)
- Remediation queue:
  [review/WP22_REMEDIATION_QUEUE.md](review/WP22_REMEDIATION_QUEUE.md)
- Next milestone handoff:
  [NEXT_MILESTONE_HANDOFF_v0.91.1.md](NEXT_MILESTONE_HANDOFF_v0.91.1.md)
- End-of-milestone report:
  [END_OF_MILESTONE_REPORT_v0.91.1.md](END_OF_MILESTONE_REPORT_v0.91.1.md)

## Success Criteria

v0.91.1 is ready to close when the runtime, lifecycle-state, standing, state,
memory, ToM, capability, intelligence, learning, ANRM/Gemma, ACIP/A2A, and
observatory work have landed as implemented, validated, reviewable surfaces
rather than planning-only notes.

The strongest closeout proof should be a real agent-shaped run inside the CSM
boundary with observatory-visible evidence, secure local communication, runtime
state, and explicit non-claims for identity, birthday, and external transport.
