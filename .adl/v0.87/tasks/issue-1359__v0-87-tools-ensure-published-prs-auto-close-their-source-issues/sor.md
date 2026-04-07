# v0-87-tools-ensure-published-prs-auto-close-their-source-issues

Task ID: issue-1359
Run ID: issue-1359
Version: v0.87
Title: [v0.87][tools] Ensure published PRs auto-close their source issues
Branch: codex/1359-v0-87-tools-ensure-published-prs-auto-close-their-source-issues
Status: DONE

Execution:
- Actor: Codex
- Model: GPT-5
- Provider: OpenAI
- Start Time: 2026-04-06T20:10:33-0700
- End Time: 2026-04-06T20:10:33-0700

## Summary

Surfaced the missing `pr-finish` operational skill into the live `.adl` skill set and updated the operator guide so PR publication goes through the repo-native finish boundary that preserves issue-closing linkage.

## Artifacts produced

- `.adl/skills/pr-finish/SKILL.md`
- `.adl/skills/pr-finish/adl-skill.yaml`
- `.adl/skills/pr-finish/agents/openai.yaml`
- `.adl/skills/pr-finish/references/finish-playbook.md`
- `.adl/skills/pr-finish/references/output-contract.md`
- `.adl/docs/skills/PR_FINISH_SKILL_INPUT_SCHEMA.md`
- `.adl/docs/skills/OPERATIONAL_SKILLS_GUIDE.md`

## Actions taken

- Confirmed `pr.sh finish` and the Rust finish path already preserve closing linkage.
- Identified the real gap as missing surfaced `pr-finish` skill materials under `.adl/skills/` and `.adl/docs/skills/`.
- Copied the existing tracked `pr-finish` bundle into the surfaced skill location.
- Added the finish input schema doc to the surfaced skill docs.
- Updated the operational guide to include `pr-finish` in the live skill set, workflow, selector, failure modes, and recommended default chain.
- Recorded that skipping `pr-finish` is the specific failure mode that led to issue 1354 not auto-closing.

## Main Repo Integration

- Main-repo paths updated: none yet
- Worktree-only paths remaining:
  - `.adl/skills/pr-finish/SKILL.md`
  - `.adl/skills/pr-finish/adl-skill.yaml`
  - `.adl/skills/pr-finish/agents/openai.yaml`
  - `.adl/skills/pr-finish/references/finish-playbook.md`
  - `.adl/skills/pr-finish/references/output-contract.md`
  - `.adl/docs/skills/PR_FINISH_SKILL_INPUT_SCHEMA.md`
  - `.adl/docs/skills/OPERATIONAL_SKILLS_GUIDE.md`
- Integration state: pr_open
- Verification scope: worktree
- Integration method used: branch-local `.adl` edits to be published through PR
- Verification performed:
  - `rg -n "pr-finish|PR_FINISH_SKILL_INPUT_SCHEMA" .adl/docs/skills/OPERATIONAL_SKILLS_GUIDE.md .adl/docs/skills/PR_FINISH_SKILL_INPUT_SCHEMA.md .adl/skills/pr-finish`
  - `bash adl/tools/test_install_adl_operational_skills.sh`
- Result: worktree contains the intended surfaced skill and the installer test passes

## Validation

- Validation commands and their purpose:
  - `rg -n "pr-finish|PR_FINISH_SKILL_INPUT_SCHEMA" .adl/docs/skills/OPERATIONAL_SKILLS_GUIDE.md .adl/docs/skills/PR_FINISH_SKILL_INPUT_SCHEMA.md .adl/skills/pr-finish`
    - verified the guide, schema doc, and skill bundle all reference `pr-finish` consistently
  - `bash adl/tools/test_install_adl_operational_skills.sh`
    - verified the operational skill installer test still passes with the surfaced finish skill present
- Results:
  - guide references are present and consistent
  - skill bundle exists under `.adl/skills/pr-finish`
  - installer test passed

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "rg pr-finish consistency check"
      - "test_install_adl_operational_skills"
  determinism:
    status: PASS
    replay_verified: true
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
      present: true
      approved: not_applicable
```

## Determinism Evidence

- Determinism tests executed: repeated `rg` lookup against fixed file paths and installer regression script
- Fixtures or scripts used: `adl/tools/test_install_adl_operational_skills.sh`
- Replay verification (same inputs -> same artifacts/order): rerunning the installer test produced the same `PASS` result
- Ordering guarantees (sorting / tie-break rules used): not applicable for this docs-and-skill-surface change
- Artifact stability notes: the change is static file content plus a copied skill bundle

## Security / Privacy Checks

- Secret leakage scan performed: manual review of edited docs and copied skill metadata; no secrets present
- Prompt / tool argument redaction verified: no prompt or tool transcripts were added to tracked artifacts
- Absolute path leakage check: final output record uses repository-relative paths only
- Sandbox / policy invariants preserved: no destructive git operations and no merge/closeout side effects

## Replay Artifacts

- Trace bundle path(s): not applicable for this docs/process task
- Run artifact root: not applicable
- Replay command used for verification: `bash adl/tools/test_install_adl_operational_skills.sh`
- Replay result: PASS

## Artifact Verification

- Primary proof surface: `.adl/docs/skills/OPERATIONAL_SKILLS_GUIDE.md`
- Required artifacts present: yes
- Artifact schema/version checks: `PR_FINISH_SKILL_INPUT_SCHEMA.md` is present and referenced from the guide
- Hash/byte-stability checks: not applicable
- Missing/optional artifacts and rationale: no additional artifacts required

## Decisions / Deviations

- Did not modify `pr.sh finish` or Rust finish code because the repo-native finish path already preserves closing references.
- Chose the smaller fix: surface the existing finish skill and make the operator guide require it.

## Follow-ups / Deferred work

- Publish this branch with `pr-finish` so the issue closes through the corrected workflow.
- Keep issue 1332 focused on making issue bodies more human-readable and machine-usable; that is related process cleanup but not required for this fix.
