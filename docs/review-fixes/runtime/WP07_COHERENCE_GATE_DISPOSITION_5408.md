# WP-07 Coherence Gate Disposition (#5408)

## Disposition

`#4906` remains `blocked_with_evidence`. This issue does not claim that the
final CSM runtime coherence gate is closed or release-ready.

The implemented WP-07 repair is independently complete for the bounded
surfaces in this issue:

- governed emergency-stop authorization is verified with an Ed25519 signature
  against the pre-established locked agent spec;
- named API Gateway route and negative-case proof is retained, with incomplete
  live failure-matrix coverage classified as `bounded_smoke`;
- forged authorization, wrong operator, route omission, malformed denial, and
  identity mismatch remain negative-proof cases.

## Operator Disposition

Keep WP-07 coherence consumers blocked until the owner of #4906 supplies its
remaining assembled-runtime evidence and records a separate reviewed closure.
This is an explicit hold, not a waiver, release approval, or substitute for
the missing live gate proof.

## Evidence

- `adl/src/long_lived_agent.rs`
- `adl/src/long_lived_agent/tests.rs`
- `adl/src/csm_api_gateway_bridge.rs`
- `adl/tools/validate_runtime_hardening_5408.py`
- `docs/review-fixes/runtime/WP07_HARDENING_REPAIR_5408.md`
