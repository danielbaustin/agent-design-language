# ADL v0.91.4 External Review Remediation

- Date: `2026-06-01`
- Source review: `V0914_EXTERNAL_REVIEW_FINDINGS_2026-06-01.md`
- Remediation issue: `#3560`
- Parent remediation lane: `WP-18` / `#3368`

## Scope

This note records the WP-18 disposition for the four findings from the WP-17
external review. It does not close WP-17 or WP-18 by itself.

## Finding Disposition

| Finding | Severity | Disposition | Evidence |
| --- | --- | --- | --- |
| `R1` PVF policy test retains the F001 bug class on the release lane | `P2` | fixed in `#3560` | `adl/tools/test_pvf_ci_release_policy.sh` now captures release-mode runner status before assertions, matching the docs/runtime lanes. |
| `R2` WildClawBench host path persists after replayability-boundary cleanup | `P3` | fixed in `#3560` | `docs/milestones/v0.91.4/WILDCLAW_SAFETY_ALIGNMENT_RESULTS_2026-05-27.md` now uses an operator-local checkout placeholder and records that the checkout is not tracked release evidence. |
| `R3` PVF CI wiring is path-policy gated | `P3` | fixed in `#3560` | `adl/tools/test_ci_path_policy.sh` now asserts that changes to `adl/tools/run_pvf_validation_lane.sh` require CI contract validation. |
| `R4` F003 provider identity closure is not verifiable from tracked state | `P2` | disposition recorded in `#3560`; underlying fix already landed | `#3544` and merged PR `#3548` explicitly covered `WP16-F003`; PR `#3548` records the focused test `provider_substrate_uses_http_transport_for_ollama_with_endpoint`, verifying remote Ollama identity classification. |

## R4 Provider Identity Disposition Detail

The external review correctly found that the handoff table did not make F003's
closure easy to verify. The tracked issue graph does contain the evidence:

- `#3544` source finding scope: `WP16-F003` remote Ollama provider identity
  uses misleading `hosted_http`.
- `#3548` merged remediation summary: remote Ollama HTTP identities are labeled
  as `ollama_http`.
- `#3548` validation: focused test
  `provider_substrate_uses_http_transport_for_ollama_with_endpoint`.

Therefore R4 is closed as a traceability defect: the provider implementation fix
was already merged, and this remediation records the missing audit trail.

## Validation Plan

Focused validation for `#3560` should include:

```bash
bash adl/tools/test_pvf_ci_release_policy.sh
bash adl/tools/test_ci_path_policy.sh
git diff --check
```

No broad Rust or slow-proof run is required unless later review finds runtime
behavior changed outside the bounded script/path-policy surfaces.

## Non-Claims

This remediation note does not claim:

- release approval
- WP-17 closeout
- WP-18 closeout
- WildClawBench benchmark replayability from tracked repository artifacts alone
- new provider/model matrix implementation beyond the already merged `#3548`
  provider identity fix
