# Using Claude Opus for ADL reviews

This is the operator procedure for a bounded, evidence-first review through
the Rust `adl-provider-adapter`. It is review evidence, not lifecycle
authority: the typed `csdlc-review` record remains authoritative.

## Preconditions

- Work is in an issue-bound, non-`main` worktree.
- The exact revision and changed-path scope are known.
- Focused validation has passed and `git diff --check` is clean.
- The approved credential is available at `$HOME/keys/claude2.key`. Read it
  only for the adapter command; never print, copy, commit, or persist it.
- The prompt contains the actual diff or source excerpts. A request that only
  says “review this” is not sufficient evidence.

## Build a structured request

The current binary accepts one JSON request and writes separate JSON result and
JSONL log files. The request shape is defined by the provider communication
schema in `adl/src/provider_communication.rs`:

```json
{
  "request_id": "issue-1234-opus-review-<sha>",
  "run_id": "issue-1234-opus-review-<sha>",
  "route": {
    "provider_kind": "hosted",
    "provider": "anthropic",
    "provider_model_id": "claude-opus-5",
    "runtime_surface": "hosted_api",
    "credential_ref": "env:ANTHROPIC_API_KEY"
  },
  "model_identity": {
    "provider_kind": "hosted",
    "provider": "anthropic",
    "model_ref": "claude-opus-5",
    "provider_model_id": "claude-opus-5",
    "runtime_surface": "hosted_api",
    "identity_strength": "provider_asserted",
    "observed_at": "unix:1"
  },
  "prompt_contract_ref": "csdlc.review.findings.v1",
  "lane_ref": "anthropic:claude-opus-5__exact-head-review",
  "attempt_policy": {
    "timeout_ms": 120000,
    "max_attempts": 1,
    "retry_backoff_ms": 1000
  },
  "max_output_tokens": 12000,
  "input_text": "Findings first. Review the exact revision and listed paths..."
}
```

Keep the review prompt bounded. It should name the issue, exact revision,
changed paths, acceptance criteria, validation results, provider limitations,
and the required finding format (severity, evidence, disposition, residual
risk). Do not place credentials, full sensitive responses, or machine-local
absolute paths in the request.

## Invoke the Rust adapter

Use the actual structured interface: `--request`, `--out`, and `--log`.

```sh
ANTHROPIC_API_KEY="$(< "$HOME/keys/claude2.key")" \
  cargo run --quiet --manifest-path adl/Cargo.toml --bin adl-provider-adapter -- \
  --request request.json \
  --out result.json \
  --log run.jsonl
```

The adapter's bounded request policy is the safety boundary; do not replace it
with a tiny arbitrary output limit. Inspect the result JSON and JSONL log for
the provider/model identity, HTTP status, attempt status, and final status.
An HTTP 200 only proves the adapter reached the provider. It does not prove the
review is correct, complete, or suitable for publication.

## Review and record findings

Treat the response as review evidence. Fix every actionable in-scope finding,
rerun focused validation, and obtain a fresh review at the final exact source
revision. Classify out-of-scope findings as routed follow-ups; do not hide them
inside the implementation issue.

Record the final result with the typed v2 command and a request that names the
exact clean revision, scope, reviewer identity, findings, and dispositions:

```sh
cargo run --quiet --manifest-path csdlc-v2/Cargo.toml --bin csdlc-review -- \
  --root . record --request .csdlc/review-record-<issue>.json
```

Run the review guard before publication. Publication must fail closed when
review truth is absent or stale.

## Truth boundaries and minimum evidence

- Provider reachability, adapter success, and review correctness are separate
  claims.
- Live-provider failures (authentication, balance, quota, or rate limit) stay
  typed and truthful; never turn them into a completion claim.
- Metadata-only commits can stale source review. Re-review the exact final
  source revision before publication.
- Retain focused validation output, `git diff --check`, the exact revision and
  path scope, the review summary and dispositions, live-probe limitations, and
  the typed review/guard results.

The repository contract check is `bash adl/tools/test_opus_review_runbook.sh`.
It verifies the documented flags and request fields against the current Rust
binary help and rejects retired flag-form instructions, secrets, and absolute
machine paths.
