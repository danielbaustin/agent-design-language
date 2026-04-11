# hide-compromised-issues-1609-1626-and-preserve-only-approved-survivors

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1634
Run ID: issue-1634
Version: v0.87.1
Title: [v0.87.1][security] Hide compromised issues 1609-1626 and preserve only approved survivors
Branch: codex/1634-hide-compromised-issues-1609-1626-and-preserve-only-approved-survivors
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5.4
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-11T22:45:00Z
- End Time: 2026-04-11T23:09:49Z

## Summary

Completed the security/process containment pass for the compromised public issue range `#1609` through `#1626`: deleted the approved public issues from that range except for survivors `#1614` and `#1618`, created a protected local preservation map under `.adl/docs/security/REDACTED_BACKLOG_1609_1626.md`, and scrubbed tracked `v0.88` milestone docs so they no longer advertise deleted issue numbers like `#1609`.

## Artifacts produced
- `.adl/docs/security/REDACTED_BACKLOG_1609_1626.md`
- updated `docs/milestones/v0.88/README.md`
- updated `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
- updated `docs/milestones/v0.88/WBS_v0.88.md`
- updated `docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`
- updated `docs/milestones/v0.88/DECISIONS_v0.88.md`

## Actions taken
- deleted public issues `#1609`, `#1610`, `#1611`, `#1612`, `#1613`, `#1615`, `#1616`, `#1617`, `#1619`, `#1622`, `#1623`, `#1624`, `#1625`, and `#1626`
- preserved the removed-range planning truth locally in `.adl/docs/security/REDACTED_BACKLOG_1609_1626.md`
- retained only public survivors `#1614` and `#1618` in the removed issue range
- updated tracked `v0.88` milestone docs to replace deleted-issue references with neutral protected-planning wording

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `docs/milestones/v0.88/README.md`
  - `docs/milestones/v0.88/FEATURE_DOCS_v0.88.md`
  - `docs/milestones/v0.88/WBS_v0.88.md`
  - `docs/milestones/v0.88/RELEASE_NOTES_v0.88.md`
  - `docs/milestones/v0.88/DECISIONS_v0.88.md`
- Worktree-only paths remaining:
  - `.adl/docs/security/REDACTED_BACKLOG_1609_1626.md`
  - `.adl/v0.87.1/tasks/issue-1634__hide-compromised-issues-1609-1626-and-preserve-only-approved-survivors/*`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: tracked docs edited in bound worktree branch; local preservation surfaces retained under ignored `.adl`
- Verification performed:
  - `gh issue view <n>` to confirm deleted issues are no longer public and survivors remain
  - `rg -n '#(1609|1610|1611|1612|1613|1615|1616|1617|1619|1622|1623|1624|1625|1626)\\b|issue-(1609|1610|1611|1612|1613|1615|1616|1617|1619|1622|1623|1624|1625|1626)\\b' docs -S` to verify tracked docs no longer expose deleted issue references
  - `git diff --check` to verify the tracked patch is clean
  - `git status --short` to verify the change set stays bounded to the intended tracked files
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
  - `gh issue delete <n> --yes` removed the approved public issues in the compromised range
  - `gh issue view <n> --json number,title,state` verified survivors and merged PR history still visible while deleted issues are missing
  - `rg -n '#(1609|1610|1611|1612|1613|1615|1616|1617|1619|1622|1623|1624|1625|1626)\\b|issue-(1609|1610|1611|1612|1613|1615|1616|1617|1619|1622|1623|1624|1625|1626)\\b' docs -S` verified tracked docs do not advertise deleted issue numbers
  - `git diff --check` verified the tracked patch is whitespace-clean
  - `bash adl/tools/pr.sh doctor 1634` verified the issue bundle remains structurally ready after prompt/STP refinement
- Results:
  - deletion/visibility verification: PASS
  - tracked-doc scrub verification: PASS
  - diff cleanliness: PASS
  - doctor readiness: PASS with expected preflight block due open PR wave (`#1629`)

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
      - "gh issue delete / gh issue view verification"
      - "tracked-doc issue-reference scrub grep"
      - "git diff --check"
      - "doctor 1634"
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
- Determinism tests executed: none beyond bounded verification commands; this issue is a security/process cleanup rather than a runtime determinism change
- Fixtures or scripts used: repeated `gh issue view` checks over the same issue range and deterministic grep over tracked docs
- Replay verification (same inputs -> same artifacts/order): not applicable; GitHub deletion is state-changing, but verification commands returned stable survivor/deletion state after completion
- Ordering guarantees (sorting / tie-break rules used): explicit numeric issue range `1609` through `1626`
- Artifact stability notes: the preservation map is a static local record of the redacted range and the tracked docs now use neutral wording rather than deleted issue numbers

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: no new secrets or credentials introduced; issue deletions and docs edits only
- Prompt / tool argument redaction verified: tracked docs do not expose deleted issue numbers from the compromised range after the scrub
- Absolute path leakage check: tracked docs patch contains no host-path additions; local `.adl` preservation doc remains intentionally local-only
- Sandbox / policy invariants preserved: yes; all work stayed within repo, GitHub issue metadata, and local planning surfaces

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable; this issue is a process/security containment change, not a runtime replay surface

## Artifact Verification
- Primary proof surface: reduced public issue surface plus scrubbed tracked docs and the local preservation map
- Required artifacts present: yes
- Artifact schema/version checks: not applicable beyond standard issue/task-bundle contracts already validated by `doctor`
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no additional demo artifacts required

## Decisions / Deviations

- kept merged PR history `#1620` and `#1621` visible because merged PR records are not treated like deletable open issues
- kept survivor issues `#1614` and `#1618` public by explicit direction
- preserved removed-range planning locally rather than trying to encode sensitive future intent in new public issues

## Follow-ups / Deferred work

- publish this tracked-doc scrub through the normal PR path for `#1634`
- if additional tracked/public references to the removed issue range appear later, scrub them under this same process issue or a tightly scoped follow-on

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
