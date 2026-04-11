# add-post-merge-automatic-closeout-normalization-for-canonical-adl-issue-bundles

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1632
Run ID: issue-1632
Version: v0.87.1
Title: [v0.87.1][tools] Add post-merge automatic closeout normalization for canonical .adl issue bundles
Branch: codex/1632-add-post-merge-automatic-closeout-normalization-for-canonical-adl-issue-bundles
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: OpenAI Codex
- Start Time: 2026-04-11T00:00:00Z
- End Time: 2026-04-11T00:00:00Z

## Summary

Added automatic post-merge closeout attachment to the normal `pr finish` publication path so merged issues can normalize their canonical `.adl` bundle without a separate manual cleanup pass. Added a repo-native watcher script, finish-path regression coverage, and a bounded shell test that proves merged PR plus closed/completed issue state triggers automatic closeout and emits a reviewable summary artifact.

## Artifacts produced
- `adl/tools/attach_post_merge_closeout.sh`
- `adl/tools/test_attach_post_merge_closeout.sh`
- updated finish-path coverage in `adl/src/cli/tests/pr_cmd_inline/finish.rs`

## Actions taken
- added a `finish`-path attachment hook for post-merge closeout alongside PR janitor auto-attach
- implemented a background watcher that waits for merged PR plus `CLOSED/COMPLETED` issue truth before invoking `pr closeout`
- added a summary artifact for automatic normalization outcomes so the result is reviewable
- added a finish regression for closeout auto-attach failure and preserved the normal draft-PR publication path
- added a shell success-path test for the watcher script

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: not yet; changes are currently in the issue worktree branch pending PR publication
- Worktree-only paths remaining: `adl/tools/attach_post_merge_closeout.sh`, `adl/tools/test_attach_post_merge_closeout.sh`, `adl/src/cli/pr_cmd.rs`, `adl/src/cli/pr_cmd/github.rs`, `adl/src/cli/tests/pr_cmd_inline/finish.rs`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: managed issue worktree execution before repo-native `pr finish` publication
- Verification performed:
  - `git status --short` verified the exact changed surfaces are limited to the issue worktree paths listed above
  - `git diff --check` verified there are no whitespace or patch-integrity defects in the worktree changes
- Result: FAIL

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
  - `cargo fmt --manifest-path adl/Cargo.toml --all` verified formatting for the touched Rust sources
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_fails_when_post_merge_closeout_auto_attach_fails -- --nocapture` verified finish fails loudly when post-merge closeout auto-attach cannot be started
  - `cargo test --manifest-path adl/Cargo.toml real_pr_finish_creates_draft_pr_and_commits_branch_changes -- --nocapture` verified the normal draft PR publication path still succeeds with the new post-merge closeout attach
  - `bash adl/tools/test_attach_post_merge_closeout.sh` verified the watcher triggers `pr closeout` after merged PR plus `CLOSED/COMPLETED` issue truth and writes a summary artifact
  - `bash -n adl/tools/attach_post_merge_closeout.sh adl/tools/test_attach_post_merge_closeout.sh` verified shell syntax for the new helper and its test
  - `git diff --check` verified no patch-integrity defects remain in the worktree
- Results: PASS; all targeted validation completed successfully for the new automatic post-merge closeout path

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
      - "cargo fmt --manifest-path adl/Cargo.toml --all"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_fails_when_post_merge_closeout_auto_attach_fails -- --nocapture"
      - "cargo test --manifest-path adl/Cargo.toml real_pr_finish_creates_draft_pr_and_commits_branch_changes -- --nocapture"
      - "bash adl/tools/test_attach_post_merge_closeout.sh"
      - "bash -n adl/tools/attach_post_merge_closeout.sh adl/tools/test_attach_post_merge_closeout.sh"
      - "git diff --check"
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
- Determinism tests executed: `bash adl/tools/test_attach_post_merge_closeout.sh`
- Fixtures or scripts used: fake `gh` and `pr.sh` executables inside the shell test fixture
- Replay verification (same inputs -> same artifacts/order): identical merged-PR and closed/completed-issue JSON responses trigger the same `pr closeout` invocation and the same summary status
- Ordering guarantees (sorting / tie-break rules used): watcher evaluation order is fixed as PR merge state first, then issue lifecycle state, then bounded closeout invocation
- Artifact stability notes: the summary artifact content is emitted from fixed status strings and the same declared normalized surface list for accepted closeout runs

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the new helper, test fixture, and SOR content
- Prompt / tool argument redaction verified: yes; the helper records only issue/PR metadata and bounded closeout status, not prompts or tool arguments
- Absolute path leakage check: passed; recorded commands and artifact references in this card are repository-relative
- Sandbox / policy invariants preserved: yes; the automation only invokes existing repo-native closeout after GitHub closure truth is unambiguous

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue adds operational logs rather than ADL replay traces
- Run artifact root: `.adl/logs/post-merge-closeout/issue-<n>/`
- Replay command used for verification: `bash adl/tools/test_attach_post_merge_closeout.sh`
- Replay result: PASS; the watcher path is reproducible under the fixture-controlled merged/closed lifecycle responses

## Artifact Verification
- Primary proof surface: finish-path Rust regression tests plus `adl/tools/test_attach_post_merge_closeout.sh`
- Required artifacts present: true
- Artifact schema/version checks: not applicable; no new ADL card schema or runtime artifact schema was introduced
- Hash/byte-stability checks: not applicable; this issue proves behavioral determinism through fixed fixtures rather than byte-hash snapshots
- Missing/optional artifacts and rationale: no standalone demo is required because this is bounded workflow automation behavior

## Decisions / Deviations
- implemented post-merge normalization as a separate attachment alongside janitor rather than widening janitor responsibilities, preserving the existing phase model
- kept the strict `CLOSED/COMPLETED` gate in the existing closeout path rather than allowing pre-closure normalization from `finish` or `doctor`

## Follow-ups / Deferred work
- none in issue scope; broader closed-record cleanup remains tracked separately by `#1555`

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
