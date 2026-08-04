# Structured Output Record

Template: 1.0.0

Issue: 5347

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Deleted obsolete externally-owned standalone demo/proof entrypoints from the incumbent ADL bin surface, removed the matching retired WP-12 Cargo bin declaration, retained current operational binaries, preserved #5346 as the later final-core deletion lane, and proved no WP-16/#5346 terminal execution dependency.

## Artifacts

- adl/Cargo.toml
- adl/src/bin/demo_v086_candidate_selection.rs
- adl/src/bin/demo_v086_fast_slow.rs
- adl/src/bin/demo_v086_freedom_gate.rs
- adl/src/bin/demo_v086_review_surface.rs
- adl/src/bin/demo_v0905_local_gemma_model_evaluation.rs
- adl/src/bin/demo_v0905_model_proposal_benchmark.rs
- adl/src/bin/demo_v0911_capability_aptitude_testing.rs
- adl/src/bin/demo_v0912_gws_live_capability_execution_surface.rs
- adl/src/bin/demo_v0912_gws_live_content_card_roundtrip.rs
- adl/src/bin/demo_v0912_gws_live_safety_package.rs
- adl/src/bin/demo_v0912_provider_native_tool_call_comparison.rs
- adl/src/bin/demo_v0912_rust_native_gws_adapter_boundary.rs
- adl/src/bin/demo_v0912_speculative_decoding_prototype.rs
- adl/src/bin/demo_v0912_uts_acc_multi_model_benchmark.rs
- adl/src/bin/demo_v0917_dspark_speculative_decoding_evaluation.rs
- adl/src/bin/run_v0916_acip_aee_memory_integration.rs
- adl/src/bin/run_v0916_integrated_runtime_soak.rs
- adl/src/bin/run_v0916_runtime_failure_injection.rs
- adl/src/bin/run_v0917_integrated_resilience_failure_injection.rs
- adl/src/bin/run_wp12_acip_websocket_transport_proof.rs
- docs/milestones/v0.91.8/evidence/wp13-external-bands/external-band-deletion-manifest.json
- docs/milestones/v0.91.8/evidence/wp13-external-bands/wp13-deletion-coordination.json
- docs/milestones/v0.91.8/evidence/wp13-external-bands/deletion-accounting.json
- .csdlc/prepared/issues/5347/run-validation-lane.rb

## Execution

- Deleted 20 obsolete demo/proof bin files under adl/src/bin
- Removed the explicit run_wp12_acip_websocket_transport_proof Cargo bin declaration
- Added a compact #5347 deletion manifest, #5346 coordination manifest, and line accounting
- Replaced the overbuilt future-gate validator with a focused execution validator
- Restored incidental adl/Cargo.lock churn and reacquired an exact deletion-only claim

## Validation

[
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/5347/validation-request.json"
    ],
    "purpose": "Prove #5347 preparation contract, future lane contract, diff hygiene, and fail-closed blocked execution admission with product_changes=0",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/preparation-validation/{preparation-contract.log,future-lane-contract.log,blocked-execution-admission.log,diff-hygiene.log}"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5347/run-validation-lane.rb",
      "execution"
    ],
    "purpose": "Prove #5347 deletes only manifest-listed obsolete external demo/proof binaries, preserves retained current binaries, does not depend on WP-16 or #5346 terminal closeout, remains disjoint from #5346 reserved core deletion paths, and removes 1476 net lines.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/execution-validation/external-band-deletion.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5347/run-validation-lane.rb",
      "execution"
    ],
    "purpose": "Final proof that #5347 deletes only manifest-listed obsolete external demo/proof binaries, preserves retained current binaries, has no Cargo.lock churn, does not depend on WP-16 or #5346 terminal closeout, remains disjoint from #5346 reserved core deletion paths, and removes 1476 net lines.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/execution-validation/external-band-deletion-final.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5347/run-validation-lane.rb",
      "execution"
    ],
    "purpose": "Final cleaned proof that #5347 deletes only manifest-listed obsolete external demo/proof binaries, preserves retained current binaries, has no Cargo.lock diff, does not depend on WP-16 or #5346 terminal closeout, remains disjoint from #5346 reserved core deletion paths, and removes 1476 net lines.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/execution-validation/external-band-deletion-cleaned.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5347/run-validation-lane.rb",
      "execution"
    ],
    "purpose": "Prove #5347 review-finding fixes removed obsolete Rust test references, repaired shell/tool stale references, strengthened the recurrence guard, preserved #5346 disjointness, and compiled surviving integration tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/execution-validation/review-finding-fix-validation.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5347/run-validation-lane.rb",
      "execution"
    ],
    "purpose": "Prove #5347 review-finding fixes removed obsolete Rust test references, repaired shell/tool stale references, strengthened the recurrence guard, preserved #5346 disjointness, and compiled surviving integration tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5347/execution-validation/review-finding-fix-validation.json"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
