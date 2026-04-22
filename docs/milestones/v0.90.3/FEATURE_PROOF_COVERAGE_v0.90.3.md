# Feature Proof Coverage - v0.90.3

## Status

WP-14A / D13 is landed. Every v0.90.3 citizen-state substrate feature claim
now has one explicit proof home before WP-15 docs, quality, and review
convergence.

This record is a reviewer map with matching runtime evidence. It does not
create new runtime authority by itself. It binds the demo matrix to the landed
proof docs, fixtures, focused tests, the generated
`runtime_v2/feature_proof_coverage/feature_proof_coverage.json` packet, and
bounded non-proving boundaries for D1 through D14.

## Coverage Rule

Each feature claim must have one of:

- runnable demo command
- test-backed proof packet
- fixture-backed artifact
- documented non-proving status
- explicit deferral with owner and rationale

For v0.90.3, D1 through D13 are proving, bounded, and reviewable. D14 is landed
as a design architecture artifact and is explicitly non-runtime proof.

The runtime packet also records one `working_demo_command` for every row. For
D1 through D13, that command is the working proof or demo surface for the
feature. For D14, the command verifies the landed design artifact and preserves
the non-runtime boundary.

## Working Demo Commands

| Demo | Working command |
| --- | --- |
| D1 | `test -f docs/milestones/v0.90.3/CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md` |
| D2 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture` |
| D3 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_envelope -- --nocapture && cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_trust_root_matches_golden_fixture -- --nocapture` |
| D4 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sealing -- --nocapture` |
| D5 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_lineage -- --nocapture` |
| D6 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_witness -- --nocapture && cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state -- --nocapture` |
| D7 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_anti_equivocation -- --nocapture` |
| D8 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_sanctuary -- --nocapture && cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_quarantine -- --nocapture` |
| D9 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_private_state_observatory -- --nocapture` |
| D10 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_standing -- --nocapture && cargo test --manifest-path adl/Cargo.toml runtime_v2_access_control -- --nocapture` |
| D11 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_continuity_challenge -- --nocapture` |
| D12 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_observatory_flagship -- --nocapture && cargo run --manifest-path adl/Cargo.toml -- runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship` |
| D13 | `cargo test --manifest-path adl/Cargo.toml runtime_v2_feature_proof_coverage -- --nocapture && cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0903/feature-proof-coverage.json` |
| D14 | `test -f docs/milestones/v0.90.3/OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md && test -f docs/milestones/v0.90.3/assets/csm_observatory_multimode_ui_mockups.png` |

## Coverage Table

| Demo | Owner | Coverage Kind | Primary Evidence | Validation |
| --- | --- | --- | --- | --- |
| D1 Citizen-state inheritance audit | WP-02 | fixture-backed audit | `CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md` | referenced v0.90.2 artifacts exist; unsafe assumptions are mapped to v0.90.3 requirements |
| D2 Private state format fixture | WP-03 | test-backed fixture and decision record | `PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md`, canonical binary byte contract, redacted projection fixture | `runtime_v2_private_state` focused tests |
| D3 Signed envelope and trust-root negative cases | WP-04 | test-backed proof packet | `SIGNED_PRIVATE_STATE_ENVELOPE_v0.90.3.md`, signed envelope fixture, trust-root fixture, negative cases | `runtime_v2_private_state_envelope`, `runtime_v2_private_state_trust_root_matches_golden_fixture` |
| D4 Local sealed quintessence checkpoint | WP-05 | test-backed fixture | `LOCAL_PRIVATE_STATE_SEALING_v0.90.3.md`, sealed checkpoint fixture, key policy, backend seam | `runtime_v2_private_state_sealing` focused tests |
| D5 Append-only lineage replay | WP-06 | test-backed ledger proof | `APPEND_ONLY_LINEAGE_LEDGER_v0.90.3.md`, ledger fixture, accepted-head calculation, negative cases | `runtime_v2_private_state_lineage` focused tests |
| D6 Continuity witness and citizen receipt | WP-07 | test-backed witness and receipt proof | `CONTINUITY_WITNESSES_AND_RECEIPTS_v0.90.3.md`, continuity witnesses, citizen receipts, transition examples | `runtime_v2_private_state_witness`, `runtime_v2_private_state` focused tests |
| D7 Anti-equivocation conflict | WP-08 | test-backed negative-case proof | `ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md`, conflicting-successor fixture, sanctuary/quarantine disposition | `runtime_v2_private_state_anti_equivocation` focused tests |
| D8 Sanctuary/quarantine ambiguous wake | WP-09 | test-backed sanctuary and quarantine proof | `SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`, ambiguous wake fixture, quarantine artifact, operator report | `runtime_v2_private_state_sanctuary`, `runtime_v2_csm_quarantine` focused tests |
| D9 Redacted Observatory projection | WP-10 | test-backed projection proof | `REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`, redaction policy, projection packet, operator report | `runtime_v2_private_state_observatory` focused tests |
| D10 Standing, communication, and access boundary | WP-11 / WP-12 | test-backed boundary proof | `STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`, `ACCESS_CONTROL_SEMANTICS_v0.90.3.md`, standing policy/events, authority matrix, denial fixtures | `runtime_v2_standing`, `runtime_v2_access_control` focused tests |
| D11 Challenge, appeal, and threat review | WP-13 | test-backed due-process proof | `CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`, challenge artifact, freeze artifact, appeal/review artifact, threat model, economics placement record | `runtime_v2_continuity_challenge` focused tests |
| D12 Inhabited CSM Observatory flagship | WP-14 | runnable demo command and proof packet | `OBSERVATORY_FLAGSHIP_DEMO_v0.90.3.md`, `OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md`, `runtime_v2/observatory/flagship_proof_packet.json` | `runtime_v2_observatory_flagship`, `cargo run --manifest-path adl/Cargo.toml -- runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship` |
| D13 Feature proof coverage record | WP-14A | runnable coverage packet and reviewer proof map | `FEATURE_PROOF_COVERAGE_v0.90.3.md`, `DEMO_MATRIX_v0.90.3.md`, `runtime_v2/feature_proof_coverage/feature_proof_coverage.json` | `runtime_v2_feature_proof_coverage`, `cargo run --manifest-path adl/Cargo.toml -- runtime-v2 feature-proof-coverage --out artifacts/v0903/feature-proof-coverage.json` |
| D14 Observatory multimode UI architecture | WP-14A | documented non-runtime design artifact | `OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md`, multimode mockups, Observatory diagrams, Corporate Investor UI fallback rule | design docs and diagrams are landed; no runtime UI implementation is claimed |

## Named Deferrals

No v0.90.3 citizen-state substrate feature claim is left without a proof home.
The following later-band claims remain explicitly deferred:

- v0.91 moral, emotional, kindness, humor, wellbeing, cultivation, and
  harm-prevention substrate
- v0.92 identity/capability rebinding, migration, and birthday record
- full v0.90.4 citizen economics, contract markets, payment rails,
  subcontracting, and inter-polis trade
- production cloud enclave deployment
- production UI readiness for the Observatory multimode architecture

## Non-Proving Boundaries

This coverage record does not add new runtime behavior beyond the referenced
D1 through D12 evidence surfaces. It also does not claim:

- first true Godel-agent birth
- personhood
- unrestricted operator inspection of private citizen state
- production security or privacy hardening
- complete migration or cross-polis continuity
- full citizen economics or contract-market execution
- mandatory cloud enclaves

## WP-15 Handoff

WP-15 should consume this D13 record during docs, quality, and review
convergence. If WP-15 identifies a missing or overstated claim, it should
either link the claim to one of the proof surfaces above, correct the claim, or
create an explicit deferral with owner and rationale.
