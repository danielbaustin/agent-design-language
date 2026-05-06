# v0.93 Release Plan

## Status

Forward release plan. v0.93 has not started implementation.

## Release Readiness Themes

v0.93 should not be released until it has evidence for:

- constitutional citizenship contract
- human/guest/operator/service/tool/citizen boundary
- rights and duties model
- Theory of Mind schema and signed update-event contract
- reputation and shared social memory boundary
- standing transition evidence
- constitutional review packet
- challenge and appeal flow
- delegation and IAM authority evidence
- zero-trust trust-boundary model and deny-by-default evidence
- policy-enforcement and least-privilege authorization evidence
- secrets/key lifecycle, signing, encryption, rotation, and revocation evidence
- tamper-evident audit, compliance-evidence, and incident-record surfaces
- tenant/polis isolation, data-governance, retention, projection, and privacy
  evidence
- security operations, adversarial regression, provenance, and runtime-hardening
  evidence
- at least one governance proof demo
- at least one enterprise-security proof demo or integrated security proof row
- privacy-preserving reviewer packet

## Review Gates

- Docs must separate engineering substrate, policy model, and contextual claims.
- Release notes must not claim legal personhood, production citizenship, or
  complete constitutional authority.
- Demo evidence must cite concrete artifacts, not policy prose alone.
- Redaction checks must cover private state, host paths, endpoints, and
  secret-like strings.
- Redaction checks must cover private ToM and reputation projections.
- Security review must check default-deny behavior, authority boundaries,
  key/secrets lifecycle, audit integrity, isolation, provenance, and incident
  evidence.
- Release docs must not claim external certification or production compliance
  approval.
- The final review handoff must identify which v0.90.3, v0.91, and v0.92
  prerequisite surfaces were consumed.

## Closeout Notes For Later WP-20

The release ceremony should follow the exact closeout pattern used by recent
milestones at the time v0.93 closes. If the standard ceremony script or release
tail changes before then, WP-20 should point to the then-current canonical
pattern rather than inventing a new one.

## Non-Release Conditions

Do not ship v0.93 if:

- standing changes can be made without evidence
- review packets expose raw private state
- reputation or constitutional findings expose private ToM without authority and
  redaction
- human out-of-band action can masquerade as citizen action
- delegation can bypass policy
- zero-trust trust-boundary decisions can be bypassed or default-allow
- secrets, keys, signatures, encryption, rotation, or revocation remain hidden
  implementation folklore rather than reviewable lifecycle records
- audit, compliance, or incident packets cannot be reviewed without leaking
  private state
- isolation and data-governance negative cases are missing
- adversarial regression, provenance, or runtime-hardening evidence is absent
- constitutional review duplicates or contradicts moral trace
- release notes describe planned social or legal authority as landed behavior
