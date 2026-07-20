# v0.91.7 WP-19 External Finding Register

Status: complete_with_provider_degradation

Issue: #4646

Target revision: `bd9b7a3c58417d20768b31bc1fede03ec8e3cfe5`

Packet digest: `ccc7c9dfeb404d3855b8184d5da05367c992771d4c09ec97ff2845dc022fdb32`

Corpus: 33 manifest entries expanding to 70 tracked blobs

## Outcome

All 70 frozen files received findings-first review. The operator-dispatched
automated Fable 5 lane completed the 182,171-byte `adl/src/csm_runtime_api.rs`
lane. The remaining three Fable
calls could not complete after the Anthropic account became billing-blocked;
three independent shadow reviewers covered the remaining 69 files. This is a
completed bounded review with explicit provider degradation, not a claim that
all 70 files received independently controlled third-party model coverage.

The combined result contains 22 actionable findings: 2 P1, 11 P2, and 9 P3.
WP-20 #4647 owns deduplication, acceptance, remediation, and any issue routing.

## Findings

| ID | Severity | Lane | Evidence | Finding and bounded remediation |
| --- | --- | --- | --- | --- |
| WP19-01 | P1 | shadow | `adl/tools/run_authoritative_coverage_lane.sh:103,169,201-204` | Shared LLVM profile storage can delete or mix concurrent runs. Isolate or serialize profile directories and add a concurrency test. |
| WP19-02 | P1 | shadow | `adl/src/provider/http_family.rs:699-710,737-743,262-273` | Bedrock validates the profile name but not the approved AWS account identity. Compare STS identity to an operator-approved expected source before invocation. |
| WP19-03 | P2 | Fable 5 | `adl/src/csm_runtime_api.rs:300-370` | Request-path auth audit writes discard errors. Surface failures and choose a documented fail-closed or availability-preserving policy. |
| WP19-04 | P2 | Fable 5 | `adl/src/csm_runtime_api.rs:1000-1020` and response redaction assertion | Redaction key and assertion gaps can falsify the `secret_material` and cloud-account non-return claims. Align case-insensitive checks, expand keys, and add negative tests. |
| WP19-05 | P2 | shadow | `adl/tools/run_authoritative_coverage_lane.sh:2,195,199-204` | A failing partition exits before the promised final coverage summary. Capture status, generate available evidence, then return the failure. |
| WP19-06 | P2 | shadow | `adl/src/provider/local.rs:100,143,183,195,216-219` | Chunk-local lossy UTF-8 decoding can corrupt streamed Ollama output. Use an incremental decoder and test a split multibyte character. |
| WP19-07 | P2 | shadow | `docs/reviews/v0.91.7/internal-review-4645/redaction-audit-5571/manual_review.json:4-22` | The #5571 publication audit is bound to the superseded #5579 corpus, not the replacement digest. Retain it as historical and bind a replacement audit to the exact target. |
| WP19-08 | P2 | shadow | `adl/src/provider/http_family/config.rs:25,51-60,84` | Bracketed IPv6 loopback endpoints are classified as non-loopback. Parse to `IpAddr` and use `is_loopback`; add IPv6 tests. |
| WP19-09 | P2 | shadow | `docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:94,103,119` | The canonical register still directs operators to resolve already-closed #5406. Record its terminal evidence and remove the stale action. |
| WP19-10 | P2 | shadow | `adl/src/provider/http_family.rs:24-50,139-143,443-447,515-520` | A crashed invocation-record writer can leave a permanent lock and cause duplicate billable calls. Add lease metadata, stale-lock recovery, and partial-success classification. |
| WP19-11 | P2 | shadow | `adl/tools/check_coverage_impact.sh:261-264,350-352,605-617` | `adl/src/provider/http_family.rs` has no focused coverage mapping. Map it to the provider-hardening lane and add a regression case. |
| WP19-12 | P2 | shadow | `docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md:45-71` | The v0.91.8 packet digest hashes filenames only. Hash sorted `git ls-tree` object records so content drift changes identity. |
| WP19-13 | P2 | shadow | `docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml:19,24,327` | WP-21A is simultaneously closed and `status: open`. Set the work-package status to closed and validate summary/detail consistency. |
| WP19-14 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs:100-125` | Loopback validation occurs after socket bind. Validate literal configuration before binding and retain the post-bind check. |
| WP19-15 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs:1450-1470` | Authenticated GETs compute the full status projection twice. Compute one coherent response and reuse one loaded spec. |
| WP19-16 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs` `exact_pid_is_live` | PID liveness treats EPERM as live and does not detect PID reuse. Downgrade unattributable state or cross-check process start identity. |
| WP19-17 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs` OPTIONS handler | Unauthenticated OPTIONS discloses admission state and consumes test shutdown budget. Omit the header and count only authenticated or non-OPTIONS requests. |
| WP19-18 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs` `emit_runtime_api_client_error` | Error logging masks separators but can retain usernames and path structure. Route errors through the existing sanitizer. |
| WP19-19 | P3 | Fable 5 | `adl/src/csm_runtime_api.rs` `runtime_api_axum_response` | Serialization fallback can return an error body with the original success status. Force HTTP 500 on fallback. |
| WP19-20 | P3 | shadow | `docs/milestones/v0.91.7/review/external_review_4646/PUBLICATION_SAFE_MANIFEST.md:26,33-38` | The path-safety claim contradicts its synthetic `/Users/example/` exception. Say “no operator-specific paths” and declare fixture exceptions. |
| WP19-21 | P3 | shadow | `docs/milestones/v0.91.7/review/wp20_remediation_5544/RELEASE_TRUTH_GATE_STATUS_5544.md:3,20-28` | A superseded snapshot self-identifies as active. Mark it historical and link the current register. |
| WP19-22 | P3 | shadow | `docs/milestones/v0.91.8/README.md:32`; `docs/milestones/v0.91.8/setup/5383/DIAGRAM.mmd:2` | The setup diagram presents closed #5383 as pending. Render it as closed historical setup authority. |

## Provider Evidence

- Fable 5 health probe: passed with `FABLE5_OK`.
- Full 706,468-byte call: HTTP 200, failed with `provider_empty_text_output`.
- Fable lane 1 with 16,384 output tokens: passed in 162,144 ms.
- Fable lane 2: timed out at 180 seconds while heartbeats continued.
- Later Fable lanes: failed closed with `provider_billing_blocked` after the
  Anthropic credit balance was exhausted.
- Raw provider artifacts remain local under
  `.adl/local-artifacts/fable5-wp19-final/` and are not publication claims.

## Non-Claims

- No AWS operation or deployment was performed.
- No v0.91.7 release or v0.92 activation approval is recorded here.
- The shadow lanes are independent review evidence, not third-party provider
  identity.
- Closed #4906 retains its unresolved `blocked_with_evidence` release impact.
- No issue is created automatically per finding; WP-20 owns grouping.
