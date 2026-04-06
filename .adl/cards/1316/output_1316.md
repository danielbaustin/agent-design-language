# distribute-remaining-tbd-roadmap-docs-to-roadmap-aligned-milestones

Canonical Template Source: `adl/templates/cards/output_card_template.md`
Consumed by: `adl/tools/pr.sh` (`OUTPUT_TEMPLATE`) with legacy fallback support for `.adl/templates/output_card_template.md`.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-1316
Run ID: issue-1316
Version: v0.87
Title: [v0.87][docs] Distribute remaining TBD roadmap docs to roadmap-aligned milestones
Branch: codex/1316-distribute-remaining-tbd-roadmap-docs-to-roadmap-aligned-milestones
Status: DONE

Execution:
- Actor: codex
- Model: gpt-5
- Provider: openai
- Start Time: 2026-04-05
- End Time: 2026-04-05

## Summary

Redistributed the active roadmap/feature docs from TBD into milestone-aligned planning directories, introduced the new `v0.87.1planning` runtime-completion band, and reconciled the two roadmap-map docs so they now agree on milestone ownership, MTT placement, skills ownership, and the OSS-vs-enterprise identity/security boundary.

## Artifacts produced
- `.adl/docs/TBD/FEATURE_SPRINT_MAP.md`
- `.adl/docs/TBD/NEW_FEATURE_MAP.md`
- `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT.md`
- `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
- `.adl/docs/v0.87.1planning/AGENT_LIFECYCLE.md`
- `.adl/docs/v0.87.1planning/EXECUTION_BOUNDARIES.md`
- `.adl/docs/v0.87.1planning/LOCAL_RUNTIME_RESILIENCE.md`
- `.adl/docs/v0.87.1planning/SHEPHERD_RUNTIME_MODEL.md`
- `.adl/docs/v0.88planning/CHRONOSENSE_AND_IDENTITY.md`
- `.adl/docs/v0.88planning/TEMPORAL_SCHEMA_V01.md`
- `.adl/docs/v0.89planning/GHB_EXECUTION_MODEL.md`
- `.adl/docs/v0.89planning/GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md`
- `.adl/docs/v0.89planning/REASONING_PATTERNS_CATALOG.md`
- `.adl/docs/v0.92planning/CAPABILITY_MODEL.md`
- `.adl/docs/v0.92planning/CONTINUITY_VALIDATION.md`
- `.adl/docs/v0.92planning/CONTINUITY_VALIDATION_SCHEMA.md`
- `.adl/docs/v0.92planning/FORK_JOIN_AND_IDENTITY.md`
- `.adl/docs/v0.93planning/COGNITIVE_ETHICS.md`
- `.adl/docs/v0.93planning/A_LA_RECHERCHE_DU_TEMPS_PERDU_MENTAL_TIME_TRAVEL_MTT_V1.md`
- `.adl/docs/v0.94planning/CBAC_ARCHITECTURE.md`
- `.adl/docs/v0.94planning/POLICY_ENGINE.md`
- `.adl/docs/v0.94planning/PROVIDER_TRUST_AND_ISOLATION_ARCHITECTURE.md`
- `.adl/docs/v0.94planning/SANDBOX_RUNTIME_ISOLATION_ARCHITECTURE.md`
- `.adl/docs/v0.94planning/SECRETS_AND_DATA_GOVERNANCE.md`
- `.adl/docs/v0.94planning/SECURITY_MODEL_PLANNING.md`
- `.adl/docs/v0.94planning/SECURE_EXECUTION_MODEL.md`
- `.adl/docs/v0.94planning/IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md`

## Actions taken
- rebased the `1316` worktree onto `origin/main`
- created the missing planning-home directories needed for redistribution
- copied the active TBD feature/architecture docs into their milestone planning homes instead of promoting them publicly
- reconciled `FEATURE_SPRINT_MAP.md` and `NEW_FEATURE_MAP.md` into one consistent milestone story
- added the janitor-skill location note so it is treated as part of `1299` rather than a missing TBD doc
- added explicit scope notes to `SECURE_EXECUTION_MODEL.md` and `IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md` so their OSS-vs-enterprise boundary is clear without physically splitting the docs
- force-added the `.adl` doc set because the repo ignore rules would otherwise hide these planning artifacts from git

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: none
- Worktree-only paths remaining:
  - `.adl/docs/TBD/FEATURE_SPRINT_MAP.md`
  - `.adl/docs/TBD/NEW_FEATURE_MAP.md`
  - `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT.md`
  - `.adl/docs/v0.87.1planning/ADL_RUNTIME_ENVIRONMENT_ARCHITECTURE.md`
  - `.adl/docs/v0.87.1planning/AGENT_LIFECYCLE.md`
  - `.adl/docs/v0.87.1planning/EXECUTION_BOUNDARIES.md`
  - `.adl/docs/v0.87.1planning/LOCAL_RUNTIME_RESILIENCE.md`
  - `.adl/docs/v0.87.1planning/SHEPHERD_RUNTIME_MODEL.md`
  - `.adl/docs/v0.88planning/CHRONOSENSE_AND_IDENTITY.md`
  - `.adl/docs/v0.88planning/TEMPORAL_SCHEMA_V01.md`
  - `.adl/docs/v0.89planning/GHB_EXECUTION_MODEL.md`
  - `.adl/docs/v0.89planning/GHB_ALGORITHM_AND_STATE_SPACE_COMPRESSION.md`
  - `.adl/docs/v0.89planning/REASONING_PATTERNS_CATALOG.md`
  - `.adl/docs/v0.92planning/CAPABILITY_MODEL.md`
  - `.adl/docs/v0.92planning/CONTINUITY_VALIDATION.md`
  - `.adl/docs/v0.92planning/CONTINUITY_VALIDATION_SCHEMA.md`
  - `.adl/docs/v0.92planning/FORK_JOIN_AND_IDENTITY.md`
  - `.adl/docs/v0.93planning/COGNITIVE_ETHICS.md`
  - `.adl/docs/v0.93planning/A_LA_RECHERCHE_DU_TEMPS_PERDU_MENTAL_TIME_TRAVEL_MTT_V1.md`
  - `.adl/docs/v0.94planning/CBAC_ARCHITECTURE.md`
  - `.adl/docs/v0.94planning/POLICY_ENGINE.md`
  - `.adl/docs/v0.94planning/PROVIDER_TRUST_AND_ISOLATION_ARCHITECTURE.md`
  - `.adl/docs/v0.94planning/SANDBOX_RUNTIME_ISOLATION_ARCHITECTURE.md`
  - `.adl/docs/v0.94planning/SECRETS_AND_DATA_GOVERNANCE.md`
  - `.adl/docs/v0.94planning/SECURITY_MODEL_PLANNING.md`
  - `.adl/docs/v0.94planning/SECURE_EXECUTION_MODEL.md`
  - `.adl/docs/v0.94planning/IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local redistribution in the issue worktree, staged and prepared for commit/push/PR
- Verification performed:
  - `git status --short` to verify the staged redistribution set and absence of unrelated changes
  - `git diff --cached --stat` to verify the expected file set and scope of the redistribution
  - `rg -n "PR_JANITOR_SKILL_INPUT_SCHEMA|promotion into docs/milestones|sentience, continuity|Scope note:" ...` to verify the key policy notes and split-boundary annotations
- Result: PASS; the branch contains the intended redistribution set and the policy notes required for execution from `1316`

## Validation
- Validation commands and their purpose:
  - `git -C .worktrees/adl-wp-1316 rebase origin/main` to ensure the redistribution starts from current mainline history
  - `find .worktrees/adl-wp-1316/.adl/docs -maxdepth 2 -type f` to verify the copied milestone-planning surfaces exist in the worktree
  - `git -C .worktrees/adl-wp-1316 diff --cached --stat` to verify the exact redistributed file set
  - `rg -n "PR_JANITOR_SKILL_INPUT_SCHEMA|promotion into docs/milestones|sentience, continuity|Scope note:" ...` to verify the key editorial policy changes
- Results:
  - rebase succeeded cleanly
  - redistributed docs exist in the expected planning directories
  - staged diff is limited to the intended roadmap and planning-doc redistribution set
  - policy notes and split-boundary scope notes are present

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "git rebase origin/main"
      - "find .adl/docs milestone planning paths"
      - "git diff --cached --stat"
      - "rg policy/scope-note checks"
  determinism:
    status: NOT_RUN
    replay_verified: unknown
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
- Determinism tests executed: not applicable for this docs-only redistribution pass
- Fixtures or scripts used: not applicable; this run moved and reconciled planning docs rather than executing a deterministic runtime artifact flow
- Replay verification (same inputs -> same artifacts/order): not run
- Ordering guarantees (sorting / tie-break rules used): not applicable
- Artifact stability notes: the moved docs are direct copies into milestone planning homes with small editorial scope notes added where required

## Security / Privacy Checks
- Secret leakage scan performed: manual doc-surface check during review; no secrets or tokens were introduced
- Prompt / tool argument redaction verified: the redistributed planning docs and map docs do not record prompts or tool arguments
- Absolute path leakage check: verified final artifact references in this record use repository-relative paths
- Sandbox / policy invariants preserved: yes; all work stayed within repository docs and did not require elevated filesystem access

## Replay Artifacts
- Trace bundle path(s): not applicable for docs-only work
- Run artifact root: not applicable for docs-only work
- Replay command used for verification: not applicable
- Replay result: not applicable

## Artifact Verification
- Primary proof surface: the staged 1316 doc set in `.adl/docs/TBD/` and `.adl/docs/v0.*planning/`
- Required artifacts present: yes; all planned milestone-home docs listed in the reconciled maps were copied into the branch
- Artifact schema/version checks: not applicable; no runtime artifact schema changed
- Hash/byte-stability checks: not applicable for docs-only redistribution
- Missing/optional artifacts and rationale:
  - `PR_JANITOR_SKILL_INPUT_SCHEMA.md` was not moved from `TBD` because it already lives under `.adl/docs/skills/` and is explicitly treated as part of `1299`

## Decisions / Deviations
- Used milestone planning directories rather than public milestone feature directories because the redistribution policy is planning-first and public promotion happens only when a milestone opens
- Did not physically split `SECURE_EXECUTION_MODEL.md` or `IDENTITY_AND_AUTHENTICATION_ARCHITECTURE.md`; instead, added explicit scope notes so the dominant-home placement is clear without premature doc surgery
- Force-added `.adl` files because `.gitignore` would otherwise hide the redistributed planning artifacts from git

## Follow-ups / Deferred work
- Open or expand the `v0.87` skills work under `1299` so the janitor schema joins the bounded PR-process skill family
- Decide later whether the two split-boundary docs need physical splitting after the roadmap bands mature
- After milestone openings, selectively promote the relevant planning docs into `docs/milestones/.../features/` when you intentionally want them public

Global rule:
- No section header may be left empty.
- If a field is included, it must contain either concrete content or a one-line justification for why it does not apply.
