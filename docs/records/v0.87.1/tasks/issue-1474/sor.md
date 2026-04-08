# v0-87-1-tools-add-user-entered-provider-credentials-setup-flow

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1474
Run ID: issue-1474
Version: v0.87.1
Title: [v0.87.1][tools] Add user-entered provider credentials setup flow
Branch: codex/1474-v0-87-1-tools-add-user-entered-provider-credentials-setup-flow
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: Codex desktop
- Start Time: 2026-04-08T20:05:00Z
- End Time: 2026-04-08T20:05:00Z

## Summary
Added a user-facing `adl provider setup <family>` flow that generates local provider credential/setup bundles for supported remote provider families, documented the setup contract, and added focused CLI coverage so the setup path stays stable.

## Artifacts produced
- new provider setup CLI surface in `adl/src/cli/provider_cmd.rs`
- CLI dispatch and usage updates in `adl/src/cli/mod.rs` and `adl/src/cli/usage.rs`
- focused CLI coverage in `adl/src/cli/tests.rs`
- user-facing setup documentation in `docs/tooling/PROVIDER_SETUP.md`

## Actions taken
- added a new `provider` CLI command family with a `setup` subcommand
- implemented setup bundle generation for `chatgpt`, `openai`, `anthropic`, `gemini`, `deepseek`, and `http`
- generated local-only setup artifacts consisting of `provider.adl.yaml`, `.env.example`, and `README.md`
- documented the generated bundle format and the bounded HTTP endpoint contract
- added regression coverage for direct command dispatch and setup bundle generation

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `adl/src/cli/provider_cmd.rs`, `adl/src/cli/mod.rs`, `adl/src/cli/usage.rs`, `adl/src/cli/tests.rs`, `docs/tooling/PROVIDER_SETUP.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: worktree implementation on the issue branch, to be published through the PR opened by `pr finish`
- Verification performed:
  - `git status --short`
    - verified the diff is bounded to the five intended tracked paths
  - `cargo fmt --manifest-path adl/Cargo.toml --all --check`
    - verified formatting is clean
  - `cargo test --manifest-path adl/Cargo.toml provider_setup_dispatch_path_succeeds -- --nocapture`
    - verified the top-level CLI dispatch path creates a real setup bundle
  - `cargo test --manifest-path adl/Cargo.toml provider_setup_ -- --nocapture`
    - verified the targeted provider setup unit tests all pass
  - `cargo run --manifest-path adl/Cargo.toml --bin adl -- provider setup chatgpt --out <tmpdir> --force`
    - verified the real user-facing command writes `provider.adl.yaml`, `.env.example`, and `README.md`
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
- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
  - verified formatting is clean
- `cargo test --manifest-path adl/Cargo.toml provider_setup_dispatch_path_succeeds -- --nocapture`
  - verified the top-level CLI dispatch path succeeds and writes the expected files
- `cargo test --manifest-path adl/Cargo.toml provider_setup_ -- --nocapture`
  - verified the focused provider setup test set passes
- `cargo run --manifest-path adl/Cargo.toml --bin adl -- provider setup chatgpt --out <tmpdir> --force`
  - verified the actual CLI command works from the user surface
- Results:
  - formatting passed
  - targeted provider setup tests passed
  - end-to-end CLI setup command passed

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

Rules:
- Replace the example values below with one actual final value per field.
- Do not leave pipe-delimited enum menus or placeholder text in a finished record.

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - cargo fmt --manifest-path adl/Cargo.toml --all --check
      - cargo test --manifest-path adl/Cargo.toml provider_setup_dispatch_path_succeeds -- --nocapture
      - cargo test --manifest-path adl/Cargo.toml provider_setup_ -- --nocapture
      - cargo run --manifest-path adl/Cargo.toml --bin adl -- provider setup chatgpt --out <tmpdir> --force
  determinism:
    status: PASS
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `provider_setup_dispatch_path_succeeds` and the targeted `provider_setup_` test set
- Fixtures or scripts used: temp output directories created by the tests and by the bounded CLI proof command
- Replay verification (same inputs -> same artifacts/order): not run as a separate replay harness; the setup command is covered through repeated deterministic file-content assertions
- Ordering guarantees (sorting / tie-break rules used): setup output is template-driven and does not depend on map iteration or unstable ordering
- Artifact stability notes: generated bundle content is emitted from fixed templates for each family and does not contain timestamps or host-specific paths

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the changed tracked files plus confirmation that the generated env artifact is an example template rather than a real credential file
- Prompt / tool argument redaction verified: yes; no prompts or tool arguments were added to tracked surfaces
- Absolute path leakage check: repository-relative references only in tracked surfaces and this output card
- Sandbox / policy invariants preserved: yes

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `adl/src/cli/provider_cmd.rs` plus the generated bundle checks exercised by the tests and CLI command
- Required artifacts present: yes
- Artifact schema/version checks: not applicable
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no demo or trace artifact was required because this issue ships a setup/configuration flow rather than a runtime execution feature

## Decisions / Deviations
- kept the setup flow local and file-based rather than building a credential store in this pass
- documented the bounded HTTP contract explicitly so users are not misled into pointing ADL directly at raw vendor-native endpoints
- used provider families rather than every individual model profile so the setup flow stays compact and understandable

## Follow-ups / Deferred work
- `#1468` remains the right place to create the actual provider demos/tests on top of this setup flow
- richer secret management backends can be considered later if the local env-template approach proves insufficient

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
