# Demo Matrix - v0.90.3

## Status

Planning/live-wave matrix. The v0.90.3 issue wave is open as #2327-#2347,
with the explicit demo matrix and feature proof lane at WP-14A / #2341.

The matrix has three layers:

- feature proof rows: D1 through D11 prove individual citizen-state safety,
  access, continuity, projection, and challenge surfaces
- flagship demo row: D12 ties those proofs into one inhabited CSM Observatory
  scenario with agents inside the polis
- design architecture row: D14 defines the room/lens/memory-dot architecture
  that D12 should consume without treating design assets as runtime proof

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Citizen-state inheritance audit | WP-02 | v0.90.3 targets actual v0.90.2 citizen, snapshot, wake, quarantine, and Observatory artifacts | `CITIZEN_STATE_INHERITANCE_AUDIT_v0.90.3.md`, unsafe-assumption list | LANDED |
| D2 | Private state format fixture | WP-03 | Authoritative state is typed and distinct from JSON projection | `PRIVATE_STATE_FORMAT_DECISION_v0.90.3.md`, canonical binary byte contract, redacted projection fixture | LANDED |
| D3 | Signed envelope and trust-root negative cases | WP-04 | Missing, unknown, revoked, mismatched, regressed, and broken-predecessor states are rejected | envelope fixture, trust-root fixture, negative tests | PLANNED |
| D4 | Local sealed quintessence checkpoint | WP-05 | Private checkpoint can be sealed locally without making cloud enclaves mandatory | sealed checkpoint fixture, key policy, open/decrypt refusal cases | PLANNED |
| D5 | Append-only lineage replay | WP-06 | Current state is accepted only when it matches append-only lineage | ledger fixture, accepted head calculation, tamper/truncation tests | PLANNED |
| D6 | Continuity witness and citizen receipt | WP-07 | Admission, snapshot, wake, and quarantine transitions produce explainable continuity evidence | witness schema, receipt schema, sample receipt | PLANNED |
| D7 | Anti-equivocation conflict | WP-08 | Conflicting signed successors for the same sequence are detected and cannot both become active | `ANTI_EQUIVOCATION_CONFLICT_v0.90.3.md`, conflict fixture, sanctuary/quarantine disposition, negative-case proof | LANDED |
| D8 | Sanctuary/quarantine ambiguous wake | WP-09 | Ambiguous wake preserves evidence and blocks unsafe activation | `SANCTUARY_QUARANTINE_BEHAVIOR_v0.90.3.md`, state policy, ambiguous wake fixture, quarantine artifact, operator report, negative-case proof | LANDED |
| D9 | Redacted Observatory projection | WP-10 | Operators see continuity status without raw private state | `REDACTED_OBSERVATORY_PROJECTIONS_v0.90.3.md`, redaction policy, projection packet, operator report, leakage/authority negative cases | LANDED |
| D10 | Standing, communication, and access boundary | WP-11 / WP-12 | Guests and service actors cannot silently acquire citizen rights or inspection access, and every sensitive access path emits an auditable event | `STANDING_COMMUNICATION_BOUNDARY_v0.90.3.md`, `ACCESS_CONTROL_SEMANTICS_v0.90.3.md`, standing policy, standing events, communication examples, authority matrix, access events, denial fixtures | LANDED |
| D11 | Challenge, appeal, and threat review | WP-13 | A challenged wake or projection freezes destructive transition and preserves evidence, with threat-model coverage before demo claims widen | `CONTINUITY_CHALLENGE_APPEAL_v0.90.3.md`, challenge artifact, freeze artifact, appeal/review artifact, threat model, economics placement record | LANDED |
| D12 | Inhabited CSM Observatory flagship | WP-14 | Reviewer can inspect one bounded end-to-end citizen-state scenario through World / Reality, Operator / Governance, optional bounded Cognition / Internal State, and Corporate Investor fallback surfaces | `OBSERVATORY_FLAGSHIP_DEMO_v0.90.3.md`, `OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md`, integrated proof packet, witness, receipt, redacted projection, access event, challenge/quarantine artifact, operator report | PLANNED |
| D13 | Feature proof coverage record | WP-14A | Every v0.90.3 feature claim has a runnable demo, proof packet, fixture-backed artifact, non-proving status, or explicit deferral | feature proof coverage record and demo matrix update | PLANNED |
| D14 | Observatory multimode UI architecture | WP-14A | The flagship Observatory demo has a reviewed room/lens/memory-dot architecture before demo redesign | `OBSERVATORY_UI_ARCHITECTURE_v0.90.3.md`, multimode mockup, Corporate Investor UI fallback rule | LANDED |

## Flagship Demo Rule

WP-14 / D12 should show an inhabited CSM:

- citizen-like actor with continuity evidence
- guest with no silent rights escalation
- service actor with bounded authority
- operator with redacted visibility
- challenged or quarantined transition when continuity or authority is unsafe
- room traversal from World / Reality Mode into Operator / Governance Mode
- optional Cognition / Internal State Mode only for bounded implemented signals
- Corporate Investor UI fallback for demo continuity and investor-safe summary

WP-14A / D13 should keep every feature proof row mapped to proof, fixture,
non-proving status, or explicit deferral.

## Non-Proving Boundaries

- These demos do not prove first true Gödel-agent birth.
- These demos do not prove full moral, emotional, kindness, or wellbeing
  substrate.
- These demos do not prove complete migration or cross-polis continuity.
- These demos do not prove full citizen economics or contract-market execution.
- These demos do not prove cloud enclave deployment.
- These demos prove bounded local citizen-state safety surfaces, not personhood.
- D14 is a design artifact, not a runtime UI implementation or proof artifact.
