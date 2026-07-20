# WP-20 Remediation Matrix for WP-19 Findings (#4647)

Status: in_progress_remediated_locally

Issue: #4647

Source register: `docs/milestones/v0.91.7/review/external_review_4646/FINDINGS_REGISTER.md`

Retained historical finding-validation input:
`docs/milestones/v0.91.7/review/V0917_EXTERNAL_REVIEW_VERIFICATION_2026-07-19.md`

## Disposition Matrix

| Finding | Disposition | Remediation / Evidence |
| --- | --- | --- |
| WP19-01 | fixed | `adl/tools/run_authoritative_coverage_lane.sh` rejects unsafe run ids and makes each declared run id own its LLVM profile directory plus run-scoped component/final summary output under the coverage build root. The root `adl/coverage-summary*.json` files remain last-writer compatibility copies only; the authoritative per-run evidence is the run-scoped output directory. `adl/tools/test_run_authoritative_coverage_lane.sh` proves unsafe run-id rejection and two concurrent distinct run ids using isolated profile/output roots without deleting or mixing each other's run-scoped summaries. |
| WP19-02 | fixed | `AwsBedrockProvider` now requires `ADL_AWS_BEDROCK_ACCOUNT_SHA256` or `config.expected_account_sha256`, treats the operator env pin as authoritative with fail-closed config conflict detection, compares the selected expected hash to the STS account hash before invocation, and records `account_hash_verified` instead of false `sts_verified`. |
| WP19-03 | fixed | Runtime API auth-event write failures now return `500 Internal Server Error` instead of being discarded; `runtime_api_auth_event_write_failure_fails_closed` proves the policy. |
| WP19-04 | fixed | Runtime API redaction now includes common secret keys (`api_key`, `password`, `private_key`, `access_key`) plus cloud/account-id key variants; the redaction regression checks arbitrary secret marker values and account identifiers are absent. |
| WP19-05 | fixed | The authoritative coverage runner captures partition failure status, attempts workspace/runtime summary reports, then exits with the recorded failure. The fake-cargo regression injects a partition failure and verifies both reports are attempted. |
| WP19-06 | fixed | Local Ollama streaming now buffers partial UTF-8 before invoking stream callbacks and drains malformed bytes with one replacement so later valid chunks are not blocked. `ollama_streaming_buffers_split_multibyte_utf8` proves split multibyte output and invalid-byte recovery. |
| WP19-07 | fixed | The #5571 audit remains historical. Current replacement dispatch authority is `external_review_4646/REVIEW_CORPUS.v1.txt` plus `PUBLICATION_SAFE_MANIFEST.md`, which limits publication to the replacement corpus. |
| WP19-08 | fixed | Provider endpoint validation parses IPv4/IPv6 loopback hosts with `IpAddr::is_loopback`; bracketed `[::1]` bearer endpoint coverage was added to `http_family` tests. |
| WP19-09 | fixed | `V0917_SPRINT_REVIEW_REGISTER.md` no longer directs operators to resolve already-closed #5406 and now consumes the closed terminal evidence. |
| WP19-10 | fixed | Provider invocation artifact locks now use the existing `fs2` OS advisory exclusive file-lock primitive instead of custom stale-directory reclamation. The OS releases the lock on process exit, removing the stale-lock reclaim/drop TOCTOU path while preserving atomic artifact writes and non-retryable partial-success classification when post-provider lock acquisition fails. The provider test stress-checks repeated concurrent lock contention and mutual exclusion. |
| WP19-11 | fixed | `adl/src/provider/http_family.rs` maps to the `provider_hardening` coverage-impact lane with a regression in `test_check_coverage_impact.sh`. |
| WP19-12 | fixed | The v0.91.8 review handoff digest procedure now hashes sorted tracked object records and a normalized handoff record that replaces only the mutable digest cell, avoiding self-inclusion of the digest-bearing document. |
| WP19-13 | fixed | `WP_ISSUE_WAVE_v0.91.7.yaml` now records WP-21A as closed in both summary and detail truth; YAML parse passed. |
| WP19-14 | fixed | `resolve_main_runtime_api_listener` parses and rejects non-loopback binds before `TcpListener::bind`; `validate_loopback_bind` remains as post-bind defense. Added `main_runtime_api_rejects_non_loopback_bind_before_listener_creation` as direct production-order negative proof for the resolver path. |
| WP19-15 | fixed | Authenticated GET now computes one identity-aware runtime API response through `runtime_api_get_response`; the admission header is derived from the same loaded response body when shutdown state is present, avoiding incoherent double snapshots. `runtime_api_get_admission_header_matches_loaded_shutdown_body` proves the header/body coherence. |
| WP19-16 | fixed | `exact_pid_is_live` still distinguishes missing PIDs as stale, while `daemon_supervisor_pid_liveness` reports existing but unauthenticated PIDs as `unknown` unless a future start-identity proof is available. `daemon_pid_liveness_does_not_claim_live_without_start_identity` uses the current process PID to prove no live claim is made from PID existence alone. |
| WP19-17 | fixed | Unauthenticated OPTIONS responses return before `record_request` and omit `x-csm-admission`, so they no longer consume bounded shutdown/test request budget or disclose admission state. |
| WP19-18 | fixed | `emit_runtime_api_client_error` now routes through the existing sanitizer before separator masking, and the sanitizer covers `/Users`, `/home`, `/private`, `/var/folders`, `/Volumes`, `/tmp`, and Windows user-path forms. |
| WP19-19 | fixed | `runtime_api_axum_response` forces HTTP 500 when serialization fallback is used. |
| WP19-20 | fixed | `PUBLICATION_SAFE_MANIFEST.md` now says no operator-specific paths and explicitly declares synthetic fixture exceptions. |
| WP19-21 | fixed | `RELEASE_TRUTH_GATE_STATUS_5544.md` now self-identifies as a historical snapshot superseded by the current sprint review register. |
| WP19-22 | fixed | v0.91.8 setup docs retain #5383 as closed historical setup authority; `setup/5383/DIAGRAM.mmd` no longer presents #5383 as pending. |

## Focused Validation Run

- `cargo fmt`
- `bash adl/tools/test_run_authoritative_coverage_lane.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_check_coverage_impact.sh`
- `cargo test -p adl provider::http_family::tests:: -- --test-threads=1 --nocapture`
- `cargo test -p adl provider::local::tests::ollama_streaming_buffers_split_multibyte_utf8 -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_options_does_not_consume_test_request_budget -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_redacts_secret_and_host_path_event_payloads -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_redacts_spec_derived_account_ids_and_windows_paths -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_get_admission_header_matches_loaded_shutdown_body -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::daemon_pid_liveness_does_not_claim_live_without_start_identity -- --nocapture`
- `cargo test -p adl csm_runtime_api::tests::runtime_api_auth_event_write_failure_fails_closed -- --nocapture`
- `cargo test -p adl csm_networking::tests::main_runtime_api_rejects_non_loopback_bind_before_listener_creation -- --nocapture`
- `ruby -e 'require "yaml"; require "date"; YAML.safe_load(File.read("docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml"), permitted_classes: [Date], aliases: true); puts "yaml-ok"'`

## Non-Claims

- No AWS operation was performed.
- WP-23 remains blocked until WP-20 publication, exact-revision review, CI, and truthful closeout complete.
- This matrix records local remediation state only until the PR exact head is reviewed and merged.
