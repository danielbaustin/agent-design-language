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

Use fixture mode when you want a deterministic artifact-shape proof path or
when no live model is available. Fixture mode is intentionally non-proving: it
must not bless PR publication because it does not perform semantic code review.

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/current \
  --backend fixture \
  --visibility read-only-repo \
  --issue <issue-number> \
  --writer-session <writer-session-id> \
  --reviewer-session fixture-reviewer
```

Expected fixture signal:

```text
CODE_REVIEW_GATE=block_pr_open
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

Reasoning-style Ollama models may return their final JSON in a provider
`thinking` field with an empty `response` field. The tool treats `thinking` as a
fallback only when `response` is empty, and the prompt includes `/no_think` plus
an explicit final-JSON instruction so Qwen-style models can participate in the
same gate.

Some tool-trained local models may try to emit `action` / `action_input`
tool-call JSON instead of a review result. The prompt explicitly forbids tool
calls, web search requests, repository commands, and tool-call-shaped top-level
JSON; such output remains `non_proving` because it does not satisfy the review
schema.

Use `--max-diff-bytes <n>` to control per-file diff and context excerpt size.
The accepted range is 256 to 100000 bytes so a malformed or accidental command
cannot turn review packet generation into an unbounded file read.

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

When `--include-working-tree` is supplied, the packet includes the committed
`--base...--head` diff, staged working-tree diff, and unstaged working-tree diff
for each selected file. This prevents dirty local edits or staged-but-uncommitted
edits from hiding committed PR hunks.

Use `--file <repo-relative-path>` to slice a review to one or more specific
changed files when a model cannot review the whole PR packet well in one pass:

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out artifacts/reviews/code-review-rs \
  --backend ollama \
  --base origin/main \
  --head HEAD \
  --include-working-tree \
  --file adl/src/cli/tooling_cmd/code_review.rs \
  --writer-session <writer-session-id> \
  --reviewer-session ollama-reviewer \
  --allow-live-ollama
```

`--file` values must be repo-relative, forward-slash paths with portable
characters only: ASCII letters, digits, `/`, `.`, `_`, and `-`. They must
already be present in the changed-file set for the selected `--base`/`--head`
plus the working tree when `--include-working-tree` is used. Absolute paths,
parent-directory traversal, backslash paths, spaces, refspec-like punctuation,
and unrelated repository files are rejected before packet generation.

When slicing a model review, include enough coupled files for the reviewer to
understand semantics. For example, a test-only packet can produce confident but
wrong findings when the implementation file that defines the tested contract is
not included.

The tool also rejects common secret-bearing path patterns before packet
generation, including `.env` files, `.ssh/` paths, and private-key/certificate
extensions such as `.pem`, `.key`, `.p12`, and `.pfx`. If a changed file cannot
be read for context, the packet records a `read_error` entry instead of silently
pretending the context was complete. Explicit `--file` context failures remain
hard errors.

## Visibility Modes

`packet-only` gives the reviewer only the bounded review packet. This is safer
and works better for constrained local models, but it is weaker review evidence.

`read-only-repo` records that the reviewer may inspect the bounded repository
slice in the packet, but the local Ollama backend still receives only the
serialized review packet. `review_result.json` therefore records no live
repository read tools, no writes, no pushes, no merges, and no arbitrary command
authority.

## Gate Policy

The tool blocks PR opening when:

- the reviewer is the same session as the writer
- the reviewer is the same model as the writer
- static diff checks fail
- a blocking `P0`, `P1`, or `P2` finding is present
- the reviewer disposition is `blocked`
- the reviewer disposition is `skipped`
- the reviewer disposition is `non_proving`
- the live-model packet contains absolute host paths or secret-like values

Only `blessed` review with no blocking reasons produces:

```text
CODE_REVIEW_GATE=allow_with_evidence
```

This is review evidence only. Human/operator merge authority remains outside
the tool.

## Reviewer Model Policy

Until v0.91, the default external reviewer lane is ChatGPT/GPT-5.5 using the
same bounded packet and findings-first review contract. The local Ollama backend
remains useful as an experimental and secondary review lane, but it is not the
default authority for PR-open evidence.

Observed reviewer behavior in the #2603 benchmark:

- Codex agent: strongest implementation-level review when pointed at code.
- ChatGPT/GPT-5.5 API: cleanest default external review output and strongest
  practical fit for near-term reviewer-agent use.
- Claude Sonnet 4.6 API: strong evidence-boundary critic; useful as a
  conservative second opinion.
- Qwen 3.6 27B: usable after `/no_think` plus `thinking`-field fallback, but
  sensitive to packet scope and latency.
- Gemma4 31B: promising local/remote fallback with strict prompting and bounded
  complete packets.
- DeepSeek R1 and GPT-OSS: not proving under the current schema/prompt; keep as
  research candidates, not gate authorities.

## Agent Workflow

1. Finish the code change in the issue worktree.
2. Run the normal issue validation commands.
3. Run the default ChatGPT/GPT-5.5 reviewer lane when available.
4. Optionally run `adl tooling code-review` with Ollama as a secondary model
   lane or local fallback.
5. Inspect `gate_result.json` and the external reviewer artifact.
6. If any proving reviewer blocks, fix or route the finding before PR creation.
7. If the review gate is clear, continue to `pr finish`.

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

## Coding-Agent Handoff

Comms-06 makes the upstream coding-agent side explicit rather than leaving it
implicit in issue-worktree practice.

When the reviewer tool is called after coding-agent work, the expected handoff
is:

- a coding invocation or outcome surface that identifies the writer session and
  model
- a bounded patch, patch manifest, or structured proposal artifact
- validation evidence from the coding lane
- a review-handoff packet that points at the coding outputs without granting
  approval

The reviewer tool should therefore be treated as the next governed step after a
coding-agent runner, not as something the coding lane can collapse into or
self-satisfy.

## Current Limits

- The fixture backend proves artifact shape and gate behavior, not semantic code
  review quality. It is always non-proving unless it is explicitly exercising a
  blocked fixture case.
- Ollama output must return parseable normalized JSON to become proving review
  evidence; otherwise the result is `non_proving`.
- Live Ollama prompts redact absolute-host-path markers before HTTP dispatch,
  and the gate still blocks PR opening when the source packet contained absolute
  host paths so the operator can inspect the redaction boundary. Live invocation
  is suppressed before any HTTP request if the packet contains secret-like
  values.
- Ollama findings must include concrete title, file, body, evidence,
  confidence, and suggested fix scope. Placeholder or empty findings are treated
  as `non_proving`, even when the model says `blessed`.
- A `blessed` review must include concrete residual-review rationale and cannot
  bless truncated hunks, file-limit truncation, truncated file context, or
  context read errors.
- Blocking `P0`/`P1` security findings must include concrete bypass evidence.
  Speculative security language without a `bypass:` evidence item is treated as
  `non_proving` rather than as merge-blocking proof.
- When a model supplies a CLI bypass for `--file`, `--base`, or `--head`, the
  tool checks that bypass against the same validators used by packet generation.
  A bypass that the current CLI rejects is also treated as `non_proving`.
- Findings must be actionable risks or defects. Positive observations,
  confirmations, and `None`/`no action` fix scopes are treated as
  `non_proving` because they are not review findings.
- Packet compression is bounded: up to 40 diff files and 24 file-context
  excerpts are included in one packet. The packet records these limits and marks
  file-limit truncation so a model cannot silently bless incomplete context.
- Operator-waiver handling is intentionally not implemented in this first demo.
