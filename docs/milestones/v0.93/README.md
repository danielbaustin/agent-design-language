# v0.93 Milestone README

## Status

Forward planning. v0.93 is not yet an active implementation milestone and has
no final issue wave. Its boundary was rechecked during the `v0.90.4` WP-19
handoff pass so reputation and social-cognition work stay here instead of
turning into loose `v0.90.4` follow-on debt.

## Purpose

v0.93 is the planned constitutional citizenship, social-cognition, and
polis-governance milestone. It should convert earlier citizen-state,
moral-trace, identity, and standing work into a bounded governance layer for the
ADL polis.

The canonical allocation is recorded in
[CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md](CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md).

## Milestone Role

v0.93 should establish:

- constitutional citizenship as a trace-grounded policy model
- bounded Theory of Mind, reputation, and shared social memory as distinct
  social-cognition surfaces
- rights, duties, standing, challenge, appeal, delegation, and IAM semantics
- enterprise-security foundations for zero-trust polis operation
- cryptographic trust, secrets/key lifecycle, isolation, audit, and incident
  evidence as first-class governance surfaces
- reviewer-facing governance evidence that does not expose raw private state
- a clear boundary between CSM citizen identity and human/operator action

v0.93 should not claim legal personhood, production constitutional authority,
or complete social-contract theory.

## Dependency Boundary

v0.93 depends on:

- v0.90.3 for citizen state, standing, access control, projection, lineage,
  challenge, sanctuary, and quarantine
- v0.91 for Freedom Gate moral events, moral trace, validation, outcome
  linkage, trajectory review, wellbeing, moral resources, and anti-harm
  evidence
- v0.92 for durable identity, names, continuity, memory grounding, capability
  envelopes, and the first true Gödel-agent birthday
- v0.90.4 and v0.90.5 only where economics or governed-tool authority has
  landed before v0.93

## Parallel Python Reduction Tranche

v0.93 should preserve room for the final Python-burn-down tranche if the
footprint has not already reached zero. The cross-milestone rule is recorded in
[Python Elimination Staged Plan](../../planning/PYTHON_ELIMINATION_STAGED_PLAN.md).

The likely `v0.93` tranche is:

- remaining low-count helpers and odd one-off scripts
- CI zero-tracked-Python enforcement if the footprint is already near zero
- parity audit and final cleanup rather than another large migration wave

## Scope Summary

### In scope

- Constitutional citizenship contract.
- Citizen, guest, human-provider, service-actor, and operator boundary.
- Rights and duties model.
- Bounded Theory of Mind and shared social memory.
- Reputation as a redacted, challengeable projection distinct from private ToM.
- Standing maintenance, degradation, restoration, suspension, and revocation.
- Constitutional review packet shape.
- Challenge and appeal flow.
- Delegation and IAM policy model.
- Zero-trust architecture and trust-boundary model.
- Policy enforcement, authorization, and least-privilege checks.
- Secrets, key lifecycle, signing, encryption, and rotation boundaries.
- Tamper-evident audit, compliance, and incident evidence.
- Tenant/polis isolation, data governance, retention, and privacy controls.
- Security operations, adversarial regression, provenance, and runtime
  hardening.
- Bounded social-contract representation.
- Reviewer-facing governance proof candidates.

### Out of scope

- Runtime implementation in this planning pass.
- Legal personhood.
- Production citizenship or complete constitutional authority.
- Full economics, payments, or markets.
- Replacing v0.90.3 citizen-state/security work.
- Replacing v0.91 moral trace.
- Replacing v0.92 identity and birthday semantics.
- Claiming complete enterprise certification, SOC 2, ISO 27001, FedRAMP,
  HIPAA, or other external compliance approval.
- Production cross-polis networking before the required transport-security
  prerequisites exist.
- Treating private Theory of Mind as public reputation or constitutional
  verdict.

## Document Map

- Vision: [VISION_v0.93.md](VISION_v0.93.md)
- Design: [DESIGN_v0.93.md](DESIGN_v0.93.md)
- WBS: [WBS_v0.93.md](WBS_v0.93.md)
- Sprint plan: [SPRINT_v0.93.md](SPRINT_v0.93.md)
- Decisions: [DECISIONS_v0.93.md](DECISIONS_v0.93.md)
- Demo matrix: [DEMO_MATRIX_v0.93.md](DEMO_MATRIX_v0.93.md)
- Milestone checklist: [MILESTONE_CHECKLIST_v0.93.md](MILESTONE_CHECKLIST_v0.93.md)
- Release plan: [RELEASE_PLAN_v0.93.md](RELEASE_PLAN_v0.93.md)
- Release notes: [RELEASE_NOTES_v0.93.md](RELEASE_NOTES_v0.93.md)
- Feature plans: [features/README.md](features/README.md)
- Constitutional citizenship and polis governance allocation:
  [CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md](CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md)
- Theory of Mind and social cognition:
  [THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md](features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.md)
- Enterprise security:
  [ENTERPRISE_SECURITY_v0.93.md](features/ENTERPRISE_SECURITY_v0.93.md)

## Execution Model

Later WP planning should preserve the standard milestone rhythm:

- WP-01: promote reviewed milestone docs and issue wave
- feature WPs: implement constitutional citizenship, ToM/social-cognition,
  standing, review, delegation, IAM, and social-contract surfaces
- security WPs: implement the six enterprise-security foundations for
  zero-trust polis operation
- demo WP: build constitutional/polis proof demos
- quality/review WPs: validate docs, tests, demo evidence, and review packets
- release WP: close the milestone under the normal ceremony pattern

The exact WP sequence is intentionally deferred until v0.93 planning is active.

## Success Criteria

v0.93 is ready to execute when:

- every governance feature consumes earlier substrate instead of redefining it
- the human/citizen boundary is explicit in docs, fixtures, and tests
- constitutional review packets are trace-grounded and privacy-preserving
- ToM, reputation, standing, and constitutional review remain distinct
- delegation and IAM decisions are reviewable
- demo candidates prove behavior rather than merely describing policy
- philosophical claims remain separated from implemented engineering behavior
