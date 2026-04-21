# Citizen-State Inheritance And Gap Audit - v0.90.3

## Purpose

This is the WP-02 / D1 proof artifact for v0.90.3. It audits the v0.90.2
Runtime v2 / CSM evidence spine before v0.90.3 turns provisional citizen state
into durable, protected, continuity-bearing private state.

The audit answers three questions:

- what v0.90.2 already proved and can be inherited
- what v0.90.2 artifacts must not be promoted to authoritative citizen state
- what v0.90.3 requirements follow from those gaps

## Result

Status: PASS

v0.90.3 can inherit the v0.90.2 CSM run, provisional citizen records, snapshot
and wake-continuity evidence, recovery/quarantine artifacts, Observatory
projection/report, and hardening probes as source evidence.

Important boundary: none of those inherited artifacts is authoritative private
citizen state. They are JSON proof fixtures, projection surfaces, operator
reports, and bounded Runtime v2 evidence. v0.90.3 must treat them as inputs to a
new citizen-state substrate, not as the substrate itself.

## Source Evidence

| Evidence | Inheritance Use |
| --- | --- |
| `docs/milestones/v0.90.2/RELEASE_EVIDENCE_v0.90.2.md` | Release-level index of v0.90.2 proof, review, remediation, and handoff state |
| `docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md` | D1-D11 proof map for the bounded first CSM run and hardening package |
| `docs/milestones/v0.90.2/features/RUNTIME_V2_HARDENING.md` | End-to-end first-run and hardening target, including non-claims |
| `docs/milestones/v0.90.2/features/RECOVERY_AND_QUARANTINE.md` | Recovery eligibility and quarantine boundary for unsafe resume |
| `docs/milestones/v0.90.2/features/OPERATOR_REVIEW_SURFACES.md` | Observatory packet/report and integrated reviewer entrypoint |
| `docs/milestones/v0.90.2/features/INVARIANT_EXPANSION_AND_COVERAGE.md` | Invariant coverage for citizen continuity, snapshot restore, trace order, and quarantine preservation |
| `adl/src/runtime_v2/` | Runtime v2 implementation tree inherited by v0.90.3 |
| `adl/tests/fixtures/runtime_v2/` | Fixture-backed CSM run, citizen, snapshot, wake, quarantine, Observatory, and hardening artifacts |

## Inherited v0.90.2 Proof Surface

v0.90.2 provides a coherent bounded evidence spine:

| Area | Primary Inherited Artifacts | What They Prove |
| --- | --- | --- |
| CSM run contract | `adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json` | the bounded first-run artifact set and stage sequence are explicit |
| Provisional citizens | `adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json`, `adl/tests/fixtures/runtime_v2/citizens/proto-citizen-alpha.json`, `adl/tests/fixtures/runtime_v2/citizens/proto-citizen-beta.json` | citizens can be represented in a bounded run with active/proposed lifecycle state |
| Snapshot and wake | `adl/tests/fixtures/runtime_v2/snapshots/snapshot-0001.json`, `adl/tests/fixtures/runtime_v2/rehydration_report.json`, `adl/tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json` | snapshot restore checks can run before wake and preserve one unique active head |
| Recovery and quarantine | `adl/tests/fixtures/runtime_v2/recovery/safe_resume_decision.json`, `adl/tests/fixtures/runtime_v2/recovery/quarantine_required_decision.json`, `adl/tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json`, `adl/tests/fixtures/runtime_v2/quarantine/evidence_preservation_artifact.json` | unsafe recovery can be refused, quarantined, and held for operator review with evidence preserved |
| Observatory projection | `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json`, `adl/tests/fixtures/runtime_v2/observatory/operator_report.md` | operator/reviewer visibility can summarize bounded continuity evidence without claiming a live unbounded run |
| Hardening probes | `adl/tests/fixtures/runtime_v2/hardening/hardening_proof_packet.json` and related probe fixtures | duplicate activation, snapshot integrity, trace/replay gap, and adversarial pressure checks can fail closed around the bounded run |

## Unsafe Assumptions To Reject

The following assumptions are unsafe for v0.90.3 and must not be carried
forward as hidden design premises.

| Unsafe Assumption | Why It Is Unsafe | v0.90.3 Requirement |
| --- | --- | --- |
| Provisional JSON citizen records are private citizen state | `runtime_v2.provisional_citizen.v1` records are fixture/projection records with explicit provisional boundaries | WP-03 must define the authoritative private-state format and prove JSON is only a projection |
| A snapshot manifest is a durable private checkpoint | `runtime_v2.snapshot_manifest.v1` is JSON fixture evidence with a structural checksum, not a signed, sealed, lineage-bound private-state checkpoint | WP-03 through WP-05 must add canonical state, signed envelopes, trust roots, and local sealing semantics |
| Wake-continuity proof is sufficient identity continuity | The wake proof preserves one active head in a bounded run, but it does not define single-lineage identity, ledger authority, or citizen-facing continuity receipt | WP-06 through WP-08 must add append-only lineage, witnesses/receipts, and anti-equivocation |
| Quarantine artifact is complete sanctuary semantics | v0.90.2 quarantine blocks unsafe recovery and preserves evidence, but does not define sanctuary mode, continuity challenge, appeal, or release governance | WP-09 and WP-13 must define sanctuary, challenge, appeal, and review-resolution behavior |
| Observatory visibility is safe private-state inspection | The packet/report are review projections, not raw-state access; treating them as inspection authority would turn Observatory into a private-state browser | WP-10 through WP-12 must define redaction, projection authority, standing, and access-control semantics |
| Local JSON fixtures imply keying and trust | v0.90.2 has validation and fixture checks, but no citizen-state signing trust root, key rotation semantics, or encrypted private-state authority | WP-04 and WP-05 must define signed envelopes, trust roots, key handling, and sealing failure modes |
| Hardening probes prove full security ecology | v0.90.2 proves bounded hardening around one run, not full adversarial ecology or complete insider/operator abuse handling | WP-13 must threat-model citizen-state abuse paths before integrated demo claims widen |

## Source-To-v0.90.3 Requirement Map

| Inherited Source | Observed Boundary | Required v0.90.3 Follow-Up |
| --- | --- | --- |
| `runtime_v2.provisional_citizen.v1` citizen fixtures | provisional, JSON, run-local, no signed lineage | WP-03 private-state schema/fixture with required identity, lineage, schema, and projection fields |
| `runtime_v2.snapshot_manifest.v1` snapshot fixture | manifest/checksum evidence, not sealed private state | WP-03 canonical serialization, WP-04 signed envelope, WP-05 sealed checkpoint |
| `runtime_v2.csm_wake_continuity_proof.v1` | duplicate-safe wake proof, not full continuity proof across all operations | WP-06 ledger, WP-07 continuity witnesses/receipts, WP-08 anti-equivocation |
| `runtime_v2.csm_quarantine_artifact.v1` | execution blocked pending review, not full sanctuary/challenge/release governance | WP-09 sanctuary/quarantine behavior and WP-13 challenge/appeal |
| `adl.csm_visibility_packet.v1` | operator/reviewer projection, not authoritative state or inspection right | WP-10 redacted projections and WP-12 access-control events/denials |
| `runtime_v2.csm_hardening_proof_packet.v1` | bounded probes around first-run evidence | WP-13 citizen-state threat model and integrated evidence boundary |
| v0.90.2 release/review docs | proof package is accepted as bounded, with explicit non-claims | v0.90.3 must preserve no-birthday, no-v0.91, no-v0.92, no-full-economics, and no-cloud-enclave-required boundaries |

## D1 Proof Classification

Classification: proving

D1 proves that v0.90.3 targets actual v0.90.2 CSM run artifacts and extracts
explicit requirements from them. It proves this by checking the inherited source
docs and fixture paths and by mapping each inherited surface to a v0.90.3
follow-up requirement.

D1 does not prove canonical private-state format, signed envelopes, sealed
checkpoints, append-only lineage, continuity witnesses, anti-equivocation,
redacted projections, access control, or integrated citizen-state execution.
Those proofs belong to WP-03 through WP-14.

## Review Checklist

- v0.90.2 release evidence found: PASS
- v0.90.2 feature proof coverage found: PASS
- Runtime v2 implementation tree found: PASS
- CSM run packet fixture found: PASS
- provisional citizen records found: PASS
- snapshot, rehydration, and wake-continuity fixtures found: PASS
- recovery and quarantine fixtures found: PASS
- Observatory packet/report found: PASS
- hardening proof packet found: PASS
- no inherited proof promoted to durable private citizen identity: PASS

## Validation Commands

The WP-02 authoring pass validated this audit with:

```sh
test -f docs/milestones/v0.90.2/RELEASE_EVIDENCE_v0.90.2.md
test -f docs/milestones/v0.90.2/FEATURE_PROOF_COVERAGE_v0.90.2.md
test -d adl/src/runtime_v2
test -f adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json
test -f adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json
test -f adl/tests/fixtures/runtime_v2/citizens/proto-citizen-alpha.json
test -f adl/tests/fixtures/runtime_v2/citizens/proto-citizen-beta.json
test -f adl/tests/fixtures/runtime_v2/snapshots/snapshot-0001.json
test -f adl/tests/fixtures/runtime_v2/rehydration_report.json
test -f adl/tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json
test -f adl/tests/fixtures/runtime_v2/recovery/safe_resume_decision.json
test -f adl/tests/fixtures/runtime_v2/recovery/quarantine_required_decision.json
test -f adl/tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json
test -f adl/tests/fixtures/runtime_v2/quarantine/evidence_preservation_artifact.json
test -f adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json
test -f adl/tests/fixtures/runtime_v2/observatory/operator_report.md
test -f adl/tests/fixtures/runtime_v2/hardening/hardening_proof_packet.json
```

The final WP-02 validation should also run a stale-path/secret scan against the
touched tracked docs and `git diff --check`.
