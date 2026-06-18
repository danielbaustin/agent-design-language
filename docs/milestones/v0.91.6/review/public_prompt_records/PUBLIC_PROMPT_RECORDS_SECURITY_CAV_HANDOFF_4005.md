# Public Prompt Records Security Review And CAV Handoff for #4005

## Scope

This packet records the bounded security review and CAV handoff for the public
prompt-record surface defined in WP-04. It is not a full-repository threat
model, not a completed CAV program, and not a public distribution approval
record.

## Source evidence

- [SECURITY_BRIDGE_AND_CAV_v0.91.6.md](../../features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md)
- [PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md](../../features/PUBLIC_PROMPT_RECORDS_EXPORT_v0.91.6.md)
- [PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md](PUBLIC_PROMPT_RECORDS_REDACTION_PUBLICATION_SAFETY_4003.md)
- [PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md](PUBLIC_PROMPT_RECORDS_VALIDATION_INDEXING_4004.md)
- [V0915_SECOND_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-17.md](../../../v0.91.5/review/internal_review/V0915_SECOND_INTERNAL_REVIEW_FINDINGS_REGISTER_2026-06-17.md)
- [V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md](../../../v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md)

## Review goal

Determine whether the current public prompt-record contract has enough reviewed
security posture to remain on the activation path without claiming broader
security closure, and record which residuals must be carried into WP-07/CAV
before distribution proof and closeout.

## Review classes

`#4005` uses three result classes.

1. `reviewed_and_currently_covered`
2. `reviewed_and_routed`
3. `distribution_blocker`

These are review-result classes for this packet, not product enums.

## Evidence matrix

| Review class | Meaning | Representative evidence | Current disposition |
| --- | --- | --- | --- |
| `reviewed_and_currently_covered` | The abuse class is already bounded by the current public prompt packet contract and tracked proof surfaces. | `#4003` publication-safety packet; `#4004` validation/indexing packet | pass_with_current_contract |
| `reviewed_and_routed` | The abuse class has been reviewed here, but broader adversarial/security closure belongs to WP-07 or later distribution proof. | `SECURITY_BRIDGE_AND_CAV_v0.91.6.md`; the residual boundaries in `#4003` and `#4004` | routed_to_wp07 |
| `distribution_blocker` | Distribution must remain blocked until this class is reviewed, fixed, or explicitly routed. | This packet plus the shared feature doc | block_until_reviewed_or_routed |

## Abuse-class review

| Abuse class | Why it matters | Current evidence | Result |
| --- | --- | --- | --- |
| Secret exposure in public packets or reviewer/public projections | Leaks credentials or secret-bearing prompt artifacts into durable public state. | `#4003` packet records refuse-not-rewrite packet posture plus explicit redacted projections; v0.91.5 review findings show the raw OpenRouter prompt/output issue and its repair. | `reviewed_and_currently_covered` |
| Raw provider log or raw provider payload exposure | Durable public artifacts can become over-broad or publish unsafe provider output. | `#4003` packet plus v0.91.5 internal/external review evidence on OpenRouter raw prompt/output remediation. | `reviewed_and_currently_covered` |
| Private path leakage / worktree-local provenance leakage | Public packet provenance can falsely expose machine-local state or make packets non-portable. | `#4003` and `#4004` packets cover host-path/worktree provenance refusal and repo-relative packet expectations. | `reviewed_and_currently_covered` |
| Misleading provenance, spoofed tracker identity, or invalid packet metadata | Packets can look canonical or reviewed when they are not, corrupting downstream trust. | `#4004` validator contract covers tracker/provider/provenance/refused-packet rules. | `reviewed_and_currently_covered` |
| Prompt injection or malicious text inside packet/projection content | Reviewers or downstream consumers can be misled by hostile text even if the packet is structurally valid. | The current packet/export contract does not claim dynamic adversarial or semantic-injection resistance; `SECURITY_BRIDGE_AND_CAV_v0.91.6.md` keeps malformed-output/prompt-injection handling on the activation path. | `reviewed_and_routed` |
| Provider-trust confusion or transformed-artifact ambiguity | A redacted/public projection can be mistaken for raw authoritative history, or provider trust assumptions can leak into publication safety. | `#4003` distinguishes allowed packet vs redacted projection, but broader provider/model trust remains owned by the security bridge lane. | `reviewed_and_routed` |
| Cross-surface security with ACIP/A2A/provider communications | Public prompt records may interact with broader message, provider, or adversarial surfaces outside the packet contract. | `SECURITY_BRIDGE_AND_CAV_v0.91.6.md` explicitly keeps those boundaries in scope for the security bridge lane, not WP-04 alone. | `reviewed_and_routed` |

## Findings and dispositions

### Current findings

1. Distribution must remain blocked until the public prompt-record surface has a
   bounded security review result.  
   Disposition: fixed by this packet and the shared feature-doc update.

2. Dynamic adversarial/prompt-injection/provider-trust work is still broader
   than the current packet contract.  
   Disposition: explicitly routed to WP-07 / Security Bridge And CAV.

3. Public prompt records must not be treated as security-cleared merely because
   export, redaction, and validation contracts exist.  
   Disposition: fixed in the shared feature doc; retained as an explicit
   non-claim for distribution proof.

## Distribution decision for WP-04

Current decision:

- `distribution_blocked_pending_wp07_and_wp04_closeout`

Reason:

- the public prompt-record surface now has a bounded security review and honest
  routing truth
- but broader CAV/adversarial/provider-trust residuals remain intentionally
  routed to WP-07
- and final public distribution proof still belongs to `#4006`

## Handoff to WP-07

WP-07 should consume the following residual security items from this packet:

- dynamic adversarial verification for prompt-injection-style or misleading
  packet/projection content
- broader provider/model trust boundaries beyond the current packet contract
- malformed-output and semantic-confusion checks that exceed static packet
  validation and public-safety rules
- any cross-surface security implications involving ACIP/A2A/provider-message
  trust that affect public prompt-record publication posture

## What this packet does not claim

This packet does not claim:

- completed CAV coverage
- full threat-model closure for ADL
- public distribution approval for prompt records
- that all provider/model trust issues are solved in WP-04
- that prompt injection is fully mitigated by the current packet contract

## Reviewer takeaway

`#4005` is ready when reviewers can confirm that:

- public prompt records are now explicitly reviewed on the activation-path
  security boundary
- already-covered packet safety classes stay in WP-04
- broader adversarial/provider/security residuals are truthfully handed to WP-07
- distribution remains blocked until `#4006` and routed security work are done
