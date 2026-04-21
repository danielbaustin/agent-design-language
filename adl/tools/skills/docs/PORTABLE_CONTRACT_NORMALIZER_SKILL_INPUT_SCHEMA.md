# Portable Contract Normalizer Skill Input Schema

Schema id: `portable_contract_normalizer.v1`

Use this schema when invoking `portable-contract-normalizer` with structured
input.

## Required Top-Level Fields

- `skill_input_schema`: must be `portable_contract_normalizer.v1`
- `mode`: one of `scan_contracts`, `scan_and_apply_safe_fixes`, or
  `inspect_normalizer_report`
- `target`: bounded paths or report path
- `policy`: scan and safe-fix policy

## Target Object

Recommended fields:

- `paths`: bounded files or directories to scan
- `report_path`: existing report for inspect mode
- `operator_approval`: required for `scan_and_apply_safe_fixes`

## Policy Object

Recommended fields:

- `write_report`: boolean
- `apply_safe_fixes`: boolean
- `bounded_paths`: explicit path list
- `preserve_legitimate_host_evidence`: must be true
- `stop_before_broad_rewrite`: must be true
- `stop_before_design_decision`: must be true

## Example

```yaml
skill_input_schema: portable_contract_normalizer.v1
mode: scan_contracts
target:
  paths:
    - adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh
    - adl/tools/skills/repo-packet-builder
policy:
  write_report: true
  apply_safe_fixes: false
  bounded_paths:
    - adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh
    - adl/tools/skills/repo-packet-builder
  preserve_legitimate_host_evidence: true
  stop_before_broad_rewrite: true
  stop_before_design_decision: true
```

## Output

Default output root:

```text
.adl/reviews/portable-contract-normalizer/<run_id>/
```

Required artifacts:

- `portable_contract_normalizer_report.md`
- `portable_contract_normalizer_report.json`

The report must distinguish safe mechanical normalization from design decisions.
It must not broadly rewrite tests, hide legitimate evidence, or claim portability
outside the bounded scan.

