# v0.93 Demo Matrix: Candidate Constitutional Governance Proofs

## Status

Candidate demo planning only. Commands and artifacts will be finalized when the
v0.93 implementation WPs exist.

## Purpose

The v0.93 demo program should prove that constitutional citizenship and polis
governance are evidence-bearing runtime behavior, not rhetoric.

## Candidate Coverage Summary

| Demo ID | Candidate demo | Milestone claim | Primary proof surface | Status |
| --- | --- | --- | --- | --- |
| D1 | Constitutional review of a challenged action | A citizen action can be evaluated against rights, duties, policy, trace, outcome, and standing evidence. | Review packet with finding, evidence references, and appeal state. | Planned candidate |
| D2 | Standing degradation and restoration | Standing changes require evidence and can include restoration when conditions are met. | Standing transition fixture and reviewer summary. | Planned candidate |
| D3 | Human guest versus citizen-mode boundary | Human input does not become citizen action unless mediated through CSM identity, Freedom Gate, signed trace, and temporal anchoring. | Two-case fixture showing guest-only and mediated citizen-mode paths. | Planned candidate |
| D4 | Delegated authority chain | Delegated action is allowed or denied based on explicit authority and policy. | Delegation/IAM fixture with allow/deny decision events. | Planned candidate |
| D5 | Communication without inspection | Communication does not grant private-state inspection. | Communication event, redacted projection, and failed inspection attempt. | Planned candidate |
| D6 | Polis governance health packet | Reviewers can inspect governance health without scalar moral verdicts or raw private-state exposure. | Governance report with evidence references, caveats, and redactions. | Planned candidate |

## Demo Rules

- Every demo must identify the actor class: citizen, guest, human provider,
  operator, service actor, tool, or external counterparty.
- Every governance finding must cite evidence rather than narrative alone.
- Every private-state boundary must have a redaction or denial proof.
- Every denial must explain authority and policy without leaking protected data.
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

### D6) Polis Governance Health Packet

The demo should generate a review packet over a small polis state.

Expected proof:

- standing distribution
- open challenges or appeals
- governance findings
- redaction report
- caveats and unresolved risks

## Non-Claims

- These demos do not prove production citizenship.
- These demos do not establish legal personhood.
- These demos do not replace v0.91 moral trace or v0.92 identity work.
- These demos do not expose raw private state.
