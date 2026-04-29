# Local Model PR Reviewer Tool

## Status

Initial working demo/tool surface for issue `#2603`.

This tool is designed for coding agents to request a bounded code review before
opening a PR. It is still a demo-grade surface, but it is operational: it builds
a review packet, runs a reviewer backend, writes normalized artifacts, and emits
a gate result.

## Command

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/v0905/local-model-pr-reviewer-fixture \
  --backend fixture \
  --visibility read-only-repo \
  --issue 2603 \
  --writer-session codex-writer \
  --reviewer-session fixture-reviewer
```

The command writes:

- `review_packet.json`
- `review_result.json`
- `gate_result.json`
- `run_summary.json`

The gate result is the file an agent should inspect before opening a PR.

## Fixture Review

Use fixture mode when you want a deterministic proof path or when no live model
is available:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/current \
  --backend fixture \
  --visibility read-only-repo \
  --issue <issue-number> \
  --writer-session <writer-session-id> \
  --reviewer-session fixture-reviewer
```

Expected success signal:

```text
CODE_REVIEW_GATE=allow_with_evidence
```

To prove the blocking path:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/blocked-fixture \
  --backend fixture \
  --fixture-case blocked \
  --visibility read-only-repo \
  --issue <issue-number> \
  --writer-session <writer-session-id> \
  --reviewer-session fixture-reviewer
```

Expected blocking signal:

```text
CODE_REVIEW_GATE=block_pr_open
```

## Ollama / Gemma4 Review

The Ollama backend calls the Ollama HTTP API directly. It does not require a new
Ollama-specific CLI wrapper.

By default, live Ollama review is gated off. Running without
`--allow-live-ollama` records an explicit skipped-review blocker:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/ollama-skip \
  --backend ollama \
  --visibility packet-only \
  --issue <issue-number> \
  --writer-session <writer-session-id> \
  --reviewer-session ollama-reviewer \
  --model gemma4:latest
```

Expected signal:

```text
CODE_REVIEW_GATE=block_pr_open
```

To run a live local model review:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/ollama-live \
  --backend ollama \
  --visibility packet-only \
  --issue <issue-number> \
  --writer-session <writer-session-id> \
  --reviewer-session ollama-reviewer \
  --model gemma4:latest \
  --timeout-secs 240 \
  --allow-live-ollama
```

The default Ollama endpoint is:

```text
http://127.0.0.1:11434/api/generate
```

Set a different local endpoint with either:

```bash
OLLAMA_HOST=http://127.0.0.1:11434
```

or:

```bash
--ollama-url http://127.0.0.1:11434
```

Use `--timeout-secs <n>` for larger remote models. The default timeout is 120
seconds; the accepted range is 1 to 900 seconds.

## Diff Scope

By default, the packet contains only the requested committed diff:

```text
--base <ref> --head <ref>
```

This keeps historical PR reviews and merge-commit reviews from accidentally
absorbing unrelated local edits in the reviewer worktree.

Use `--include-working-tree` only when the reviewer intentionally needs to
inspect uncommitted local edits before they are committed:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/current-uncommitted \
  --backend fixture \
  --base origin/main \
  --head HEAD \
  --include-working-tree \
  --writer-session <writer-session-id> \
  --reviewer-session fixture-reviewer
```

## Visibility Modes

`packet-only` gives the reviewer only the bounded review packet. This is safer
and works better for constrained local models, but it is weaker review evidence.

`read-only-repo` records that the reviewer may inspect the bounded repository
slice, but still has no write, push, merge, or arbitrary command authority.

## Gate Policy

The tool blocks PR opening when:

- the reviewer is the same session as the writer
- static diff checks fail
- a blocking `P0`, `P1`, or `P2` finding is present
- the reviewer disposition is `blocked`
- the reviewer disposition is `skipped`
- the reviewer disposition is `non_proving`

Only `blessed` review with no blocking reasons produces:

```text
CODE_REVIEW_GATE=allow_with_evidence
```

This is review evidence only. Human/operator merge authority remains outside
the tool.

## Agent Workflow

1. Finish the code change in the issue worktree.
2. Run the normal issue validation commands.
3. Run `adl tooling code-review`.
4. Inspect `gate_result.json`.
5. If `pr_open_allowed` is `false`, fix or route the finding before PR creation.
6. If `pr_open_allowed` is `true`, continue to `pr finish`.

## ACIP Alignment

This reviewer tool should now be understood as a concrete backend and artifact
producer for the ACIP review specialization rather than as a standalone review
transport.

The alignment is:

- `srp.md` remains the durable review-policy artifact
- `review_packet.json` is the bounded review evidence packet
- the ACIP review invocation contract carries invocation identity, routing,
  authority boundary, `srp_ref`, and packet refs
- `review_result.json` and `gate_result.json` are the primary declared review
  outputs
- `allow_with_evidence` is equivalent to the blessed review handoff that may
  proceed to `pr finish`
- `block_pr_open` is equivalent to a review handoff that must fix findings or
  route through an operator waiver path

The reviewer remains independent evidence, not merge authority. Human or
operator merge authority stays outside the reviewer result and outside the ACIP
review specialization.

## Current Limits

- The fixture backend proves artifact shape and gate behavior, not semantic code
  review quality.
- Ollama output must return parseable normalized JSON to become proving review
  evidence; otherwise the result is `non_proving`.
- Ollama findings must include concrete title, file, body, evidence,
  confidence, and suggested fix scope. Placeholder or empty findings are treated
  as `non_proving`, even when the model says `blessed`.
- Findings must be actionable risks or defects. Positive observations,
  confirmations, and `None`/`no action` fix scopes are treated as
  `non_proving` because they are not review findings.
- Packet compression is simple and may need improvement for large diffs.
- Operator-waiver handling is intentionally not implemented in this first demo.
