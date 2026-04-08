# v0-87-1-meta-create-provider-demo-and-test-issue-and-card-set

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1468
Run ID: issue-1468
Version: v0.87.1
Title: [v0.87.1][meta] Create provider demo and test issue and card set
Branch: codex/1468-v0-87-1-meta-create-provider-demo-and-test-issue-and-card-set
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5.4
- Provider: Codex desktop
- Start Time: 2026-04-08T20:25:00Z
- End Time: 2026-04-08T20:47:54Z

## Summary

Created the four family-level provider demo/test issues and local task bundles for `ollama`, `http`, `mock`, and `chatgpt`, then added a reviewer-facing issue map to the `v0.87.1` demo matrix.

## Artifacts produced
- new GitHub issues:
  - `#1485` local Ollama provider demo + acceptance test
  - `#1486` bounded HTTP provider demo + acceptance test
  - `#1487` mock provider demo + acceptance test
  - `#1488` ChatGPT provider demo + acceptance test
- new root local task bundles under `.adl/v0.87.1/tasks/issue-1485__...` through `.adl/v0.87.1/tasks/issue-1488__...`
- updated tracked demo-planning surface:
  - `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`

## Actions taken
- started issue `1468` in its bound worktree through the repo-native run flow
- identified the current provider-family scope from code and prior prerequisite issues
- authored four family-level issue bodies with bounded demo + acceptance-test scope
- created the GitHub issues and seeded the root source/STP/SIP/SOR bundles for each one
- normalized the generated SIPs for `#1485` through `#1488` into truthful pre-run state (`Branch: not bound yet`)
- added a `Provider Family Demo / Test Issue Map` section to the `v0.87.1` demo matrix

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: worktree implementation for tracked demo-matrix changes plus root-checkout issue/card creation for the new local bundles
- Verification performed:
  - `git status --short`
    - verified the tracked diff in the worktree is limited to the intended demo-matrix update
  - `gh issue view 1485 --json url,title`
    - verified the local Ollama family issue exists on GitHub
  - `gh issue view 1486 --json url,title`
    - verified the bounded HTTP family issue exists on GitHub
  - `gh issue view 1487 --json url,title`
    - verified the mock family issue exists on GitHub
  - `gh issue view 1488 --json url,title`
    - verified the ChatGPT family issue exists on GitHub
  - `bash adl/tools/pr.sh doctor 1485 --version v0.87.1 --mode preflight --json`
    - verified the created local Ollama issue/card set is review-ready apart from the expected open-PR wave block
  - `bash adl/tools/pr.sh doctor 1488 --version v0.87.1 --mode preflight --json`
    - verified the created ChatGPT issue/card set is review-ready apart from the expected open-PR wave block
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
- `bash adl/tools/pr.sh doctor 1485 --version v0.87.1 --mode preflight --json`
  - verified the created local Ollama issue/card set parses and only blocks on the current open-PR wave
- `bash adl/tools/pr.sh doctor 1488 --version v0.87.1 --mode preflight --json`
  - verified the created ChatGPT issue/card set parses and only blocks on the current open-PR wave
- Results:
  - child issue creation succeeded for all four families
  - child bundles were seeded successfully
  - spot-checked preflight/doctor output is consistent with review-ready bootstrap state

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
      - bash adl/tools/pr.sh doctor 1485 --version v0.87.1 --mode preflight --json
      - bash adl/tools/pr.sh doctor 1488 --version v0.87.1 --mode preflight --json
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
- Determinism tests executed: none beyond the repo-native bootstrap/doctor paths
- Fixtures or scripts used: `adl/tools/pr.sh create` and `adl/tools/pr.sh doctor`
- Replay verification (same inputs -> same artifacts/order): not applicable for this meta/bootstrap issue
- Ordering guarantees (sorting / tie-break rules used): issue creation was intentionally family-scoped and the created issue map was recorded in a fixed order (`ollama`, `http`, `mock`, `chatgpt`)
- Artifact stability notes: the created source/STP/SIP/SOR bundle layout is the canonical repo-native issue bootstrap layout

## Security / Privacy Checks
- Secret leakage scan performed: reviewed the authored issue bodies and demo-matrix update to ensure no credentials or local secret values were recorded
- Prompt / tool argument redaction verified: no prompts or tool arguments were written into tracked artifacts
- Absolute path leakage check: tracked demo-matrix changes do not include host-absolute paths
- Sandbox / policy invariants preserved: no branches or worktrees were created for the child issues; only the meta issue used a worktree

## Replay Artifacts
- Trace bundle path(s): not applicable; this meta/bootstrap issue does not generate runtime traces
- Run artifact root: not applicable; proof is the created issue/card set and the updated demo matrix
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- Required artifacts present: yes; the four GitHub issues and their root local bundles exist
- Artifact schema/version checks: issue bodies and seeded STP/SIP/SOR bundles passed bootstrap contract validation during creation
- Hash/byte-stability checks: not applicable for this issue
- Missing/optional artifacts and rationale: no tracked code/test/demo artifacts are expected because this issue stops at review-ready bootstrap state

## Decisions / Deviations
- Kept the new work family-scoped rather than per-profile to avoid multiplying execution issues without adding review value
- Updated the demo matrix with an issue map instead of inventing new milestone-demo rows before the provider demos themselves exist
- Normalized the generated child SIPs into truthful pre-run state because the default generated wording still assumed started worktrees

## Follow-ups / Deferred work
- `#1485` local Ollama provider demo + acceptance test
- `#1486` bounded HTTP provider demo + acceptance test
- `#1487` mock provider demo + acceptance test
- `#1488` ChatGPT provider demo + acceptance test

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
