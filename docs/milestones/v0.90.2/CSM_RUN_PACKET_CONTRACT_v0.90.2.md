# CSM Run Packet Contract - v0.90.2

## Status

WP-03 / D2 packet contract artifact: LANDED.
WP-04 / D2 invariant and violation contract artifacts: LANDED.
WP-05 / D3 boot and admission artifacts: LANDED.
WP-06 / D4 resource-pressure scheduling artifacts: LANDED.
WP-07 / D4 Freedom Gate mediation artifacts: LANDED.
WP-08 / D5 invalid-action rejection artifacts: LANDED.
WP-09 / D6 snapshot rehydrate wake continuity artifacts: LANDED.
WP-10 / D7 Observatory packet and operator report artifacts: LANDED.
WP-11 / D8 recovery eligibility model and decision records: LANDED.
WP-12 / D8 quarantine state machine and evidence preservation artifacts: LANDED.
WP-13 / D9 governed adversarial hook and hardening probe artifacts: LANDED.

This document defines the first bounded CSM run packet contract for
`proto-csm-01`. It is intentionally a contract and fixture gate, not a live run
claim. Later work packages must consume this shape instead of inventing their
own run packet surfaces.

## Source Evidence

| Evidence | Role |
| --- | --- |
| `adl/src/runtime_v2/csm_run.rs` | Code-backed contract type, prototype, validation, and serialization |
| `adl/tests/fixtures/runtime_v2/csm_run/run_packet_contract.json` | Golden contract fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/invariants/csm_run_invariant_map.json` | Golden invariant map fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/violations/violation_artifact_schema.json` | Golden violation schema fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/boot_manifest.json` | Golden boot manifest fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/citizen_roster.json` | Golden citizen roster fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/boot_admission_trace.jsonl` | Golden boot/admission trace fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/resource_pressure_fixture.json` | Golden resource-pressure fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/scheduling_decision.json` | Golden scheduler decision fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/citizen_action_fixture.json` | Golden citizen action fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/freedom_gate_decision.json` | Golden Freedom Gate decision fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/invalid_action_fixture.json` | Golden invalid-action fixture used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/invalid_action_violation.json` | Golden invalid-action violation packet used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/csm_run/wake_continuity_proof.json` | Golden D6 wake continuity proof used by Runtime v2 tests |
| `adl/tests/fixtures/runtime_v2/snapshots/snapshot-0001.json` | Golden snapshot manifest consumed by D6 wake continuity |
| `adl/tests/fixtures/runtime_v2/rehydration_report.json` | Golden rehydration report consumed by D6 wake continuity |
| `adl/tests/fixtures/runtime_v2/csm_run/first_run_trace.jsonl` | Golden first-run trace fixture used by Runtime v2 tests |
| `adl/src/runtime_v2/observatory.rs` | Code-backed D7 visibility packet and operator-report integration |
| `adl/tests/fixtures/runtime_v2/observatory/visibility_packet.json` | Golden D7 Observatory visibility packet fixture |
| `adl/tests/fixtures/runtime_v2/observatory/operator_report.md` | Golden D7 operator report rendered from the packet |
| `adl/src/runtime_v2/recovery.rs` | Code-backed D8 recovery eligibility model and decision records |
| `adl/tests/fixtures/runtime_v2/recovery/eligibility_model.json` | Golden D8 recovery eligibility model fixture |
| `adl/tests/fixtures/runtime_v2/recovery/safe_resume_decision.json` | Golden D8 safe-resume decision fixture |
| `adl/tests/fixtures/runtime_v2/recovery/quarantine_required_decision.json` | Golden D8 quarantine-required decision fixture |
| `adl/src/runtime_v2/quarantine.rs` | Code-backed D8 quarantine state machine and evidence preservation artifacts |
| `adl/tests/fixtures/runtime_v2/quarantine/unsafe_recovery_fixture.json` | Golden D8 unsafe recovery fixture |
| `adl/tests/fixtures/runtime_v2/quarantine/quarantine_artifact.json` | Golden D8 quarantine state machine artifact |
| `adl/tests/fixtures/runtime_v2/quarantine/evidence_preservation_artifact.json` | Golden D8 quarantine evidence-preservation artifact |
| `adl/src/runtime_v2/hardening.rs` | Code-backed D9 governed adversarial hook and hardening probes |
| `adl/tests/fixtures/runtime_v2/hardening/rules_of_engagement.json` | Golden D9 rules of engagement fixture |
| `adl/tests/fixtures/runtime_v2/hardening/adversarial_hook_packet.json` | Golden D9 adversarial hook fixture |
| `adl/tests/fixtures/runtime_v2/hardening/duplicate_activation_probe.json` | Golden D9 duplicate activation probe |
| `adl/tests/fixtures/runtime_v2/hardening/snapshot_integrity_probe.json` | Golden D9 snapshot integrity probe |
| `adl/tests/fixtures/runtime_v2/hardening/trace_replay_gap_probe.json` | Golden D9 trace/replay gap probe |
| `adl/tests/fixtures/runtime_v2/hardening/hardening_proof_packet.json` | Golden D9 hardening summary proof packet |
| `demos/fixtures/csm_run/proto-csm-01-run-packet.json` | Reviewer-facing fixture definition for the first bounded run |
| `docs/milestones/v0.90.2/RUNTIME_V2_INHERITANCE_AND_COMPRESSION_AUDIT_v0.90.2.md` | WP-02 inheritance gate that this contract consumes |
| `docs/milestones/v0.90.2/DEMO_MATRIX_v0.90.2.md` | D2 proof target |

## Contract Scope

The CSM run packet contract fixes:

- the schema id: `runtime_v2.csm_run_packet_contract.v1`
- the target manifold: `proto-csm-01`
- the D2 demo/proof mapping
- the required pre-live-run artifact set
- the first-run stage sequence
- the reviewer entrypoint and validation command
- explicit non-claims for live execution, later invariant expansion, and later
  milestone scopes

## Required Artifacts

The first bounded run must use these artifact requirements:

| Artifact | Owner | Required By | Purpose |
| --- | --- | --- | --- |
| `runtime_v2/csm_run/run_packet_contract.json` | WP-03 | WP-04 | Stable first-run packet contract |
| `runtime_v2/csm_run/proto-csm-01-run-packet.json` | WP-03 | WP-05 | Fixture definition for the first live run |
| `runtime_v2/invariants/csm_run_invariant_map.json` | WP-04 | WP-05 | Expanded invariant map before live work widens |
| `runtime_v2/violations/violation_artifact_schema.json` | WP-04 | WP-08 | Stable invalid-action and violation artifact shape |
| `runtime_v2/csm_run/boot_manifest.json` | WP-05 | WP-14 | Live manifold boot evidence |
| `runtime_v2/csm_run/first_run_trace.jsonl` | WP-06 | WP-14 | Ordered trace spine for scheduling, mediation, rejection, and wake continuity |
| `runtime_v2/csm_run/resource_pressure_fixture.json` | WP-06 | WP-14 | Bounded pressure input for the governed episode scheduler |
| `runtime_v2/csm_run/scheduling_decision.json` | WP-06 | WP-07 | Reviewable scheduler choice before Freedom Gate mediation |
| `runtime_v2/csm_run/citizen_action_fixture.json` | WP-07 | WP-08 | Scheduled non-trivial citizen action routed through the Freedom Gate |
| `runtime_v2/csm_run/freedom_gate_decision.json` | WP-07 | WP-08 | Bounded Freedom Gate mediation decision for the scheduled action |
| `runtime_v2/csm_run/invalid_action_fixture.json` | WP-08 | WP-11 | Invalid action input that must be rejected before commit |
| `runtime_v2/csm_run/invalid_action_violation.json` | WP-08 | WP-11 | Stable violation packet proving rejection without side effects |
| `runtime_v2/snapshots/snapshot-0001.json` | WP-09 | WP-10 | Captured manifold snapshot used by D6 wake continuity |
| `runtime_v2/rehydration_report.json` | WP-09 | WP-10 | Rehydration report proving invariants ran before wake |
| `runtime_v2/csm_run/wake_continuity_proof.json` | WP-09 | WP-14 | Proof that wake resumes one unique active citizen head |
| `runtime_v2/observatory/visibility_packet.json` | WP-10 | WP-14 | Operator-visible projection of the bounded run |
| `runtime_v2/observatory/operator_report.md` | WP-10 | WP-14 | Human-readable operator report rendered from the visibility packet |
| `runtime_v2/recovery/eligibility_model.json` | WP-11 | WP-12 | D8 rules distinguishing safe resume from required quarantine |
| `runtime_v2/recovery/safe_resume_decision.json` | WP-11 | WP-14 | Positive D8 decision proving validated wake can resume safely |
| `runtime_v2/recovery/quarantine_required_decision.json` | WP-11 | WP-12 | Negative D8 decision handing unsafe recovery to quarantine |
| `runtime_v2/quarantine/unsafe_recovery_fixture.json` | WP-12 | WP-13 | Unsafe recovery input consumed by the quarantine state machine |
| `runtime_v2/quarantine/quarantine_artifact.json` | WP-12 | WP-13 | State machine artifact blocking unsafe recovery pending review |
| `runtime_v2/quarantine/evidence_preservation_artifact.json` | WP-12 | WP-14 | Evidence hold proving unsafe recovery artifacts are retained |
| `runtime_v2/hardening/rules_of_engagement.json` | WP-13 | WP-14 | D9 operator-scoped rules for adversarial pressure |
| `runtime_v2/hardening/adversarial_hook_packet.json` | WP-13 | WP-14 | D9 governed adversarial hook proving quarantine containment |
| `runtime_v2/hardening/duplicate_activation_probe.json` | WP-13 | WP-14 | D9 duplicate active-head negative probe |
| `runtime_v2/hardening/snapshot_integrity_probe.json` | WP-13 | WP-14 | D9 snapshot integrity negative probe |
| `runtime_v2/hardening/trace_replay_gap_probe.json` | WP-13 | WP-14 | D9 trace/replay gap negative probe |
| `runtime_v2/hardening/hardening_proof_packet.json` | WP-13 | WP-14 | D9 hardening summary proof packet |

## Stage Contract

The first-run stage order is fixed and must remain contiguous:

| Sequence | Stage | Owner | Exit Artifact |
| --- | --- | --- | --- |
| 1 | `contract_and_fixture` | WP-03 | `runtime_v2/csm_run/run_packet_contract.json` |
| 2 | `invariant_and_violation_contract` | WP-04 | `runtime_v2/invariants/csm_run_invariant_map.json` |
| 3 | `boot_and_admission` | WP-05 | `runtime_v2/csm_run/boot_manifest.json` |
| 4 | `governed_episode_and_rejection` | WP-06-WP-08 | `runtime_v2/csm_run/first_run_trace.jsonl` |
| 5 | `snapshot_wake_and_observatory` | WP-09-WP-10 | `runtime_v2/observatory/visibility_packet.json` |
| 6 | `governed_adversarial_hardening` | WP-11-WP-13 | `runtime_v2/hardening/hardening_proof_packet.json` |

Later WPs may add evidence fields to their own artifacts, but they must not
silently reorder this spine or produce competing first-run packet contracts.

## D2 Classification

D2 is contract-proving after WP-04.

D3 is proving after WP-05.

D4 has scheduling and Freedom Gate mediation evidence after WP-07.

D5 is proving after WP-08.

D6 is proving after WP-09.

D7 is proving after WP-10.

D8 is proving after WP-12: the recovery eligibility model, decision records,
quarantine state machine, unsafe recovery fixture, and evidence-preservation
artifact are landed.

D9 is proving after WP-13: one governed adversarial hook is contained under
explicit rules of engagement, and duplicate activation, snapshot integrity, and
trace/replay gap failures are recorded as fail-closed hardening probes.

Proved now:

- a code-backed CSM run packet contract exists
- the contract round-trips to a golden fixture
- the fixture has a bounded claim boundary and stable review target
- the run spine and pre-live-run gates are explicit
- the invariant map is code-backed and golden-fixture checked
- the violation artifact schema is code-backed and golden-fixture checked
- the positive packet fixture and negative violation fixture are paired
- `proto-csm-01` boot/admission evidence is code-backed and golden-fixture checked
- the boot manifest admits `proto-citizen-alpha` and `proto-citizen-beta` with traceable identity handles
- the citizen roster and boot/admission trace preserve the provisional boundary
- resource pressure exceeds the available bounded budget
- the scheduler chooses the only admitted executable worker under pressure
- the first-run trace records resource loading, candidate ranking, scheduling, and deferral in contiguous order
- the scheduled action fixture binds the non-trivial action to `proto-citizen-alpha`
- the Freedom Gate decision allows the action only after bounded mediation
- the first-run trace records the Freedom Gate mediation event after scheduling
- the invalid-action fixture attempts to bypass the mediated Freedom Gate result
- the violation packet rejects that action before commit with unchanged state
- the first-run trace records the rejection event after Freedom Gate mediation
- the snapshot manifest and rehydration report validate before wake
- the wake continuity proof ties the restored active citizen to the snapshot record
- the duplicate-active-head guard is explicitly checked before wake
- the first-run trace records snapshot capture, rehydration validation, and wake continuity in contiguous order
- the D7 Observatory packet is generated from the run packet, boot/admission, first-run trace, and wake-continuity proof
- the D7 operator report is rendered from the same packet and checked against packet truth
- the operator-visible surface includes the invalid-action refusal, wake-continuity proof, and no-birthday boundary
- the D8 recovery eligibility model is generated from the invalid-action and wake-continuity evidence
- the safe-resume decision requires a declared predecessor, validated rehydration, and a unique active head
- the reject/quarantine decision refuses ambiguous predecessor linkage and duplicate active-head risk
- the D8 quarantine state machine accepts the quarantine-required decision, preserves evidence, and blocks execution pending operator review
- the quarantine evidence artifact retains the decision, violation, wake proof, snapshot, and rehydration report as immutable review evidence
- the D9 rules of engagement bound the adversarial hook to operator-scoped review
- the D9 adversarial hook attempts unsafe resume from quarantine and remains contained by the quarantine execution block
- the duplicate activation probe refuses a second active citizen head before commit
- the snapshot integrity probe refuses unverified wake before active state
- the trace/replay gap probe refuses replay with missing trace sequence and preserves evidence
- the hardening proof packet classifies D9 as proving while preserving live-run, first-birthday, and complete-security-ecology non-claims

Not proved yet:

- Observatory output is not a live Runtime v2 capture
- WP-14 still owns the integrated first CSM run proof

## Validation

Focused validation:

```sh
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_run_packet_contract -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_invariant_and_violation_contract -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_boot_admission -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_governed_episode -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_freedom_gate_mediation -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_invalid_action_rejection -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_wake_continuity -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_observatory -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_recovery_eligibility -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_quarantine -- --nocapture
cargo test --manifest-path adl/Cargo.toml runtime_v2_csm_hardening -- --nocapture
```

This validates the contract prototypes, golden fixtures, path hygiene, positive
and negative fixture pairing, and negative cases for unsafe paths,
non-contiguous stages, missing invariant/violation coverage, boot/admission
trace ordering, resource-pressure scheduling ambiguity, first-run trace
ordering, Freedom Gate action/decision mismatches, invalid-action rejection
before commit, duplicate-active wake rejection, wake continuity proof drift, and
operator-report drift, missing D7 source artifacts, recovery decision polarity,
ambiguous safe-resume rejection, complete recovery rule evaluation, and live-run
overclaiming, quarantine transition ordering, immutable evidence preservation,
unsafe release-to-active transitions, governed adversarial hook containment,
hardening probe refusal, and D9 proof-packet overclaim rejection.

## Non-Claims

This contract does not prove:

- a live CSM run
- first true Gödel-agent birth
- full v0.91 moral or emotional civilization
- v0.92 identity, migration, capability rebinding, or birthday semantics
