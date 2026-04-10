# v0-88-runtime-replace-live-demo-python-adapter-with-rust-native-provider-adapters

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1537
Run ID: issue-1537
Version: v0.88
Title: [v0.88][runtime] Replace live-demo Python adapter with Rust-native provider adapters
Branch: codex/1537-v0-88-runtime-replace-live-demo-python-adapter-with-rust-native-provider-adapters
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: openai
- Start Time: 2026-04-10T02:10:00Z
- End Time: 2026-04-10T02:31:00Z

## Summary

Implemented Rust-native OpenAI and Anthropic provider adapters for the ADL runtime, added a v0.88 live ChatGPT + Claude multi-agent demo path that no longer starts a Python vendor-translation adapter, and updated provider setup documentation so native OpenAI/Anthropic setup is distinguished from bounded HTTP compatibility profiles.

## Artifacts produced
- `adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml`
- `adl/tools/demo_v088_real_multi_agent_discussion.sh`
- `adl/tools/test_demo_v088_real_multi_agent_discussion.sh`
- `demos/v0.88/real_chatgpt_claude_multi_agent_discussion_demo.md`

## Actions taken
- Added `openai` and `anthropic` provider kinds to provider validation, provider substrate normalization, and provider construction.
- Implemented Rust-native request construction and response extraction for OpenAI Responses API and Anthropic Messages API.
- Added sanitized native-provider invocation records that capture provider family, model, HTTP status, and prompt/output character counts without recording key material or Authorization headers.
- Added loopback unit tests for native OpenAI and Anthropic request translation, representative response parsing, missing-auth handling, and provider substrate validation.
- Updated `adl provider setup openai` and `adl provider setup anthropic` to emit native provider snippets instead of generic bounded HTTP gateway snippets.
- Added a shell-only live demo wrapper for key loading, runtime execution, transcript assembly, manifest generation, and artifact leak checks.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch and will be published by the finish-created PR.
- Worktree-only paths remaining: none for required tracked implementation artifacts; generated live-demo proof artifacts remain local-only by design and are not committed.
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits in the bound issue worktree, to be published through the repo-native finish flow.
- Verification performed:
  - `git status --short` verified the tracked and untracked branch changes before finish.
  - `git diff --stat` verified the implementation and documentation change set.
  - `find .adl/v0.88/tasks/issue-1537__v0-88-runtime-replace-live-demo-python-adapter-with-rust-native-provider-adapters -type f` verified the local task bundle paths.
- Result: PASS

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- `worktree_only` means at least one required path still exists only outside the main repository path.
- `pr_open` should pair with truthful `Worktree-only paths remaining` content; list those paths when they still exist only in the worktree or say `none` only when the branch contents are fully represented in the main repository path.
- If `Integration state` is `pr_open`, verify the actual proof artifacts rather than only the containing directory or card path.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- If `Verification scope` and `Integration method used` differ in a non-obvious way, explain the difference in one line.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check` verified Rust formatting.
  - `cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings` verified warning-free Rust lint behavior.
  - `cargo test --manifest-path adl/Cargo.toml` verified the full Rust test suite.
  - `cargo test --manifest-path adl/Cargo.toml --test provider_tests` verified the provider test suite, including native OpenAI/Anthropic loopback request translation and sanitized missing-auth behavior.
  - `cargo test --manifest-path adl/Cargo.toml validate_accepts_native_openai_and_anthropic_provider_kinds --test adl_tests` verified ADL validation accepts native provider kinds.
  - `cargo test --manifest-path adl/Cargo.toml provider_substrate_accepts_native_openai_and_anthropic_kinds` verified provider substrate normalization for native kinds.
  - `cargo test --manifest-path adl/Cargo.toml provider_setup -- --nocapture` verified provider setup templates, including native OpenAI/Anthropic output.
  - `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned` verified the new demo resolves to five ordered runtime steps.
  - `env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY ADL_OPENAI_KEY_FILE=<missing> ADL_ANTHROPIC_KEY_FILE=<missing> bash adl/tools/test_demo_v088_real_multi_agent_discussion.sh` verified the live demo test skips cleanly when credentials are unavailable.
  - `bash adl/tools/demo_v088_real_multi_agent_discussion.sh <temporary local artifact directory>` exercised the real live-provider path with local operator-managed keys.
- Results: formatting and targeted Rust tests passed; the real live-provider run successfully completed the OpenAI turn and then stopped at the Anthropic API boundary with a sanitized non-retryable insufficient-credits response.
- Post-rebase retry result: after the operator attempted to add Anthropic API credits, the live demo was retried and still stopped at the Anthropic API boundary with the same sanitized insufficient-credits response; OpenAI turn execution still succeeded and the generated artifact secret scan still returned no matches.

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PARTIAL
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml --all --check"
      - "cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings"
      - "cargo test --manifest-path adl/Cargo.toml"
      - "cargo test --manifest-path adl/Cargo.toml --test provider_tests"
      - "cargo test --manifest-path adl/Cargo.toml validate_accepts_native_openai_and_anthropic_provider_kinds --test adl_tests"
      - "cargo test --manifest-path adl/Cargo.toml provider_substrate_accepts_native_openai_and_anthropic_kinds"
      - "cargo test --manifest-path adl/Cargo.toml provider_setup -- --nocapture"
      - "cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned"
      - "env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY ADL_OPENAI_KEY_FILE=<missing> ADL_ANTHROPIC_KEY_FILE=<missing> bash adl/tools/test_demo_v088_real_multi_agent_discussion.sh"
      - "bash adl/tools/demo_v088_real_multi_agent_discussion.sh <temporary local artifact directory>"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: targeted unit tests and demo plan resolution verified stable provider normalization and ordered five-step demo execution.
- Fixtures or scripts used: loopback TCP provider fixtures in `adl/tests/provider_tests.rs`, `adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml`, and `adl/tools/test_demo_v088_real_multi_agent_discussion.sh`.
- Replay verification (same inputs -> same artifacts/order): `--print-plan` for the v0.88 demo resolved the same five ordered steps with fixed providers and tasks; loopback provider tests produce stable request/response assertions.
- Ordering guarantees (sorting / tie-break rules used): the demo YAML declares a linear five-step sequence; provider substrate tests verify deterministic normalization for native provider kinds.
- Artifact stability notes: generated live outputs are model-dependent and are not claimed byte-stable; runtime structure, manifest shape, transcript contract, and invocation-record schema are stable for accepted runs.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: grep-based scan of generated local live-demo artifacts for Authorization headers, bearer header material, `x-api-key`, and key-shaped tokens returned no matches.
- Prompt / tool argument redaction verified: provider invocation records store provider family, model, status, timestamp, and character counts only; they do not store prompts, outputs, keys, or raw request headers.
- Absolute path leakage check: this output card uses repository-relative paths and replaces temporary local artifact roots with explicit placeholders.
- Sandbox / policy invariants preserved: secrets were loaded only from environment variables or local operator-managed key files and were never printed; CI-safe test behavior skips when credentials are absent.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): generated during the local live run under the temporary artifact root; not committed because live-provider artifacts are operator-local and credential-adjacent.
- Run artifact root: temporary local artifact directory used for operator validation; not a repository artifact.
- Replay command used for verification: `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned`.
- Replay result: PASS for deterministic plan replay; live execution PARTIAL because Anthropic returned insufficient account credits after OpenAI completed turn 1.

## Artifact Verification
- Primary proof surface: `adl/examples/v0-88-real-multi-agent-tea-discussion.adl.yaml`, `adl/tools/demo_v088_real_multi_agent_discussion.sh`, `adl/tools/test_demo_v088_real_multi_agent_discussion.sh`, and `demos/v0.88/real_chatgpt_claude_multi_agent_discussion_demo.md`.
- Required artifacts present: true; all required tracked implementation and documentation artifacts exist on the issue branch.
- Artifact schema/version checks: native provider invocation records use `adl.native_provider_invocations.v1`; transcript contract uses the existing `multi_agent_discussion_transcript.v1` shape.
- Hash/byte-stability checks: not applicable to live model text outputs; deterministic code/test artifacts are source-controlled and stable under git diff.
- Missing/optional artifacts and rationale: a complete five-turn live transcript was not produced because the local Anthropic account returned an insufficient-credits API error; this is an external account-state block, not a Rust adapter or orchestration failure.

## Decisions / Deviations

- Kept existing `chatgpt:` and `claude:` profile-family semantics on the bounded HTTP compatibility path to avoid widening this v0.88 follow-up into a profile migration.
- Added native `openai` and `anthropic` provider kinds and wired the new v0.88 live demo to those kinds directly.
- Kept Python in the existing transcript validator only; no new Python provider adapter or vendor-translation bridge was added.

## Follow-ups / Deferred work

- Consider a later profile migration issue if `chatgpt:` and `claude:` should expand directly to native vendor provider kinds.
- Re-run the full live demo when the local Anthropic account has sufficient API credits.
- The live demo was retried after an attempted Anthropic credit update; the account still reported insufficient API credits.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
