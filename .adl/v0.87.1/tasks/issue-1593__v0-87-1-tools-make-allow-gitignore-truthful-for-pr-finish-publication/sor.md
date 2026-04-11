# v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1593
Run ID: issue-1593
Version: v0.87.1
Title: [v0.87.1][tools] Make --allow-gitignore truthful for pr finish publication
Branch: codex/1593-v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-11T17:48:00Z
- End Time: 2026-04-11T18:07:27Z

## Summary
Clarified the `pr finish` contract so `--allow-gitignore` only covers staged ignore-policy changes, while canonical issue bundle files are explicitly described as automatically staged. Added regression coverage for the updated guard behavior and parser acceptance.

## Artifacts produced
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- `adl/tools/pr.sh`

## Actions taken
- Updated the `pr finish` `.gitignore` guard error text to describe the actual post-1592 behavior.
- Tightened `adl/tools/pr.sh` help text so canonical issue-bundle staging and `--allow-gitignore` scope are no longer ambiguous.
- Added parser coverage for `--allow-gitignore` and a regression test proving staged `.gitignore` changes are rejected without the flag.
- Opened PR `#1606` stacked on PR `#1599` so the contract cleanup is reviewed against the runtime fix it describes.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/v0.87.1/tasks/issue-1593__v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication/sor.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: normalized the canonical root SOR directly on `main` after verifying the issue is already closed and linked to merged PR `#1606`
- Verification performed:
  - `gh issue view 1593 --json title,url,state,stateReason,closedByPullRequestsReferences`
    - verified the issue is closed and captured the final closure metadata used for this normalization pass
  - `gh pr view 1606 --json state,url`
    - verified the linked closing PR remains available as the final publication surface
  - `ls .adl/v0.87.1/tasks/issue-1593__v0-87-1-tools-make-allow-gitignore-truthful-for-pr-finish-publication/sor.md`
    - verified the canonical root SOR path exists on the main repository path
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `bash -n adl/tools/pr.sh` to verify the updated shell help surface remains syntactically valid
  - `cargo test --manifest-path adl/Cargo.toml parse_finish_args_requires_title_and_accepts_finish_flags -- --nocapture` to verify parser acceptance of the updated finish flag set
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_rejects_staged_gitignore_changes_without_allow_flag -- --nocapture` to verify the new guard contract and explanatory error path
  - `git diff --check` to verify no whitespace or patch-format regressions remain
- Results: PASS. All targeted validations completed successfully.

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
      - "bash -n adl/tools/pr.sh"
      - "cargo test --manifest-path adl/Cargo.toml parse_finish_args_requires_title_and_accepts_finish_flags -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_rejects_staged_gitignore_changes_without_allow_flag -- --nocapture"
      - "git diff --check"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: not_applicable
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
- Determinism tests executed: repeated targeted Rust test runs over the same parser and finish guard paths
- Fixtures or scripts used: built-in Rust unit and inline integration tests under `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- Replay verification (same inputs -> same artifacts/order): confirmed for the targeted tests; rerunning the same test filters produced the same pass result and error-path semantics
- Ordering guarantees (sorting / tie-break rules used): not applicable because this issue changes operator-facing wording and guard behavior, not emitted ordering logic
- Artifact stability notes: the proof surface is limited to one Rust command module, one inline test module, and one shell help surface

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of touched diffs; no secrets or credential material were introduced
- Prompt / tool argument redaction verified: yes; the new wording only documents flag scope and does not emit sensitive inputs
- Absolute path leakage check: passed via review of the final SOR and touched repo files; only repository-relative paths are recorded here
- Sandbox / policy invariants preserved: yes; the change does not widen ignored-path staging and keeps generic ignored-file publication blocked

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; no ADL runtime trace bundle was produced for this CLI/help-text fix
- Run artifact root: not applicable; validation used targeted repository tests only
- Replay command used for verification: not applicable beyond rerunning the targeted cargo tests listed above
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: PR `#1606` plus the tracked files `adl/src/cli/pr_cmd.rs`, `adl/src/cli/tests/pr_cmd_inline/finish.rs`, and `adl/tools/pr.sh`
- Required artifacts present: yes; all issue-specific tracked artifacts are present on the pushed branch
- Artifact schema/version checks: not applicable; no schema-bearing documents changed in this issue
- Hash/byte-stability checks: not run; issue scope is bounded to source and help-text updates validated through tests
- Missing/optional artifacts and rationale: no additional artifacts were required beyond the tracked code and the canonical issue record

## Decisions / Deviations
- Stacked PR `#1606` on `#1599` because the contract clarification depends on the runtime publication behavior introduced by issue `#1592`.

## Follow-ups / Deferred work
- A future cleanup can rename `--allow-gitignore` to a more self-describing flag, but this issue intentionally kept the public surface stable.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
