# Issue #4681 Pre-PR Review Gate Blocker

## Status

PR publication remains blocked. The review packet hygiene issue was repaired and
rerun with repo-native tooling, but the required proving independent review did
not complete because the live Ollama backend returned `skipped`.

## Packet Hygiene Evidence

- Packet hygiene rerun:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_packet_hygiene/gate_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_packet_hygiene/review_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_packet_hygiene/run_summary.json`
- Local full packet path:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_packet_hygiene/review_packet.json`
- Review packet redaction flags:
  - `absolute_host_paths_present: false`
  - `secret_like_values_present: false`
- Gate result: `block_pr_open`
- Gate reason: `review disposition is non_proving; operator waiver is not implemented`
- Disposition explanation: the fixture backend proves packet shape only and
  cannot satisfy the independent pre-PR review requirement.

## Live Review Attempt

- Initial live review output:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama/gate_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama/review_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama/run_summary.json`
- Local full packet path:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama/review_packet.json`
- Review packet redaction flags:
  - `absolute_host_paths_present: false`
  - `secret_like_values_present: false`
- Backend: `ollama`
- Model: `hf.co/mitkox/FastContext-1.0-4B-RL-Q4_K_M-GGUF:latest`
- Reviewer session: `ollama-reviewer-4681-20260704`
- Writer session: `019f2b2d-4681-continuation`
- Same session as writer: `false`
- Gate result: `block_pr_open`
- Gate reason: `review disposition is skipped; operator waiver is not implemented`
- Residual risk: `Ollama unavailable: error sending request for url (http://127.0.0.1:11434/api/generate)`

## Live Review Retries

- Service probe:
  - `cargo run --manifest-path adl/Cargo.toml -- process status --port 11434 --json`
  - Result: loopback port `11434` was bound.
  - `curl -sS --max-time 30 http://127.0.0.1:11434/api/generate -H 'Content-Type: application/json' -d '{"model":"phi4-mini:latest","prompt":"Return exactly OK.","stream":false}'`
  - Result: Ollama returned `OK` for the small direct prompt.
- FastContext retry:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_retry/gate_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_retry/review_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_retry/run_summary.json`
  - Gate result: `block_pr_open`
  - Gate reason: `review disposition is skipped; operator waiver is not implemented`
  - Residual risk: `Ollama unavailable: error sending request for url (http://127.0.0.1:11434/api/generate)`
- Full `phi4-mini:latest` retry:
  - Command used the same changed-file list with `--timeout-secs 600`.
  - Result: interrupted after overrunning the configured timeout and producing no gate files.
- Compact `phi4-mini:latest` retry:
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_phi4_compact/gate_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_phi4_compact/review_result.json`
  - `docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama_phi4_compact/run_summary.json`
  - Command used `--max-diff-bytes 12000` and `--timeout-secs 240`.
  - Gate result: `block_pr_open`
  - Gate reason: `review disposition is skipped; operator waiver is not implemented`
  - Residual risk: `Ollama unavailable: error sending request for url (http://127.0.0.1:11434/api/generate)`

## Retention Boundary

The committed durable evidence is this blocker packet plus the small
`gate_result.json`, `review_result.json`, and `run_summary.json` files for each
review attempt. The full `review_packet.json` files remain as local worktree
evidence because they are large raw diff packets; the committed summaries record
their exact local paths and the redaction status reported by the review tool.

## Commands

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out docs/milestones/v0.91.7/review/runtime/code_review_4681_packet_hygiene \
  --backend fixture \
  --visibility packet-only \
  --base origin/main \
  --head HEAD \
  --issue 4681 \
  --writer-session 019f2b2d-4681-continuation \
  --reviewer-session fixture-reviewer-4681-hygiene \
  --fixture-case clean \
  --file adl/src/runtime_v2/minimal_integrated_runtime_path.rs \
  --file adl/src/runtime_v2/tests/minimal_integrated_runtime_path.rs \
  --file docs/milestones/v0.91.7/review/runtime/MINIMAL_INTEGRATED_RUNTIME_PATH_4681.md \
  --file adl/src/cli/runtime_v2_cmd/commands.rs \
  --file adl/src/cli/runtime_v2_cmd/helpers.rs \
  --file adl/src/cli/runtime_v2_cmd/tests.rs \
  --file adl/src/cli/usage.rs \
  --file adl/src/runtime_v2/contracts.rs \
  --file adl/src/runtime_v2/mod.rs \
  --file adl/src/runtime_v2/tests.rs \
  --file adl/src/cli/tooling_cmd/code_review.rs \
  --file adl/src/cli/tooling_cmd/code_review_build.rs \
  --file adl/src/cli/tooling_cmd/tests/code_review.rs
```

```bash
cargo run --manifest-path adl/Cargo.toml -- tooling code-review \
  --out docs/milestones/v0.91.7/review/runtime/code_review_4681_ollama \
  --backend ollama \
  --model hf.co/mitkox/FastContext-1.0-4B-RL-Q4_K_M-GGUF:latest \
  --visibility packet-only \
  --base origin/main \
  --head HEAD \
  --issue 4681 \
  --writer-session 019f2b2d-4681-continuation \
  --reviewer-session ollama-reviewer-4681-20260704 \
  --allow-live-ollama \
  --timeout-secs 300 \
  --fixture-case clean \
  --file adl/src/runtime_v2/minimal_integrated_runtime_path.rs \
  --file adl/src/runtime_v2/tests/minimal_integrated_runtime_path.rs \
  --file docs/milestones/v0.91.7/review/runtime/MINIMAL_INTEGRATED_RUNTIME_PATH_4681.md \
  --file adl/src/cli/runtime_v2_cmd/commands.rs \
  --file adl/src/cli/runtime_v2_cmd/helpers.rs \
  --file adl/src/cli/runtime_v2_cmd/tests.rs \
  --file adl/src/cli/usage.rs \
  --file adl/src/runtime_v2/contracts.rs \
  --file adl/src/runtime_v2/mod.rs \
  --file adl/src/runtime_v2/tests.rs \
  --file adl/src/cli/tooling_cmd/code_review.rs \
  --file adl/src/cli/tooling_cmd/code_review_build.rs \
  --file adl/src/cli/tooling_cmd/tests/code_review.rs
```

## Publication Decision

Do not run `adl/tools/pr.sh finish` for #4681 until a proving independent
review/subagent result passes the repo-native review gate or the operator
provides an explicit review-waiver path supported by lifecycle tooling.
