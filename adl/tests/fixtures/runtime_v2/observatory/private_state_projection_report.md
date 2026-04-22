# Private-State Observatory Projection Report

## Report Identity
| Field | Value |
| --- | --- |
| Packet | private-state-observatory-packet-proto-citizen-alpha-0001 |
| Schema | runtime_v2.private_state_observatory_packet.v1 |
| Demo | D9 |
| Generated | 2026-04-22T00:00:00Z |
| Authority | non_authoritative_review_projection |

## Continuity Summary
- proto-citizen-alpha has a hash-linked private-state projection
- continuity witness and citizen receipt refs are visible without raw private-state bytes
- sanctuary/quarantine status is visible as evidence, not as release authority

## Audience Projections
| Audience | Projection | Raw private state | Authority | Allowed action |
| --- | --- | --- | --- | --- |
| operator | operator_continuity_projection | false | false | read_only_projection |
| reviewer | reviewer_evidence_projection | false | false | read_only_projection |
| public | public_status_projection | false | false | read_only_projection |
| debug | redacted_debug_projection | false | false | read_only_projection |

## Evidence
- runtime_v2/private_state/proto-citizen-alpha.projection.json
- runtime_v2/private_state/continuity_witnesses.json
- runtime_v2/private_state/citizen_receipts.json
- runtime_v2/private_state/sanctuary_quarantine_artifact.json
- runtime_v2/private_state/sanctuary_quarantine_operator_report.json

## Prohibited Uses
- wake_from_projection
- migrate_from_projection
- decrypt_from_projection
- release_from_quarantine_from_projection
- treat_projection_as_canonical_state

## Claim Boundary
This packet proves bounded redacted Observatory projections for private citizen state; it does not implement live Runtime v2 execution, unrestricted inspection, access-control grants, first true Godel-agent birth, or v0.92 identity rebinding.

This report is an Observatory projection surface. It provides continuity status without raw private-state inspection and must not be used as canonical citizen-state authority.
