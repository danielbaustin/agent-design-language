# Structured Output Record

Template: 1.0.0

Issue: 5404

Repository: danielbaustin/agent-design-language

Card: sor

Status: complete

## Summary

Resolved WP-12 review findings by downgrading unproven CAV integrated-path claims to boundary-proven retained proof, classifying credential proof events as synthetic non-operational evidence, and wiring focused WP-12 validators into PR-fast CAV coverage.

## Artifacts

- docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json
- docs/milestones/v0.91.7/review/security/wp12_csm_credential_policy_4920/credential_policy_summary.json
- docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json

## Execution

- Downgraded #4914 CAV retained proof and gate truth from integrated CSM HTTP runtime execution to boundary_proven local/static red-blue evidence.
- Regenerated retained #4914 CAV and #4920 credential policy artifacts from the patched CSM owner binary path.
- Marked credential lifecycle proof events as synthetic negative-case evidence excluded from operational audit streams.
- Updated WP-12 validators and PR-fast coverage companion checks to fail closed on stale #4914 integrated_proven or missing synthetic credential classification.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove PR-fast CAV companion routing and fail-closed #4657/#4660 negative fixtures.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_pr_fast_coverage_lane.sh"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_cav_red_blue"
    ],
    "purpose": "Prove bounded CAV red-blue artifact behavior and unsafe run-id rejection.",
    "outcome": "passed",
    "evidence_ref": "adl/src/csm_cav_red_blue.rs"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "csm_credential_policy"
    ],
    "purpose": "Prove synthetic credential-event classification and unsafe run-id rejection.",
    "outcome": "passed",
    "evidence_ref": "adl/src/csm_credential_policy.rs"
  },
  {
    "command": [
      "python3",
      "adl/tools/validate_wp12_cav_red_blue_4914.py",
      "--proof",
      "docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json",
      "--parent-gate",
      "docs/milestones/v0.91.7/review/security/wp12_security_cav_gate_4656.json",
      "--coherence",
      "docs/milestones/v0.91.7/review/runtime/final_csm_coherence_4906/runtime_coherence_matrix_4906.json"
    ],
    "purpose": "Prove the downgraded boundary-only CAV claim and canonical coherence linkage.",
    "outcome": "passed",
    "evidence_ref": "docs/milestones/v0.91.7/review/security/wp12_cav_red_blue_4914/cav_red_blue_summary.json"
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
