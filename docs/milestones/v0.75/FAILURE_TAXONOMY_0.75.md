# ADL v0.75 Failure Taxonomy (WP-04)

## Purpose
Define stable, deterministic failure codes used by runtime execution, replay tooling, and analytics surfaces.

## Categories
- `runtime_failure`
- `tool_failure`
- `policy_denied`
- `verification_failed`
- `replay_invariant_violation`
- `user_abort`
- `external_abort`

## Stable failure codes (v0.75)
Canonical stable codes emitted by current runtime/replay surfaces:
- `policy_denied`
- `verification_failed`
- `replay_invariant_violation`
- `provider_error`
- `timeout`
- `panic`
- `schema_error`
- `sandbox_denied`
- `io_error`

## Mapping rules
Deterministic category mapping for v0.75:
- `policy_denied` -> `policy_denied`
- `verification_failed` -> `verification_failed`
- `replay_invariant_violation` -> `replay_invariant_violation`
- `provider_error`, `timeout` -> `tool_failure`
- `panic`, `schema_error`, `sandbox_denied`, `io_error` -> `runtime_failure`
- unknown code fallback -> `runtime_failure`

## Surface behavior
- Runtime execution (`--run`) records stable code in `run_status.json` (`failure_kind`) and `run_summary.json` (`error_kind`) when classification is available.
- Replay/trace instrumentation uses `REPLAY_INVARIANT_VIOLATION` for activation-log contract/schema mismatches.
- Verification failures in signing/trust policy map to stable code `verification_failed`.

## Determinism requirements
- Classification is based on typed error kinds where available.
- Codes are static strings; no dynamic code generation.
- Replay contract mismatches are deterministic and code-stable across runs.

## Out of scope
- Retry strategy policies.
- Rich remediation engines.
- v0.8+ category expansion.
