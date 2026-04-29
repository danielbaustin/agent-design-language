pub fn usage() -> &'static str {
    "Usage:
  adl <adl.yaml> [--print-plan] [--print-prompts] [--trace] [--run] [--resume <run.json>] [--steer <steering.json>] [--overlay <overlay.json>] [--out <dir>] [--quiet] [--open]
  adl resume <run_id> [--steer <steering.json>]
  adl agent tick --spec <agent-spec.yaml> [--recover-stale-lease] [--json]
  adl agent run --spec <agent-spec.yaml> --max-cycles <n> [--interval-secs <n>] [--no-sleep] [--recover-stale-lease] [--json]
  adl agent status --spec <agent-spec.yaml> [--json]
  adl agent inspect --spec <agent-spec.yaml> [--cycle <cycle-id>] [--json]
  adl agent stop --spec <agent-spec.yaml> --reason <text> [--json]
  adl artifact validate-control-path --root <dir>
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]
  adl demo <name> [--print-plan] [--trace] [--run] [--out <dir>] [--quiet] [--open] [--no-open]
  adl identity init --name <display-name> --birthday <rfc3339> --timezone <IANA> [--agent-id <id>] [--created-by <name>] [--force]
  adl identity show [--path <path>]
  adl identity now [--timezone <IANA>] [--path <identity-profile>] [--out <path>]
  adl identity foundation [--out <path>]
  adl identity adversarial-runtime [--out <path>]
  adl identity red-blue-architecture [--out <path>]
  adl identity adversarial-runner [--out <path>]
  adl identity exploit-replay [--out <path>]
  adl identity continuous-verification [--out <path>]
  adl identity operational-skills [--out <path>]
  adl identity skill-composition [--out <path>]
  adl identity delegation-refusal-coordination [--out <path>]
  adl identity provider-extension-packaging [--out <path>]
  adl identity demo-proof-entry-points [--out <path>]
  adl identity schema [--out <path>]
  adl identity continuity [--out <path>]
  adl identity retrieval [--out <path>]
  adl identity commitments [--out <path>]
  adl identity causality [--out <path>]
  adl identity cost [--out <path>]
  adl identity phi [--out <path>]
  adl identity instinct [--out <path>]
  adl identity instinct-runtime [--out <path>]
  adl runtime-v2 operator-controls [--out <path>]
  adl runtime-v2 security-boundary [--out <path>]
  adl runtime-v2 foundation-demo [--out <dir>]
  adl runtime-v2 integrated-csm-run-demo [--out <dir>]
  adl runtime-v2 observatory-flagship-demo [--out <dir>]
  adl runtime-v2 contract-market-demo [--out <dir>]
  adl runtime-v2 feature-proof-coverage [--out <path>]
  adl provider setup <family> [--out <dir>] [--force]
  adl pr create --title <title> [--slug <slug>] [--body <text> | --body-file <path>] [--labels <csv>] [--version <v>]
  adl pr init <issue> [--slug <slug>] [--title <title>] [--no-fetch-issue] [--version <v>]
  adl pr run <issue> [--prefix <prefix>] [--slug <slug>] [--title <title>] [--no-fetch-issue] [--version <v>] [--allow-open-pr-wave]
  adl pr run <adl.yaml> [--trace] [--signature-key <public_key_path>] [--allow-embedded-signature-key] [--allow-unsigned] [--runs-root <dir>] [--out <dir>]
  adl pr doctor <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue] [--mode full|ready|preflight] [--json]
  adl pr finish <issue> --title <title> [--body <text>] [--paths <csv>] [-f|--input <path>] [--output-card <path>] [--no-checks] [--no-close] [--ready] [--merge] [--no-open]
  adl pr closeout <issue> [--slug <slug>] [--version <v>] [--no-fetch-issue]
  adl godel run --run-id <id> --workflow-id <id> --failure-code <code> --failure-summary <text> [--evidence-ref <path> ...] [--runs-dir <dir>]
  adl godel inspect --run-id <id> [--runs-dir <dir>]
  adl godel evaluate --failure-code <code> --experiment-result <ok|blocked> --score-delta <int>
  adl godel affect-slice --initial-run-id <id> --adapted-run-id <id> --godel-run-id <id> [--aee-runs-dir <dir>] [--godel-runs-dir <dir>]
  adl keygen --out-dir <dir>
  adl sign <adl.yaml> --key <private_key_path> [--key-id <id>] [--out <signed_file>]
  adl instrument <graph|replay|replay-bundle|diff-plan|diff-trace|trace-schema|validate-trace-v1|provider-substrate|provider-substrate-schema> ...
  adl learn export --format <jsonl|bundle-v1|trace-bundle-v2> [--runs-dir <dir>] [--run-id <id> ...] --out <path>
  adl tooling <card-prompt|code-review|lint-prompt-spec|validate-structured-prompt|review-card-surface|review-runtime-surface|verify-review-output-provenance|verify-repo-review-contract> ...
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
  --signature-key <path>
                    Verify signed runtime workflows against this trusted public key
  --allow-embedded-signature-key
                    Allow embedded self-signed workflow keys (dev-only override)
  --allow-unsigned   Allow running unsigned workflows (dev-only override)
  -V, --version      Show the ADL CLI version
  -h, --help         Show this help

Examples:
  adl resume hitl-pause-seq
  adl resume hitl-pause-seq --steer /tmp/steer.json
  adl agent tick --spec .adl/long_lived_agents/example-agent.yaml
  adl agent run --spec .adl/long_lived_agents/example-agent.yaml --max-cycles 3 --no-sleep
  adl agent status --spec .adl/long_lived_agents/example-agent.yaml --json
  adl agent inspect --spec .adl/long_lived_agents/example-agent.yaml --json
  adl agent stop --spec .adl/long_lived_agents/example-agent.yaml --reason \"operator pause\"
  adl artifact validate-control-path --root /tmp/adl-v086-control-path-demo/demo-g-v086-control-path
  adl csm observatory --packet demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json --format bundle --out artifacts/v0901/csm-observatory
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
  adl demo demo-g-v086-control-path --run --trace --out ./out
  adl demo demo-h-v0891-adversarial-self-attack --run --trace --out .adl/reports/adversarial-demo --no-open
  adl demo demo-l-v0901-runtime-v2-foundation --run --trace --out artifacts/v0901 --no-open
  adl identity operational-skills --out .adl/state/operational_skills_substrate_v1.json
  adl identity skill-composition --out .adl/state/skill_composition_model_v1.json
  adl identity delegation-refusal-coordination --out .adl/state/delegation_refusal_coordination_v1.json
  adl identity provider-extension-packaging --out .adl/state/provider_extension_packaging_v1.json
  adl identity demo-proof-entry-points --out .adl/state/demo_proof_entry_points_v1.json
  adl runtime-v2 operator-controls --out .adl/state/runtime_v2_operator_control_report.v1.json
  adl runtime-v2 security-boundary --out .adl/state/runtime_v2_security_boundary_proof.v1.json
  adl runtime-v2 foundation-demo --out artifacts/v0901/demo-l-v0901-runtime-v2-foundation
  adl runtime-v2 integrated-csm-run-demo --out artifacts/v0902/demo-d10-integrated-csm-run
  adl runtime-v2 observatory-flagship-demo --out artifacts/v0903/demo-d12-observatory-flagship
  adl runtime-v2 contract-market-demo --out artifacts/v0904/demo-d12-contract-market
  adl runtime-v2 feature-proof-coverage --out artifacts/v0904/feature-proof-coverage.json
  adl provider setup chatgpt
  adl provider setup claude
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
  adl learn export --format trace-bundle-v2 --runs-dir .adl/trace-archive --out /tmp/archived-trace-bundle
  adl tooling lint-prompt-spec --issue 761
  adl tooling card-prompt --issue 761 --out /tmp/issue-761.prompt.md
  adl tooling code-review --out artifacts/reviews/pr-review --backend fixture --visibility packet-only
  adl tooling code-review --out artifacts/reviews/file-review --backend ollama --file adl/src/lib.rs --allow-live-ollama
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
