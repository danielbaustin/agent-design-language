pub fn usage() -> &'static str {
    "Usage:
  adl <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--resume <run.json>] [--overlay <overlay.json>] [--out <dir>] [--quiet] [--open]
  adl resume <run_id>
  adl demo <name> [--print-plan] [--trace] [--run] [--out <dir>] [--quiet] [--open] [--no-open]
  adl godel run --run-id <id> --workflow-id <id> --failure-code <code> --failure-summary <text> [--evidence-ref <path> ...] [--runs-dir <dir>]
  adl godel inspect --run-id <id> [--runs-dir <dir>]
  adl godel evaluate --failure-code <code> --experiment-result <ok|blocked> --score-delta <int>
  adl keygen --out-dir <dir>
  adl sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]
  adl instrument <graph|replay|replay-bundle|diff-plan|diff-trace> ...
  adl learn export --format <jsonl|bundle-v1|trace-bundle-v2> [--runs-dir <dir>] [--run-id <id> ...] --out <path>
  adl verify <adl.yaml> [--key <public_key_path>]

Options:
  --print-plan       Print the resolved plan
  --print-prompts    Print assembled prompts (--print-prompt also accepted)
  --trace            Emit trace events (dry-run unless --run)
  --run              Execute the workflow
  --resume <path>    Resume a previously paused run from run.json
  --overlay <path>   Apply overlay v1 config changes (opt-in only)
  --out <dir>        Write step outputs to files under this directory (default: ./out)
  --quiet            Suppress per-step output bodies (--no-step-output also accepted)
  --open             Open the first written HTML artifact after a successful run
  --no-open          Disable artifact auto-open for demo runs
  --allow-unsigned   Allow running unsigned workflows (dev-only override)
  -h, --help         Show this help

Examples:
  adl resume hitl-pause-seq
  ADL_OLLAMA_BIN=swarm/tools/mock_ollama_v0_4.sh adl examples/v0-4-demo-fork-join-swarm.adl.yaml --run --trace --out ./out
  adl examples/v0-3-concurrency-fork-join.adl.yaml --print-plan
  adl examples/v0-3-on-error-retry.adl.yaml --print-plan
  adl examples/v0-3-remote-http-provider.adl.yaml --print-plan
  adl examples/adl-0.1.yaml --print-plan   # legacy regression example
  adl examples/v0-2-coordinator-agents-sdk.adl.yaml
  adl demo demo-a-say-mcp --run --trace --open
  adl demo demo-b-one-command --run --out ./out
  adl demo demo-c-godel-runtime --run --out ./out
  adl demo demo-d-godel-obsmem-loop --run --trace --out ./out
  adl demo demo-e-multi-agent-card-pipeline --run --trace --out ./out
  adl demo demo-f-obsmem-retrieval --run --trace --out ./out
  adl godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary \"step failed with deterministic parse error\" --evidence-ref runs/run-745-a/run_status.json
  adl godel inspect --run-id run-745-a --runs-dir .adl/runs
  adl godel evaluate --failure-code tool_failure --experiment-result ok --score-delta 1
  adl keygen --out-dir ./.keys
  adl sign examples/v0-5-pattern-linear.adl.yaml --key ./.keys/ed25519-private.b64 --out /tmp/signed.adl.yaml
  adl instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format dot
  adl instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format json
  adl instrument replay /tmp/trace.json
  adl instrument replay-bundle /tmp/trace_bundle_v2 run-123
  adl instrument diff-trace /tmp/trace-a.json /tmp/trace-b.json
  adl learn export --format bundle-v1 --runs-dir .adl/runs --out /tmp/learning-bundle
  adl learn export --format trace-bundle-v2 --runs-dir .adl/runs --out /tmp/trace-bundle
  adl verify /tmp/signed.adl.yaml --key ./.keys/ed25519-public.b64"
}

pub fn resume_usage() -> &'static str {
    "Usage:
  adl resume <run_id>

Semantics:
  - Loads .adl/runs/<run_id>/pause_state.json
  - Strict validation only: schema_version, status=paused, run_id, execution_plan_hash
  - Resumes only at step boundary (no checkpoint engine, no mid-step resume)"
}
