# Demo Matrix - v0.90.3

## Status

Planning draft. No v0.90.3 issue wave has been opened yet.

| ID | Demo | WP | Proof Claim | Required Artifacts | Status |
| --- | --- | --- | --- | --- | --- |
| D1 | Citizen-state inheritance audit | WP-02 | v0.90.3 targets actual v0.90.2 citizen, snapshot, wake, quarantine, and Observatory artifacts | inheritance report, unsafe-assumption list | PLANNED |
| D2 | Private state format fixture | WP-03 | Authoritative state is typed and distinct from JSON projection | schema, authoritative fixture, redacted projection fixture | PLANNED |
| D3 | Signed envelope and trust-root negative cases | WP-04 | Missing, unknown, revoked, mismatched, regressed, and broken-predecessor states are rejected | envelope fixture, trust-root fixture, negative tests | PLANNED |
| D4 | Local sealed quintessence checkpoint | WP-05 | Private checkpoint can be sealed locally without making cloud enclaves mandatory | sealed checkpoint fixture, key policy, open/decrypt refusal cases | PLANNED |
| D5 | Append-only lineage replay | WP-06 | Current state is accepted only when it matches append-only lineage | ledger fixture, accepted head calculation, tamper/truncation tests | PLANNED |
| D6 | Continuity witness and citizen receipt | WP-07 | Admission, snapshot, wake, and quarantine transitions produce explainable continuity evidence | witness schema, receipt schema, sample receipt | PLANNED |
| D7 | Anti-equivocation conflict | WP-08 | Conflicting signed successors for the same sequence are detected | conflict fixture and quarantine/sanctuary disposition | PLANNED |
| D8 | Sanctuary/quarantine ambiguous wake | WP-09 | Ambiguous wake preserves evidence and blocks unsafe activation | ambiguous wake fixture, quarantine artifact, operator report | PLANNED |
| D9 | Redacted Observatory projection | WP-10 / WP-14 | Operators see continuity status without raw private state | projection schema, leakage tests, Observatory packet/report | PLANNED |
| D10 | Standing and communication boundary | WP-11 / WP-13 | Guests and service actors cannot silently acquire citizen rights or inspection access | standing events, access-denial events, communication examples | PLANNED |
| D11 | Integrated citizen-state proof | WP-12 | Reviewer can inspect one bounded end-to-end citizen-state scenario | integrated proof packet, witness, receipt, projection, operator report | PLANNED |
| D12 | Challenge and appeal flow | WP-15 | A challenged wake or projection freezes destructive transition and preserves evidence | challenge artifact, appeal/review artifact, resolution record | PLANNED |

## Non-Proving Boundaries

- These demos do not prove first true Gödel-agent birth.
- These demos do not prove full moral, emotional, kindness, or wellbeing
  substrate.
- These demos do not prove complete migration or cross-polis continuity.
- These demos do not prove full citizen economics or contract-market execution.
- These demos do not prove cloud enclave deployment.
- These demos prove bounded local citizen-state safety surfaces, not personhood.
