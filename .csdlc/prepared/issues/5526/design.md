# Issue 5526 Design

Status: approved for preparation only.

## Scope

Issue #5526 prepares the WP-09 provider/model expansion lane for later
execution. The future implementation may add distinct provider identities,
model profiles, capability truth, deterministic fixtures, and bounded smoke
plans for the current Kimi, MiniMax, hosted Qwen, xAI, Mistral, Cohere,
DeepSeek, Z.ai, Gemini, OpenAI, and Anthropic surfaces named by the source
issue.

This preparation packet does not implement product behavior, perform provider
calls, read credentials, create a PR, or mutate GitHub. It only records the
execution-ready issue boundary, expected files, validation posture, and review
focus.

## Execution Gate

Execution is gated by live WP-09 merge and ancestry truth:

- parent WP-09 issue #5349 must be merged into the current `origin/main`;
- the issue #5526 branch must be based on, or rebased onto, an `origin/main`
  revision that contains that merge;
- retained closeout or receipt artifacts are non-blocking audit evidence only;
- receipts are non-blocking audit evidence, never an execution gate.

If live ancestry and merge proof disagree with receipts, execution must fail
closed and refresh from live Git truth.

## Architecture

Provider identity is separate from wire compatibility. A provider can reuse an
OpenAI-compatible transport only when its authentication, base URL, model
catalog, capability set, error taxonomy, streaming behavior, tool-call
semantics, usage reporting, caching, and retry boundaries are explicitly
classified for that vendor.

Model selection must preserve deterministic execution records:

- aliases resolve to retained provider/model/version truth before execution;
- model discovery is bounded, timeout-controlled, and snapshot-backed;
- dynamic discovery cannot be required for deterministic replay;
- direct-provider evidence remains distinct from OpenRouter routing evidence;
- local Ollama or vLLM profiles remain distinct from hosted vendor providers.

## Validation Strategy

Future product validation should include focused provider configuration/schema
tests, deterministic mock-server positive and negative cases for every
provider, model-resolution snapshot and alias-drift tests, stdout/stderr and
redaction checks, and a bounded live-smoke lane only when the operator supplies
approved credentials. Missing credentials must produce a truthful skipped or
blocked result rather than failing deterministic adapter implementation.

The preparation validation lane only checks the packet shape and gate language.

## Review Focus

The single pre-PR review should verify provider identity separation, no secret
or credential leakage, deterministic model identity recording, bounded
discovery, direct-provider proof separation, scheduler authority boundaries,
and consistency with the WP-09 live merge gate.

## Non-Goals

- No AWS or Bedrock provider route.
- No provider credential access or provider API calls.
- No PR publication during preparation.
- No reduction of all OpenAI-compatible endpoints to one ambiguous provider.
- No autonomous review, merge, conductor, janitor, or closeout authority from
  benchmark/model scores.
