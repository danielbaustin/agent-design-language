# v0-87-1-runtime-add-conversation-native-multi-agent-turn-primitives-to-the-adl-runtime

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1501
Run ID: issue-1501
Version: v0.87.1
Title: [v0.87.1][runtime] Add conversation-native multi-agent turn primitives to the ADL runtime
Branch: codex/1501-v0-87-1-runtime-add-conversation-native-multi-agent-turn-primitives-to-the-adl-runtime
Status: DONE

Execution:
- Actor: codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-10T00:57:01Z
- End Time: 2026-04-10T01:03:29Z

## Summary

Implemented bounded conversation turn metadata for ADL workflow steps. The runtime now accepts, validates, resolves, and records first-class turn metadata without changing workflow ordering or saved-state semantics.

## Artifacts produced
- updated ADL schema/runtime support for optional `conversation` step metadata
- updated `steps.json` runtime artifact output with optional conversation metadata
- updated multi-agent tea discussion example to declare five explicit conversation turns
- updated demo documentation and demo validation to check runtime-visible turn metadata

## Actions taken
- added `ConversationTurnSpec` with turn id, speaker, sequence, thread id, and response linkage
- carried conversation metadata through resolution into runtime step artifacts
- added validation for empty turn ids, empty speakers, zero sequences, self-referential responses, and duplicate turn ids
- added focused tests for resolution, validation, and runtime artifact emission
- updated the D13 demo and docs to describe bounded turn metadata rather than a full conversation platform
- added janitor follow-up tests for each conversation validation rejection branch after GitHub coverage reported `adl/src/adl/validation.rs` below the per-file floor

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked repository paths from this issue are present on main via merged PR #1523.
- Worktree-only paths remaining: none for required tracked artifacts; issue branch changes have merged to main via PR #1523.
- Integration state: merged
- Verification scope: worktree
- Integration method used: issue branch/worktree changes were published and merged via PR #1523.
- Verification performed:
  - `cargo fmt --manifest-path adl/Cargo.toml --all`
    - verified Rust formatting for the modified crate files
  - `cargo test --manifest-path adl/Cargo.toml conversation_turn -- --nocapture`
    - verified conversation turn metadata parsing, validation, and resolution
  - `cargo test --manifest-path adl/Cargo.toml run_writes_run_state_artifacts -- --nocapture`
    - verified runtime `steps.json` records conversation metadata for executed steps
  - `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
    - verified the multi-agent demo still runs and exposes conversation metadata in runtime artifacts
  - `cargo test --manifest-path adl/Cargo.toml conversation --test adl_tests -- --nocapture`
    - verified every conversation validation rejection branch added for the coverage repair
- Result: PASS

## Validation
- `cargo fmt --manifest-path adl/Cargo.toml --all`
  - verified formatting for Rust changes
- `cargo test --manifest-path adl/Cargo.toml conversation_turn -- --nocapture`
  - verified new focused schema/resolution validation behavior
- `cargo test --manifest-path adl/Cargo.toml run_writes_run_state_artifacts -- --nocapture`
  - verified runtime artifact emission for turn metadata
- `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
  - verified the D13 demo still passes with runtime-visible conversation metadata
- `cargo test --manifest-path adl/Cargo.toml conversation --test adl_tests -- --nocapture`
  - verified the post-PR janitor coverage repair tests for conversation metadata validation
- Results:
  - focused Rust tests pass
  - focused demo test passes
  - post-PR janitor validation tests pass
  - no generated runtime artifacts were checked into the repository

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - cargo fmt --manifest-path adl/Cargo.toml --all
      - cargo test --manifest-path adl/Cargo.toml conversation_turn -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml run_writes_run_state_artifacts -- --nocapture
      - bash adl/tools/test_demo_v0871_multi_agent_discussion.sh
      - cargo test --manifest-path adl/Cargo.toml conversation --test adl_tests -- --nocapture
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
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: focused runtime artifact test and multi-agent discussion demo test
- Fixtures or scripts used: `run_writes_run_state_artifacts` fixture and `adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Replay verification (same inputs -> same artifacts/order): the demo script reruns the bounded workflow from scratch and verifies the five-turn artifact shape
- Ordering guarantees (sorting / tie-break rules used): workflow execution order remains governed by existing sequential workflow and `@state:` dependencies; conversation metadata does not alter scheduler ordering
- Artifact stability notes: accepted metadata is copied directly into `steps.json` from the resolved ADL document, so identical accepted ADL input produces identical turn metadata fields

## Security / Privacy Checks
- Secret leakage scan performed: manual review of changed files; no secrets or credentials were introduced
- Prompt / tool argument redaction verified: validation commands use repository-relative paths and no sensitive prompt/tool payloads
- Absolute path leakage check: passed for final tracked artifacts; the SOR records repository-relative paths only
- Sandbox / policy invariants preserved: yes; the change adds metadata and artifact projection only, with no new filesystem, network, or provider authority

## Replay Artifacts
- Trace bundle path(s): not checked in; generated only in temporary demo output
- Run artifact root: temporary output from `adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Replay command used for verification: `bash adl/tools/test_demo_v0871_multi_agent_discussion.sh`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: runtime `steps.json` generated by the focused artifact test and the D13 demo script
- Required artifacts present: yes; changed source, tests, docs, and demo script are present in the worktree
- Artifact schema/version checks: serde `deny_unknown_fields` remains active and focused validation rejects malformed conversation metadata
- Hash/byte-stability checks: not separately hashed; deterministic field projection and existing runtime artifact tests cover the metadata path
- Missing/optional artifacts and rationale: no persistent runtime artifacts are committed because generated demo output remains transient

## Decisions / Deviations

- Used `--allow-open-pr-wave` because issue `1501` was intentionally executed while milestone PRs `#1522` and `#1517` were open.
- GitHub coverage initially failed because `adl/src/adl/validation.rs` fell below the per-file coverage floor; janitor repair added focused tests without changing production behavior.
- Kept conversation metadata descriptive only; this issue does not infer execution dependencies from `responds_to`.
- Avoided transcript artifact contract work because issue `1502` owns that follow-on.

## Follow-ups / Deferred work

- Issue `1502` should formalize the transcript artifact contract on top of these runtime turn primitives.
