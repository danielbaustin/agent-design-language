pub fn usage() -> &'static str {
    "Usage:
  adl <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--quiet] [--open]
  adl resume <run_id> [--steer <steering.json>]
  adl artifact validate-control-path --root <dir>
  adl demo <name> [--print-plan] [--trace] [--run] [--out <dir>] [--quiet] [--open] [--no-open]
  adl identity init --name <display-name> --birthday <rfc3339> --timezone <IANA> [--agent-id <id>] [--created-by <name>] [--force]
  adl identity show [--path <path>]
  adl identity now [--timezone <IANA>] [--path <identity-profile>] [--out <path>]
  adl provider setup <family> [--out <dir>] [--force]
  adl pr create --title <title> [--slug <slug>] [--body <text> | --body-file <path>] [--labels <csv>] [--version <v>]
  adl pr init <issue> [--slug <slug>] [--title <title>] [--no-fetch-issue] [--version <v>]
  adl pr run <issue> [--prefix <prefix>] [--slug <slug>] [--title <title>] [--no-fetch-issue] [--version <v>] [--allow-open-pr-wave]
  adl pr run <adl.yaml> [--trace] [--allow-unsigned] [--runs-root <dir>] [--out <dir>]
  adl pr doctor <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--mode full|ready|preflight] [--json]
  adl pr finish <issue> --title <title> [--body <text>] [--paths <csv>] [-f|--input <path>] [--output-card <path>] [--no-checks] [--no-close] [--ready] [--merge] [--no-open]
  adl godel run --run-id <id> --workflow-id <id> --failure-code <code> --failure-summary <text> [--evidence-ref <path> ...] [--runs-dir <dir>]
  adl godel inspect --run-id <id> [--runs-dir <dir>]
  adl godel evaluate --failure-code <code> --experiment-result <ok|blocked> --score-delta <int>
  adl godel affect-slice --initial-run-id <id> --adapted-run-id <id> --godel-run-id <id> [--aee-runs-dir <dir>] [--godel-runs-dir <dir>]
  adl keygen --out-dir <dir>
  adl sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]
  adl instrument <graph|replay|replay-bundle|diff-plan|diff-trace|trace-schema|validate-trace-v1|provider-substrate|provider-substrate-schema> ...
  adl learn export --format <jsonl|bundle-v1|trace-bundle-v2> [--runs-dir <dir>] [--run-id <id> ...] --out <path>
  adl tooling <card-prompt|lint-prompt-spec|validate-structured-prompt|review-card-surface|review-runtime-surface|verify-review-output-provenance|verify-repo-review-contract> ...
  adl verify <adl.yaml> [--key <public_key_path>]

Options:
  --print-plan       Print the resolved plan
  --print-prompts    Print assembled prompts (--print-prompt also accepted)
  --trace            Emit trace events (dry-run unless --run)
  --run              Execute the workflow
  --resume <path>    Resume a previously paused run from run.json
  --steer <path>     Apply a checkpoint-bound steering patch while resuming
  --overlay <path>   Apply overlay v1 config changes (opt-in only)
  --out <dir>        Write step outputs to files under this directory (default: ./out)
  --quiet            Suppress per-step output bodies (--no-step-output also accepted)
  --open             Open the first written HTML artifact after a successful run
  --no-open          Disable artifact auto-open for demo runs
  --allow-unsigned   Allow running unsigned workflows (dev-only override)
  -V, --version      Show the ADL CLI version
  -h, --help         Show this help

Examples:
  adl resume hitl-pause-seq
  adl resume hitl-pause-seq --steer /tmp/steer.json
  adl artifact validate-control-path --root /tmp/adl-v086-control-path-demo/demo-g-v086-control-path
  ADL_OLLAMA_BIN=adl/tools/mock_ollama_v0_4.sh adl examples/v0-4-demo-fork-join.adl.yaml --run --trace --out ./out
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
  adl provider setup chatgpt
  adl provider setup anthropic --out ./.adl/provider-setup/anthropic
  adl godel run --run-id run-745-a --workflow-id wf-godel-loop --failure-code tool_failure --failure-summary \"step failed with deterministic parse error\" --evidence-ref runs/run-745-a/run_status.json
  adl godel inspect --run-id run-745-a --runs-dir .adl/runs
  adl godel evaluate --failure-code tool_failure --experiment-result ok --score-delta 1
  adl godel affect-slice --initial-run-id v0-3-aee-recovery-initial --adapted-run-id v0-3-aee-recovery-adapted --godel-run-id review-godel-affect-001 --aee-runs-dir .adl/runs --godel-runs-dir .adl/reports/demo-affect-godel-vertical-slice/runs
  adl keygen --out-dir ./.keys
  adl sign examples/v0-5-pattern-linear.adl.yaml --key ./.keys/ed25519-private.b64 --out /tmp/signed.adl.yaml
  adl instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format dot
  adl instrument graph examples/v0-5-pattern-fork-join.adl.yaml --format json
  adl instrument replay /tmp/trace.json
  adl instrument replay-bundle /tmp/trace_bundle_v2 run-123
  adl instrument diff-trace /tmp/trace-a.json /tmp/trace-b.json
  adl instrument trace-schema
  adl instrument validate-trace-v1 /tmp/trace-v1.json
  adl instrument provider-substrate examples/v0-6-provider-profile-delegation.adl.yaml
  adl instrument provider-substrate-schema
  adl learn export --format bundle-v1 --runs-dir .adl/runs --out /tmp/learning-bundle
  adl learn export --format trace-bundle-v2 --runs-dir .adl/runs --out /tmp/trace-bundle
  adl tooling lint-prompt-spec --issue 761
  adl tooling card-prompt --issue 761 --out /tmp/issue-761.prompt.md
  adl verify /tmp/signed.adl.yaml --key ./.keys/ed25519-public.b64"
}

pub fn resume_usage() -> &'static str {
    "Usage:
  adl resume <run_id> --adl <path> [--steer <steering.json>]

Semantics:
  - Loads .adl/runs/<run_id>/pause_state.json
  - Uses the explicit --adl path as the trusted document source for resume
  - Treats pause_state.json path fields as untrusted observational metadata only
  - Strict validation only: schema_version, status=paused, run_id, execution_plan_hash
  - Optional steering patch applies only at the resume boundary
  - Resumes only at step boundary (no checkpoint engine, no mid-step resume)"
}
