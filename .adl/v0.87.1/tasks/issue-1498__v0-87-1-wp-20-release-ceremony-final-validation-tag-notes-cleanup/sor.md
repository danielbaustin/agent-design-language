# [v0.87.1][WP-20] Release ceremony (final validation + tag + notes + cleanup)

Task ID: issue-1498
Run ID: issue-1498
Version: v0.87.1
Title: [v0.87.1][WP-20] Release ceremony (final validation + tag + notes + cleanup)
Branch: codex/1498-v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-12T16:46:45Z
- End Time: 2026-04-12T16:55:18Z

## Summary

Prepared the `v0.87.1` release-ceremony issue for final execution by adding the canonical release helper, documenting it under the tools surface, correcting stale release-tail wording in the milestone docs, and explicitly treating repo-root `artifacts/` as local internal/generated output rather than public tracked repo content. The final ceremony script, tag creation, and GitHub Release actions have not been run yet.

## Artifacts produced
- `adl/tools/release_ceremony.sh`
- updated release-tail milestone docs for `v0.87.1`
- worktree-local `#1498` execution bundle (`sip.md`, `sor.md`)

## Actions taken
- added the canonical `adl/tools/release_ceremony.sh` helper to the `#1498` branch
- documented the helper in `adl/tools/README.md`
- added repo-root `artifacts/` to `.gitignore` so internal/generated spill does not present itself as public repo content
- updated `v0.87.1` README/checklist/release-plan/release-notes wording so the docs reflect that `WP-17` through `WP-19` are closed and `WP-20` remains the active release step
- created the missing worktree-local `sip.md` and `sor.md` surfaces for issue `#1498`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated:
  - `.gitignore`
  - `adl/tools/README.md`
  - `adl/tools/release_ceremony.sh`
  - `docs/milestones/v0.87.1/README.md`
  - `docs/milestones/v0.87.1/MILESTONE_CHECKLIST_v0.87.1.md`
  - `docs/milestones/v0.87.1/RELEASE_PLAN_v0.87.1.md`
  - `docs/milestones/v0.87.1/RELEASE_NOTES_v0.87.1.md`
- Worktree-only paths remaining:
  - `.adl/v0.87.1/tasks/issue-1498__v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup/sip.md`
  - `.adl/v0.87.1/tasks/issue-1498__v0-87-1-wp-20-release-ceremony-final-validation-tag-notes-cleanup/sor.md`
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: direct edits in the issue worktree
- Verification performed:
  - `git status --short`
  - path existence checks for the touched tracked release surfaces
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `cargo fmt --manifest-path adl/Cargo.toml --all -- --check`
    - verify the repo remains formatting-clean after the shell/doc prep changes
  - `bash -n adl/tools/release_ceremony.sh`
    - verify the new release helper is syntactically valid
  - release-doc consistency review across `README`, `MILESTONE_CHECKLIST`, `RELEASE_PLAN`, and `RELEASE_NOTES`
    - verify the release-tail docs agree on the current open/closed state
- Results:
  - `cargo fmt --manifest-path adl/Cargo.toml --all -- --check` passed
  - `bash -n adl/tools/release_ceremony.sh` passed
  - manual consistency review confirmed the touched `v0.87.1` release-tail docs now reflect `#1495`, `#1496`, and `#1497` as closed and `#1498` as the remaining active release step

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml --all -- --check"
      - "bash -n adl/tools/release_ceremony.sh"
      - "manual release-tail consistency review across README/checklist/release-plan/release-notes"
  determinism:
    status: PASS
    replay_verified: false
    ordering_guarantees_verified: unknown
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
- Determinism tests executed: none yet beyond deterministic doc/script preparation
- Fixtures or scripts used: not applicable
- Replay verification (same inputs -> same artifacts/order): not applicable for this pre-ceremony docs/tooling preparation pass
- Ordering guarantees (sorting / tie-break rules used): not applicable
- Artifact stability notes: the new helper and release-tail doc edits are deterministic text changes scoped to `#1498`

## Security / Privacy Checks
- Secret leakage scan performed: none yet beyond manual review of the helper and doc changes
- Prompt / tool argument redaction verified: the helper does not record prompts or tool arguments
- Absolute path leakage check: recorded paths in this card are repository-relative
- Sandbox / policy invariants preserved: final release mutation was not run during this preparation pass

## Replay Artifacts
- Trace bundle path(s): not applicable
- Run artifact root: not applicable
- Replay command used for verification: not applicable
- Replay result: not applicable; this issue is release-preparation work, not runtime replay work

## Artifact Verification
- Primary proof surface: the aligned `v0.87.1` release-tail docs plus `adl/tools/release_ceremony.sh`
- Required artifacts present: yes; the helper and tracked release-preparation docs are present, and final ceremony outputs are intentionally deferred to the next execution step
- Artifact schema/version checks: not applicable
- Hash/byte-stability checks: not run
- Missing/optional artifacts and rationale: tag/release outputs are intentionally absent because the ceremony script has not been run yet

## Decisions / Deviations
- Kept repo-root `artifacts/` out of the `#1498` branch payload and treated it as internal/generated spill rather than a public release surface.
- Prepared the release helper and docs without running the mutating ceremony actions, per instruction.

## Follow-ups / Deferred work
- Run the final `v0.87.1` release ceremony validation pass on the `#1498` branch.
- Create/push the `v0.87.1` tag only after the final preflight is green.
- Draft/publish the GitHub Release only after final release-note validation is complete.

Closes #1498
