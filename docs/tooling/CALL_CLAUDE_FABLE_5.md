# Call Claude Fable 5 Through ADL

This HOW-TO runs one bounded Claude Fable 5 request through ADL's Rust-native
provider adapter. It retains the normalized result and operational event log
under `.adl/local-artifacts/` without copying a credential into the repository.

## Prerequisites

- Run from an ADL issue worktree, not the primary `main` checkout.
- Use the existing repo binary. Do not rebuild merely to make the call.
- The operator-approved Anthropic key is at `$HOME/keys/claude2.key`.
- Never print, inspect, copy, or commit the key.

Set reusable paths:

```sh
ADL_REPO=/Users/daniel/git/agent-design-language
ADAPTER="$ADL_REPO/adl/target/debug/adl-provider-adapter"
RUN_DIR=.adl/local-artifacts/fable5-review
test -x "$ADAPTER"
mkdir -p "$RUN_DIR"
```

If that binary is absent, stop and use the repo's owner-binary installation or
build workflow. Do not silently download or compile tooling inside a validation
job.

## Prepare The Prompt

Write the bounded review or diagnosis prompt to:

```text
.adl/local-artifacts/fable5-review/prompt.txt
```

The prompt should state the evidence, desired decision, non-goals, and expected
output. Ask Fable to distinguish findings from inference and to avoid widening
the tracked issue.

## Render The Request

```sh
jq -n \
  --rawfile prompt "$RUN_DIR/prompt.txt" \
  '{
    request_id: "fable5-review-001",
    run_id: "fable5-review-001",
    route: {
      provider_kind: "hosted",
      provider: "anthropic",
      provider_model_id: "claude-fable-5",
      runtime_surface: "hosted_api",
      credential_ref: "env:ANTHROPIC_API_KEY"
    },
    model_identity: {
      provider_kind: "hosted",
      provider: "anthropic",
      model_ref: "claude-fable-5",
      provider_model_id: "claude-fable-5",
      runtime_surface: "hosted_api",
      identity_strength: "provider_asserted",
      observed_at: "live-provider-call"
    },
    prompt_contract_ref: "bounded_fable5_review.v1",
    lane_ref: "bounded_code_review",
    max_output_tokens: 4096,
    input_text: $prompt,
    attempt_policy: {
      timeout_ms: 180000,
      max_attempts: 1,
      retry_backoff_ms: 1000
    }
  }' >"$RUN_DIR/request.json"
```

Use unique `request_id` and `run_id` values for repeated calls. Keep retries at
one for diagnostic review unless the issue explicitly requires resilience
proof; repeated model calls are not a substitute for evidence.

## Run Fable 5

Map the key into `ANTHROPIC_API_KEY` only for the adapter process:

```sh
ANTHROPIC_API_KEY="$(<"$HOME/keys/claude2.key")" \
  "$ADAPTER" \
    --request "$RUN_DIR/request.json" \
    --out "$RUN_DIR/result.json" \
    --log "$RUN_DIR/run.log.jsonl"
```

The adapter emits heartbeat events while a provider response is pending. A
heartbeat is liveness evidence, not a completed result.

## Verify The Result

```sh
jq '{final_status, duration_ms, request_id, artifact_ref, trace_ref}' \
  "$RUN_DIR/result.json"
jq -e '.final_status == "ok" and (.output_text | length > 0)' \
  "$RUN_DIR/result.json" >/dev/null
jq -r '.output_text' "$RUN_DIR/result.json"
```

Expected retained files:

- `request.json`: provider route, bounded prompt, and attempt policy
- `result.json`: normalized status, model identity, timing, and response text
- `run.log.jsonl`: redacted start, heartbeat, attempt, and completion events

Treat model identity as provider-asserted unless stronger provider evidence is
retained. Fable's response is review input; source, tests, and live run evidence
remain authoritative.

## Failure Handling

- `missing_auth`: confirm the approved file exists and the command maps it to
  `ANTHROPIC_API_KEY`; do not print the file.
- `timeout`: inspect `run.log.jsonl`, increase the bounded timeout only when the
  prompt genuinely requires it, and do not loop calls blindly.
- HTTP authentication failure: stop and resolve account/key state outside the
  repository.
- Empty or malformed result: retain the artifacts and treat the call as failed.
- Slow response with continuing heartbeats: allow the bounded attempt to finish.

Never commit requests containing private customer material, provider responses
that are not publication-safe, credentials, account identifiers, or raw
authorization diagnostics.
