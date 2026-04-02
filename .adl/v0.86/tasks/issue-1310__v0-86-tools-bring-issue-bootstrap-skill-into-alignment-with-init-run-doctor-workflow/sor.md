# v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1310
Run ID: issue-1310
Version: v0.86
Title: [v0.86][tools] Bring issue-bootstrap skill into alignment with init-run-doctor workflow
Branch: codex/1310-v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow
Status: IN_PROGRESS

Execution:
- Actor: Codex
- Model: GPT-5 Codex
- Provider: OpenAI
- Start Time: 2026-04-02T14:24:00Z
- End Time: 2026-04-02T14:42:00Z

## Summary
This run brings the issue-bootstrap skill bundle up to current workflow truth by making the skill describe mechanical bootstrap as the Step 1 concern, qualitative card review as the immediate handoff, issue-mode `pr run` as the later execution-time binder, and `doctor` as the diagnostic surface. It also removes stale reliance on ignored planning-path references and aligns the main skill file, manifest, playbook, agent metadata, and output contract.

## Artifacts produced
- Updated `.adl/skills/issue-bootstrap/SKILL.md`.
- Updated `.adl/skills/issue-bootstrap/adl-skill.yaml`.
- Updated `.adl/skills/issue-bootstrap/agents/openai.yaml`.
- Updated `.adl/skills/issue-bootstrap/references/bootstrap-playbook.md`.
- Updated `.adl/skills/issue-bootstrap/references/output-contract.md`.
- Updated issue execution surfaces for `#1310` so the worktree-local source prompt, STP, and SIP match the reviewed scope.

## Actions taken
- Created `#1310` through the canonical issue-bootstrap flow and rewrote the root issue prompt, STP, and SIP to reflect the real reviewed scope before execution.
- Bound the issue with `pr run` to create the branch and worktree at execution time.
- Materialized the ignored issue-bootstrap skill bundle into the issue worktree so the fix could live on the branch instead of only in the primary checkout.
- Reworked the skill bundle so it points at tracked feature and architecture docs, treats `create` and `init` as Step 1 bootstrap shapes, hands off to qualitative review and then issue-mode `pr run`, and preserves `doctor` as a diagnostic surface rather than a lifecycle phase.
- Synced the corrected source prompt and task cards into the started worktree after `pr run` had copied the pre-edit bootstrap bundle.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/skills/issue-bootstrap/SKILL.md`, `.adl/skills/issue-bootstrap/adl-skill.yaml`, `.adl/skills/issue-bootstrap/agents/openai.yaml`, `.adl/skills/issue-bootstrap/references/bootstrap-playbook.md`, `.adl/skills/issue-bootstrap/references/output-contract.md`
- Worktree-only paths remaining: none
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: tracked repair in the issue branch worktree, including force-staging ignored `.adl/skills` paths for PR publication.
- Verification performed:
  - `find .adl/skills/issue-bootstrap -maxdepth 2 -type f | sort` to verify the expected skill bundle surfaces exist in the worktree.
  - `rg -n "pr start|run/start|\\.adl/docs/v0.87planning/PR_TOOLING_SKILLS|later-step handoff after review" .adl/skills/issue-bootstrap` to verify stale lifecycle teaching and dead planning-path references were removed or reduced to truthful diagnostic notes.
  - `diff -u <root sip> <worktree sip>` to verify the corrected execution card was synced into the started worktree after binding.
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
  - `find .adl/skills/issue-bootstrap -maxdepth 2 -type f | sort` to verify the skill bundle contains the expected five files.
  - `rg -n "pr start|run/start|\\.adl/docs/v0.87planning/PR_TOOLING_SKILLS|later-step handoff after review" .adl/skills/issue-bootstrap` to verify no stale bootstrap-to-start teaching or dead planning-doc reference remained in the bundle.
  - `git check-ignore -v .adl/skills/issue-bootstrap/SKILL.md .adl/skills/issue-bootstrap/adl-skill.yaml .adl/skills/issue-bootstrap/references/bootstrap-playbook.md .adl/skills/issue-bootstrap/references/output-contract.md` to verify these paths are still ignored by default and therefore require intentional force-staging.
  - `diff -u /Users/daniel/git/agent-design-language/.adl/v0.86/tasks/issue-1310__v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow/sip.md /Users/daniel/git/agent-design-language/.worktrees/adl-wp-1310/.adl/v0.86/tasks/issue-1310__v0-86-tools-bring-issue-bootstrap-skill-into-alignment-with-init-run-doctor-workflow/sip.md` to verify the corrected input card was synced into the execution worktree.
- Results:
  - The issue-bootstrap bundle exists with the expected file set.
  - The bundle now teaches qualitative review followed by issue-mode `pr run`, with `doctor` preserved as a diagnostic surface.
  - The dead `.adl/docs/v0.87planning/PR_TOOLING_SKILLS.md` planning reference was removed from the skill bundle.
  - The ignored `.adl/skills` paths are intentionally outside default git tracking and must be force-added for publication.
  - The corrected execution card now matches between root and worktree copies.

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
      - "find .adl/skills/issue-bootstrap -maxdepth 2 -type f | sort"
      - "rg -n \"pr start|run/start|\\.adl/docs/v0.87planning/PR_TOOLING_SKILLS|later-step handoff after review\" .adl/skills/issue-bootstrap"
      - "git check-ignore -v .adl/skills/issue-bootstrap/SKILL.md .adl/skills/issue-bootstrap/adl-skill.yaml .adl/skills/issue-bootstrap/references/bootstrap-playbook.md .adl/skills/issue-bootstrap/references/output-contract.md"
      - "diff -u <root sip> <worktree sip>"
  determinism:
    status: PARTIAL
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
- Determinism tests executed: repeated file-discovery and text-scan checks over the same skill bundle.
- Fixtures or scripts used: `find`, `rg`, and `diff` over the bounded skill-bundle surfaces.
- Replay verification (same inputs -> same artifacts/order): rerunning the same scans over the same checkout yields the same file set and the same absence of stale planning-path references.
- Ordering guarantees (sorting / tie-break rules used): `find ... | sort` was used so the recorded bundle order is stable.
- Artifact stability notes: this issue updates markdown and YAML skill surfaces only; no runtime artifact schema changed.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review plus targeted `rg` checks over the edited skill bundle found no secrets or tokens.
- Prompt / tool argument redaction verified: the updated skill language describes workflow behavior only and does not embed sensitive prompt/tool payloads.
- Absolute path leakage check: the skill bundle still contains intentional absolute repository paths in instructional references; the output record itself avoids recording new unjustified host-path details beyond required proof commands.
- Sandbox / policy invariants preserved: the skill remains explicitly bounded to mechanical bootstrap and still forbids branch/worktree creation and implementation work.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this issue does not produce runtime trace bundles.
- Run artifact root: not applicable; this is a skill-bundle repair.
- Replay command used for verification: `find .adl/skills/issue-bootstrap -maxdepth 2 -type f | sort` and the targeted `rg` check over the same bundle.
- Replay result: PASS. The same bundle surfaces and corrected lifecycle wording are reproduced on repeated inspection.

## Artifact Verification
- Primary proof surface: the updated issue-bootstrap skill bundle under `.adl/skills/issue-bootstrap/`.
- Required artifacts present: yes.
- Artifact schema/version checks: `adl-skill.yaml` remains on the existing `version: "0.1"` schema and no new manifest fields were introduced beyond a populated `notes` required section declaration.
- Hash/byte-stability checks: not run; this issue relies on bounded text-surface verification rather than byte-hash proof.
- Missing/optional artifacts and rationale: no additional planning doc was added under `.adl/docs/v0.87planning/` because the current repo does not track that path reliably; the skill now points at tracked feature docs and its own local references instead.

## Decisions / Deviations
- The skill now treats `pr create` and `pr init` as two current command shapes for the same Step 1 bootstrap concern rather than claiming `init` alone already owns new-issue creation.
- The bundle intentionally keeps a negative reference to `pr start` only to say it should not be taught as the public execution binder.
- The ignored skill bundle had to be copied into the started worktree explicitly because `pr run` does not mirror ignored `.adl/skills` content automatically.

## Follow-ups / Deferred work
- Use this corrected bundle as the source for the next three workflow skills rather than re-deriving the lifecycle model from memory.
- If `.adl/docs/v0.87planning/` is meant to become a tracked planning corpus later, add that as a separate docs issue instead of letting skills point at ignored paths by assumption.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
