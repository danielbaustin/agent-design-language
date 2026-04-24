---
name: repo-review-docs
description: Specialist docs reviewer for a multi-agent repository review. Use when a review packet needs a bounded documentation role focused on misleading docs, stale commands, onboarding gaps, release and demo truth drift, API/CLI contract drift, and whether documentation overclaims behavior without editing docs.
---

# Repo Review Docs

Review documentation truth and usability as one specialist in the multi-agent
repo review suite.

This skill focuses on docs as operational surfaces. It is not a copyediting
role unless copy clarity affects real user or reviewer behavior.

## Quick Start

1. Confirm the repo, branch, path, diff, or review packet scope.
2. Identify docs that make operational claims: README, milestone docs, CLI
   commands, release notes, demos, schemas, and onboarding guides.
3. Compare docs against repo-visible commands, files, and behavior.
4. Emit findings first, with stale command or truth-drift evidence.
5. Hand the artifact to `repo-review-synthesis` for cross-role assembly.

## Focus

Prioritize:
- stale commands and broken paths
- docs that claim behavior not present in code, tests, or artifacts
- onboarding gaps that prevent a reviewer or operator from reproducing work
- demo, release, milestone, and closeout truth drift
- API, CLI, schema, or skill-contract documentation drift
- ambiguity that can cause unsafe operation or wrong workflow execution

Defer primary ownership of these areas to other specialists:
- executable defects: `repo-review-code`
- security exploitability: `repo-review-security`
- missing tests: `repo-review-tests`
- final dedupe and ordering: `repo-review-synthesis`

## Required Inputs

At minimum, gather:
- `repo_root`
- one concrete target:
  - `target.target_path`
  - `target.branch`
  - `target.diff_base`
  - `target.review_packet_path`

Useful additional inputs:
- `changed_paths`
- `claimed_commands`
- `demo_docs`
- `release_docs`
- `validation_mode`

If there is no concrete repo or slice target, stop and report `blocked`.

## Output Expectations

Default output should include:
- findings first
- reviewed docs surfaces
- commands or claims checked
- validation performed or not run
- residual docs risk

Use the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` when ADL expects
a structured artifact.

## Required Review Outputs

For each scoped review packet, this specialist is expected to identify and list:
- documentation objects inspected (for example schemas, skills, manifests,
  runbooks, demos, and release surfaces),
- evidence for each major claim that is checked, and
- skipped-object rationale when a bounded docs lane is intentionally not run.

All doc findings must preserve severity and role ownership exactly as discovered,
matching the severity framing used by other lanes.

## Stop Boundary

Stop after producing the docs-review artifact.

Do not:
- rewrite documentation
- spend the review on style nits when truth drift exists
- claim demo/release readiness without proof
- hide docs findings that affect operator safety or reviewer reproducibility
