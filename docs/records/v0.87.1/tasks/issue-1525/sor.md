# feature-status-list

Task ID: issue-1525
Run ID: issue-1525
Version: v0.87.1
Title: Restore ADL feature list with status and target milestones
Branch: codex/1525-feature-status-list
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-10T01:10:00Z
- End Time: 2026-04-10T01:22:07Z

## Summary

Restored the empty ADL feature-list surface by creating a durable tracked
feature status document at `docs/milestones/v0.75/ADL_FEATURE_LIST.md`.

The document now includes implementation status, implemented features missing
from the prior narrative, and scheduled completion milestones through `v0.95`.
The ignored `.adl/docs/v0.75planning/ADL_FEATURE_LIST.md` file was also
restored as a pointer to the tracked source of truth.

## Artifacts produced

- `docs/milestones/v0.75/ADL_FEATURE_LIST.md`
- `.adl/docs/v0.75planning/ADL_FEATURE_LIST.md` local planning pointer
- GitHub issue `#1525`
- Local task bundle `.adl/v0.87.1/tasks/issue-1525__feature-status-list/`
- Local cards `.adl/cards/1525/`

## Actions taken

- Created GitHub issue `#1525` for the feature-list recovery/status work.
- Created branch `codex/1525-feature-status-list`.
- Initialized the ADL task bundle with `adl/tools/pr.sh init`.
- Rebuilt the tracked feature-list document with conservative status language.
- Restored the ignored `.adl` planning file as a pointer to the durable tracked
  document.
- Ran validation checks for non-empty files, expected status headings, and
  whitespace safety.

## Main Repo Integration (REQUIRED)

- Main-repo paths updated: `docs/milestones/v0.75/ADL_FEATURE_LIST.md`
- Worktree-only paths remaining: none for tracked artifacts
- Integration state: worktree_only
- Verification scope: main_repo
- Integration method used: direct write in current branch
- Verification performed:
  - `test -s docs/milestones/v0.75/ADL_FEATURE_LIST.md`
  - `test -s .adl/docs/v0.75planning/ADL_FEATURE_LIST.md`
  - `rg -n "Feature Status Matrix|Implemented Features Missing|v0.95|Zed Integration|Trace v1 substrate|Control-plane lifecycle" docs/milestones/v0.75/ADL_FEATURE_LIST.md .adl/docs/v0.75planning/ADL_FEATURE_LIST.md`
  - `git diff --check`
  - `git status --short --untracked-files=all`
- Result: PASS

## Validation

- Validation commands and their purpose:
  - `test -s docs/milestones/v0.75/ADL_FEATURE_LIST.md`: verified the tracked feature list is no longer empty.
  - `test -s .adl/docs/v0.75planning/ADL_FEATURE_LIST.md`: verified the local planning copy is no longer empty.
  - `rg -n ...`: verified the restored docs contain feature status, missing implemented features, v0.95 scheduling, Zed status, trace, and control-plane entries.
  - `git diff --check`: verified no whitespace errors in tracked changes.
  - `git status --short --untracked-files=all`: verified the tracked diff is limited to the feature-list doc.
- Results: PASS

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "test -s docs/milestones/v0.75/ADL_FEATURE_LIST.md"
      - "test -s .adl/docs/v0.75planning/ADL_FEATURE_LIST.md"
      - "rg feature-status anchors in restored docs"
      - "git diff --check"
      - "git status --short --untracked-files=all"
  determinism:
    status: NOT_RUN
    replay_verified: false
    ordering_guarantees_verified: false
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

- Determinism tests executed: not applicable; this was a docs/status recovery task.
- Fixtures or scripts used: not applicable.
- Replay verification: not applicable for markdown-only feature inventory.
- Ordering guarantees: not applicable.
- Artifact stability notes: status table and milestone schedule are deterministic markdown artifacts.

## Security / Privacy Checks

- Secret leakage scan performed: no secrets were introduced; only public repo paths and issue numbers were used.
- Prompt / tool argument redaction verified: no prompt payloads or tool arguments were added to tracked docs.
- Absolute path leakage check: tracked docs use repository-relative paths only.
- Sandbox / policy invariants preserved: edits were limited to docs and ignored local planning/card surfaces.

## Replay Artifacts

- Trace bundle path(s): not applicable.
- Run artifact root: not applicable.
- Replay command used for verification: not applicable.
- Replay result: docs-only task; runtime replay not required.

## Artifact Verification

- Primary proof surface: `docs/milestones/v0.75/ADL_FEATURE_LIST.md`
- Required artifacts present: yes.
- Artifact schema/version checks: not applicable; no schema changes.
- Hash/byte-stability checks: not run; markdown content was manually reviewed through grep/status checks.
- Missing/optional artifacts and rationale: PR was not opened in this step; final PR creation can be done with `adl/tools/pr.sh finish`.

## Decisions / Deviations

- The requested `.adl/docs/v0.75planning/ADL_FEATURE_LIST.md` file is ignored by git, so the durable feature list was restored in tracked form at `docs/milestones/v0.75/ADL_FEATURE_LIST.md`.
- The ignored `.adl` file was restored as a pointer to avoid future silent divergence.
- `adl/tools/pr.sh run 1525 ...` was attempted, but it refused because the target branch was already checked out in the current main checkout. Work continued safely in the active branch.
- No PR was opened from this planning-thread finish attempt.

## Follow-ups / Deferred work

- Optionally run `adl/tools/pr.sh finish 1525 --title "Restore ADL feature list with status and target milestones" --paths "docs/milestones/v0.75/ADL_FEATURE_LIST.md" -f .adl/cards/1525/input_1525.md --output-card .adl/cards/1525/output_1525.md` to open the PR.
