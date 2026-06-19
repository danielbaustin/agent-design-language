# Security Bridge And Continuous Adversarial Verification

## Metadata

- Feature Name: Security Bridge And CAV
- Milestone Target: `v0.91.6`
- Status: in_progress
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: review, tests, threat-model

## Purpose

Keep security on the activation path by defining first-tranche threat-model,
CAV, malformed-output, provider-trust, prompt-record, and ACIP security work.

## Scope

In scope:

- threat-model refresh;
- Continuous Adversarial Verification route;
- malformed output and prompt-injection handling;
- provider/model trust boundaries;
- public prompt record security;
- ACIP/A2A access and message security.

Out of scope:

- external compliance certification;
- full enterprise security implementation;
- residual `v0.91.7` security closure.

## Required Decisions

- Which security checks block public prompt export?
- Which provider/model failures are security-relevant?
- Which ACIP/A2A messages require access checks or signing?
- Which residual security work blocks `v0.92`?

## Dependencies

- Public prompt records feature doc.
- Provider/model reliability feature doc.
- ACIP/A2A/provider communications feature doc.
- `v0.93` enterprise security planning.

## Security Bridge Completion Ledger

This ledger is the WP-07 closeout gate for activation-path security. WP-07
cannot be called complete from bridge-doc presence, generic milestone status,
or vague "security reviewed" language. Each required surface must remain tied
to a concrete owning issue state, named proof or review packet, and an explicit
residual route when the surface is still open.

### Completion accounting rules

- `closed` issue state plus named proof or review surfaces may count as
  completed for this ledger.
- `open` issue state means the surface remains active even if a bridge doc or
  planning packet already exists.
- Cross-surface security claims must name the downstream issue that owns the
  remaining work instead of collapsing that residual into WP-07 narrative text.
- A residual is acceptable only when it names both an owner and a target
  milestone or routed guard.
- WP-07 closeout must consume this ledger row by row rather than re-inferring
  milestone security status from chat or memory.

### Surface inventory and dependency table

| Surface | Current owner / issue state | Security question carried by WP-07 | Required proof or review surface | Current ledger state | Residual owner / target |
| --- | --- | --- | --- | --- | --- |
| WP-03 logging, redaction, and projection observability dependency | `#3968` closed; `#3995`-`#4001` closed | Are logging, projection, token, and path-redaction boundaries strong enough for later security-consuming lanes? | WP-03 closeout and proof surfaces, especially logging validation/redaction and GitHub/projection observability packets from `#4000` and `#4001` | prerequisite complete and consumable | WP-07 consuming lanes `#4022` and `#4023` must use this proof; no new WP-03 implementation is required unless a later review finds a gap |
| WP-04 public prompt records export and publication safety | `#3969` closed; security handoff `#4005` closed | Are public prompt records export, redaction, validation, and publication safety complete enough to be consumed without treating local authoring state as public truth? | `docs/milestones/v0.91.6/review/public_prompt_records/PUBLIC_PROMPT_RECORDS_SECURITY_CAV_HANDOFF_4005.md`, the WP-04 closeout packet set, and the WP-07 packet `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` | completed with security handoff; cross-memory/publication privacy consumption now reviewed with explicit residual routes | WP-07 `#4022` consumes WP-04 as a prerequisite; broader adversarial/security residuals stay visible under WP-07 and the `v0.91.7` residual guard |
| WP-05 provider/model reliability and private endpoint sanitation | `#3970` closed; `#4010`-`#4012` closed | Which provider/model reliability claims are safe, and what provider/private-endpoint behavior remains security-sensitive? | WP-05 closeout plus `#4010`, `#4011`, `#4012`, and the WP-07 review packet `docs/milestones/v0.91.6/review/security/PROVIDER_MODEL_CAV_TRUST_BOUNDARY_REVIEW_4020.md` | implementation tranche complete; trust-boundary review packet added with explicit residual routing | WP-07 `#4020` owns provider/model/CAV trust-boundary review in `v0.91.6`; unresolved prompt-injection/CAV taxonomy work remains routed to `#4064` |
| WP-06 ACIP, A2A, and provider communications | `#3971` open; `#4013` closed; `#4014`-`#4018` open | Which access rules, delegation boundaries, and transport decisions are safe enough for activation-path consumption? | WP-06 bridge doc, the WP-07 review packet `docs/milestones/v0.91.6/review/security/ACIP_A2A_ACCESS_RULE_SECURITY_REVIEW_4021.md`, plus the eventual closeout proof for `#4016`-`#4018` | active and blocking WP-07 closeout; access-rule security review packet added with explicit protocol residual routing | WP-07 `#4021` owns the security review lane; unresolved protocol closure remains owned by open WP-06 issues in `v0.91.6` and, if still unresolved, the `v0.91.7` residual guard |
| WP-07 security bridge, provider trust, CAV, and malformed-output posture | `#3972` open; `#4019`, `#4020`, `#4021`, `#4022`, `#4023`, `#4064`, and final closeout `#4024` are the active security tranche | Does the milestone have one activation-path security accounting surface instead of scattered, implicit trust claims? | This ledger plus the child security review packets and final WP-07 closeout packet | active by definition until child lanes and closeout finish | Current sprint owners are the open WP-07 child issues in `v0.91.6`; unresolved enterprise/security-governance residuals route to `v0.93` planning |
| WP-08 identity continuity and capability-selector bridge | `#3973` open | Which identity, continuity, and capability-evidence boundaries matter for security-sensitive activation claims? | WP-08 bridge outputs and later negative-case or continuity review surfaces | dependency active | WP-08 remains the owner for identity/continuity delivery; WP-07 consuming lanes `#4022` and `#4023` must not overclaim identity-safe publication or inhabitant-safe display before WP-08 closes |
| WP-09 Observatory and Unity consumption readiness | `#3974` open; `#4030`-`#4034` open | Can Unity Observatory consume ADL evidence without leaking private paths, logs, memory, credentials, or unreviewed identity/profile data? | WP-09 implementation and consumption proof surfaces, especially `#4034` | dependency active | WP-07 `#4023` owns the security review lane; WP-09 remains the implementation owner in `v0.91.6` |
| WP-10 AEE, Memory/ObsMem, Memory Palace, and ACP/cognitive-profile privacy | `#3975` open; `#4036`-`#4041` open | Which memory, profile, and publication/privacy boundaries are safe enough to expose or consume before `v0.92`? | WP-10 ledger/closeout set, the bridge doc `docs/milestones/v0.91.6/features/AEE_MEMORY_ACP_BRIDGE_ACCOUNTING_v0.91.6.md`, and the WP-07 packet `docs/milestones/v0.91.6/review/security/PUBLIC_RECORD_MEMORY_PROFILE_SECURITY_REVIEW_4022.md` | dependency active; publication/privacy review now records that WP-10 closure is still open | WP-07 `#4022` owns the publication/privacy security review lane; WP-10 remains the implementation owner in `v0.91.6` |

### Residual routing rules

- If a surface row is `active` or `dependency active`, WP-07 must remain open.
- If a surface claims `completed with security handoff`, later WP-07 lanes may
  consume it, but they must not erase the named routed residuals.
- If a later review finds a new security gap in a closed upstream lane, that
  gap must be recorded as a new remediation issue rather than silently
  re-opening milestone truth inside this table.
- If `#4020`, `#4021`, `#4022`, `#4023`, or `#4064` remain open at closeout
  time, `#4024` must record WP-07 as incomplete or explicitly routed rather
  than passing the gate.
- Residuals that are not owned by an active `v0.91.6` issue must route through
  the `v0.91.7` residual guard in [FEATURE_DOCS_v0.91.6.md](../FEATURE_DOCS_v0.91.6.md).

## Validation And Review

- Run focused threat-model review.
- Route CAV checks into deterministic issue proof where possible.
- Record malformed-output and provider-trust test gaps.
- Flag residuals for `v0.91.7` or `v0.93`.

## v0.92 Consumption

`v0.92` may consume only reviewed security boundaries. Security cannot be
silently deferred out of activation.

## Non-Goals

- No broad compliance claims.
- No assumption that public records or provider messages are safe by default.
- No security-by-narrative closure.
