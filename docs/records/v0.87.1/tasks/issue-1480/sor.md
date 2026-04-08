# [v0.87.1][docs] Backfill truthful replay claim in issue-1436 tracked SOR

Task ID: issue-1480
Run ID: issue-1480
Version: v0.87.1
Title: [v0.87.1][docs] Backfill truthful replay claim in issue-1436 tracked SOR
Branch: codex/1480-v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI
- Start Time: 2026-04-08T20:05:00Z
- End Time: 2026-04-08T20:22:35Z

## Summary
Published the already-reviewed truth correction for the tracked `issue-1436` SOR through a proper bound issue/worktree flow instead of leaving the change dirty on `main`. The only repo change is the narrower replay claim and accompanying wording cleanup in `docs/records/v0.87.1/tasks/issue-1436/sor.md`.

## Artifacts produced
- `docs/records/v0.87.1/tasks/issue-1436/sor.md`

## Actions taken
- moved the pending tracked-SOR truth fix off `main` into the bound 1480 worktree
- rewrote the bootstrap source prompt, STP, SIP, and SOR into a truthful tiny docs-follow-up bundle
- synced the source prompt back to GitHub issue `#1480`
- kept the repository diff strictly bounded to the existing issue-1436 record correction

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `docs/records/v0.87.1/tasks/issue-1436/sor.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: bounded worktree update pending `pr finish`
- Verification performed:
  - `git diff --check`
  - `git diff -- docs/records/v0.87.1/tasks/issue-1436/sor.md`
  - `bash adl/tools/validate_structured_prompt.sh --type stp --phase plan --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/stp.md`
  - `bash adl/tools/validate_structured_prompt.sh --type sip --phase input --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sip.md`
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase output --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sor.md`
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `git diff --check` to verify the bounded docs follow-up has no patch-format problems
  - `git diff -- docs/records/v0.87.1/tasks/issue-1436/sor.md` to verify the branch contains only the intended replay-claim downgrade and wording-tightening in the tracked issue-1436 SOR
  - `bash adl/tools/validate_structured_prompt.sh --type stp --phase plan --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/stp.md` to verify the rewritten STP is contract-valid
  - `bash adl/tools/validate_structured_prompt.sh --type sip --phase input --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sip.md` to verify the rewritten SIP is contract-valid
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase output --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sor.md` to verify this output record is contract-valid
- Results:
  - bounded diff is exactly the intended tracked SOR truth fix
  - card validations passed
  - no code or unrelated docs changes were introduced

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git diff --check"
      - "git diff -- docs/records/v0.87.1/tasks/issue-1436/sor.md"
      - "bash adl/tools/validate_structured_prompt.sh --type stp --phase plan --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/stp.md"
      - "bash adl/tools/validate_structured_prompt.sh --type sip --phase input --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sip.md"
      - "bash adl/tools/validate_structured_prompt.sh --type sor --phase output --input .adl/v0.87.1/tasks/issue-1480__v0-87-1-docs-backfill-truthful-replay-claim-in-issue-1436-tracked-sor/sor.md"
  determinism:
    status: NOT_RUN
    replay_verified: not_applicable
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
- Determinism tests executed: none; this is a docs-only truth correction.
- Fixtures or scripts used: none.
- Replay verification (same inputs -> same artifacts/order): not applicable.
- Ordering guarantees (sorting / tie-break rules used): not applicable.
- Artifact stability notes: the tracked record change is a manual wording correction, not a generated artifact flow.

## Security / Privacy Checks
- Secret leakage scan performed: manual record review only; no secret-bearing content was added.
- Prompt / tool argument redaction verified: yes; the record contains no prompts or tool arguments.
- Absolute path leakage check: passed; only repository-relative paths are recorded.
- Sandbox / policy invariants preserved: yes; no destructive commands and no edits outside the bounded issue/worktree flow.

## Replay Artifacts
- Trace bundle path(s): none
- Run artifact root: none
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: `docs/records/v0.87.1/tasks/issue-1436/sor.md`
- Required artifacts present: yes
- Artifact schema/version checks: no schema change; this is a truth-alignment wording correction inside the existing record
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: no trace or runtime artifacts are expected for this docs-only follow-up

## Decisions / Deviations
- The issue exists solely to move an already-approved tracked-record correction off `main` and through normal review flow.
- The change remains worktree-only until `pr finish` publishes the branch and tracked review surface.

## Follow-ups / Deferred work
- none
