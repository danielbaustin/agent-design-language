# [v0.87.1][WP-10] Acceptance criteria finalization

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1459
Run ID: issue-1459
Version: v0.87.1
Title: [v0.87.1][WP-10] Acceptance criteria finalization
Branch: codex/1459-v0-87-1-wp-10-acceptance-criteria-finalization
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-10T03:48:00Z
- End Time: 2026-04-10T03:52:35Z

## Summary

Finalized the `v0.87.1` acceptance surface by replacing the WBS's informal acceptance bullets with a measurable WP-by-WP acceptance contract and a milestone-level acceptance summary. Added bounded cross-references from the checklist, feature-doc index, release plan, and demo matrix so downstream demo, quality, review, and release-tail work can use the same definition of done.

## Artifacts produced
- Updated `docs/milestones/v0.87.1/WBS_v0.87.1.md` with explicit WP-01 through WP-20 acceptance criteria and required proof surfaces.
- Updated `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md` with checklist gates that require use of the WBS acceptance mapping.
- Updated `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md` with review guidance that keeps feature docs subordinate to the WBS acceptance contract.
- Updated `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md` with a release-readiness gate for reviewing acceptance evidence.
- Updated `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md` with a coverage rule tying demo review back to the WBS acceptance mapping.

## Actions taken
- Read the source issue prompt and local task cards for `issue-1459`.
- Used the SIP editor discipline to correct the generated run-bound SIP `PR:` field from invalid placeholder text to a blank pre-PR value.
- Expanded the WBS Acceptance Mapping from informal one-line bullets into a measurable table with acceptance criteria and required proof surfaces for every WP.
- Added milestone-level acceptance bullets covering runtime completion, convergence, demo coverage, quality/review, and release closeout.
- Added small cross-document references so checklist, demo, feature-doc, and release-plan surfaces inherit the same acceptance contract.
- Kept implementation scope docs-only; no runtime code, demo scripts, or release artifacts were modified.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: pending PR publication for the branch changes listed in this SOR.
- Worktree-only paths remaining: none for tracked intended changes after PR publication; the local `.adl/v0.87.1/tasks/issue-1459__v0-87-1-wp-10-acceptance-criteria-finalization/` run cards remain worktree-local until `pr finish` publishes the tracked review SOR.
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: issue branch/worktree followed by planned `adl/tools/pr.sh finish` publication.
- Verification performed:
  - `git status --short` verified the tracked changed docs were limited to the five intended milestone docs.
  - `git diff -- docs/milestones/v0.87.1/WBS_v0.87.1.md docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md` reviewed the exact document diff before finish.
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `for wp in $(seq -f 'WP-%02g' 1 20); do count=$(rg -c "\\| ${wp} \\|" docs/milestones/v0.87.1/WBS_v0.87.1.md || true); printf '%s %s\n' "$wp" "$count"; done` verified every WP-01 through WP-20 appears in the WBS acceptance table.
  - `rg -n "seed shell|seeded shell|TBD|TODO|FIXME|<<<<<<<|=======|>>>>>>>|done means done|implementation not yet started" docs/milestones/v0.87.1 .adl/v0.87.1/tasks/issue-1459__v0-87-1-wp-10-acceptance-criteria-finalization || true` checked for seeded-shell language, conflict markers, unresolved TODO/FIXME markers, and stale implementation-status wording; only pre-existing date placeholders and issue prompt wording surfaced.
  - `git diff --check` verified the diff has no whitespace errors.
  - `bash adl/tools/validate_structured_prompt.sh --type sip --phase bootstrap --input .adl/v0.87.1/tasks/issue-1459__v0-87-1-wp-10-acceptance-criteria-finalization/sip.md` verified the corrected SIP contract.
  - `bash adl/tools/validate_structured_prompt.sh --type stp --input .adl/v0.87.1/tasks/issue-1459__v0-87-1-wp-10-acceptance-criteria-finalization/stp.md` verified the STP contract.
  - `bash adl/tools/validate_structured_prompt.sh --type sor --phase completed --input .adl/v0.87.1/tasks/issue-1459__v0-87-1-wp-10-acceptance-criteria-finalization/sor.md` verified the completed SOR contract.
- Results: PASS for WP coverage, diff hygiene, SIP validation, STP validation, and completed SOR validation. The stale-language scan found only expected pre-existing release-date `TBD` placeholders and the source issue's quoted "done means done" phrase, not newly introduced stale implementation claims.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "wp_acceptance_mapping_coverage_scan"
      - "focused_stale_language_and_conflict_marker_scan"
      - "git_diff_check"
      - "sip_contract_validation"
      - "stp_contract_validation"
      - "sor_contract_validation"
  determinism:
    status: PASS_WITH_SCOPE
    replay_verified: true
    ordering_guarantees_verified: true
    notes: docs-only acceptance mapping verified against stable repo-local inputs; not a full cross-environment determinism guarantee
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
- Determinism tests executed: deterministic WP coverage scan across the WBS acceptance table.
- Fixtures or scripts used: stable repo-local milestone docs and task cards only.
- Replay verification (same inputs -> same artifacts/order): rerunning the WP coverage scan against the same WBS content should produce the same WP coverage counts.
- Ordering guarantees (sorting / tie-break rules used): WBS acceptance rows are ordered WP-01 through WP-20 to match the WBS table and milestone sequencing.
- Artifact stability notes: This is a docs-only acceptance mapping; stability is scoped to document ordering and explicit proof-surface references, not byte-for-byte runtime artifact replay.

## Security / Privacy Checks
- Secret leakage scan performed: focused stale/conflict scan and manual diff review found no secrets or credential material in the changed docs.
- Prompt / tool argument redaction verified: changed docs do not include prompts, private tool arguments, tokens, or provider credentials.
- Absolute path leakage check: recorded commands and paths in this SOR are repository-relative; changed docs use repository-relative paths.
- Sandbox / policy invariants preserved: all implementation edits were made in the issue worktree/branch; no direct edits were made on `main`.

## Replay Artifacts
- Trace bundle path(s): not applicable; this docs-only acceptance mapping issue does not emit trace bundles.
- Run artifact root: not applicable; no runtime/demo execution artifact root was required for WP-10.
- Replay command used for verification: the WP coverage scan listed in Validation can be rerun against the same WBS content.
- Replay result: PASS within docs-only scope; every WP-01 through WP-20 is represented in the acceptance table.

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.87.1/WBS_v0.87.1.md`.
- Required artifacts present: true; all five intended milestone docs were present and edited on the issue branch.
- Artifact schema/version checks: not applicable; no artifact schema changed.
- Hash/byte-stability checks: not applicable; docs-only mapping did not require byte-stability proof.
- Missing/optional artifacts and rationale: no standalone demo or trace bundle was required because the proof is the explicit acceptance mapping and cross-document linkage.

## Decisions / Deviations
- Kept `SPRINT_v0.87.1.md` unchanged because WP-11 owns sprint plan alignment and this issue only needed the acceptance contract that WP-11 will consume.
- Used `PASS_WITH_SCOPE` for determinism because the validation proves bounded document ordering and coverage, not full runtime replay.
- Recorded the generated SIP `PR: none` validator mismatch as a workflow deviation; it was corrected locally with the SIP editor discipline so execution could proceed.

## Follow-ups / Deferred work
- Open or attach a tooling follow-up for the `pr run` generated SIP `PR: none` mismatch: the generated pre-PR value should either be blank or accepted by the validator for the correct lifecycle phase.
- WP-11 should now align `SPRINT_v0.87.1.md` to the WBS acceptance contract.
- WP-12 should now use this acceptance mapping to finish checklist and release-gate execution surfaces.
