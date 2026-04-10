# backfill-closed-issue-sor-truth-and-card-hygiene

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1546
Run ID: issue-1546
Version: v0.87.1
Title: [v0.87.1][records] Backfill closed issue SOR truth and card hygiene
Branch: codex/1546-backfill-closed-issue-sor-truth-and-card-hygiene
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5-codex
- Provider: openai
- Start Time: 2026-04-10T03:38:00Z
- End Time: 2026-04-10T03:48:18Z

## Summary

Scanned closed v0.87.1 issue records and normalized stale tracked SOR integration truth for closed/merged issues. Corrected 27 existing tracked SORs from stale `worktree_only` or `pr_open` lifecycle wording to `merged` where the associated PR is merged, preserved original validation evidence, and recorded missing/deferred tracked SORs in a hygiene index instead of inventing unsupported records.

## Artifacts produced
- `docs/records/v0.87.1/CLOSED_ISSUE_RECORD_HYGIENE_1546.md`
- Updated tracked SORs under `docs/records/v0.87.1/tasks/issue-*/sor.md` for issues `1436`, `1437`, `1438`, `1439`, `1440`, `1441`, `1442`, `1449`, `1455`, `1462`, `1468`, `1469`, `1473`, `1474`, `1477`, `1480`, `1500`, `1501`, `1502`, `1518`, `1519`, `1525`, `1527`, `1528`, `1529`, `1533`, and `1541`.

## Actions taken
- Created and bootstrapped GitHub issue `#1546` with the repo-native PR lifecycle.
- Ran `pr doctor` for issue `#1546`; structural readiness passed and preflight was blocked only by unrelated open PR wave state.
- Bound the issue worktree with `pr run --allow-open-pr-wave`, recording the preflight override as intentional because this task is a closed-record hygiene pass.
- Queried closed `version:v0.87.1` GitHub issues and merged PR mappings.
- Scanned tracked `docs/records/v0.87.1/tasks/` SORs for stale `IN_PROGRESS`, `NOT_STARTED`, `worktree_only`, `pr_open`, pending-publication, and failed integration claims.
- Normalized 27 tracked SOR Main Repo Integration sections to `Integration state: merged` where merged PR evidence exists.
- Preserved original validation commands and result evidence, except issue `#1533` Main Repo Integration result was corrected from `FAIL` to `PASS` while keeping the live-validation partial note intact.
- Added a hygiene index listing scanned counts, corrected records, and missing/deferred tracked SORs.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: tracked record-hygiene changes are present on the issue branch and will be published by the PR for issue `#1546`.
- Worktree-only paths remaining: none for required tracked artifacts; the worktree-local `.adl` SOR is a workflow compatibility surface and will be synced to the tracked review surface by `pr finish`.
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: issue branch/worktree edits prepared for PR publication through the repo-native finish flow.
- Verification performed:
  - `git status --short --branch` verified the branch contains only the intended tracked SOR/report edits before finish.
  - `rg -n 'Integration state: (worktree_only|pr_open)|Status: (IN_PROGRESS|NOT_STARTED)|Main-repo paths updated: none yet|Main-repo paths updated: none$|PR publication is pending|pending PR|Result: FAIL' docs/records/v0.87.1/tasks docs/records/v0.87.1/CLOSED_ISSUE_RECORD_HYGIENE_1546.md || true` verified the corrected tracked SOR set no longer contains the targeted stale lifecycle claims.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase output --input <tracked v0.87.1 sor>` loop verified every tracked v0.87.1 SOR still satisfies the SOR contract after the hygiene edits.
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
- Validation commands and their purpose:
  - `bash adl/tools/pr.sh doctor 1546 --slug backfill-closed-issue-sor-truth-and-card-hygiene --version v0.87.1 --mode full --json` verified the issue bundle was structurally ready and identified the unrelated open-PR preflight block.
  - `bash adl/tools/pr.sh run 1546 --slug backfill-closed-issue-sor-truth-and-card-hygiene --version v0.87.1 --allow-open-pr-wave` verified the worktree could be bound while explicitly overriding the unrelated open-PR wave block.
  - `gh issue list --state closed --label version:v0.87.1 --limit 200 --json number,title,closedAt,url` gathered the closed issue set used for the hygiene pass.
  - `gh pr list --state merged --search 'repo:danielbaustin/agent-design-language is:pr is:merged <issue>' --limit 5 --json number,title,mergedAt,url,headRefName` gathered merged PR evidence for the corrected records.
  - `rg -n 'Integration state: (worktree_only|pr_open)|Status: (IN_PROGRESS|NOT_STARTED)|Main-repo paths updated: none yet|Main-repo paths updated: none$|PR publication is pending|pending PR|Result: FAIL' docs/records/v0.87.1/tasks docs/records/v0.87.1/CLOSED_ISSUE_RECORD_HYGIENE_1546.md || true` verified the targeted stale lifecycle claims were removed from tracked SORs.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase output --input <tracked v0.87.1 sor>` loop verified all tracked v0.87.1 SORs remain contract-valid.
- Results: all validation commands completed successfully; doctor preflight remained blocked by unrelated open PR wave state, and execution proceeded under the explicit `--allow-open-pr-wave` override.

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "pr doctor 1546 full json"
      - "pr run 1546 with explicit open-pr-wave override"
      - "closed v0.87.1 issue scan"
      - "merged PR evidence scan"
      - "stale lifecycle rg scan"
      - "tracked v0.87.1 SOR contract validation loop"
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
- Determinism tests executed: closed issue scan, merged PR evidence scan, targeted stale lifecycle `rg` scan, and SOR contract validation loop.
- Fixtures or scripts used: GitHub issue/PR metadata from `gh`, tracked SOR files under `docs/records/v0.87.1/tasks/`, and `adl/tools/validate_structured_prompt.sh`.
- Replay verification (same inputs -> same artifacts/order): the corrected issue list and hygiene index are sorted by issue number, and the stale lifecycle scan can be rerun against the branch to reproduce the absence of targeted stale states.
- Ordering guarantees (sorting / tie-break rules used): issue and SOR lists are sorted numerically by issue number; when multiple merged PRs matched an issue, the record fixes used the direct issue branch PR for the target issue.
- Artifact stability notes: SOR text changes are source-controlled and deterministic for the current GitHub closed/merged state.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: no secret-bearing files were read or written; this records-only pass did not touch provider keys, generated runtime secret surfaces, or local credential stores.
- Prompt / tool argument redaction verified: commands recorded use issue numbers, repository-relative paths, and placeholders only; no sensitive prompt or credential material is included.
- Absolute path leakage check: tracked artifacts use repository-relative paths; worktree absolute paths are not recorded in the corrected SOR content or hygiene report.
- Sandbox / policy invariants preserved: all edits were bounded to tracked documentation/record surfaces in the issue worktree.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this records hygiene pass did not run an ADL workflow that emits runtime traces.
- Run artifact root: not applicable; no runtime artifact root was produced.
- Replay command used for verification: rerun the closed issue scan, stale lifecycle `rg` scan, and SOR validation loop listed in Validation.
- Replay result: PASS for the tracked SOR lifecycle scan and SOR validation loop.

## Artifact Verification
- Primary proof surface: `docs/records/v0.87.1/CLOSED_ISSUE_RECORD_HYGIENE_1546.md` and the corrected tracked SORs under `docs/records/v0.87.1/tasks/`.
- Required artifacts present: true; the hygiene report exists and all corrected tracked SOR files exist on the issue branch.
- Artifact schema/version checks: all tracked v0.87.1 SOR files pass `validate_structured_prompt.sh --type sor --phase output` after correction.
- Hash/byte-stability checks: not required; source-controlled diff review and contract validation are the stability checks for this docs-only pass.
- Missing/optional artifacts and rationale: missing tracked SORs for closed issues are listed in the hygiene report and intentionally not invented without sufficient evidence.

## Decisions / Deviations

- Proceeded past the doctor preflight open-PR wave block with `--allow-open-pr-wave` because the issue was structurally ready and the blocker was unrelated to this closed-record hygiene pass.
- Did not create new SORs for closed issues with no tracked SOR surface; those are recorded as missing/deferred until enough evidence can be recovered.
- Preserved original validation evidence in corrected SORs rather than retroactively rewriting validation narratives.

## Follow-ups / Deferred work

- Backfill missing tracked SORs listed in `docs/records/v0.87.1/CLOSED_ISSUE_RECORD_HYGIENE_1546.md` only when sufficient source evidence is available.
- Consider adding a dedicated batch closeout/hygiene skill if this pattern recurs often enough to justify automation.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
