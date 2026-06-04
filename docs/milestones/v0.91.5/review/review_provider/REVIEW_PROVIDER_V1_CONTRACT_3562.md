# Review Provider V1 Contract For CodeFriend

Issue: #3562
Milestone: v0.91.5
Status: proposed contract slice

## Summary

`ReviewProviderV1` gives ADL and CodeFriend a first-class lane for asking an
external model provider to review a bounded packet without treating that model
as an authoritative reviewer.

The lane reuses the ADL provider communication substrate. A review provider is
therefore a specialized review envelope over `ProviderInvocationRequestV1` and
`ProviderInvocationResultV1`, not a separate transport system.

## Authority Boundary

External provider-backed reviewers are advisory only.

They may produce candidate findings, summaries, and risk notes. They may not:

- replace native Codex subagents
- modify a worktree directly
- close issues or PRs
- make merge, release, or publication decisions
- bypass CodeFriend synthesis or human/operator review

Every review-provider run must record an `authority_boundary` string. The
canonical boundary for the first slice is:

```text
advisory_findings_only_requires_codefriend_synthesis
```

The first validator intentionally accepts only this boundary string. New
authority modes should be introduced by versioning the contract rather than by
silently accepting ambiguous prose.

## Contract Objects

The Rust substrate now carries these review-provider objects in
`adl/src/provider_communication.rs`:

- `ReviewProviderV1`
- `ReviewProviderRequestV1`
- `ReviewProviderResultV1`
- `ReviewRunRecordV1`
- `ReviewFindingV1`
- `ReviewProviderRoleV1`
- `ReviewResultStatusV1`
- `ReviewRedactionStatusV1`
- `ReviewFindingSeverityV1`

The review-provider request embeds `ProviderInvocationRequestV1`. The result
embeds `ProviderInvocationResultV1`. This preserves provider route, model
identity, attempts, failures, duration, and log semantics across normal provider
execution and review-provider execution.

## Required Request Fields

A `ReviewProviderRequestV1` must include:

- `schema_version`
- `review_request_id`
- `review_provider`
- `review_packet_ref`
- `rubric_ref`
- `requested_at`
- at least one scoped review target:
  - `issue_ref`
  - `pr_ref`
  - `diff_ref`
  - `file_refs`

Scope references must contain non-empty strings. Empty or whitespace-only issue,
PR, diff, or file references fail validation.

The embedded `ReviewProviderV1` must include:

- `provider_ref`
- `role`
- `provider_request`
- `authority_boundary`

The embedded provider request must still pass `validate_provider_request`.

## Required Result Fields

A `ReviewProviderResultV1` must include:

- `schema_version`
- `review_request_id`
- `provider_result`
- `review_status`
- `redaction_status`
- `findings`
- `started_at`
- `completed_at`
- `elapsed_ms`
- `log_ref`

Provider errors remain provider errors. An auth failure, timeout, empty output,
or malformed provider response must not become a scored review finding. The
review lane fails closed with one of these statuses:

- `failed_provider`
- `failed_malformed`
- `blocked`
- `skipped`

A successful review with findings uses `findings`. A successful review with no
findings uses `passed`.

Failed, blocked, or skipped review-provider results must not carry scored
findings. A `findings` status must carry at least one finding. A `passed` status
must carry no findings.

## Provider Support

The contract is provider-family neutral. It supports these families through the
same provider route/request shape:

- OpenAI
- Anthropic/Claude
- Gemini
- Ollama/local
- DeepSeek
- mock providers for tests

DeepSeek sequencing depends on #3549 for native provider availability. This
contract does not claim all providers are fully implemented review executors in
this PR.

## Proposed CLI Boundary

The recommended CLI entrypoint is a narrow wrapper over the existing provider
adapter substrate:

```bash
adl review-provider run \
  --request review/requests/<run>.json \
  --out review/results/<run>.json \
  --log review/logs/<run>.jsonl
```

The command should:

1. read `ReviewProviderRequestV1`
2. validate review scope and authority boundary
3. call the existing provider adapter using the embedded provider request
4. parse the model output into `ReviewProviderResultV1`
5. fail closed on provider failure, empty output, timeout, auth failure, or
   malformed findings
6. write tail-friendly JSONL log events with prompts and raw provider details
   redacted
7. write one final JSON result artifact

## CodeFriend Ingestion

CodeFriend should treat `ReviewProviderResultV1` as one review artifact among
many. Synthesis remains responsible for:

- severity normalization
- duplicate finding collapse
- source-evidence checks
- unsupported-claim removal
- final findings-first report structure
- residual-risk recording

Provider-backed review findings are never automatically authoritative.

## Validation Performed In This Slice

Focused Rust tests cover:

- review-provider request validation
- required review scope
- required authority boundary
- schema-version checks for request and provider envelopes
- reuse of `ProviderInvocationRequestV1`
- review result shape with provider failures preserved separately from findings
- rejection of scored findings on failed, blocked, skipped, or passed results

## Follow-On Slices

Recommended follow-ons:

1. Add `adl review-provider run` CLI using the existing provider adapter.
2. Add one hosted provider smoke proof and one Ollama/mock proof.
3. Add CodeFriend ingestion of `ReviewProviderResultV1` artifacts.
4. Add JSON schema export for review-provider objects.
5. Add UTS/CodeFriend compatibility notes once provider evidence contracts are
   shared across repos.

## Non-Claims

- This PR does not replace Codex subagents.
- This PR does not implement every provider executor for review runs.
- This PR does not claim external reviewer output is correct by default.
- This PR does not expose credentials or raw prompts in tracked artifacts.
