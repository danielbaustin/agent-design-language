# ADL Output Card

Canonical Template Source: `swarm/templates/cards/output_card_template.md`
Consumed by: `swarm/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Task ID:
Run ID:
Version:
Title:
Branch:
Status: IN_PROGRESS

Execution:
- Actor:
- Model:
- Provider:
- Start Time:
- End Time:

## Summary

## Artifacts produced
- 

## Actions taken
- 

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
- Worktree-only paths remaining: none | list explicitly
- Integration state: worktree_only | pr_open | merged
- Verification scope: worktree | pr_branch | main_repo
- Integration method used:
- Verification performed:
  - `git status`
  - `ls <path>` / equivalent path check
- Result:

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout <branch> -- <path>` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- `Integration state` describes lifecycle state of the integrated artifact set, not where verification happened.
- `Verification scope` describes where the verification commands were run.
- If `Integration method used` is `direct write in main repo`, `Verification scope` should normally be `main_repo` unless the deviation is explained.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Tests / checks run:
- Results:

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.

## Verification Summary
```yaml
verification_summary:
  validation:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    checks_run:
      - ""
  determinism:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    replay_verified: true | false | unknown
    ordering_guarantees_verified: true | false | unknown
  security_privacy:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    secrets_leakage_detected: true | false | unknown
    prompt_or_tool_arg_leakage_detected: true | false | unknown
    absolute_path_leakage_detected: true | false | unknown
  artifacts:
    status: PASS | FAIL | PARTIAL | NOT_RUN
    required_artifacts_present: true | false | unknown
    schema_changes:
      present: true | false | unknown
      approved: true | false | not_applicable | unknown
```

## Determinism Evidence
- Determinism tests executed:
- Replay verification (same inputs -> same artifacts/order):
- Ordering guarantees (sorting / tie-break rules used):
- Artifact stability notes:

## Security / Privacy Checks
- Secret leakage scan performed:
- Prompt / tool argument redaction verified:
- Absolute path leakage check:
- Sandbox / policy invariants preserved:

## Replay Artifacts
- Trace bundle path(s):
- Run artifact root:
- Replay command used for verification:
- Replay result:

## Artifact Verification
- Required artifacts present:
- Artifact schema/version checks:
- Hash/byte-stability checks:
- Missing/optional artifacts and rationale:

## Decisions / Deviations

## Follow-ups / Deferred work
