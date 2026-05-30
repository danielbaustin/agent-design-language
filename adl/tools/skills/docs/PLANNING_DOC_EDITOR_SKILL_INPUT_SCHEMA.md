# Planning Doc Editor Skill Input Schema

```yaml
skill_input_schema: planning_doc_editor.v1
mode: placeholder_cleanup | required_section_repair | status_truth_normalization | planning_packet_review_cleanup | template_contract_alignment | portable_path_cleanup
repo_root: /absolute/path
target:
  target_path: docs/milestones/<version>/<doc>.md
  target_paths:
    - docs/milestones/<version>/<doc>.md
  planning_doc_type: README | WBS | SPRINT | VISION | FEATURE_DOC | DESIGN | DECISIONS | DEMO_MATRIX | MILESTONE_CHECKLIST | RELEASE_PLAN | RELEASE_NOTES | HANDOFF | planning_doc_unknown
  template_version: <version or null>
  template_registry_path: docs/templates/planning/current.json | null
  issue_number: <u32 or null>
  milestone: <version or null>
  source_issue_prompt: <repo-relative path or null>
  review_findings: <bounded finding summary or null>
  status_truth: generated | draft | reviewed | approved | historical | unknown
  validation_command: <focused command or null>
policy:
  bounded_target_required: true
  source_evidence_required: true
  no_card_edits: true
  no_release_approval: true
  stop_before_publication: true
```

## Required Behavior

- Edit only the bounded planning document target or target packet.
- Preserve the difference between generated, reviewed, and approved docs.
- Demote unsupported release, PR, review, or closeout claims.
- Route lifecycle-card defects to the appropriate card editor skill instead of
  editing `SIP`, `STP`, `SPP`, `SRP`, or `SOR`.
- Run the smallest focused validation that proves the planning-doc edit.
- Stop before PR publication, issue closeout, release approval, or broad
  repo-wide rewrite.
