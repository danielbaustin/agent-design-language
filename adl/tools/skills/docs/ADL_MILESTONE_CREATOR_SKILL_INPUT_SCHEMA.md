# ADL Milestone Creator Skill Input Schema

```yaml
skill_input_schema: adl_milestone_creator.v1
mode: create_milestone_package | create_bridge_milestone | split_or_reallocate_milestone | review_milestone_creation_plan
repo_root: /absolute/path
milestone:
  source_version: v0.91.4 | null
  target_version: v0.91.5
  downstream_version: v0.92 | null
  purpose: <bounded milestone purpose>
  setup_issue_number: <u32 or null>
  moved_issue_numbers:
    - <u32>
  kept_issue_numbers:
    - <u32>
  planning_package_paths:
    - docs/milestones/<version>/README.md
  feature_tracks:
    - id: <track-id>
      title: <track title>
      feature_doc: docs/milestones/<version>/features/<feature>.md
      proof_surface: <bounded proof or validation surface>
  adr_candidates:
    - docs/architecture/adr/<candidate>.md
  activation_map_required: true | false
policy:
  require_workflow_conductor: true
  require_versioned_prompt_templates: true
  require_bound_worktree: true
  require_full_planning_package: true
  require_issue_migration_truth: true | false
  require_focused_docs_validation: true
  allow_runtime_validation: false
  stop_before_merge: true
```

## Required Behavior

- Use `workflow-conductor` as the mandatory front door for tracked issue setup
  and execution routing.
- Use prompt cards from `docs/templates/prompts/current.json`.
- Make `SIP`, `STP`, and `SPP` design-time ready before execution begins.
- Create the full milestone planning package unless the operator explicitly
  narrows scope.
- Keep moved work visible as moved, not abandoned.
- Keep downstream docs aligned with the new milestone handoff.
- Use focused docs/YAML/link/metadata validation for docs-only milestone setup.
- Stop before merge or release approval.
