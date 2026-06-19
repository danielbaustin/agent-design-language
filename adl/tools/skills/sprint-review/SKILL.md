---
name: sprint-review
description: Findings-first review orchestrator for one completed sprint, mini-sprint, issue wave, or release-tail bundle. Use when umbrella issue truth, child issue closure truth, PR state, lifecycle cards, validation evidence, closeout artifacts, and follow-up routing need one consistent review packet composed from the existing ADL review skills rather than a hand-assembled process.
---

# Sprint Review

Review one completed sprint, mini-sprint, issue wave, or release-tail bundle with a findings-first packet that preserves scope, evidence, lifecycle truth, validation status, closeout truth, and follow-up routing.
This is a findings-first review orchestrator for sprint-scoped review work, not a new execution engine or approval surface.

This skill composes the existing review capabilities. It is not a new execution engine, not a merge/close authority, and not a replacement for issue-level lifecycle or specialist review skills.

## Quick Start

1. Confirm the review scope: `sprint`, `mini_sprint`, `issue_wave`, or `release_tail`.
2. Gather the umbrella issue, ordered child issues, PR list, changed paths, validation evidence, lifecycle cards, review docs, and closeout artifacts.
3. Build or refresh the bounded review packet first.
4. Run the required review lanes for the actual changed surface.
5. Synthesize findings first, preserve residual risk and skipped lanes, and stop before remediation, merge approval, or sprint closure.

## Required Inputs

At minimum, gather:

- `repo_root`
- `mode`
- `target.scope`
- `target.umbrella_issue`
- `target.child_issues`
- `target.pr_urls`
- `target.changed_paths`
- `target.lifecycle_cards`
- `target.validation_evidence`
- `target.closeout_artifacts`
- `policy`

Useful additional inputs:

- `target.review_docs`
- `target.execution_packet_path`
- `target.activity_log_path`
- `target.release_or_milestone`
- `target.follow_up_issues`
- `policy.required_lanes`
- `policy.require_code_review_when_code_changed`
- `policy.require_closeout_truth`
- `policy.allow_review_subagent_exception`
- `output_root`
- `run_id`

If the umbrella issue, child issue list, or evidence bundle is materially incomplete, stop and report `blocked`.

## Supported Scopes

- `sprint`: one full sprint umbrella and its child wave
- `mini_sprint`: one bounded mini-sprint umbrella and its child wave
- `issue_wave`: one explicit non-sprint issue bundle reviewed as a set
- `release_tail`: one release-facing bundle where closeout and residual risk matter more than implementation sequencing

## Review Lanes

Always treat findings as the primary output. Select lanes according to the changed surface and declared review policy.

Required lane families:

- `gap_analysis`
  - Compare intended sprint scope against observed implementation, docs, validation, review, and closeout evidence.
- `code`
  - Required when sprint work includes code changes.
  - Use `repo-review-code` or `repo-code-review` according to scope.
- `docs`
  - Review reviewer-facing docs, packets, milestone truth, runbooks, demos, and public records touched by the sprint.
- `tests`
  - Review validation adequacy, missing proof, fragile assertions, path-policy misuse, and unreviewed test surfaces.
- `evidence_and_closeout`
  - Check umbrella closure truth, child issue closure truth, PR state, lifecycle-card truth, validation evidence, closeout packets, ignored/local-only evidence, and legacy tooling residue.
- `synthesis`
  - Merge the lane artifacts into one findings-first sprint review.
- `review_quality`
  - Evaluate whether the final packet is honest, evidence-bound, and usable.

Optional lane families when the sprint surface warrants them:

- `security`
- `architecture`
- `dependency`
- `release_evidence`

## Recommended Composition

Preferred composition order:

1. `repo-packet-builder`
2. `gap-analysis`
3. `repo-review-code` or `repo-code-review` when code changed
4. `repo-review-docs`
5. `repo-review-tests`
6. `redaction-and-evidence-auditor` when publication or public records are involved
7. `repo-review-synthesis`
8. `review-quality-evaluator`
9. `finding-to-issue-planner` only after review findings exist and only when explicit follow-up issue planning is wanted

Use `release-evidence` when the sprint review is also serving a milestone or release proof surface.

## Required Checks

The review must explicitly check:

- umbrella issue state and whether it is truthfully still open or safely closable
- child issue state and whether each child is actually closed or only appears complete locally
- PR state for each child where applicable
- lifecycle cards, especially stale `SRP`/`SOR` truth and missing closeout normalization
- validation evidence and what it actually proves
- closeout artifacts and whether they are retained, truthful, and on tracked paths
- ignored or local-only artifacts that would make the sprint look more complete than `main`
- legacy or superseded tooling evidence presented as if it were canonical
- unreviewed code, test, or docs surfaces that changed during the sprint

## Follow-up Routing

Keep the review packet findings-first. Only create or propose follow-up issues when the review policy or operator explicitly wants issue planning.

Use follow-up issues when:

- the fix is out of scope for the reviewed sprint
- the finding is real but should land later
- the sprint can close with explicit residual risk and routed cleanup

Keep findings in the review packet without issue creation when:

- the purpose is review truth rather than planning
- the finding still needs operator judgment
- the remediation should happen immediately in the active issue/PR instead of via backlog routing

## Output Expectations

Default output should include:

- findings first, ordered by severity
- scope and evidence summary
- coverage matrix for the review lanes used and skipped
- lifecycle and closeout truth summary
- validation adequacy summary
- residual risk and non-goals
- recommended follow-up routing
- non-claims covering merge, closure, and remediation authority boundaries

Use the detailed output shape in `references/output-contract.md`.

## Example Invocation

Use this skill after a mini-sprint has merged and closeout evidence exists:

```yaml
skill_input_schema: sprint_review.v1
mode: review_completed_bundle
repo_root: /absolute/path/to/repo
target:
  scope: mini_sprint
  umbrella_issue: 4069
  child_issues: [4076, 4077, 3927, 4074]
  pr_urls:
    - https://github.com/danielbaustin/agent-design-language/pull/4104
    - https://github.com/danielbaustin/agent-design-language/pull/4112
  changed_paths:
    - docs/milestones/v0.91.6/review/sprint_execution_packets/
    - adl/tools/skills/sprint-conductor/
  lifecycle_cards:
    - .adl/v0.91.6/tasks/issue-4076__v0-91-6-sep-sprints-automate-sprint-readiness-sweep/
    - .adl/v0.91.6/tasks/issue-4077__v0-91-6-sep-sprints-add-deterministic-sprint-closeout-mode/
  validation_evidence:
    - adl/tools/test_sprint_conductor_helpers.sh
    - adl/tools/test_install_adl_operational_skills.sh
  closeout_artifacts:
    - docs/milestones/v0.91.6/review/sprint_execution_packets/V0916_SEP_LOCAL_AGENT_ACCELERATION_MINI_SPRINT_4069.md
policy:
  required_lanes: [gap_analysis, code, docs, tests, evidence_and_closeout, synthesis, review_quality]
  require_code_review_when_code_changed: true
  require_closeout_truth: true
  stop_before_remediation: true
  stop_before_publication: true
```

## Stop Boundary

This skill must not:

- execute sprint work
- mutate implementation code as part of review
- claim merge approval or sprint closure approval
- hide stale cards, missing closeout truth, or missing validation
- silently create follow-up issues without explicit policy or operator approval

Handoff candidates:

- `repo-packet-builder`
- `gap-analysis`
- `repo-review-code`
- `repo-review-docs`
- `repo-review-tests`
- `repo-review-synthesis`
- `review-quality-evaluator`
- `finding-to-issue-planner`
- `release-evidence`
