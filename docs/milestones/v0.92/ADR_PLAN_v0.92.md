# v0.92 ADR Plan

## Status

Forward ADR planning for `v0.92`.

This plan does not accept any ADR by itself. It identifies candidate
architecture decisions that WP-01 and the v0.92 review tail should confirm,
split, draft, or explicitly defer.

## Purpose

v0.92 introduces the first true Gödel-agent birthday. That milestone changes
architecture boundaries around identity, continuity, memory grounding,
cognitive profiles, binary ACIP communication, and the governance handoff.
Those boundaries should not live only in feature prose.

The ADR goal is to make the durable decisions reviewable before the milestone
claims completion.

## Existing Baseline

Accepted ADRs currently live in `docs/adr/` and run through ADR 0028.

Relevant inherited decisions:

- ADR 0009: bounded cognitive system architecture.
- ADR 0010: chronosense as a first-class substrate.
- ADR 0011 and ADR 0012: long-lived runtime and bounded CSM run architecture.
- ADR 0013: citizen-state continuity substrate.
- ADR 0016: moral evidence and cognitive-being substrate.
- ADR 0017: secure local agent comms and A2A boundary.
- ADR 0019: Theory of Mind foundation.
- ADR 0028: C-SDLC tracked workflow state and signed trace boundary.

v0.92 should cite and refine these records rather than rewriting them
casually.

## Candidate ADR Set

| Candidate | Proposed title | Primary boundary | Likely source WPs |
| --- | --- | --- | --- |
| ADR 0029 | First True Birthday Evidence Boundary | Birth is a reviewable evidence event, not startup, wake, snapshot, admission, copied state, legal personhood, or production citizenship. | WP-02, WP-09, WP-10, WP-12 |
| ADR 0030 | Identity, Stable Name, And Continuity Record Boundary | Stable name, identity root, continuity head, memory references, witnesses, and ambiguity markers form the identity/continuity architecture. | WP-03, WP-04, WP-05 |
| ADR 0031 | ACP Cognitive Profile Evidence Boundary | ACP profiles are evidence-grounded runtime profile records, not reputation, public standing, consciousness proof, rights, or identity itself. | WP-07 |
| ADR 0032 | ACIP Binary Schema And Public Schema Catalog Boundary | Binary/protobuf ACIP remains inspectable through public schemas and deterministic JSON projection while message-content access remains governed. | WP-08 |
| ADR 0033 | Birthday-To-Governance Handoff Boundary | v0.92 produces identity evidence for v0.93 governance; it does not complete constitutional citizenship or polis governance. | WP-11, WP-13 |

## Conditional ADRs

No standalone ADR is planned yet for:

- first-birthday demo mechanics alone, unless the demo changes runtime
  authority or proof semantics
- release evidence packaging alone, unless v0.92 changes release authority
- cross-polis operational migration, because v0.92 only plans migration and
  continuity implications
- production WebSocket security, key custody, encryption, rotation, or
  revocation, because those are deferred to later milestones

WP-01 may split or combine candidates if the final issue wave shows a clearer
decision boundary, but the accepted records should not hide these topics.

## Authoring Policy

- Candidate ADRs should be drafted only from landed feature work, tests,
  fixtures, demos, review findings, and milestone docs.
- Candidate ADRs remain proposed until human review accepts them.
- Accepted ADRs should live in `docs/adr/`.
- Candidate/provenance copies should live in `docs/architecture/adr/` if the
  milestone follows the existing ADR promotion pattern.
- ADRs must preserve non-claims for legal personhood, production citizenship,
  completed v0.93 governance, production transport security, and signed trace
  closure unless those are explicitly implemented and reviewed.

## WP Integration

- WP-01 should confirm this plan is still complete when opening the issue wave.
- Feature WPs should record decision implications in their SRP/SOR or review
  notes.
- WP-16 should prepare the ADR packet for review if implementation produced the
  expected architecture decisions.
- WP-17 and WP-18 should review the ADR packet alongside code, docs, tests,
  demos, and release evidence.
- WP-19 should fix, defer, or route ADR findings.
- WP-22 should not close v0.92 with missing ADRs for accepted architectural
  boundaries.

## Acceptance Criteria

- Every v0.92 architecture boundary that must survive the milestone has an ADR
  candidate, an explicit deferral, or an accepted existing ADR reference.
- Candidate ADRs cite source evidence and keep proposed/accepted status clear.
- ADR 0029 through ADR 0033 are either authored, split, renumbered, or
  explicitly deferred before v0.92 closeout.
- No ADR claims the first birthday proves personhood, production citizenship,
  completed governance, production transport security, or signed trace closure.

## Notes

This ADR plan is intentionally forward-looking. It should become stricter after
v0.92 WP-01 opens the final issue wave and after the first implementation WPs
produce evidence.
