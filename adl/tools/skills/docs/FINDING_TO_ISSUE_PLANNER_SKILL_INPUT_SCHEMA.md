# Finding To Issue Planner Skill Input Schema

Schema id: `finding_to_issue_planner.v1`

This schema describes structured input for the `finding-to-issue-planner`
skill. The skill drafts issue candidates from review findings and stops before
tracker creation or repository mutation.

## Required Top-Level Fields

- `skill_input_schema`: must be `finding_to_issue_planner.v1`.
- `mode`: one of the supported modes below.
- `finding_source`: review artifact path, synthesis report path, or packet root.
- `policy`: explicit approval and mutation-boundary policy.

## Supported Modes

- `plan_from_review`: plan candidates from one specialist review artifact.
- `plan_from_synthesis`: plan candidates from a synthesis or final report.
- `plan_from_packet`: plan candidates from a packet root containing review
  artifacts.
- `refresh_issue_plan`: refresh an existing candidate plan after review edits.

## Policy Fields

- `approval_required`: must be true.
- `tracker_creation_allowed`: must be false for this skill.
- `grouping_policy`: exact, conservative, or none.
- `severity_floor`: P0, P1, P2, or P3.
- `preserve_specialist_disagreement`: must be true.
- `stop_before_mutation`: must be true.

## Optional Fields

- `artifact_root`: output directory for `issue_candidates.md` and
  `issue_candidates.json`.
- `tracker`: target tracker name for wording, such as GitHub.
- `existing_plan_path`: required for `refresh_issue_plan`.
- `dependency_links`: known upstream/downstream issue or finding links.

## Output Contract

The skill emits candidate-only artifacts:

- `issue_candidates.md`
- `issue_candidates.json`

The skill must not create tracker items, pull requests, remediation branches,
tests, fixes, or customer-facing reports.
