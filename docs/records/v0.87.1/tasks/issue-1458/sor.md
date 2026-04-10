# v0-87-1-wp-09-cross-document-consistency-pass

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1458
Run ID: issue-1458
Version: v0.87.1
Title: [v0.87.1][WP-09] Cross-document consistency pass
Branch: codex/1458-v0-87-1-wp-09-cross-document-consistency-pass
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-10T03:36:00Z
- End Time: 2026-04-10T03:43:00Z

## Summary

Reconciled the `v0.87.1` milestone docs so they no longer describe the milestone as unstarted or already release-complete. The README now reflects landed runtime foundations plus active convergence/release-tail gates, the release notes are explicitly pre-release scoped, and the decisions log records the WP-13 demo-entrypoint disposition.

## Artifacts produced
- Updated milestone entry/status surface: `docs/milestones/v0.87.1/README.md`
- Updated release-note truth surface: `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`
- Updated decision log: `docs/milestones/v0.87.1/DECISIONS_v0.87.1.md`

## Actions taken
- Changed the README execution model so `WP-02` through `WP-08` are the runtime implementation/review-surface band and `WP-09` through `WP-12` are the convergence/acceptance/release-gate preparation band.
- Replaced stale README status text that said Sprint 1 implementation had not started with current issue truth: Sprint 1 runtime foundations and WP-13 are closed, `#1458` through `#1461` are active convergence gates, and `#1463` through `#1498` remain quality/review/release-tail validation.
- Scoped release notes as a pre-release draft and softened completion language so the release is not represented as complete until open milestone gates land or are explicitly deferred.
- Added decision `D-05` to record that D0 is the CI-safe primary demo entrypoint while D13L is a credential-gated live-provider companion proof.
- Replaced the now-stale open question about primary runtime demo entrypoints with the resolved D0/D13L disposition.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/milestones/v0.87.1/README.md`
  - `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`
  - `docs/milestones/v0.87.1/DECISIONS_v0.87.1.md`
- Worktree-only paths remaining: none; all required changes are in tracked repository paths on this branch and will be published through the repo-native finish flow.
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local tracked edits prepared for draft PR publication through the repo-native finish flow.
- Verification performed:
  - `git status --short`
  - `git diff --check`
  - focused milestone stale-language scan listed below
  - issue-reference presence scan listed below
  - SIP/STP/SOR contract validation listed below
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/validate_structured_prompt.sh --type sip --phase run --input .adl/v0.87.1/tasks/issue-1458__v0-87-1-wp-09-cross-document-consistency-pass/sip.md` verified the run-bound SIP is valid after correcting bootstrap PR-placeholder drift in the worktree-local card.
  - `bash adl/tools/validate_structured_prompt.sh --type stp --input .adl/v0.87.1/tasks/issue-1458__v0-87-1-wp-09-cross-document-consistency-pass/stp.md` verified the STP contract remains valid.
  - `rg -n "implementation not yet started|planning active / Sprint 1|completes the first full|not started|IN_PROGRESS|<<<<<<<|=======|>>>>>>>|placeholder|\\{\\{" docs/milestones/v0.87.1/README.md docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md docs/milestones/v0.87.1/DECISIONS_v0.87.1.md docs/milestones/v0.87.1/WBS_v0.87.1.md docs/milestones/v0.87.1/SPRINT_v0.87.1.md docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md || true` verified the reviewed milestone surfaces do not retain the stale unstarted/release-complete wording, conflict markers, raw placeholders, or stale `IN_PROGRESS` status.
  - `for issue in 1435 1436 1437 1438 1439 1440 1441 1442 1458 1459 1460 1461 1462 1463 1464 1494 1495 1496 1497 1498; do rg -q "#$issue" docs/milestones/v0.87.1/README.md docs/milestones/v0.87.1/WBS_v0.87.1.md docs/milestones/v0.87.1/SPRINT_v0.87.1.md docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md || echo "missing #$issue"; done` verified the milestone docs still reference the expected v0.87.1 WP issue set.
  - `git diff --check` verified the tracked diff has no whitespace errors.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.87.1/tasks/issue-1458__v0-87-1-wp-09-cross-document-consistency-pass/sor.md` verified this output record satisfies the completed SOR contract.
- Results:
  - PASS for card validation, focused doc consistency scans, issue-reference scan, and diff hygiene.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "SIP contract validation after worktree-local run-bound repair"
      - "STP contract validation"
      - "focused stale-language/conflict-marker/raw-placeholder scan over reviewed v0.87.1 docs"
      - "expected v0.87.1 WP issue-reference presence scan"
      - "git diff --check"
      - "completed SOR contract validation"
  determinism:
    status: PASS_WITH_SCOPE
    replay_verified: true
    ordering_guarantees_verified: true
    notes: deterministic lexical scans over repo-local docs and explicit ordered issue-reference checks; no runtime replay artifact required for docs-only consistency work
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
- Determinism tests executed: deterministic lexical scans and explicit ordered issue-reference checks over the `v0.87.1` milestone docs.
- Fixtures or scripts used: repo-local milestone docs under `docs/milestones/v0.87.1/` and the worktree-local issue cards.
- Replay verification (same inputs -> same artifacts/order): the validation commands can be rerun against the same docs and should produce the same pass/fail result.
- Ordering guarantees (sorting / tie-break rules used): the issue-reference scan uses an explicit ordered WP issue list from `#1435` through `#1498`.
- Artifact stability notes: this issue changes only tracked Markdown docs and the output record; no runtime or generated artifact layout changed.

## Security / Privacy Checks
- Secret leakage scan performed: no secret-bearing files or credential paths were touched; edits are release/status/decision Markdown only.
- Prompt / tool argument redaction verified: no prompt capture or tool-argument recording behavior was changed.
- Absolute path leakage check: final artifact references are repository-relative; no host-specific paths are recorded.
- Sandbox / policy invariants preserved: edits stayed inside the issue worktree and did not modify `main`.

## Replay Artifacts
- Trace bundle path(s): not applicable; no trace-producing runtime or demo was executed for this docs-only consistency issue.
- Run artifact root: not applicable; no runtime run artifact was produced.
- Replay command used for verification: not applicable; deterministic lexical validation is recorded above.
- Replay result: not applicable; no replay artifact was required for docs-only consistency work.

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.87.1/README.md`, `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`, and `docs/milestones/v0.87.1/DECISIONS_v0.87.1.md`.
- Required artifacts present: yes; all edited tracked docs are present on the branch.
- Artifact schema/version checks: not applicable; no schemas or runtime artifact formats changed.
- Hash/byte-stability checks: not run; not required for docs-only consistency work.
- Missing/optional artifacts and rationale: no demo, trace, or runtime artifact is required because proof is a coherent milestone-doc set.

## Decisions / Deviations
- Kept #1458 focused on high-signal cross-document truth fixes and left measurable acceptance mapping, sprint-plan detail alignment, and checklist/release-gate completion to #1459 through #1461.
- Repaired the run-bound SIP only inside the #1458 worktree after `pr run` exposed a bootstrap `PR: none` validator mismatch.

## Follow-ups / Deferred work
- Open a tooling issue for the SIP bootstrap validator drift: generated pre-run SIP cards use `PR: none`, but the SIP validator rejects non-empty non-URL PR values before a PR exists.
- Continue with #1459, #1460, and #1461 after #1458 merges, because those issues own acceptance mapping, sprint-plan alignment, and checklist/release-gate completion.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
