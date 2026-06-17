# Logging Validation And Redaction Proof (`#4000`)

## Scope

Bounded proof for the validation and hygiene checks that logging-affecting
issues in `v0.91.6` must use.

## Evidence Surfaces

- `docs/milestones/v0.91.5/LOGGING_VALIDATION_CHECKLIST_3711.md`
- `docs/milestones/v0.91.5/DOCS_ONLY_VALIDATION_BUNDLE_3736.md`
- `adl/tools/test_control_plane_observability.sh`
- `adl/tools/test_pr_json_observability.sh`
- `adl/src/cli/observability.rs`

## Validation Selection Used For WP-03

| Change class | Local proof |
| --- | --- |
| control-plane logging behavior | `adl/tools/test_pr_json_observability.sh`; `adl/tools/test_control_plane_observability.sh` |
| runtime/provider durable logs | `cargo test --manifest-path adl/Cargo.toml instrumentation::action_log -- --nocapture` |
| heartbeat / timeout / progress | focused `cli::observability` and `finish_support` tests |
| OTel boundary / Observatory example | packet inspection plus explicit non-claims |
| docs / milestone packet truth | Markdown review plus repo-relative path hygiene |

## Claimed Result

- WP-03 now has one explicit focused-validation selection matrix rather than a
  vague “tests passed” claim.
- Control-plane channel policy, redaction, and compatibility-log behavior are
  part of the required proof surface.
- The bad-sink compatibility path is proven only for the current quiet-stderr
  contract: validator stdout survives and no extra stderr fallback is emitted
  when stderr is intentionally suppressed.
- Durable runtime/provider logging claims remain bounded to the named artifact
  and contract surfaces.

## Problems Captured For Remediation

- The shared runtime progress surface is still split between `adl_event` and
  plain stderr progress lines.
- Future logging-affecting issues should keep proving only the touched surface
  rather than reflexively widening to broad suites.

## Non-Claims

- This packet does not replace a full security review.
- This packet does not claim every repo command is covered by the same logging
  validator surface.
