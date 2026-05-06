# v0.93 Demo Matrix: Candidate Constitutional Governance Proofs

## Status

Candidate demo planning only. Commands and artifacts will be finalized when the
v0.93 implementation WPs exist.

## Purpose

The v0.93 demo program should prove that constitutional citizenship, bounded
social cognition, and polis governance are evidence-bearing runtime behavior,
not rhetoric. It should also prove that enterprise-security controls are
reviewable behavior rather than perimeter language.

## Candidate Coverage Summary

| Demo ID | Candidate demo | Milestone claim | Primary proof surface | Status |
| --- | --- | --- | --- | --- |
| D1 | Constitutional review of a challenged action | A citizen action can be evaluated against rights, duties, policy, trace, outcome, and standing evidence. | Review packet with finding, evidence references, and appeal state. | Planned candidate |
| D2 | Standing degradation and restoration | Standing changes require evidence and can include restoration when conditions are met. | Standing transition fixture and reviewer summary. | Planned candidate |
| D3 | Human guest versus citizen-mode boundary | Human input does not become citizen action unless mediated through CSM identity, Freedom Gate, signed trace, and temporal anchoring. | Two-case fixture showing guest-only and mediated citizen-mode paths. | Planned candidate |
| D4 | Delegated authority chain | Delegated action is allowed or denied based on explicit authority and policy. | Delegation/IAM fixture with allow/deny decision events. | Planned candidate |
| D5 | Communication without inspection | Communication does not grant private-state inspection. | Communication event, redacted projection, and failed inspection attempt. | Planned candidate |
| D6 | ToM and reputation boundary | Private ToM can inform later cognition only through authorized, redacted, evidence-grounded projections; reputation is not the private model. | Private model fixture, signed update event, reputation projection, redaction report, denied unauthorized inspection. | Planned candidate |
| D7 | Conflict and decay in shared social memory | Stale or contradictory social cognition is marked, downgraded, and reviewable rather than silently overwritten. | Conflict fixture, decay event, social-memory projection, arbitration impact note. | Planned candidate |
| D8 | Polis governance health packet | Reviewers can inspect governance health without scalar moral verdicts, private ToM leakage, or raw private-state exposure. | Governance report with evidence references, caveats, and redactions. | Planned candidate |
| D9 | Zero-trust denied action | A citizen, service, tool, or operator action crossing a trust boundary is denied without identity, standing, capability, and policy authority. | Deny-by-default fixture with policy decision, boundary record, and redacted explanation. | Planned candidate |
| D10 | Key rotation and revocation | Cryptographic trust changes when a key is rotated or revoked, and stale signatures/messages/sealed-state access fail. | Key lifecycle fixture, accepted-before/denied-after cases, and audit record. | Planned candidate |
| D11 | Audit and incident evidence packet | Security review can inspect what happened without leaking private state or claiming external certification. | Tamper-evident audit entries, incident record, redaction report, and reviewer packet. | Planned candidate |
| D12 | Isolation and data-governance leakage prevention | Cross-polis, cross-tenant, or cross-citizen data access is blocked or redacted according to classification and retention policy. | Isolation negative case, data-classification record, retention/projection decision, and denial proof. | Planned candidate |

## Demo Rules

- Every demo must identify the actor class: citizen, guest, human provider,
  operator, service actor, tool, or external counterparty.
- Every governance finding must cite evidence rather than narrative alone.
- Every private-state boundary must have a redaction or denial proof.
- Every private-ToM boundary must have a redaction or denial proof.
- Every denial must explain authority and policy without leaking protected data.
- Every security demo must cite the trust boundary, policy decision, key or
  secret lifecycle state where relevant, audit evidence, and redaction outcome.
- Demo outputs should distinguish engineering evidence from policy
  interpretation.

## Candidate Details

### D1) Constitutional Review Of A Challenged Action

The demo should replay a synthetic incident where a citizen action is challenged
under the polis constitution.

Expected proof:

- citizen identity and standing snapshot
- relevant policy and rights/duties context
- moral trace event references
- outcome and attribution references
- finding and appeal disposition
- redaction notes

### D2) Standing Degradation And Restoration

The demo should show a standing transition that is neither arbitrary nor
permanent by default.

Expected proof:

- evidence-backed degradation or restriction
- challenge or review context
- restoration criteria
- restored or still-restricted disposition with rationale

### D3) Human Guest Versus Citizen-Mode Boundary

The demo should show that human participation is allowed while preserving the
citizen boundary.

Expected proof:

- guest-mode human input remains guest/operator activity
- citizen-mode action requires identity binding, Freedom Gate mediation, signed
  trace, and temporal anchoring
- direct out-of-band human action is rejected as citizen conduct

### D4) Delegated Authority Chain

The demo should show a delegated action request where authority is either
accepted or denied based on explicit policy.

Expected proof:

- actor identity and standing
- delegation source
- capability or IAM record
- allowed/denied action decision
- trace evidence for the final disposition

### D5) Communication Without Inspection

The demo should show two citizens or a citizen and guest communicating without
private-state inspection.

Expected proof:

- governed communication event
- consent or authorization record where needed
- redacted projection
- failed inspection attempt

### D6) ToM And Reputation Boundary

The demo should show that a private ToM model can be updated from evidence and
then projected into reputation only through explicit policy.

Expected proof:

- signed ToM update event with evidence references
- model diff with confidence basis
- private model retained as non-public evidence
- reputation projection with redaction notes
- unauthorized inspection refusal

### D7) Conflict And Decay In Shared Social Memory

The demo should show that social cognition remains uncertain and temporal.

Expected proof:

- two contradictory or aging model entries
- conflict group or decay event
- downgraded or unresolved confidence state
- shared social-memory projection that preserves uncertainty
- arbitration or review note showing that stale/conflicted ToM does not become
  a final verdict

### D8) Polis Governance Health Packet

The demo should generate a review packet over a small polis state.

Expected proof:

- standing distribution
- social cognition and reputation projection summary
- open challenges or appeals
- governance findings
- redaction report
- caveats and unresolved risks

### D9) Zero-Trust Denied Action

The demo should show that no actor receives implicit trust merely because it is
inside the polis.

Expected proof:

- actor identity and standing
- requested boundary crossing
- required capability, IAM, delegation, or tool authority
- deny-by-default decision
- redacted explanation that does not leak protected state

### D10) Key Rotation And Revocation

The demo should show that cryptographic trust is lifecycle-managed.

Expected proof:

- initial accepted signed or encrypted action
- key rotation or revocation record
- stale signature, message, or sealed-state access denied after revocation
- audit record connecting the trust change to the denial

### D11) Audit And Incident Evidence Packet

The demo should generate a security review packet for a synthetic incident.

Expected proof:

- tamper-evident audit entries
- incident scope and actor boundary
- policy, key, isolation, or provenance evidence
- redaction report
- explicit non-certification language

### D12) Isolation And Data-Governance Leakage Prevention

The demo should show a blocked or redacted data access across a protected
boundary.

Expected proof:

- data classification
- tenant/polis/citizen boundary
- retention, deletion, or projection rule
- denied or redacted access result
- leakage-prevention assertion

## Non-Claims

- These demos do not prove production citizenship.
- These demos do not establish legal personhood.
- These demos do not replace v0.91 moral trace or v0.92 identity work.
- These demos do not expose raw private state.
- These demos do not expose raw private ToM.
- These demos do not make reputation, standing, or constitutional judgment from
  private ToM without authority and redaction.
- These demos do not prove external enterprise certification or production
  compliance approval.
