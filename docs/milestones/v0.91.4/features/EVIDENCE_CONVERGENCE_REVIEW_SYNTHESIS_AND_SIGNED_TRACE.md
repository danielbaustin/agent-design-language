# Evidence Convergence, Review Synthesis, And Signed Trace Proof

## Status

Landed in `WP-06`.

## Purpose

Complete the durable proof path for C-SDLC transitions by converging evidence,
review synthesis, and signed trace verification into one inspectable record.

By the end of `v0.91.4`, durable C-SDLC proof should not depend on local-only
state. The proof must be tracked, reviewable, and able to show that the
transition record has not drifted silently.

## Scope

This feature covers:

- evidence bundle finalization
- review synthesis output
- tracked proof-packet paths
- minimal signed trace bundle shape
- signature or digest verification result
- linkage from `SRP` and `SOR` into the signed proof bundle
- release evidence that names any unsigned or deferred transition proof

## Acceptance Criteria

- Durable C-SDLC transitions emit tracked evidence and review synthesis
  surfaces.
- Durable proof includes a minimal signed trace bundle or an explicit blocker.
- Verification results are recorded in repo-relative paths.
- The `SOR` links to the evidence bundle, review synthesis, and signed trace
  proof.
- Missing or unverifiable signed trace proof blocks default-operation claims.

## Proof Surface

- `docs/milestones/v0.91.4/review/evidence/csdlc/C_SDLC_EVIDENCE_BUNDLE_PACKET_v0.91.4.md`
- `docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_transition_evidence_bundle.json`
- `docs/milestones/v0.91.4/review/evidence/csdlc/ct_demo_001_review_synthesis.json`
- `docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_signed.adl.yaml`
- `docs/milestones/v0.91.4/review/evidence/csdlc/fixtures/minimal_transition_trace_public_key.b64`
- `python3 adl/tools/validate_v0914_csdlc_evidence_bundle.py docs/milestones/v0.91.4/review/evidence/csdlc`
- `bash adl/tools/test_v0914_csdlc_evidence_bundle.sh`

## Non-Goals

- This feature does not require full trace query/TQL completion.
- This feature does not replace normal CI or PR checks.
- This feature does not make unsigned local logs acceptable durable proof.
