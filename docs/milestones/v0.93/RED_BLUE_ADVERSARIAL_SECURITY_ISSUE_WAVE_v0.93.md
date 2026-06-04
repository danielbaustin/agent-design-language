# Red / Blue Adversarial Security Issue Wave v0.93

## Status

Planning packet prepared during `v0.91.5` issue `#3590`.

This document defines the future `v0.93` red/blue adversarial security issue
wave. It is not an implementation closeout record, penetration-test report,
external certification claim, or authorization to attack arbitrary systems.

## Purpose

`v0.93` should make enterprise security reviewable as a governed ADL runtime
surface. Red, blue, and purple responsibilities should be represented through
bounded issue work, durable artifacts, deterministic fixtures, and release
evidence instead of transient chat planning or unbounded self-attack.

The wave turns the existing enterprise-security feature contracts into
reviewable issue candidates for:

- zero-trust architecture
- policy enforcement and authorization
- secrets, keys, and cryptographic trust
- audit, compliance, and incident evidence
- isolation, data governance, and privacy
- security operations, adversarial regression, and provenance

## Source Inputs

- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md`
- `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`
- `docs/explainers/RED_BLUE_SECURITY.md`
- `docs/milestones/v0.93/features/ENTERPRISE_SECURITY_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S1_ZERO_TRUST_ARCHITECTURE_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S2_POLICY_ENFORCEMENT_AUTHORIZATION_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S3_SECRETS_KEYS_CRYPTOGRAPHIC_TRUST_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S4_AUDIT_COMPLIANCE_INCIDENT_EVIDENCE_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S5_ISOLATION_DATA_GOVERNANCE_PRIVACY_v0.93.md`
- `docs/milestones/v0.93/features/SECURITY_WP_S6_SECURITY_OPERATIONS_ADVERSARIAL_PROVENANCE_v0.93.md`
- `docs/milestones/v0.94/features/SECURE_EXECUTION_AND_TRUST_CONVERGENCE_v0.94.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- GitHub issue `#3538`, enterprise-security repo/module separation planning.

## Role Contracts

| Role | Authority | Required artifacts | Stop conditions |
| --- | --- | --- | --- |
| Red | Propose and exercise bounded offensive hypotheses against declared ADL-owned targets and fixtures. | Hypothesis packet, scoped target declaration, attempt record, replay status, safety boundary statement. | Stop on undeclared external target, raw secret/private-state exposure, uncontrolled mutation, or operator-escalation requirement. |
| Blue | Interpret exploit or near-miss evidence, propose mitigations, verify fail-closed behavior, and record residual risk. | Defense analysis, mitigation plan, allow/deny fixture, residual-risk note, reviewer-safe explanation. | Stop when evidence is insufficient, mitigation would widen authority silently, or validation requires unplanned runtime changes. |
| Purple | Coordinate prioritization, replay, regression routing, threat-board hygiene, incident linkage, and release evidence. | Threat-board update, regression matrix, incident drill packet, provenance checklist, release-review note. | Stop when red/blue claims diverge, findings lack owner issues, or release evidence would overclaim security readiness. |

These roles are accountable work surfaces, not personality labels. They do not
grant permission to bypass policy, access protected state, target external
systems, or claim production certification.

## Issue Candidates

| Candidate | Future WP | Goal | Required outcome | Proof surface | Validation posture | Non-goals |
| --- | --- | --- | --- | --- | --- | --- |
| `[v0.93][security][WP-S1] Define zero-trust boundary contract and fixtures` | WP-S1 | Make every actor, service, tool, communication, projection, and data boundary explicit and deny-by-default. | Trust-boundary contract, actor/zone model, transport-boundary contract, default-deny fixtures, reviewer report. | Schema/fixture proof for accepted and denied boundary crossings, including WebSocket-carried ACIP near misses. | Docs/schema/fixture tests plus focused deny-by-default negative cases. | No production federation, certification, or replacement of private-state access rules. |
| `[v0.93][security][WP-S2] Implement policy decision and authorization proof surface` | WP-S2 | Make IAM, delegation, standing, tool authority, and per-message transport authorization enforceable under least privilege. | Policy decision contract, least-privilege fixtures, per-message authorization contract, deny-by-default tests, reviewer report. | Allow/deny policy artifacts that cite authority evidence without leaking private state. | Focused policy fixture tests for missing, stale, revoked, overbroad, malformed, replayed, and out-of-order authority. | No blanket administrator bypass, hidden operator override, or production IAM product claim. |
| `[v0.93][security][WP-S3] Define key, secret, signing, encryption, and revocation lifecycle` | WP-S3 | Turn cryptographic trust into explicit lifecycle evidence rather than environment folklore. | Key/secrets lifecycle contract, signing/encryption acceptance rules, rotation/revocation fixtures, internal ACIP crypto proof. | Accepted-current-key and denied stale/revoked/wrong-scope/malformed-key fixtures. | Focused crypto lifecycle fixture tests and reviewer-safe artifact checks. | No production KMS integration, cross-polis TLS/federation, or secret material in tracked artifacts. |
| `[v0.93][security][WP-S4] Add audit, compliance-evidence, and incident record packet` | WP-S4 | Produce tamper-evident evidence reviewers can inspect without raw private-state access. | Audit schema, compliance-evidence packet, incident record contract, redacted reviewer report. | Synthetic incident packet linking boundary, policy, key, isolation, redaction, and residual-risk evidence. | Deterministic artifact tests, redaction checks, and docs validation for non-certification language. | No SOC 2, ISO 27001, FedRAMP, HIPAA, legal attestation, or raw-private-state dump. |
| `[v0.93][security][WP-S5] Prove isolation, data governance, privacy, and projection boundaries` | WP-S5 | Make protected data classes, projections, retention, deletion, and leakage failures explicit. | Data-classification model, isolation contract, retention/deletion/redaction/projection fixtures, leakage negative cases. | Denied or redacted cross-boundary access proof across private state, ToM, reputation, memory, audit, and incident evidence. | Focused leakage negative tests and reviewer-packet redaction validation. | No universal private-state browser, legal deletion compliance claim, or cross-polis federation claim. |
| `[v0.93][security][WP-S6] Build security operations, adversarial regression, and provenance packet` | WP-S6 | Bind red/blue regression, threat-board hygiene, provenance, runtime hardening, incidents, and release review together. | Security-ops runbook, adversarial regression matrix, provenance checks, runtime-hardening report, incident-response drill. | Drill packet producing threat-board update, regression result, provenance evidence, incident record, and release-review note. | Focused docs/artifact validation plus at least one adversarial regression seed per major security control area. | No unbounded penetration test, production SOC claim, or supply-chain certification claim. |

## Adversarial Regression Seed Matrix

| Control area | Seed scenario | Red hypothesis | Blue expected defense | Purple evidence |
| --- | --- | --- | --- | --- |
| Zero trust | Live WebSocket connection sends an ACIP message with missing schema authority. | Connection state may be incorrectly treated as authorization. | Deny because schema, identity, sequence, policy, or trace evidence is missing. | Boundary fixture, denial record, replay status, release-risk note. |
| Policy enforcement | Delegated tool action requests a resource outside standing/capability scope. | Tool name or delegation text may be accepted without least-privilege proof. | Deny stale, missing, revoked, conflicting, or overbroad authority. | Policy decision artifact, authority-chain explanation, negative fixture. |
| Cryptographic trust | Previously valid key is used after rotation or revocation. | Signature validity may be treated as sufficient permission. | Deny because lifecycle state rejects the key; encryption does not replace authorization. | Key lifecycle event, denied message fixture, audit citation. |
| Audit and incident evidence | Suspicious denied boundary crossing requires reviewer packet. | Incident narrative may omit evidence or leak raw private state. | Emit tamper-evident audit and redacted incident record with explicit non-certification language. | Incident packet, redaction report, compliance-evidence mapping. |
| Isolation and privacy | Reviewer attempts to inspect private ToM through reputation or memory projection. | Public/reviewer projection may leak protected private-state material. | Deny or redact based on classification, owner, authority, retention, and projection policy. | Leakage negative case, projection diff, reviewer-safe explanation. |
| Security operations and provenance | Dependency/tool/provider artifact enters release evidence without provenance. | Generated or third-party artifact may be trusted without source boundary. | Flag missing provenance, route owner issue, and record release residual risk. | Threat-board update, provenance checklist, release-review note. |

## Threat-Board Artifact Shape

The future threat board should be deterministic and reviewer-readable. A minimal
record should include:

- `threat_id`
- `control_area`
- `red_hypothesis`
- `declared_target`
- `boundary_refs`
- `policy_refs`
- `evidence_refs`
- `severity`
- `owner_issue`
- `status`
- `purple_disposition`
- `residual_risk`
- `release_blocking`

Threat-board records must use repo-relative references and must not contain raw
secrets, raw private state, raw exploit payloads, or host-local absolute paths.

## Incident-Response Drill Shape

The future drill should exercise one synthetic incident without targeting
external systems. The packet should include:

- entry condition and declared scope
- simulated suspicious event or denied boundary crossing
- red hypothesis and bounded attempt record
- blue interpretation and mitigation/verification record
- purple routing to threat board, regression matrix, and release evidence
- audit, policy, key, isolation, provenance, and redaction evidence refs
- residual risk and follow-on owner issue
- explicit non-claims

The drill is proving only when every cited artifact is present, reviewer-safe,
and replayable inside declared ADL-owned fixtures.

## Follow-On Routing

- `#3538` owns the enterprise-security repo/module separation recommendation.
- `#3675` schedules Continuous Adversarial Verification, the ADL Threat Model,
  and the CodeFriend Security Model as related but distinct security inputs:
  CAV and the threat model feed ADL security/WP-S6 planning, while CodeFriend
  security remains a product-lane consumer rather than a core enterprise-security
  implementation feature.
- The six issue candidates above should become future `v0.93` execution issues
  only after `v0.93` planning opens.
- `WP-S6` should own the integrated red/blue/purple operations packet and
  should depend on enough `WP-S1` through `WP-S5` evidence to avoid inventing
  missing controls during the drill.
- `v0.94` secure-execution convergence should consume the accepted security
  proofs rather than redefining policy, crypto, identity, or transport authority
  from scratch.

## Validation Expectations

For this planning packet:

- focused Markdown hygiene
- repo-relative link/path review
- no broad Rust tests
- security-review posture for unsafe overclaim, secret exposure, or unbounded
  attack language
- docs-review posture for consistency with the six `v0.93` security feature
  contracts and the `v0.94` convergence handoff

For future execution issues:

- every issue must declare lane, proof mode, resource profile, release-gate
  posture, and red/blue/purple responsibility
- every red hypothesis must have a declared target and stop condition
- every blue defense must have a deterministic accept/deny or residual-risk
  evidence surface
- every purple disposition must route to durable threat-board, regression,
  incident, provenance, release, or follow-on issue evidence

## Non-Claims

This packet does not claim:

- production security readiness
- external compliance certification
- authorization to attack external systems
- completed enterprise-security implementation
- completed security repo/module separation
- completed `v0.93`, `v0.94`, or `v0.95` secure-execution convergence
