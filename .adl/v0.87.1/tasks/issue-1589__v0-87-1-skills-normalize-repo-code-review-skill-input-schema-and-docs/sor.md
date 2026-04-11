# v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1589
Run ID: issue-1589
Version: v0.87.1
Title: [v0.87.1][skills] Normalize repo-code-review skill input schema and docs
Branch: codex/1589-v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs
Status: DONE

Execution:
- Actor: Codex
- Model: gpt-5-codex
- Provider: OpenAI Codex desktop
- Start Time: 2026-04-11T16:49:00Z
- End Time: 2026-04-11T17:02:13Z

## Summary
Normalized the `repo-code-review` skill to the same typed-input standard as the newer PR-phase and editor skills. The bundle now declares a structured input schema in the manifest, ships a canonical schema reference document, and has a dedicated contract test plus updated operator guidance.

## Artifacts produced
- Updated `adl/tools/skills/repo-code-review/adl-skill.yaml` to declare `repo_code_review.v1` structured input admission rules and explicit mode/policy validation.
- Added `adl/tools/skills/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md` as the canonical input-schema reference for the skill.
- Updated `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md` so human/operator usage matches the new typed invocation surface.
- Added `adl/tools/test_repo_code_review_skill_contracts.sh` to prove the manifest, schema doc, and guide stay in sync.

## Actions taken
- Reviewed the existing `repo-code-review` skill bundle, compared it against schema-backed skills already in the repository, and confirmed the gap was input-contract formalization rather than output-contract coverage.
- Added an `admission.input_schema` section to the skill manifest with a stable schema id, required top-level fields, supported modes, and explicit prevalidation rules.
- Authored a bounded schema document that defines the canonical structured invocation shape, per-mode requirements, and validation expectations for automation.
- Updated the operational guide so user-facing instructions point to the schema-backed invocation path instead of prose-only inputs.
- Added and ran a dedicated contract test to keep the manifest, schema doc, and guide aligned over time.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.adl/v0.87.1/tasks/issue-1589__v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs/sor.md`
- Worktree-only paths remaining: none
- Integration state: merged
- Verification scope: main_repo
- Integration method used: normalized the canonical root SOR directly on `main` after verifying the issue is already closed and linked to merged PR `#1590`
- Verification performed:
  - `gh issue view 1589 --json title,url,state,stateReason,closedByPullRequestsReferences`
    - verified the issue is closed and captured the final closure metadata used for this normalization pass
  - `gh pr view 1590 --json state,url`
    - verified the linked closing PR remains available as the final publication surface
  - `ls .adl/v0.87.1/tasks/issue-1589__v0-87-1-skills-normalize-repo-code-review-skill-input-schema-and-docs/sor.md`
    - verified the canonical root SOR path exists on the main repository path
- Result: PASS

## Validation
- Validation commands and their purpose:
  - `bash adl/tools/test_repo_code_review_skill_contracts.sh` verified that the repo review skill manifest, schema doc, and operational guide expose the same structured input contract.
  - `bash adl/tools/test_repo_review_contract.sh` verified the existing repo review output-contract fixtures still pass after the input-schema normalization.
  - `git diff --check` verified there are no whitespace errors or malformed patch artifacts in the touched files.
- Results: PASS

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
      - "bash adl/tools/test_repo_code_review_skill_contracts.sh"
      - "bash adl/tools/test_repo_review_contract.sh"
      - "git diff --check"
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
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: repeated contract-oriented shell validations over the same manifest and documentation inputs.
- Fixtures or scripts used: `adl/tools/test_repo_code_review_skill_contracts.sh` and `adl/tools/test_repo_review_contract.sh`.
- Replay verification (same inputs -> same artifacts/order): rerunning the contract checks with unchanged inputs continues to produce the same PASS/FAIL disposition and validate the same bounded artifact set.
- Ordering guarantees (sorting / tie-break rules used): the new input schema fixes the top-level contract shape and per-mode required fields, so accepted structured inputs are validated against a stable field set rather than ad hoc prose ordering.
- Artifact stability notes: the touched artifacts are tracked text files with no generated timestamps or randomized content, so identical accepted edits produce stable repository content.

Rules:
- If deterministic fixtures or scripts are used, describe them as determinism evidence rather than merely listing them.
- State what guarantee is being proven (for example byte-for-byte equality, stable ordering, or stable emitted record content).
- If a script or fixture can be rerun to reproduce the same result, that counts as replay and should be described that way.

## Security / Privacy Checks
- Secret leakage scan performed: manual review of the new schema doc, manifest rules, and contract test confirmed no secrets, tokens, or credential-bearing examples were introduced.
- Prompt / tool argument redaction verified: the structured examples are bounded to schema fields and repository-relative commands; no user prompt transcripts or sensitive tool arguments are recorded.
- Absolute path leakage check: checked the final SOR and touched tracked docs/scripts for unjustified host-specific paths; only the manifest `reference_doc` uses the repository's canonical absolute path format already established by the newer skills.
- Sandbox / policy invariants preserved: the change is limited to skill metadata, documentation, and shell validation, with no widening of runtime permissions or file-write scope.

Rules:
- State what was checked and how it was checked.
- Do not leave any field blank; if a check truly does not apply, give a one-line reason.

## Replay Artifacts
- Trace bundle path(s): not applicable; this normalization work did not generate a separate trace bundle beyond the tracked skill artifacts and validation commands.
- Run artifact root: not applicable; proof is carried by the updated tracked files and contract-test results.
- Replay command used for verification: `bash adl/tools/test_repo_code_review_skill_contracts.sh` and `bash adl/tools/test_repo_review_contract.sh`
- Replay result: PASS; both contract checks are reproducible on the current branch state.

## Artifact Verification
- Primary proof surface: `adl/tools/skills/repo-code-review/adl-skill.yaml`, `adl/tools/skills/docs/REPO_CODE_REVIEW_SKILL_INPUT_SCHEMA.md`, `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`, and `adl/tools/test_repo_code_review_skill_contracts.sh`
- Required artifacts present: yes; the manifest, schema doc, operator guide updates, and dedicated contract test are all present in the worktree.
- Artifact schema/version checks: the manifest now declares `repo_code_review.v1`, and the schema doc plus guide reference the same id and structured top-level shape.
- Hash/byte-stability checks: not run separately; this issue relies on tracked-text review plus deterministic contract validation rather than emitted binary artifacts.
- Missing/optional artifacts and rationale: no additional reference docs were needed because the existing review playbook and output contract already matched repository truth.

## Decisions / Deviations
- Normalized the input side of `repo-code-review` without changing its review-playbook or output-contract semantics, keeping the scope tightly bounded to schema/documentation parity.
- Preserved the manifest's canonical absolute `reference_doc` style so the repo review skill matches the path conventions already used by the newer skill bundles.

## Follow-ups / Deferred work
- `pr finish` still needs to publish the branch and open the PR so the Main Repo Integration section can be normalized from `worktree_only` to the actual PR state.
- If we want stricter machine enforcement later, the repo review skill could grow a typed validator that consumes `repo_code_review.v1` directly at invocation time rather than relying on manifest/schema alignment plus contract tests.

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
