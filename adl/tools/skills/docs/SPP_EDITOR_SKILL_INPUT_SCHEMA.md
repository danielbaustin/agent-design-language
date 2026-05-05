# SPP Editor Skill Input Schema

```yaml
skill_input_schema: spp_editor.v1
mode: normalize_spp | tighten_for_review | repair_plan_drift
repo_root: /absolute/path
target:
  spp_path: /absolute/or/repo-relative/path/to/spp.md
  issue_number: <u32 or null>
  source_prompt_path: <path or null>
  linked_stp_path: <path or null>
  linked_sip_path: <path or null>
policy:
  preserve_planning_truth: true
  preserve_manual_schema_shape: true
  preserve_manual_markdown_shape: true
  stop_after_edit: true
  allow_execution_claims_without_evidence: false
```
