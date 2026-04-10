# v0-87-1-demo-add-real-chatgpt-claude-multi-agent-provider-demo

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1533
Run ID: issue-1533
Version: v0.87.1
Title: [v0.87.1][demo] Add real ChatGPT + Claude multi-agent provider demo
Branch: codex/1533-v0-87-1-demo-add-real-chatgpt-claude-multi-agent-provider-demo
Status: DONE

Execution:
- Actor: codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-10T01:44:00Z
- End Time: 2026-04-10T01:58:46Z

## Summary

Added a live-provider companion demo for the Claude + ChatGPT tea-discussion proof surface. The new path keeps ADL's current local HTTP completion contract, but the adapter behind that contract calls real OpenAI and Anthropic APIs using operator-managed credentials from environment variables or `$HOME/keys`.

## Artifacts produced
- `adl/tools/real_multi_agent_provider_adapter.py`
- `adl/tools/demo_v0871_real_multi_agent_discussion.sh`
- `adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
- `adl/examples/v0-87-1-real-multi-agent-tea-discussion.adl.yaml`
- `demos/v0.87.1/real_chatgpt_claude_multi_agent_discussion_demo.md`
- updates to `adl/tools/README.md`, `docs/tooling/PROVIDER_SETUP.md`, and `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

## Actions taken
- Created a live local adapter that accepts ADL `{"prompt": "..."}` calls and translates them to OpenAI Responses API and Anthropic Messages API calls.
- Added a real-provider ADL example with five sequential turns, two named provider families, saved-state handoff, and conversation metadata.
- Added a live demo wrapper that loads `OPENAI_API_KEY` and `ANTHROPIC_API_KEY` from the environment or from `$HOME/keys/openai.key` and `$HOME/keys/claude.key` without printing secret values.
- Added a validation wrapper that skips clearly when keys are absent and verifies transcript, runtime, trace, and provider-invocation proof surfaces when live provider calls succeed.
- Updated docs to distinguish the deterministic CI-safe demo from the live operator demo.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths are updated on the issue branch; PR publication is pending.
- Worktree-only paths remaining: none once the issue branch is pushed; current edits are tracked in the issue worktree before PR publication.
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: issue branch worktree via `bash adl/tools/pr.sh run 1533 --version v0.87.1 --allow-open-pr-wave`
- Verification performed:
  - `python3 -m py_compile adl/tools/real_multi_agent_provider_adapter.py`
    - verified adapter syntax
  - `bash -n adl/tools/demo_v0871_real_multi_agent_discussion.sh adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
    - verified shell syntax for live demo wrappers
  - `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-87-1-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned`
    - verified the real-provider ADL example resolves to five ordered steps
  - `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
    - verified the deterministic D13 demo remains intact
  - `env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY ADL_OPENAI_KEY_FILE=<missing-openai-key-file> ADL_ANTHROPIC_KEY_FILE=<missing-claude-key-file> bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
    - verified the no-key live validation path skips clearly
  - `bash adl/tools/demo_v0871_real_multi_agent_discussion.sh <temporary-live-debug-root>`
    - attempted the live run; OpenAI returned HTTP 200 and produced turn 1, Anthropic was contacted but returned a provider billing/credit error
  - key-value scan over `<temporary-live-debug-root>` and the worktree
    - verified no local key values were found in generated artifacts or tracked files
- Result: FAIL
- Result note: live validation is partial because the Anthropic account returned a billing/credit error; implementation and non-secret validation surfaces are present.

## Validation
- `python3 -m py_compile adl/tools/real_multi_agent_provider_adapter.py`
  - passed; validates adapter syntax
- `bash -n adl/tools/demo_v0871_real_multi_agent_discussion.sh adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
  - passed; validates shell syntax
- `cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-87-1-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned`
  - passed; validated five-step plan resolution with OpenAI and Anthropic provider ids
- `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
  - passed; validated existing deterministic D13 demo
- `env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY ADL_OPENAI_KEY_FILE=<missing-openai-key-file> ADL_ANTHROPIC_KEY_FILE=<missing-claude-key-file> bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh`
  - passed; validated missing-key skip behavior
- `bash adl/tools/demo_v0871_real_multi_agent_discussion.sh <temporary-live-debug-root>`
  - partial; OpenAI call succeeded with HTTP 200, Anthropic call reached the API and returned HTTP 400 due account credit/billing state
- Results:
  - static and deterministic validation passed
  - live OpenAI invocation passed
  - live Anthropic invocation was blocked by external account billing/credit state

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PARTIAL
    checks_run:
      - python3 -m py_compile adl/tools/real_multi_agent_provider_adapter.py
      - bash -n adl/tools/demo_v0871_real_multi_agent_discussion.sh adl/tools/test_demo_v0871_real_multi_agent_discussion.sh
      - cargo run --quiet --manifest-path adl/Cargo.toml --bin adl -- adl/examples/v0-87-1-real-multi-agent-tea-discussion.adl.yaml --print-plan --allow-unsigned
      - bash adl/tools/test_demo_v0871_multi_agent_discussion.sh
      - env -u OPENAI_API_KEY -u ANTHROPIC_API_KEY ADL_OPENAI_KEY_FILE=<missing-openai-key-file> ADL_ANTHROPIC_KEY_FILE=<missing-claude-key-file> bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh
      - bash adl/tools/demo_v0871_real_multi_agent_discussion.sh <temporary-live-debug-root>
  determinism:
    status: PARTIAL
    replay_verified: false
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
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: existing deterministic D13 demo test and real-provider print-plan validation.
- Fixtures or scripts used: `adl/tools/test_demo_v0871_multi_agent_discussion.sh` and the real-provider ADL example print-plan.
- Replay verification (same inputs -> same artifacts/order): the live model text is intentionally non-deterministic, so byte-for-byte replay is not claimed for live outputs.
- Ordering guarantees (sorting / tie-break rules used): workflow ordering remains sequential and explicit; provider invocation order follows the five-step ADL workflow.
- Artifact stability notes: transcript contract shape, provider invocation metadata schema, run id, output paths, and conversation turn ordering are stable even though live model text varies.

## Security / Privacy Checks
- Secret leakage scan performed: read local key values in-process and scanned generated debug artifacts plus tracked worktree files for those exact values; no matches were found.
- Prompt / tool argument redaction verified: provider invocation metadata records prompt/output character counts only, not prompt bodies or API keys.
- Absolute path leakage check: tracked files use repository-relative or documented operator-local `$HOME/keys` references; generated debug artifacts under an external temporary debug root were not added to git.
- Sandbox / policy invariants preserved: yes; live provider calls are explicit operator-demo behavior and standard CI can use the no-key skip path.

## Replay Artifacts
- Trace bundle path(s): `<temporary-live-debug-root>/runtime/runs/v0-87-1-real-multi-agent-tea-discussion/logs/trace_v1.json` for the attempted live run; not checked in.
- Run artifact root: `<temporary-live-debug-root>/runtime/runs/v0-87-1-real-multi-agent-tea-discussion` for the attempted live run; not checked in.
- Replay command used for verification: `bash adl/tools/demo_v0871_real_multi_agent_discussion.sh <temporary-live-debug-root>`
- Replay result: PARTIAL; OpenAI produced turn 1 and Anthropic returned a billing/credit error before turn 2 completed.

## Artifact Verification
- Primary proof surface: `provider_invocations.json` generated by the live demo plus `transcript.md` when all live provider calls succeed.
- Required artifacts present: yes; scripts, adapter, example, demo doc, and docs updates are present in the worktree.
- Artifact schema/version checks: `provider_invocations.json` uses `adl.live_provider_invocations.v1`; `transcript_contract.json` uses the existing `multi_agent_discussion_transcript.v1`.
- Hash/byte-stability checks: not applicable to live model text; stable schema and ordering are the intended verification target.
- Missing/optional artifacts and rationale: a complete five-turn live transcript was not produced locally because Anthropic returned an account billing/credit error.

## Decisions / Deviations

- Used a local live-provider adapter because ADL's current HTTP provider intentionally expects the bounded `{"prompt": "..."} -> {"output": "..."}` contract rather than raw vendor-native APIs.
- Kept the deterministic D13 demo unchanged and added a live companion rather than making CI depend on external provider credentials.
- Used `--allow-open-pr-wave` because issue 1533 was executed while other milestone PRs were open.
- Recorded the Anthropic billing/credit failure as an external validation blocker rather than adding a fake fallback to the live demo.

## Follow-ups / Deferred work

- Re-run `bash adl/tools/test_demo_v0871_real_multi_agent_discussion.sh` after the Anthropic account has usable API credits to produce the full five-turn live transcript.
