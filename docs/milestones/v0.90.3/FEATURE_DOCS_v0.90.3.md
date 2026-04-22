# Feature Docs - v0.90.3

## Implementation-Facing Features

| Feature Doc | Purpose | Execution WPs |
| --- | --- | --- |
| `features/CITIZEN_STATE_SECURITY_AND_FORMAT.md` | authoritative private state, signed envelope, local sealing, ledger, witnesses, receipts, and anti-equivocation | WP-02 through WP-09 |
| `features/STANDING_COMMUNICATION_AND_ACCESS.md` | citizen, guest, service actor, communication, inspection, access-control, and denial semantics | WP-10, WP-11, WP-12 |
| `features/SANCTUARY_QUARANTINE_AND_CHALLENGE.md` | conservative behavior when continuity is unsafe or disputed | WP-08, WP-09, WP-13 |
| `features/LOCAL_SEALED_QUINTESSENCE_CHECKPOINTS.md` | local-first protected checkpoint strategy and enclave-ready backend seam | WP-04, WP-05, WP-07 |
| `features/ECONOMICS_PLACEMENT_BRIDGE.md` | narrow v0.90.3 resource-stewardship decision and v0.90.4 handoff | WP-13 |
| `features/CSM_OBSERVATORY_DESIGN.md` | operator, reviewer, and future citizen visibility design for the inhabited CSM Observatory flagship | WP-10, WP-14, WP-14A |
| `OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md` | multimode UI architecture for World / Reality, Operator / Governance, Cognition / Internal State, and Corporate Investor fallback rooms | WP-14, WP-14A |

## Proof And Audit Docs

| Doc | Purpose | Execution WPs |
| --- | --- | --- |
| `WBS_v0.90.3.md` | WP-01 through WP-20 execution plan with explicit WP-14A demo/proof lane | WP-01 |
| `DEMO_MATRIX_v0.90.3.md` | planned proof matrix and non-proving boundaries | WP-01, WP-14, WP-14A |
| `RELEASE_READINESS_v0.90.3.md` | WP-15 review entry surface, quality posture, tracker truth, and release-tail gate record | WP-15 |
| `WP_EXECUTION_READINESS_v0.90.3.md` | per-WP required outputs, validation, and non-goals for issue-card authoring | WP-01 |
| `WP_ISSUE_WAVE_v0.90.3.yaml` | live issue-wave source with real issue mapping | WP-01 |
| `CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md` | D1 audit of v0.90.2 citizen, snapshot, wake, quarantine, Observatory, and hardening artifacts with v0.90.3 requirement mapping | WP-02 |
| `PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md` | D2 canonical private-state format decision, binary authority boundary, JSON projection non-authority rule, fixtures, and focused validation | WP-03 |
| `SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md` | D3 signed envelope and local trust-root proof for canonical private state | WP-04 |
| `LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md` | D4 local sealed checkpoint, key policy, and backend seam proof | WP-05 |
| `APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md` | D5 append-only lineage ledger, materialized-head authority, and negative-case proof | WP-06 |
| `CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md` | D6 continuity witness schema, citizen-facing receipt schema, transition examples, and privacy-preserving validation | WP-07 |
| `ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md` | D7 conflicting-successor fixture, activation refusal, sanctuary/quarantine disposition, and evidence-preservation negative cases | WP-08 |
| `SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md` | D8 sanctuary/quarantine state policy, ambiguous-wake activation block, evidence-preserving quarantine artifact, and operator report | WP-09 |
| `REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md` | D9 redaction policy, audience projection packet, operator report, and leakage/authority negative cases for private citizen-state Observatory visibility | WP-10 |
| `STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md` | D10 standing policy, standing events, communication examples, and negative cases for guest, service actor, communication-inspection, and naked-actor boundaries | WP-11 |
| `ACCESS_CONTROL_SEMANTICS_v0.90.3.md` | D10 authority matrix, access event packet, and denial fixtures for inspection, decryption, projection, migration, wake, quarantine, challenge, appeal, and release | WP-12 |
| `CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md` | D11 challenge artifact, freeze artifact, appeal/review artifact, citizen-state threat model, and economics placement record | WP-13 |
| `OBSERVATORY_FLAGSHIP_DEMO_v0.90.3.md` | D12 integrated inhabited CSM Observatory proof command, generated proof packet, operator report, and room/lens walkthrough | WP-14 |
| `FEATURE_PROOF_COVERAGE_v0.90.3.md` | D13 feature-proof coverage record and runtime packet mapping every D1-D14 feature claim to proof, fixture, command, design boundary, or named deferral before WP-15 convergence | WP-14A |

## Context / Idea Docs

| Idea Doc | Purpose | Boundary |
| --- | --- | --- |
| `ideas/CITIZEN_STANDING_CONTEXT.md` | backgrounder for standing, citizens, guests, service actors, naked actors, and communication | context only |
| `ideas/V091_V092_SCOPE_BOUNDARY.md` | preserve moral/emotional and birthday boundaries | context only |
| `ideas/V0904_ECONOMICS_HANDOFF.md` | explain why full economics and contract markets move to v0.90.4 | context only |
