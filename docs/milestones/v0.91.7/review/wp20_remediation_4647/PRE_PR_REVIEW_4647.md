# WP-20 Pre-PR Review Record (#4647)

Status: completed_with_findings_fixed

Reviewer: bounded read-only subagent review

Scope:

- Current #4647 worktree diff.
- `docs/milestones/v0.91.7/review/external_review_4646/FINDINGS_REGISTER.md`
- `docs/milestones/v0.91.7/review/wp20_remediation_4647/WP19_FINDING_REMEDIATION_MATRIX_4647.md`
- Exact verification artifact retention.

## Findings And Disposition

| Finding | Severity | Disposition |
| --- | --- | --- |
| OPTIONS still exposed `x-csm-admission` on unauthenticated preflight responses. | P3 | Fixed. OPTIONS responses now strip `x-csm-admission`, and `runtime_api_options_does_not_consume_test_request_budget` asserts the header is absent. |
| Local Ollama UTF-8 callback recovery could stall after an invalid byte. | P3 | Fixed. Malformed bytes now drain with one replacement and later valid chunks continue; `ollama_streaming_buffers_split_multibyte_utf8` covers split-valid and invalid-byte recovery. |

No P0, P1, or P2 blocking findings were reported by the bounded review.

## Rechecked Proof

- `cargo fmt`
- `cargo test -p adl provider::local::tests::ollama_streaming_buffers_split_multibyte_utf8 -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_options_does_not_consume_test_request_budget -- --nocapture`

The reviewer also verified the exact external-review artifact digest and size
before these two follow-up fixes:

- SHA-256: `e45d5841f7d341493480fabebc87e5fb08cef589fe05b0f6fecd653ab97d3cc1`
- Size: `4699` bytes

## Non-Claims

- No AWS operation was performed.
- WP-23 was not started.
- This review record is pre-PR local evidence; merge readiness still depends on typed review/publication and required CI.
