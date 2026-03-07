# ADL Deterministic Card Review Output Format

Status: Draft (v0.8 tooling)
Applies to: ADL output-card review artifacts
Primary issue: #651
Depends on: `docs/tooling/card-review-checklist.md` (#650)

## Purpose

Define the canonical, deterministic, machine-readable output artifact produced by card reviewers.

This format is the standard surface for:
- human-readable review summaries
- Card Reviewer GPT outputs (#649)
- future CI parsing and gating

## Format Contract

- Serialization: YAML
- Encoding: UTF-8
- Top-level key order: fixed (as specified below)
- Enumerations: constrained values only
- Optional sections: explicit and nullable where defined
- Paths: repository-relative only

## Canonical Top-Level Schema

Top-level key order MUST be:

1. `review_format_version`
2. `review_metadata`
3. `review_target`
4. `decision`
5. `summary`
6. `domain_results`
7. `findings`
8. `acceptance_criteria`
9. `determinism_checks`
10. `security_privacy_checks`
11. `artifact_checks`
12. `validation_checks`
13. `follow_ups`

## Field Definitions

### `review_format_version`

- type: string
- required
- fixed initial value: `card_review_output.v1`

### `review_metadata`

- type: map
- required fields:
  - `reviewer`: string (`human` | versioned reviewer id, e.g. `card_reviewer_gpt.v1`)
  - `review_time_utc`: string (ISO-8601 UTC)
  - `checklist_version`: string (for #650 checklist, use `card_review_checklist.v1`)

### `review_target`

- type: map
- required fields:
  - `issue`: string (e.g., `#660`)
  - `output_card_path`: string (repo-relative)
  - `input_card_path`: string (repo-relative)
  - `pr`: string or null

### `decision`

- type: string
- required
- enum:
  - `PASS`
  - `MINOR_FIXES`
  - `MAJOR_ISSUES`

Decision must be derived using #650 deterministic decision mapping.

### `summary`

- type: map
- required fields:
  - `status_line`: short deterministic text summary
  - `failed_rule_count`: integer
  - `failed_rule_ids`: ordered list of rule IDs

### `domain_results`

- type: ordered list
- required
- one item per fixed domain in this order:
  - `structure`
  - `acceptance`
  - `determinism`
  - `security_privacy`
  - `artifacts`
  - `validation`

Each item fields:
- `domain`: string
- `status`: enum (`PASS` | `FAIL` | `PARTIAL`)
- `failed_rules`: ordered list of checklist rule IDs

### `findings`

- type: ordered list
- required (empty list allowed)
- each finding fields:
  - `rule_id`: checklist rule id
  - `severity`: enum (`blocker` | `high` | `medium` | `low`)
  - `title`: short deterministic summary
  - `evidence`: ordered list of evidence pointers (`path:...`, `command:...`, `ci:...`)
  - `remediation`: deterministic action statement

### `acceptance_criteria`

- type: ordered list
- required
- each item fields:
  - `criterion`: exact criterion text or stable criterion ID
  - `status`: enum (`met` | `not_met` | `partial`)
  - `evidence`: ordered list

### `determinism_checks`

- type: map
- required fields:
  - `ordering_contract_verified`: boolean
  - `replay_impact`: enum (`unchanged` | `changed` | `not_applicable`)
  - `notes`: string

### `security_privacy_checks`

- type: map
- required fields:
  - `secrets_present`: boolean
  - `raw_prompt_or_tool_args_present`: boolean
  - `absolute_host_paths_present`: boolean
  - `notes`: string

### `artifact_checks`

- type: map
- required fields:
  - `required_artifacts_present`: boolean
  - `schema_change_present`: boolean
  - `schema_change_status`: enum (`approved` | `rejected` | `not_applicable`)
  - `notes`: string

### `validation_checks`

- type: map
- required fields:
  - `required_commands_run`: boolean
  - `validation_result`: enum (`PASS` | `FAIL` | `PARTIAL`)
  - `commands`: ordered list of command strings
  - `notes`: string

### `follow_ups`

- type: ordered list
- required (empty list allowed)
- each item fields:
  - `type`: enum (`fix_now` | `defer` | `new_issue`)
  - `summary`: string
  - `reference`: string or null

## Determinism Rules

- Field names and top-level order are fixed.
- Domain order is fixed.
- Findings order must be deterministic: by severity (`blocker`, `high`, `medium`, `low`), then `rule_id` ascending, then lexicographic `title`.
- Lists of rule IDs must be lexicographically sorted unless domain order is explicitly required.
- No host-specific or environment-specific values outside normalized fields.

## Normative Example

See:
- `docs/tooling/examples/card-review-output-example.yaml`

This example is normative for structure and field ordering.

## Compatibility Notes

- Reviewers MAY include additional keys only under a future version (`card_review_output.v2+`).
- v1 parsers should reject unknown top-level keys to prevent silent schema drift.
