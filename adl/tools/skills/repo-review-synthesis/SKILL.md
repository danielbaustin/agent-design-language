---
name: repo-review-synthesis
description: Synthesis reviewer for a multi-agent repository review. Use when specialist code, security, test, and docs review artifacts need to be merged into one findings-first repo review without hiding severity, disagreement, missing coverage, residual risk, or role-specific caveats, and without claiming merge approval or remediation.
---

# Repo Review Synthesis

Merge specialist review artifacts into one findings-first repository review.

This skill is the final assembly role for the multi-agent repo review suite. It
does not replace the specialists and must not dilute their findings.

## Quick Start

1. Confirm the specialist artifacts and target repo/slice.
2. Preserve each finding's source role, severity, file reference, and rationale.
3. Deduplicate only when findings truly describe the same behavioral risk.
4. Surface disagreements, missing specialist artifacts, and residual risk.
5. Emit the final review packet and stop before remediation or approval claims.

## Focus

Prioritize:
- severity-preserving merge of specialist findings
- dedupe without losing role context
- explicit disagreement and uncertainty
- reviewer-friendly ordering by impact
- coverage and validation summary across all roles
- residual risks and recommended follow-up issues

Do not treat synthesis as:
- a new monolithic review pass
- a vote that can erase a specialist finding
- a remediation plan that silently edits code
- merge approval

## Required Inputs

At minimum, gather:
- `repo_root`
- `target.specialist_artifacts`

Useful additional inputs:
- `target.target_path`
- `target.branch`
- `target.diff_base`
- `artifact_root`
- `required_roles`
- `severity_policy`

If there are no specialist artifacts, stop and report `blocked`; use
`repo-code-review` for a monolithic review instead.

## Output Expectations

Default output should include:
- findings first
- specialist coverage matrix
- dedupe and disagreement notes
- validation performed across roles
- residual risk
- recommended follow-up issues

Use the shared suite contract in
`adl/tools/skills/docs/MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` when ADL expects
a structured artifact.

## Stop Boundary

Stop after producing the synthesis artifact.

Do not:
- edit code, tests, docs, or configs
- hide severity, disagreement, or missing specialist coverage
- claim approval, merge readiness, or remediation completion
- run additional specialist reviews unless the operator explicitly asks
