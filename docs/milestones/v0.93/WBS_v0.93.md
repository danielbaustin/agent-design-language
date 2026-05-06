# v0.93 Candidate Work Breakdown Structure

## Status

Candidate allocation only. v0.93 has no final issue wave yet.

The exact WP sequence should be produced by the v0.93 WP-01 planning pass after
v0.90.3, v0.91, and v0.92 have landed enough prerequisite evidence.

## WBS Summary

v0.93 should develop the constitutional citizenship, social-cognition, and
polis-governance layer without stealing work from the earlier substrate
milestones.

## Candidate Work Areas

| Candidate | Work Area | Description | Primary deliverable | Key dependencies |
| --- | --- | --- | --- | --- |
| A | Constitutional citizenship contract | Define eligibility, rights, duties, standing, review, and non-goals. | Feature contract and fixtures. | v0.90.3 standing/state, v0.92 identity. |
| B | Human, guest, operator, and citizen-mode boundary | Make guest-by-default human entry and mediated citizen action explicit. | Boundary doc, policy fixtures, negative cases. | v0.90.3 standing, v0.91 Freedom Gate, v0.92 identity. |
| C | Rights and duties model | Define what the polis owes citizens and what citizens owe the polis. | Rights/duties schema or contract. | v0.91 moral resources and wellbeing context. |
| D | Theory of Mind contract | Define private evidence-grounded models of other participants without turning them into public verdicts. | ToM schema, update event contract, evidence/confidence rules, conflict and decay fixtures. | v0.90.3 standing/access, v0.91 moral trace, v0.92 identity. |
| E | Reputation and shared social memory boundary | Define public or governance-facing summaries that remain redacted, challengeable, and distinct from private ToM. | Reputation projection and shared social memory contract. | D plus v0.91 review evidence. |
| F | Standing maintenance and degradation | Define evidence-based transitions among good standing, monitored, restricted, suspended, restored, and revoked. | Standing transition tests and review packet. | v0.90.3 standing and challenge flow. |
| G | Constitutional review packet | Consume trace, outcome, attribution, policy, ToM projections where allowed, reputation, and standing evidence. | Review packet schema and fixtures. | v0.91 moral trace and trajectory review. |
| H | Challenge and appeal flow | Preserve evidence, allow challenge, record appeal disposition, and avoid arbitrary punishment. | Challenge/appeal state machine and proof fixture. | v0.90.3 challenge/quarantine, v0.91 review evidence. |
| I | Delegation and IAM | Model delegated authority across citizens, guests, services, operators, and tools. | Authority-chain model and allow/deny fixtures. | v0.90.5 governed tools if landed. |
| J | Communication without inspection | Ensure governed communication does not create private-state or private-ToM inspection rights. | Communication/inspection negative proof. | v0.90.3 communication and projection policy. |
| K | Social contract representation | Represent the bounded obligations of the polis and citizens. | Draft social-contract contract and review notes. | A through J. |
| L | Polis governance health evidence | Summarize governance state without scalar moral verdicts, leaked private state, or leaked private ToM. | Governance evidence packet and redacted report. | G through K. |
| M | Security WP-S1: zero-trust architecture | Define the polis trust model, actor identities, trust boundaries, default-deny zones, and cross-boundary verification rules. | Zero-trust architecture contract, trust-boundary fixtures, and negative cases. | A, B, I, v0.90.3 access/projection, v0.90.5 governed tools, v0.92 identity. |
| N | Security WP-S2: policy enforcement and authorization | Make IAM, delegation, standing, tool authority, and citizen/action policy enforceable with least privilege and fail-closed behavior. | Policy decision contract, enforcement fixtures, deny-by-default tests, and reviewer report. | I, M, v0.90.5 ACC/UTS, v0.92 capability envelopes. |
| O | Security WP-S3: secrets, keys, and cryptographic trust | Specify key custody, signing, encryption, rotation, revocation, sealed-state access, and internal ACIP cryptographic requirements. | Key/secrets lifecycle contract, rotation/revocation fixtures, encrypted-message proof surface. | M, N, v0.90.3 signed envelopes/sealing, v0.91 ACIP, v0.92 identity. |
| P | Security WP-S4: audit, compliance, and incident evidence | Produce tamper-evident audit trails, compliance evidence packets, incident records, and redaction-safe reviewer views without claiming external certification. | Audit schema, compliance-evidence packet, incident fixture, and redacted report. | G, H, L, N, O, moral trace, standing evidence. |
| Q | Security WP-S5: isolation, data governance, and privacy | Define tenant/polis isolation, data classification, retention, deletion, projection, private-state privacy, and cross-actor data-flow controls. | Isolation/data-governance contract, retention/projection fixtures, leakage negative cases. | B, J, M, O, v0.90.3 private-state/access/projection, v0.92 memory grounding. |
| R | Security WP-S6: security operations, adversarial regression, and provenance | Bind red/blue adversarial testing, supply-chain/provenance checks, runtime hardening, threat-board hygiene, and incident-response drills into the milestone. | Security-ops runbook, adversarial regression suite, provenance checks, threat-board and incident-response evidence. | M through Q plus v0.89.1 adversarial runtime and current CI/review gates. |
| S | Demo matrix and proof demos | Build constitutional review, ToM/reputation boundary, standing transition, delegation/IAM, enterprise-security, and guest/citizen boundary demos. | Demo matrix rows and runnable proof commands. | A through R. |
| T | Review, docs, and release tail | Align docs, update feature list, run review, and close the milestone. | Review handoff, release notes, ceremony evidence. | All prior work. |

## Sequencing Pressure

1. Start with the citizenship contract and actor boundary.
2. Add rights, duties, and the ToM/reputation/shared-social-memory boundary.
3. Add standing transition semantics.
4. Add review packet, challenge, and appeal.
5. Add delegation and IAM after authority prerequisites are clear.
6. Add communication and social-contract surfaces.
7. Add the six enterprise-security WPs after identity, IAM, and tool authority
   prerequisites are explicit.
8. Build proof demos only after the contracts can constrain them.

## Acceptance Mapping

- Constitutional citizenship must be tied to identity, standing, trace, and
  policy, not merely existence in the runtime.
- Human provider participation must remain guest or operator activity unless a
  CSM identity mediates the action as citizen conduct.
- Constitutional review must cite trace/outcome/standing evidence.
- Standing changes must be evidence-based, reviewable, and appealable.
- Private ToM must not become public reputation, standing, or constitutional
  judgment without redaction, authority, and evidence.
- Delegation and IAM must fail closed when authority is missing.
- Zero-trust policy must default deny at every polis, tool, service, operator,
  communication, and data boundary.
- Secrets, keys, signatures, encryption, rotation, and revocation must be
  represented as lifecycle contracts rather than hidden environment folklore.
- Audit, compliance, and incident records must be tamper-evident and
  redaction-safe without claiming external certification.
- Tenant/polis isolation and data-governance rules must prevent private-state,
  ToM, reputation, and citizen data leakage across boundaries.
- Security operations must connect adversarial regression, provenance,
  runtime-hardening, and threat-board evidence into release review.
- Public or operator-facing projections must not leak private state.
- Demos must show behavior and evidence, not just policy text.
